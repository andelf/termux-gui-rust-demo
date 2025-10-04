//! WebView component for displaying web content
//!
//! The WebView component allows you to display web pages and HTML content within your application,
//! with support for loading URLs, executing JavaScript, and more.
//!
//! # Important Usage Notes
//!
//! When displaying HTML content with dynamic effects, **you must enable JavaScript first**:
//!
//! ```no_run
//! use termux_gui_rust_demo::prelude::*;
//!
//! let mut activity = Activity::new()?;
//! let layout = LinearLayout::new(&mut activity, None, true)?;
//! let webview = WebView::new(&mut activity, Some(layout.id()))?;
//!
//! // ⚠️ Critical: Enable JavaScript first, or dynamic HTML content won't display
//! webview.allow_javascript(&mut activity, true)?;
//!
//! // Set HTML content
//! let html = r#"
//!     <html>
//!     <body style="background: purple;">
//!         <h1>Hello WebView!</h1>
//!     </body>
//!     </html>
//! "#;
//! webview.set_data(&mut activity, html)?;
//!
//! // Or load an external webpage
//! webview.load_uri(&mut activity, "https://www.google.com")?;
//! # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
//! ```

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// WebView component for displaying web content
///
/// # Important Usage Notes
///
/// 1. **JavaScript Support**: If your HTML content contains JavaScript or dynamic effects,
///    you must call `allow_javascript()` first to enable JavaScript, otherwise you may see
///    a blank page.
///
/// 2. **HTML Content Display Order**:
///    ```no_run
///    # use termux_gui_rust_demo::prelude::*;
///    # let mut activity = Activity::new()?;
///    # let webview = WebView::new(&mut activity, None)?;
///    // Step 1: Enable JavaScript (if needed)
///    webview.allow_javascript(&mut activity, true)?;
///    
///    // Step 2: Set HTML content
///    webview.set_data(&mut activity, "<html>...</html>")?;
///    # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
///    ```
///
/// 3. **Loading External URLs**: Simply call `load_uri()` - no special order required
///
/// # Examples
///
/// ```no_run
/// use termux_gui_rust_demo::prelude::*;
///
/// let mut activity = Activity::new()?;
/// let webview = WebView::new(&mut activity, None)?;
///
/// // Method 1: Display HTML content (requires JavaScript to be enabled first)
/// webview.allow_javascript(&mut activity, true)?;
/// webview.set_data(&mut activity, "<html><body><h1>Hello</h1></body></html>")?;
///
/// // Method 2: Load a webpage
/// webview.load_uri(&mut activity, "https://www.example.com")?;
/// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
/// ```
pub struct WebView {
    view: View,
    aid: i64,
}

impl WebView {
    /// Creates a new WebView
    ///
    /// # Arguments
    /// - `activity`: Reference to the Activity
    /// - `parent`: Optional parent view ID
    ///
    /// # Examples
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let layout_id = 0;
    /// let webview = WebView::new(&mut activity, Some(layout_id))?;
    /// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
    /// ```
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        eprintln!("[DEBUG] WebView::new() - creating WebView...");
        
        let mut params = json!({
            "aid": activity.id()
        });
        
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        eprintln!("[DEBUG] WebView::new() - sending createWebView...");
        let response = activity.send_read(&json!({
            "method": "createWebView",
            "params": params
        }))?;
        
