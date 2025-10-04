//! Layout components

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A LinearLayout arranges views linearly
pub struct LinearLayout {
    view: View,
    #[allow(dead_code)]
    aid: i64,
}

impl LinearLayout {
    /// Create a new vertical LinearLayout
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        Self::new_with_orientation(activity, parent, true)
    }
    
    /// Create a new LinearLayout with specified orientation
    /// 
    /// # Arguments
    /// * `vertical` - If true, arranges children vertically; if false, horizontally
    pub fn new_with_orientation(activity: &mut Activity, parent: Option<i64>, vertical: bool) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "vertical": vertical
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createLinearLayout",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(LinearLayout {
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

/// A NestedScrollView provides scrolling capability
pub struct NestedScrollView {
    view: View,
    #[allow(dead_code)]
    aid: i64,
}

impl NestedScrollView {
    /// Create a new NestedScrollView
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "nobar": false,
            "snapping": false
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createNestedScrollView",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(NestedScrollView {
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

/// A FrameLayout is a simple layout that stacks children on top of each other
pub struct FrameLayout {
    view: View,
    #[allow(dead_code)]
    aid: i64,
}

impl FrameLayout {
    /// Create a new FrameLayout
    /// 
    /// Children are drawn in the order they are added, with the last child on top.
    /// FrameLayout is useful for overlaying views or creating simple stacked layouts.
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createFrameLayout",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(FrameLayout {
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
