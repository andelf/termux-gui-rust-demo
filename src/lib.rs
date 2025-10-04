//! # Termux GUI Rust Bindings
//! 
//! This library provides Rust bindings for Termux:GUI, allowing you to create
//! native Android GUI applications using Rust in the Termux environment.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use termux_gui::{Activity, Result};
//!
//! fn main() -> Result<()> {
//!     // Create an activity (dialog mode)
//!     let mut activity = Activity::new(true)?;
//!     
//!     // Create a layout
//!     let layout = activity.create_linear_layout(None)?;
//!     
//!     // Add a title
//!     let mut title = activity.create_text_view("Hello Termux!", Some(layout.id()))?;
//!     title.set_text_size(&mut activity, 24)?;
//!     
//!     // Add a button
//!     let button = activity.create_button("Click Me", Some(layout.id()))?;
//!     
//!     // Event loop
//!     use std::io::Read;
//!     let mut buf = [0u8; 1024];
//!     loop {
//!         let n = activity.event_stream().read(&mut buf)?;
//!         if n > 0 {
//!             let event: serde_json::Value = serde_json::from_slice(&buf[..n])?;
//!             if event["type"] == "UserLeave" {
//!                 break;
//!             }
//!         }
//!     }
//!     
//!     activity.finish()?;
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! - **Connection**: Low-level socket communication with Termux GUI service
//! - **Activity**: Represents a GUI window (dialog or full-screen)
//! - **View**: Base view type with common operations
//! - **Components**: UI widgets (TextView, Button, EditText, etc.)
//!
//! ## Features
//!
//! - Object-oriented API design
//! - Type-safe error handling with `thiserror`
//! - Zero-cost abstractions using lifetimes
//! - All Termux GUI components supported

pub mod connection;
pub mod activity;
pub mod view;
pub mod components;
pub mod error;

// Re-exports for convenience
pub use connection::Connection;
pub use activity::Activity;
pub use view::{View, MATCH_PARENT, WRAP_CONTENT};
pub use error::{GuiError, Result};

// Re-export all components
pub use components::{
    TextView, Button, EditText, Checkbox, Switch,
    RadioButton, RadioGroup, Spinner,
    LinearLayout, NestedScrollView, FrameLayout, GridLayout,
    HorizontalScrollView, SwipeRefreshLayout, TabLayout,
    ImageView, ProgressBar, ToggleButton, Space, WebView,
};
