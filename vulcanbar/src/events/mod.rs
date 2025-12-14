//! Event handling for VulcanBar
//!
//! Provides event multiplexing and the main event loop infrastructure.

pub mod epoll;

pub use epoll::EventLoop;
