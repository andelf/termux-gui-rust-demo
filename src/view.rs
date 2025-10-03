//! Base View type and common view operations

use serde_json::json;
use crate::activity::Activity;
use crate::error::Result;

/// Base View structure
pub struct View {
    id: i64,
}

impl View {
    /// Create a new View with the given ID
    pub fn new(id: i64) -> Self {
        View { id }
    }
    
    /// Get the view ID
    pub fn id(&self) -> i64 {
        self.id
    }
    
    /// Set view width
    pub fn set_width(&self, activity: &mut Activity, width: i32) -> Result<()> {
        activity.send_read(&json!({
            "method": "setDimensions",
            "params": {
                "aid": activity.id(),
                "id": self.id,
                "width": width
            }
        }))?;
        Ok(())
    }
    
    /// Set view height
    pub fn set_height(&self, activity: &mut Activity, height: i32) -> Result<()> {
        activity.send_read(&json!({
            "method": "setDimensions",
            "params": {
                "aid": activity.id(),
                "id": self.id,
                "height": height
            }
        }))?;
        Ok(())
    }
    
    /// Set view width and height
    pub fn set_dimensions(&self, activity: &mut Activity, width: i32, height: i32) -> Result<()> {
        activity.send_read(&json!({
            "method": "setDimensions",
            "params": {
                "aid": activity.id(),
                "id": self.id,
                "width": width,
                "height": height
            }
        }))?;
        Ok(())
    }
    
    /// Set view margin
    pub fn set_margin(&self, activity: &mut Activity, margin: i32) -> Result<()> {
        activity.send_read(&json!({
            "method": "setMargin",
            "params": {
                "aid": activity.id(),
                "id": self.id,
                "margin": margin
            }
        }))?;
        Ok(())
    }
}
