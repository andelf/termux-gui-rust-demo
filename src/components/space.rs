//! Space component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A Space creates empty space in layouts
/// 
/// Space is useful for creating gaps between UI elements or pushing elements apart.
/// You can control its size using width, height, and layout params.
pub struct Space {
    view: View,
    aid: i64,
}

impl Space {
    /// Create a new Space
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createSpace",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(Space {
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
