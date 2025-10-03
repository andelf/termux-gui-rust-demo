//! Button component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A Button that can be clicked
pub struct Button {
    view: View,
    aid: i64,
}

impl Button {
    /// Create a new Button
    pub fn new(activity: &mut Activity, text: &str, parent: Option<i64>) -> Result<Self> {
        let parent_id = parent.unwrap_or(activity.id());
        
        let response = activity.send_read(&json!({
            "method": "createButton",
            "params": {
                "aid": activity.id(),
                "parent": parent_id,
                "text": text
            }
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(Button {
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
    
    /// Set the button text
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
}
