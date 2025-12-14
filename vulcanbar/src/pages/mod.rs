//! Multi-page system for VulcanBar
//!
//! Provides a Waybar-style multi-page layout where users can swipe
//! between different pages (Main, Media, System).

use anyhow::Result;
use cairo::ImageSurface;
use drm::control::ClipRect;
use std::time::{Duration, Instant};

use crate::config::{GeneralConfig, VulcanBarConfig};
use crate::display::Compositor;
use crate::gestures::Direction;
use crate::modules::{Action, ModuleRegistry};

/// A single page containing a compositor with its modules
pub struct Page {
    /// Page name (e.g., "main", "media", "system")
    pub name: String,
    /// Compositor managing modules on this page
    pub compositor: Compositor,
}

impl Page {
    /// Create a new page with the given name and compositor
    pub fn new(name: String, compositor: Compositor) -> Self {
        Self { name, compositor }
    }
}

/// State of an active page transition animation
#[derive(Debug, Clone)]
pub struct PageTransition {
    /// Index of the page we're transitioning from
    pub from_page: usize,
    /// Index of the page we're transitioning to
    pub to_page: usize,
    /// Direction of the transition
    pub direction: Direction,
    /// When the transition started
    pub start_time: Instant,
    /// Duration of the transition
    pub duration: Duration,
}

impl PageTransition {
    /// Create a new page transition
    pub fn new(from: usize, to: usize, direction: Direction, duration_ms: u64) -> Self {
        Self {
            from_page: from,
            to_page: to,
            direction,
            start_time: Instant::now(),
            duration: Duration::from_millis(duration_ms),
        }
    }

    /// Get the progress of the transition (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let duration = self.duration.as_secs_f64();
        (elapsed / duration).min(1.0)
    }

    /// Check if the transition is complete
    pub fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }

    /// Get the X offset for rendering based on transition progress
    /// Returns (from_offset, to_offset) in pixels
    pub fn get_offsets(&self, width: i32) -> (f64, f64) {
        let progress = self.progress();
        let width_f = width as f64;

        match self.direction {
            Direction::Left => {
                // Swiping left = going to next page (page slides left)
                let from_offset = -width_f * progress;
                let to_offset = width_f * (1.0 - progress);
                (from_offset, to_offset)
            }
            Direction::Right => {
                // Swiping right = going to previous page (page slides right)
                let from_offset = width_f * progress;
                let to_offset = -width_f * (1.0 - progress);
                (from_offset, to_offset)
            }
        }
    }
}

/// Manages multiple pages and handles transitions between them
pub struct PageManager {
    /// All available pages
    pages: Vec<Page>,
    /// Index of the currently active page
    current_page: usize,
    /// Active transition (if any)
    transition: Option<PageTransition>,
    /// Display dimensions
    width: i32,
    height: i32,
    /// Transition duration in milliseconds
    transition_duration_ms: u64,
}

impl PageManager {
    /// Create a new PageManager with the given pages
    pub fn new(pages: Vec<Page>, width: i32, height: i32) -> Self {
        Self {
            pages,
            current_page: 0,
            transition: None,
            width,
            height,
            transition_duration_ms: 200,
        }
    }

    /// Create a PageManager from configuration
    pub fn from_config(
        config: &VulcanBarConfig,
        registry: &ModuleRegistry,
        width: i32,
        height: i32,
    ) -> Result<Self> {
        let mut pages = Vec::new();

        // Check if multi-page config exists
        if let Some(ref pages_config) = config.pages {
            tracing::info!("Multi-page mode: {} pages configured", pages_config.list.len());

            // Create pages from config
            for page_def in &pages_config.list {
                tracing::debug!(
                    "Creating page '{}' with layout: left={:?}, center={:?}, right={:?}",
                    page_def.name,
                    page_def.layout.left,
                    page_def.layout.center,
                    page_def.layout.right
                );
                let compositor = Compositor::from_layout_config(
                    &page_def.layout,
                    &config.general,
                    &config.modules,
                    registry,
                    width,
                    height,
                )?;
                pages.push(Page::new(page_def.name.clone(), compositor));
            }

            let mut manager = Self::new(pages, width, height);
            manager.transition_duration_ms = pages_config.transition_duration_ms;

            // Set default page
            if let Some(idx) = manager.find_page_index(&pages_config.default) {
                manager.current_page = idx;
                tracing::debug!("Default page set to: {} (index {})", pages_config.default, idx);
            }

            Ok(manager)
        } else {
            // Fallback: Create single page from layout config
            tracing::info!("Single-page mode: using [layout] section");
            tracing::debug!(
                "Layout: left={:?}, center={:?}, right={:?}",
                config.layout.left,
                config.layout.center,
                config.layout.right
            );
            let compositor = Compositor::from_config(config, registry, width, height)?;
            let pages = vec![Page::new("main".to_string(), compositor)];
            Ok(Self::new(pages, width, height))
        }
    }

