//! Spinner (dropdown) component
//!
//! ## Important Notes on Event Handling
//!
//! When handling `itemselected` events from a Spinner:
//! - The event's `"selected"` field contains the **selected text as a string**, not an index
//! - You should match against the string value, not the index position
//! - Example event structure: `{"aid": 0, "id": 123, "selected": "Option 2"}`
//!
//! ## Example
//!
//! ```rust,no_run
//! use termux_gui::{Activity, Result};
//! use termux_gui::connection::read_message;
//!
//! # fn main() -> Result<()> {
//! let mut activity = Activity::new(false)?;
//! let layout = activity.create_linear_layout(None)?;
//!
//! let spinner = activity.create_spinner(Some(layout.id()))?;
//! spinner.set_list(&mut activity, &["Option 1", "Option 2", "Option 3"])?;
//!
//! // Event handling - use string matching, not index!
//! loop {
//!     let event = read_message(activity.event_stream())?;
//!     if event["type"] == "itemselected" {
//!         let selected_text = event["value"]["selected"].as_str().unwrap_or("");
//!         match selected_text {
//!             "Option 1" => println!("First option selected"),
//!             "Option 2" => println!("Second option selected"),
//!             _ => {}
//!         }
//!     }
//! }
//! # }
//! ```

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A Spinner is a dropdown list
///
/// **Important**: When handling `itemselected` events, the `"selected"` field 
/// contains the selected text as a string, not an index number.
pub struct Spinner {
    view: View,
    aid: i64,
}

impl Spinner {
    /// Create a new Spinner
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createSpinner",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(Spinner {
            view: View::new(id),
            aid: activity.id(),
        })
    }
    
    /// Get the view ID
    pub fn id(&self) -> i64 {
        self.view.id()
    }
    
    /// Get the underlying View
    pub fn view(&self) -> &View {
        &self.view
    }
    
    /// Set the list of options
    pub fn set_list(&self, activity: &mut Activity, items: &[&str]) -> Result<()> {
        activity.send(&json!({
            "method": "setList",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "list": items
            }
        }))?;
        Ok(())
    }
    
    /// Select an item by index
    pub fn select_item(&self, activity: &mut Activity, index: i32) -> Result<()> {
        activity.send(&json!({
            "method": "selectItem",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "item": index
            }
        }))?;
        Ok(())
    }
    
    /// Refresh the spinner (needed after setList to ensure display is updated)
    pub fn refresh(&self, activity: &mut Activity) -> Result<()> {
        activity.send(&json!({
            "method": "refreshSpinner",
            "params": {
                "aid": self.aid,
                "id": self.view.id()
            }
        }))?;
        Ok(())
    }
}
