//! ToggleButton component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A ToggleButton is a button that can be toggled on or off
/// 
/// Similar to Switch, but with a different visual style (button appearance).
pub struct ToggleButton {
    view: View,
    aid: i64,
}

impl ToggleButton {
    /// Create a new ToggleButton (unchecked by default)
    pub fn new(activity: &mut Activity, text: &str, parent: Option<i64>) -> Result<Self> {
        Self::new_with_checked(activity, text, parent, false)
    }
    
    /// Create a new ToggleButton with specified checked state
    pub fn new_with_checked(activity: &mut Activity, text: &str, parent: Option<i64>, checked: bool) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "text": text,
            "checked": checked
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createToggleButton",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(ToggleButton {
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
    
    /// Set the toggle button text
    pub fn set_text(&self, activity: &mut Activity, text: &str) -> Result<()> {
        activity.send(&json!({
            "method": "setText",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "text": text
            }
        }))?;
        Ok(())
    }
    
    /// Set checked state
    pub fn set_checked(&self, activity: &mut Activity, checked: bool) -> Result<()> {
        activity.send(&json!({
            "method": "setChecked",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "checked": checked
            }
        }))?;
        Ok(())
    }
}
