//! Radio button components

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A RadioButton in a group
pub struct RadioButton {
    view: View,
    aid: i64,
}

impl RadioButton {
    /// Create a new RadioButton
    pub fn new(activity: &mut Activity, text: &str, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "text": text
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createRadioButton",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(RadioButton {
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
    
    /// Set the radio button text
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

/// A RadioGroup manages a set of radio buttons
pub struct RadioGroup {
    view: View,
    #[allow(dead_code)]
    aid: i64,
}

impl RadioGroup {
    /// Create a new RadioGroup
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createRadioGroup",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(RadioGroup {
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
}
