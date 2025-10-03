//! Spinner (dropdown) component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A Spinner is a dropdown list
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
        activity.send_read(&json!({
            "method": "setList",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "list": items
            }
        }))?;
        Ok(())
    }
    
    /// Refresh the spinner (needed after setList to ensure display is updated)
    pub fn refresh(&self, activity: &mut Activity) -> Result<()> {
        activity.send_read(&json!({
            "method": "refreshSpinner",
            "params": {
                "aid": self.aid,
                "id": self.view.id()
            }
        }))?;
        Ok(())
    }
}
