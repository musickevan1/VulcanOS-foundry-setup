//! Epoll-based event loop for VulcanBar
//!
//! Handles multiplexing of input events, config changes, and timers.

use anyhow::{Context, Result};
use nix::{
    errno::Errno,
    sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags},
};
use std::os::fd::AsFd;
use std::time::{Duration, Instant};

/// Event source identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSource {
    /// Main keyboard input (for Fn key)
    MainKeyboard = 0,
    /// Touch bar digitizer input
    TouchBar = 1,
    /// Configuration file changes
    Config = 2,
    /// Power supply changes (for battery)
    PowerSupply = 3,
    /// Module update channel
    ModuleChannel = 4,
}

impl From<u64> for EventSource {
    fn from(value: u64) -> Self {
        match value {
            0 => EventSource::MainKeyboard,
            1 => EventSource::TouchBar,
            2 => EventSource::Config,
            3 => EventSource::PowerSupply,
            4 => EventSource::ModuleChannel,
            _ => EventSource::MainKeyboard,
        }
    }
}

/// Event loop manages epoll-based event multiplexing
pub struct EventLoop {
    epoll: Epoll,
    /// Last time modules were updated
    last_module_update: Instant,
    /// Minimum module update interval
    module_interval: Duration,
    /// Timeout for epoll wait (milliseconds)
    timeout_ms: i32,
}

impl EventLoop {
    /// Create a new event loop
    pub fn new() -> Result<Self> {
        let epoll = Epoll::new(EpollCreateFlags::empty())
            .context("Failed to create epoll instance")?;

        Ok(Self {
            epoll,
            last_module_update: Instant::now(),
            module_interval: Duration::from_secs(1),
            timeout_ms: 10_000, // 10 second default timeout
        })
    }

    /// Register a file descriptor with the event loop
    pub fn register<F: AsFd>(&self, fd: &F, source: EventSource) -> Result<()> {
        self.epoll
            .add(fd, EpollEvent::new(EpollFlags::EPOLLIN, source as u64))
            .context("Failed to register fd with epoll")?;
        Ok(())
    }

    /// Set the module update interval
    pub fn set_module_interval(&mut self, interval: Duration) {
        self.module_interval = interval;
        self.timeout_ms = interval.as_millis().min(10_000) as i32;
    }

    /// Wait for events with timeout
    /// Returns the event sources that are ready
    pub fn wait(&mut self) -> Result<Vec<EventSource>> {
        let mut events = [EpollEvent::new(EpollFlags::empty(), 0); 8];

        // Calculate timeout based on next module update
        let elapsed = self.last_module_update.elapsed();
        let timeout = if elapsed >= self.module_interval {
            0 // Update immediately
        } else {
            let remaining = self.module_interval - elapsed;
            remaining.as_millis().min(self.timeout_ms as u128) as i32
        };

        match self.epoll.wait(&mut events, timeout as u16) {
            Ok(count) => {
                let sources: Vec<EventSource> = events[..count]
                    .iter()
                    .map(|e| EventSource::from(e.data()))
                    .collect();
                Ok(sources)
            }
            Err(Errno::EINTR) => Ok(vec![]), // Interrupted, no events
            Err(e) => Err(e).context("epoll_wait failed"),
        }
    }

    /// Check if modules should be updated based on interval
    pub fn should_update_modules(&mut self) -> bool {
        if self.last_module_update.elapsed() >= self.module_interval {
            self.last_module_update = Instant::now();
            true
        } else {
            false
        }
    }

    /// Calculate next timeout in milliseconds
    pub fn next_timeout_ms(&self) -> i32 {
        let elapsed = self.last_module_update.elapsed();
        if elapsed >= self.module_interval {
            0
        } else {
            let remaining = self.module_interval - elapsed;
            remaining.as_millis().min(10_000) as i32
        }
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new().expect("Failed to create event loop")
    }
}
