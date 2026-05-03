// Detail screen scroll messages — whole-screen viewport scrolling.

/// Messages for scrolling the detail screen viewport.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DetailScrollMsg {
    ScrollDown,
    ScrollUp,
    PageDown,
    PageUp,
    GoToTop,
    GoToBottom,
}