        eprintln!("[DEBUG] WebView::new() - got response: {:?}", response);
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(WebView {
            view: View::new(id),
            aid: activity.id(),
        })
    }
    
    /// Gets the view ID
    pub fn id(&self) -> i64 {
        self.view.id()
    }
    
    /// Gets a reference to the underlying View
    pub fn view(&self) -> &View {
        &self.view
    }
    
    /// Loads a URI/URL
    ///
    /// # Arguments
    /// - `activity`: Reference to the Activity
    /// - `uri`: The URL to load, e.g., "https://www.google.com"
    ///
    /// # Examples
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let webview = WebView::new(&mut activity, None)?;
    /// webview.load_uri(&mut activity, "https://www.google.com")?;
    /// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
    /// ```
    pub fn load_uri(&self, activity: &mut Activity, uri: &str) -> Result<()> {
        activity.send(&json!({
            "method": "loadURI",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "uri": uri
            }
        }))?;
        Ok(())
    }
    
    /// Sets HTML content
    ///
    /// ⚠️ **Important**: If the HTML contains JavaScript or dynamic effects, you must call
    /// `allow_javascript(true)` first, otherwise you may see a blank page.
    ///
    /// # Arguments
    /// - `activity`: Reference to the Activity
    /// - `data`: The HTML document content
    ///
    /// # Examples
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let webview = WebView::new(&mut activity, None)?;
    /// // If HTML contains JavaScript, enable it first
    /// webview.allow_javascript(&mut activity, true)?;
    /// 
    /// // Then set the HTML content
    /// webview.set_data(&mut activity, "<html><body><h1>Hello</h1></body></html>")?;
    /// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
    /// ```
    pub fn set_data(&self, activity: &mut Activity, data: &str) -> Result<()> {
        // Use base64 encoding to support HTML content with non-ASCII characters
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data.as_bytes());
        
        activity.send(&json!({
            "method": "setData",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "doc": encoded,
                "base64": true
            }
        }))?;
        Ok(())
    }
    
    /// Allows JavaScript execution
    ///
    /// ⚠️ **Important**: When displaying HTML with JavaScript or dynamic effects, you must
    /// call this method first to enable JavaScript.
    ///
    /// If JavaScript is requested to be enabled, a user confirmation dialog will appear,
    /// and the user can deny the request. This method blocks until the user responds.
    ///
    /// # Arguments
    /// - `activity`: Reference to the Activity
    /// - `allow`: Whether to allow JavaScript
    ///
    /// # Returns
    /// Returns whether JavaScript is enabled after the call (if the user denies,
    /// it will return false even if you passed true)
    ///
    /// # Examples
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let webview = WebView::new(&mut activity, None)?;
    /// // Enable JavaScript (requires user confirmation)
    /// let enabled = webview.allow_javascript(&mut activity, true)?;
    /// if enabled {
    ///     println!("JavaScript enabled");
    ///     // Now you can set HTML with JavaScript
    ///     webview.set_data(&mut activity, "<html><body><script>alert('Hi!');</script></body></html>")?;
    /// }
    /// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
    /// ```
    pub fn allow_javascript(&self, activity: &mut Activity, allow: bool) -> Result<bool> {
        let response = activity.send_read(&json!({
            "method": "allowJavascript",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "allow": allow
            }
        }))?;
        
        Ok(response.as_bool().unwrap_or(false))
    }
    
    /// Allows loading content from content:// URIs
    ///
    /// # Arguments
    /// - `activity`: Reference to the Activity
    /// - `allow`: Whether to allow loading from content URIs
    pub fn allow_content_uri(&self, activity: &mut Activity, allow: bool) -> Result<()> {
        activity.send(&json!({
            "method": "allowContentURI",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "allow": allow
            }
        }))?;
        Ok(())
    }
    
    /// Allows navigation to different sites
    ///
    /// # Arguments
    /// - `activity`: Reference to the Activity
    /// - `allow`: Whether to allow users and JavaScript to navigate to different sites
    pub fn allow_navigation(&self, activity: &mut Activity, allow: bool) -> Result<()> {
        activity.send(&json!({
            "method": "allowNavigation",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "allow": allow
            }
        }))?;
        Ok(())
    }
    
    /// Executes JavaScript code in the WebView
    ///
    /// ⚠️ **Prerequisite**: You must enable JavaScript via `allow_javascript(true)` first,
    /// otherwise the code will not execute.
    ///
    /// # Arguments
    /// - `activity`: Reference to the Activity
    /// - `code`: The JavaScript code to execute
    ///
    /// # Examples
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let webview = WebView::new(&mut activity, None)?;
    /// // Step 1: Enable JavaScript
    /// webview.allow_javascript(&mut activity, true)?;
    ///
    /// // Step 2: Execute JavaScript code
    /// webview.evaluate_js(&mut activity, "document.body.style.background = 'red';")?;
    /// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
    /// ```
    pub fn evaluate_js(&self, activity: &mut Activity, code: &str) -> Result<()> {
        activity.send(&json!({
            "method": "evaluateJS",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "code": code
            }
        }))?;
        Ok(())
    }
    
    /// Goes back to the previous page in history
    pub fn go_back(&self, activity: &mut Activity) -> Result<()> {
        activity.send(&json!({
            "method": "goBack",
            "params": {
                "aid": self.aid,
                "id": self.view.id()
            }
        }))?;
        Ok(())
    }
    
    /// Goes forward to the next page in history
    pub fn go_forward(&self, activity: &mut Activity) -> Result<()> {
        activity.send(&json!({
            "method": "goForward",
            "params": {
                "aid": self.aid,
                "id": self.view.id()
            }
        }))?;
        Ok(())
    }
}