    /// Find page index by name
    fn find_page_index(&self, name: &str) -> Option<usize> {
        self.pages.iter().position(|p| p.name == name)
    }

    /// Get the current page index
    pub fn current_page_index(&self) -> usize {
        self.current_page
    }

    /// Get the number of pages
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get the current page name
    pub fn current_page_name(&self) -> &str {
        &self.pages[self.current_page].name
    }

    /// Check if a transition is in progress
    pub fn is_transitioning(&self) -> bool {
        self.transition.is_some()
    }

    /// Get the target page name during a transition (or current if not transitioning)
    pub fn target_page_name(&self) -> &str {
        if let Some(ref transition) = self.transition {
            &self.pages[transition.to_page].name
        } else {
            &self.pages[self.current_page].name
        }
    }

    /// Get the target page index during a transition (or current if not transitioning)
    pub fn target_page_index(&self) -> usize {
        if let Some(ref transition) = self.transition {
            transition.to_page
        } else {
            self.current_page
        }
    }

    /// Switch to the next or previous page based on swipe direction
    pub fn switch_page(&mut self, direction: Direction) {
        // Don't start a new transition while one is in progress
        if self.is_transitioning() {
            return;
        }

        let target = match direction {
            Direction::Left => {
                // Swipe left = go to next page
                if self.current_page + 1 < self.pages.len() {
                    self.current_page + 1
                } else {
                    return; // Already at last page
                }
            }
            Direction::Right => {
                // Swipe right = go to previous page
                if self.current_page > 0 {
                    self.current_page - 1
                } else {
                    return; // Already at first page
                }
            }
        };

        self.transition = Some(PageTransition::new(
            self.current_page,
            target,
            direction,
            self.transition_duration_ms,
        ));
    }

    /// Switch to a specific page by index
    pub fn switch_to_page(&mut self, index: usize) {
        if index >= self.pages.len() || index == self.current_page {
            return;
        }

        let direction = if index > self.current_page {
            Direction::Left
        } else {
            Direction::Right
        };

        self.transition = Some(PageTransition::new(
            self.current_page,
            index,
            direction,
            self.transition_duration_ms,
        ));
    }

    /// Switch to a specific page by name
    /// Returns true if a transition was started, false if page not found or already on that page
    pub fn switch_to_page_by_name(&mut self, name: &str) -> bool {
        if let Some(index) = self.find_page_index(name) {
            if index != self.current_page && self.transition.is_none() {
                self.switch_to_page(index);
                return true;
            }
        } else {
            tracing::warn!(
                "Page '{}' not found. Available pages: {:?}",
                name,
                self.pages.iter().map(|p| &p.name).collect::<Vec<_>>()
            );
        }
        false
    }

    /// Update transition state, returns true if redraw needed
    pub fn update(&mut self) -> bool {
        if let Some(ref transition) = self.transition {
            if transition.is_complete() {
                self.current_page = transition.to_page;
                self.transition = None;
                return true;
            }
            return true; // Still animating
        }
        false
    }

    /// Update all modules on the current page
    pub fn update_modules(&mut self) -> Result<bool> {
        self.pages[self.current_page].compositor.update_modules()
    }

    /// Handle a touch event on the current page
    pub fn handle_touch(&mut self, x: f64, y: f64, pressed: bool, slot: i32) -> Option<Action> {
        // Don't handle touches during transitions
        if self.is_transitioning() {
            return None;
        }
        self.pages[self.current_page]
            .compositor
            .handle_touch(x, y, pressed, slot)
    }

    /// Render the current page (and transition if active)
    pub fn render(
        &self,
        surface: &ImageSurface,
        config: &GeneralConfig,
        pixel_shift: (f64, f64),
    ) -> Result<Vec<ClipRect>> {
        if let Some(ref transition) = self.transition {
            // During transition, render both pages with offsets
            tracing::debug!(
                "Rendering transition: {} -> {} (progress: {:.1}%)",
                self.pages[transition.from_page].name,
                self.pages[transition.to_page].name,
                transition.progress() * 100.0
            );
            self.render_transition(surface, config, pixel_shift, transition)
        } else {
            // Normal render of current page
            tracing::debug!("Rendering page: {}", self.pages[self.current_page].name);
            self.pages[self.current_page]
                .compositor
                .render(surface, config, pixel_shift, true)
        }
    }

    fn render_transition(
        &self,
        surface: &ImageSurface,
        config: &GeneralConfig,
        pixel_shift: (f64, f64),
        transition: &PageTransition,
    ) -> Result<Vec<ClipRect>> {
        // For now, just render the destination page
        // TODO: Implement smooth sliding animation
        self.pages[transition.to_page]
            .compositor
            .render(surface, config, pixel_shift, true)
    }

    /// Get minimum update interval across all pages
    pub fn min_update_interval(&self) -> Option<Duration> {
        self.pages
            .iter()
            .filter_map(|p| p.compositor.min_update_interval())
            .min()
    }
}
