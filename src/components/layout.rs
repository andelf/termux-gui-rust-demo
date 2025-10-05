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

/// A GridLayout arranges children in a grid
pub struct GridLayout {
    view: View,
    #[allow(dead_code)]
    aid: i64,
    #[allow(dead_code)]
    rows: i32,
    #[allow(dead_code)]
    cols: i32,
}

impl GridLayout {
    /// Create a new GridLayout with specified rows and columns
    /// 
    /// # Arguments
    /// * `rows` - Number of rows in the grid
    /// * `cols` - Number of columns in the grid
    pub fn new(activity: &mut Activity, rows: i32, cols: i32, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "rows": rows,
            "cols": cols
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createGridLayout",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(GridLayout {
            view: View::new(id),
            aid: activity.id(),
            rows,
            cols,
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

/// A HorizontalScrollView provides horizontal scrolling for content
///
/// ## Important Usage Notes for TabLayout
///
/// When using HorizontalScrollView with TabLayout for page switching:
///
/// 1. **Use pixel units**: Page widths must be set using `set_width_px()`, not `set_width()`
/// 2. **Match dimensions**: Use `get_dimensions()` to get screen width, then set each page width to match
/// 3. **Scroll position**: Calculate scroll position as `page_width * tab_index`
///
/// ### Example
///
/// ```rust,no_run
/// # use termux_gui::{Activity, Result};
/// # fn example(activity: &mut Activity, root_id: i64, content_id: i64) -> Result<()> {
/// // Create HorizontalScrollView with snapping and no scrollbar
/// let scroll = activity.create_horizontal_scroll_view_with_params(
///     Some(root_id), true, true, true
/// )?;
///
/// // Get width in pixels
/// let (page_width, _) = scroll.view().get_dimensions(activity)?;
///
/// // Create pages with pixel-based width
/// let page1 = activity.create_linear_layout(Some(content_id))?;
/// page1.view().set_width_px(activity, page_width)?;  // Use px, not dp!
///
/// // Scroll to page 2
/// scroll.set_scroll_position(activity, page_width * 2, 0, true)?;
/// # Ok(())
/// # }
/// ```
pub struct HorizontalScrollView {
    view: View,
    #[allow(dead_code)]
    aid: i64,
}

impl HorizontalScrollView {
    /// Create a new HorizontalScrollView
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "nobar": false,
            "snapping": false,
            "fillviewport": true  // Set to true to allow child views to fill the viewport
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createHorizontalScrollView",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(HorizontalScrollView {
            view: View::new(id),
            aid: activity.id(),
        })
    }
    
    /// Create a new HorizontalScrollView with custom parameters
    pub fn new_with_params(activity: &mut Activity, parent: Option<i64>, 
                          fillviewport: bool, snapping: bool, nobar: bool) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "nobar": nobar,
            "snapping": snapping,
            "fillviewport": fillviewport
        });
        
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createHorizontalScrollView",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(HorizontalScrollView {
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
    
    /// Get the scroll position (x, y) in pixels
    pub fn get_scroll_position(&self, activity: &mut Activity) -> Result<(i32, i32)> {
        let response = activity.send_read(&json!({
            "method": "getScrollPosition",
            "params": {
                "aid": self.aid,
                "id": self.view.id()
            }
        }))?;
        
        // Response is an array [x, y]
        if let Some(arr) = response.as_array() {
            let x = arr.get(0).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let y = arr.get(1).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            Ok((x, y))
        } else {
            Ok((0, 0))
        }
    }
    
    /// Set the scroll position
    /// 
    /// # Arguments
    /// * `x` - Horizontal scroll position in pixels
    /// * `y` - Vertical scroll position in pixels (usually 0 for HorizontalScrollView)
    /// * `smooth` - Whether to scroll smoothly or jump immediately
    pub fn set_scroll_position(&self, activity: &mut Activity, x: i32, y: i32, smooth: bool) -> Result<()> {
        activity.send(&json!({
            "method": "setScrollPosition",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "x": x,
                "y": y,
                "soft": smooth
            }
        }))?;
        Ok(())
    }
}

/// A SwipeRefreshLayout provides pull-to-refresh functionality
pub struct SwipeRefreshLayout {
    view: View,
    aid: i64,
}

impl SwipeRefreshLayout {
    /// Create a new SwipeRefreshLayout
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createSwipeRefreshLayout",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(SwipeRefreshLayout {
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
    
    /// Set whether the refresh animation is showing
    /// 
    /// Call with false after refresh is complete to stop the animation
    pub fn set_refreshing(&self, activity: &mut Activity, refreshing: bool) -> Result<()> {
        activity.send(&json!({
            "method": "setRefreshing",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "refresh": refreshing
            }
        }))?;
        Ok(())
    }
}

/// A TabLayout displays a horizontal row of tabs
/// 
/// TabLayout is useful for creating tabbed interfaces. It emits 'itemselected' 
/// events when a tab is clicked, with the tab index as the value.
pub struct TabLayout {
    view: View,
    aid: i64,
}

impl TabLayout {
    /// Create a new TabLayout
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        // Only set parent if explicitly provided
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createTabLayout",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(TabLayout {
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
    
    /// Set the list of tab labels
    /// 
    /// # Arguments
    /// * `tabs` - A slice of strings representing the tab labels
    /// 
    /// # Example
    /// ```no_run
    /// tab_layout.set_list(activity, &["Page 1", "Page 2", "Page 3"])?;
    /// ```
    pub fn set_list(&self, activity: &mut Activity, tabs: &[&str]) -> Result<()> {
        activity.send(&json!({
            "method": "setList",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "list": tabs
            }
        }))?;
        Ok(())
    }
    
    /// Programmatically select a tab
    /// 
    /// # Arguments
    /// * `index` - The zero-based index of the tab to select
    /// 
    /// # Example
    /// ```no_run
    /// // Select the second tab (index 1)
    /// tab_layout.select_tab(activity, 1)?;
    /// ```
    pub fn select_tab(&self, activity: &mut Activity, index: usize) -> Result<()> {
        activity.send(&json!({
            "method": "selectTab",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "tab": index
            }
        }))?;
        Ok(())
    }
}
