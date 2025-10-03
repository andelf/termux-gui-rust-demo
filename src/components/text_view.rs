//! TextView component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A TextView displays text to the user
pub struct TextView {
    view: View,
    aid: i64,
}

impl TextView {
    /// Create a new TextView
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
            "method": "createTextView",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(TextView {
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
    
    /// Set the text content
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
    
    /// Set text size
    pub fn set_text_size(&self, activity: &mut Activity, size: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setTextSize",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "size": size
            }
        }))?;
        Ok(())
    }
    
    /// Set text color (ARGB format)
    pub fn set_text_color(&self, activity: &mut Activity, color: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setTextColor",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "color": color
            }
        }))?;
        Ok(())
    }
}
