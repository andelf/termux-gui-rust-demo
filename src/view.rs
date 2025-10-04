//! Base View type and common view operations

use serde_json::json;
use crate::activity::Activity;
use crate::error::Result;

/// Special dimension constants for Android layouts
pub const MATCH_PARENT: i32 = -1;
pub const WRAP_CONTENT: i32 = -2;

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
        activity.send(&json!({
            "method": "setWidth",
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
        activity.send(&json!({
            "method": "setHeight",
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
        // Call both methods since there's no combined setDimensions
        self.set_width(activity, width)?;
        self.set_height(activity, height)?;
        Ok(())
    }
    
    /// Set view margin
    pub fn set_margin(&self, activity: &mut Activity, margin: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setMargin",
            "params": {
                "aid": activity.id(),
                "id": self.id,
                "margin": margin
            }
        }))?;
        Ok(())
    }
    
    /// Set view width to WRAP_CONTENT
    pub fn set_width_wrap_content(&self, activity: &mut Activity) -> Result<()> {
        self.set_width(activity, WRAP_CONTENT)
    }
    
    /// Set view height to WRAP_CONTENT
    pub fn set_height_wrap_content(&self, activity: &mut Activity) -> Result<()> {
        self.set_height(activity, WRAP_CONTENT)
    }
    
    /// Set view width to MATCH_PARENT
    pub fn set_width_match_parent(&self, activity: &mut Activity) -> Result<()> {
        self.set_width(activity, MATCH_PARENT)
    }
    
    /// Set view height to MATCH_PARENT
    pub fn set_height_match_parent(&self, activity: &mut Activity) -> Result<()> {
        self.set_height(activity, MATCH_PARENT)
    }
    
    /// Set LinearLayout parameters for this view
    /// 
    /// # Arguments
    /// * `activity` - The activity
    /// * `weight` - Layout weight (higher weight = more space)
    /// * `position` - Optional position index in the layout
    pub fn set_linear_layout_params(&self, activity: &mut Activity, weight: i32, position: Option<i32>) -> Result<()> {
        let mut params = json!({
            "aid": activity.id(),
            "id": self.id,
            "weight": weight
        });
        
        if let Some(pos) = position {
            params["position"] = json!(pos);
        }
        
        activity.send(&json!({
            "method": "setLinearLayoutParams",
            "params": params
        }))?;
        Ok(())
    }
}
