//! ProgressBar component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A ProgressBar displays progress from 0 to 100
pub struct ProgressBar {
    view: View,
    aid: i64,
}

impl ProgressBar {
    /// Create a new ProgressBar
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createProgressBar",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(ProgressBar {
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
    
    /// Set the progress value (0-100)
    /// 
    /// The progress must be an integer from 0 to 100 (inclusive).
    pub fn set_progress(&self, activity: &mut Activity, progress: i32) -> Result<()> {
        // Clamp progress to valid range
        let progress = progress.clamp(0, 100);
        
        activity.send(&json!({
            "method": "setProgress",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "progress": progress
            }
        }))?;
        Ok(())
    }
}
