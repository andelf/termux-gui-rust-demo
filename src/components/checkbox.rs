//! Checkbox component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A Checkbox can be checked or unchecked
pub struct Checkbox {
    view: View,
    aid: i64,
}

impl Checkbox {
    /// Create a new Checkbox with optional initial checked state
    pub fn new(activity: &mut Activity, text: &str, parent: Option<i64>) -> Result<Self> {
        Self::new_with_checked(activity, text, parent, false)
    }
    
    /// Create a new Checkbox with specified checked state
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
            "method": "createCheckbox",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(Checkbox {
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
    
    /// Set the checkbox text
    pub fn set_text(&self, activity: &mut Activity, text: &str) -> Result<()> {
        activity.send_read(&json!({
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
        activity.send_read(&json!({
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
