pub enum Style {}

impl Style {
    /// Window styles
    pub const NONE: u32 = 0;
    pub const TITLEBAR: u32 = 1;
    pub const RESIZE: u32 = 2;
    pub const CLOSE: u32 = 4;
    pub const FULLSCREEN: u32 = 8;

    /// Default window style (combination of titlebar, resize, and close)
    pub const DEFAULT_STYLE: u32 = Self::TITLEBAR | Self::RESIZE | Self::CLOSE;
}
