//! EditText component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// An EditText allows text input
pub struct EditText {
    view: View,
    aid: i64,
}

impl EditText {
    /// Create a new EditText
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
            "method": "createEditText",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(EditText {
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
    
    /// Set hint text
    pub fn set_hint(&self, activity: &mut Activity, hint: &str) -> Result<()> {
        activity.send_read(&json!({
            "method": "setHint",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "hint": hint
            }
        }))?;
        Ok(())
    }
}
