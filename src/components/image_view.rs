//! ImageView component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// An ImageView displays images
pub struct ImageView {
    view: View,
    aid: i64,
}

impl ImageView {
    /// Create a new ImageView
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createImageView",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(ImageView {
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
    
    /// Set image from base64 encoded string
    /// 
    /// The image should be base64 encoded PNG or JPEG data.
    /// You can use the `base64` crate to encode image bytes.
    pub fn set_image(&self, activity: &mut Activity, img_base64: &str) -> Result<()> {
        activity.send(&json!({
            "method": "setImage",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "img": img_base64
            }
        }))?;
        Ok(())
    }
    
    /// Refresh the ImageView
    /// 
    /// This redraws the ImageView. Needed when using shared buffers.
    pub fn refresh(&self, activity: &mut Activity) -> Result<()> {
        activity.send(&json!({
            "method": "refreshImageView",
            "params": {
                "aid": self.aid,
                "id": self.view.id()
            }
        }))?;
        Ok(())
    }
}
