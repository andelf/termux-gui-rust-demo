//! Base View type and common view operations
//!
//! ## Important Notes on Dimensions and Units
//!
//! Android uses two different units for View dimensions:
//! - **dp (density-independent pixels)**: Default unit, scales with screen density
//! - **px (pixels)**: Actual screen pixels
//!
//! ### Critical for TabLayout/ScrollView
//!
//! When working with `HorizontalScrollView` or `TabLayout`:
//! 1. `get_dimensions()` returns values in **pixels (px)**
//! 2. `set_width()` and `set_height()` use **dp** by default
//! 3. **You must use `set_width_px()` and `set_height_px()`** to match dimensions!
//!
//! ### Example
//!
//! ```rust,no_run
//! # use termux_gui::{Activity, Result};
//! # fn example(activity: &mut Activity, scroll_view_id: i64, page_id: i64) -> Result<()> {
//! // Get dimensions (returns pixels)
//! let (width_px, height_px) = scroll_view.view().get_dimensions(activity)?;
//!
//! // WRONG: This uses dp, not px!
//! // page.view().set_width(activity, width_px)?;  // Width mismatch!
//!
//! // CORRECT: Use set_width_px to match pixel values
//! page.view().set_width_px(activity, width_px)?;  // Perfect match!
//! # Ok(())
//! # }
//! ```

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
    
    /// Set view width in dp (density-independent pixels)
    /// 
    /// **Note**: If you need to match dimensions from `get_dimensions()`,
    /// use `set_width_px()` instead, as `get_dimensions()` returns pixels.
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
    
    /// Set view width in pixels
    /// 
    /// **Use this when matching dimensions from `get_dimensions()`**
    /// which returns pixel values, not dp values.
    pub fn set_width_px(&self, activity: &mut Activity, width: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setWidth",
            "params": {
                "aid": activity.id(),
                "id": self.id,
                "width": width,
                "px": true
            }
        }))?;
        Ok(())
    }
    
    /// Set view height in dp (density-independent pixels)
    /// 
    /// **Note**: If you need to match dimensions from `get_dimensions()`,
    /// use `set_height_px()` instead, as `get_dimensions()` returns pixels.
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
    
    /// Set view height in pixels
    /// 
    /// **Use this when matching dimensions from `get_dimensions()`**
    /// which returns pixel values, not dp values.
    pub fn set_height_px(&self, activity: &mut Activity, height: i32) -> Result<()> {
        activity.send(&json!({
            "method": "setHeight",
            "params": {
                "aid": activity.id(),
                "id": self.id,
                "height": height,
                "px": true
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
    
    /// Get the dimensions (width, height) of this view in pixels
    /// Returns (width, height)
    pub fn get_dimensions(&self, activity: &mut Activity) -> Result<(i32, i32)> {
        let response = activity.send_read(&json!({
            "method": "getDimensions",
            "params": {
                "aid": activity.id(),
                "id": self.id
            }
        }))?;
        
        // Response is an array [width, height]
        if let Some(arr) = response.as_array() {
            let width = arr.get(0).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let height = arr.get(1).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            Ok((width, height))
        } else {
            Ok((0, 0))
        }
    }
}
