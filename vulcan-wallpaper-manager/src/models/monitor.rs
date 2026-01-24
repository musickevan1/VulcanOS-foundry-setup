use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub width: u32,      // Physical resolution
    pub height: u32,     // Physical resolution
    pub x: i32,          // Position in layout
    pub y: i32,          // Position in layout
    pub scale: f64,      // HiDPI scaling factor
    pub transform: u32,  // Rotation: 0=normal, 1=90deg, etc.
    pub focused: bool,
}

impl Monitor {
    /// Returns logical dimensions (physical / scale)
    pub fn logical_size(&self) -> (f64, f64) {
        (self.width as f64 / self.scale, self.height as f64 / self.scale)
    }

    /// Returns true if monitor is rotated 90 or 270 degrees
    pub fn is_vertical(&self) -> bool {
        self.transform == 1 || self.transform == 3
    }
}
