//! WebView component - 网页视图组件
//!
//! WebView 用于在应用中显示网页内容，支持加载URL、运行JavaScript等功能。
//!
//! # 重要提示
//!
//! 显示 HTML 内容时，**必须先启用 JavaScript** 才能看到动态效果：
//!
//! ```no_run
//! use termux_gui_rust_demo::prelude::*;
//!
//! let mut activity = Activity::new()?;
//! let layout = LinearLayout::new(&mut activity, None, true)?;
//! let webview = WebView::new(&mut activity, Some(layout.id()))?;
//!
//! // ⚠️ 关键：必须先启用 JavaScript，否则 HTML 中的动态内容不会显示
//! webview.allow_javascript(&mut activity, true)?;
//!
//! // 设置 HTML 内容
//! let html = r#"
//!     <html>
//!     <body style="background: purple;">
//!         <h1>Hello WebView!</h1>
//!     </body>
//!     </html>
//! "#;
//! webview.set_data(&mut activity, html)?;
//!
//! // 或者加载外部网页
//! webview.load_uri(&mut activity, "https://www.google.com")?;
//! # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
//! ```

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// WebView 组件，用于显示网页内容
///
/// # 重要使用说明
///
/// 1. **JavaScript 支持**：如果 HTML 内容包含 JavaScript 或动态效果，必须先调用
///    `allow_javascript()` 启用 JavaScript，否则可能看到空白页面。
///
/// 2. **HTML 内容显示顺序**：
///    ```no_run
///    # use termux_gui_rust_demo::prelude::*;
///    # let mut activity = Activity::new()?;
///    # let webview = WebView::new(&mut activity, None)?;
///    // 第一步：启用 JavaScript（如果需要）
///    webview.allow_javascript(&mut activity, true)?;
///    
///    // 第二步：设置 HTML 内容
///    webview.set_data(&mut activity, "<html>...</html>")?;
///    # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
///    ```
///
/// 3. **加载外部 URL**：直接调用 `load_uri()` 即可，无需特殊顺序
///
/// # 示例
///
/// ```no_run
/// use termux_gui_rust_demo::prelude::*;
///
/// let mut activity = Activity::new()?;
/// let webview = WebView::new(&mut activity, None)?;
///
/// // 方式1：显示 HTML 内容（需要先启用 JavaScript）
/// webview.allow_javascript(&mut activity, true)?;
/// webview.set_data(&mut activity, "<html><body><h1>Hello</h1></body></html>")?;
///
/// // 方式2：加载网页
/// webview.load_uri(&mut activity, "https://www.example.com")?;
/// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
/// ```
pub struct WebView {
    view: View,
    aid: i64,
}

impl WebView {
    /// 创建一个新的 WebView
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `parent`: 可选的父视图ID
    ///
    /// # 示例
    /// ```no_run
    /// let webview = WebView::new(&mut activity, Some(layout_id))?;
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
    
    /// 获取视图ID
    pub fn id(&self) -> i64 {
        self.view.id()
    }
    
    /// 获取视图引用
    pub fn view(&self) -> &View {
        &self.view
    }
    
    /// 加载URI/URL
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `uri`: 要加载的URL，例如 "https://www.google.com"
    ///
    /// # 示例
    /// ```no_run
    /// webview.load_uri(&mut activity, "https://www.google.com")?;
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
    
    /// 设置HTML内容
    ///
    /// ⚠️ **重要**：如果 HTML 包含 JavaScript 或动态效果，必须先调用 `allow_javascript(true)`，
    /// 否则可能看到空白页面。
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `data`: HTML文档内容
    ///
    /// # 示例
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let webview = WebView::new(&mut activity, None)?;
    /// // 如果 HTML 包含 JavaScript，必须先启用
    /// webview.allow_javascript(&mut activity, true)?;
    /// 
    /// // 然后设置 HTML 内容
    /// webview.set_data(&mut activity, "<html><body><h1>Hello</h1></body></html>")?;
    /// # Ok::<(), termux_gui_rust_demo::error::GuiError>(())
    /// ```
    pub fn set_data(&self, activity: &mut Activity, data: &str) -> Result<()> {
        // 使用 base64 编码来支持包含非ASCII字符的HTML内容
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
    
    /// 允许JavaScript执行
    ///
    /// ⚠️ **重要**：显示包含 JavaScript 或动态效果的 HTML 时，必须先调用此方法启用 JavaScript。
    ///
    /// 如果请求允许JavaScript，会弹出用户确认对话框，用户可以拒绝。
    /// 此方法会阻塞直到用户响应。
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `allow`: 是否允许JavaScript
    ///
    /// # 返回
    /// 返回调用后JavaScript是否启用（如果用户拒绝，即使传入 true 也会返回 false）
    ///
    /// # 示例
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let webview = WebView::new(&mut activity, None)?;
    /// // 启用 JavaScript（用户需要确认）
    /// let enabled = webview.allow_javascript(&mut activity, true)?;
    /// if enabled {
    ///     println!("JavaScript 已启用");
    ///     // 现在可以设置包含 JavaScript 的 HTML
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
    
    /// 允许从 content:// URI 加载内容
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `allow`: 是否允许
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
    
    /// 允许导航到不同站点
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `allow`: 是否允许用户和JavaScript导航到不同站点
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
    
    /// 在WebView中执行JavaScript代码
    ///
    /// ⚠️ **前置条件**：必须先通过 `allow_javascript(true)` 启用JavaScript，否则代码不会执行。
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `code`: 要执行的JavaScript代码
    ///
    /// # 示例
    /// ```no_run
    /// # use termux_gui_rust_demo::prelude::*;
    /// # let mut activity = Activity::new()?;
    /// # let webview = WebView::new(&mut activity, None)?;
    /// // 第一步：启用 JavaScript
    /// webview.allow_javascript(&mut activity, true)?;
    ///
    /// // 第二步：执行 JavaScript 代码
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
    
    /// 后退到历史记录的上一页
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
    
    /// 前进到历史记录的下一页
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
