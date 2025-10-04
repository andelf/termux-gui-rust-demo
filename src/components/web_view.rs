//! WebView component - 网页视图组件
//!
//! WebView 用于在应用中显示网页内容，支持加载URL、运行JavaScript等功能。

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// WebView 组件，用于显示网页内容
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
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `data`: HTML文档内容
    ///
    /// # 示例
    /// ```no_run
    /// webview.set_data(&mut activity, "<html><body><h1>Hello</h1></body></html>")?;
    /// ```
    pub fn set_data(&self, activity: &mut Activity, data: &str) -> Result<()> {
        activity.send(&json!({
            "method": "setData",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "doc": data
            }
        }))?;
        Ok(())
    }
    
    /// 允许JavaScript执行
    ///
    /// 如果请求允许JavaScript，会弹出用户确认对话框，用户可以拒绝。
    /// 此方法会阻塞直到用户响应。
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `allow`: 是否允许JavaScript
    ///
    /// # 返回
    /// 返回调用后JavaScript是否启用
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
    /// 需要先通过 allow_javascript() 启用JavaScript。
    ///
    /// # 参数
    /// - `activity`: Activity 引用
    /// - `code`: 要执行的JavaScript代码
    ///
    /// # 示例
    /// ```no_run
    /// webview.evaluate_js(&mut activity, "alert('Hello from JavaScript!');")?;
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
