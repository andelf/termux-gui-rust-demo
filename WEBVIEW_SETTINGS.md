# WebView 访问外部网站配置说明

## 📋 可用的 WebView 设置方法

根据对 Termux GUI Python 版本和协议的分析，WebView 提供以下配置方法：

### 1. ✅ JavaScript 支持
```rust
webview.allow_javascript(&mut activity, true)?;
```
- **功能**: 启用/禁用 JavaScript 执行
- **需要**: 用户确认（会弹出对话框）
- **返回**: bool 表示是否成功启用

### 2. ✅ Content URI 访问
```rust
webview.allow_content_uri(&mut activity, true)?;
```
- **功能**: 允许加载 `content://` URI 的本地内容
- **用途**: 访问本地文件、媒体等

### 3. ✅ 导航控制
```rust
webview.allow_navigation(&mut activity, true)?;
```
- **功能**: 允许用户和 JavaScript 导航到不同站点
- **默认**: 可能限制导航
- **建议**: 访问外部网站时设置为 `true`

### 4. ✅ HTML 内容设置
```rust
webview.set_data(&mut activity, html_content)?;
```
- **功能**: 设置 HTML 文档内容
- **特点**: Rust 版本使用 base64 编码，支持非 ASCII 字符

### 5. ✅ URL 加载
```rust
webview.load_uri(&mut activity, "https://example.com")?;
```
- **功能**: 加载外部网页
- **支持**: HTTP/HTTPS 协议

### 6. ✅ JavaScript 执行
```rust
webview.evaluate_js(&mut activity, js_code)?;
```
- **功能**: 在 WebView 中执行任意 JavaScript 代码
- **前提**: 需要先启用 JavaScript

### 7. ✅ 导航历史
```rust
webview.go_back(&mut activity)?;    // 后退
webview.go_forward(&mut activity)?; // 前进
```

---

## 🔍 User-Agent 设置

### ❌ 不支持直接设置

**重要发现**: Termux GUI 的 WebView 协议中**没有提供直接设置 User-Agent 的方法**。

经过对 Python 版本源码和 API 文档的详细分析，可用的方法只有：
- `allowjavascript`
- `allowcontenturi`
- `allownavigation`
- `setdata`
- `loaduri`
- `evaluatejs`
- `goback`
- `goforward`

### ✅ 替代方案：通过 JavaScript 查看

虽然不能设置，但可以通过 JavaScript 查看当前的 User-Agent：

```rust
// 查看当前 User-Agent
webview.allow_javascript(&mut activity, true)?;
webview.evaluate_js(&mut activity, r#"
    console.log('User-Agent:', navigator.userAgent);
    // 或者显示在页面上
    document.body.innerHTML = '<h3>User-Agent:</h3><p>' + navigator.userAgent + '</p>';
"#)?;
```

### 📱 默认 User-Agent

Android WebView 的默认 User-Agent 通常格式如下：

```
Mozilla/5.0 (Linux; Android {version}; {device}) 
AppleWebKit/537.36 (KHTML, like Gecko) 
Chrome/{version} Mobile Safari/537.36
```

例如：
```
Mozilla/5.0 (Linux; Android 13; SM-G991B) 
AppleWebKit/537.36 (KHTML, like Gecko) 
Chrome/120.0.0.0 Mobile Safari/537.36
```

---

## 🌐 访问外部网站的完整配置

### 推荐配置流程

```rust
use termux_gui::{Activity, WebView, Result};

fn setup_webview_for_external_sites(activity: &mut Activity) -> Result<()> {
    // 1. 创建 WebView
    let webview = activity.create_web_view(None)?;
    
    // 2. 允许导航（重要！）
    webview.allow_navigation(activity, true)?;
    println!("✓ 导航已启用");
    
    // 3. 启用 JavaScript（如果需要）
    let js_enabled = webview.allow_javascript(activity, true)?;
    if js_enabled {
        println!("✓ JavaScript 已启用");
    } else {
        println!("⚠️  JavaScript 未启用（用户拒绝）");
    }
    
    // 4. 加载外部网站
    webview.load_uri(activity, "https://www.google.com")?;
    println!("✓ 正在加载网页...");
    
    Ok(())
}
```

### 完整示例

```rust
use termux_gui::{Activity, LinearLayout, WebView, Result};

fn main() -> Result<()> {
    let mut activity = Activity::new(false)?;
    let root = activity.create_linear_layout(None)?;
    let webview = activity.create_web_view(Some(root.id()))?;
    
    // 设置为全屏
    webview.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    // 配置 WebView 访问外部网站
    println!("正在配置 WebView...");
    
    // 启用导航
    webview.allow_navigation(&mut activity, true)?;
    
    // 启用 JavaScript
    webview.allow_javascript(&mut activity, true)?;
    
    // 加载网站
    webview.load_uri(&mut activity, "https://www.github.com")?;
    
    // 可选：查看 User-Agent
    std::thread::sleep(std::time::Duration::from_secs(2));
    webview.evaluate_js(&mut activity, r#"
        // 在页面顶部显示 User-Agent
        const div = document.createElement('div');
        div.style.cssText = 'position:fixed;top:0;left:0;right:0;background:yellow;padding:10px;z-index:9999;font-size:12px;';
        div.innerHTML = '<b>UA:</b> ' + navigator.userAgent;
        document.body.insertBefore(div, document.body.firstChild);
    "#)?;
    
    // 事件循环...
    
    Ok(())
}
```

---

## 🎯 其他 WebView 限制和注意事项

### 协议限制
- ✅ 支持: `http://`, `https://`, `file://`, `content://`
- ❌ 不支持自定义协议（需要 Android 原生支持）

### 存储
- **Cookie**: 默认支持，自动管理
- **LocalStorage**: 需要启用 JavaScript
- **SessionStorage**: 需要启用 JavaScript

### 网络
- **WiFi/数据**: 使用系统网络设置
- **代理**: 使用系统代理设置
- **证书**: 使用系统证书存储

### 权限
- **网络访问**: 需要 Termux 有网络权限
- **文件访问**: 通过 `allow_content_uri` 控制
- **JavaScript**: 通过 `allow_javascript` 控制（需要用户确认）

---

## 💡 实用技巧

### 1. 检测网页加载完成

通过 JavaScript 注入来检测：

```rust
webview.evaluate_js(&mut activity, r#"
    if (document.readyState === 'complete') {
        console.log('Page loaded');
    } else {
        window.addEventListener('load', function() {
            console.log('Page loaded');
        });
    }
"#)?;
```

### 2. 修改页面样式

```rust
// 夜间模式
webview.evaluate_js(&mut activity, r#"
    document.body.style.background = '#1a1a1a';
    document.body.style.color = '#ffffff';
    document.body.style.filter = 'invert(1) hue-rotate(180deg)';
"#)?;
```

### 3. 注入自定义 CSS

```rust
webview.evaluate_js(&mut activity, r#"
    const style = document.createElement('style');
    style.textContent = `
        * { 
            font-size: 18px !important; 
        }
        img { 
            max-width: 100% !important; 
        }
    `;
    document.head.appendChild(style);
"#)?;
```

### 4. 禁用图片（加速加载）

```rust
webview.evaluate_js(&mut activity, r#"
    const style = document.createElement('style');
    style.textContent = 'img { display: none !important; }';
    document.head.appendChild(style);
"#)?;
```

### 5. 自动滚动

```rust
webview.evaluate_js(&mut activity, r#"
    window.scrollTo(0, document.body.scrollHeight);
"#)?;
```

---

## 📌 总结

### 支持的配置
| 配置项 | 方法 | 说明 |
|-------|------|------|
| JavaScript | `allow_javascript()` | ✅ 支持，需用户确认 |
| 导航控制 | `allow_navigation()` | ✅ 支持 |
| Content URI | `allow_content_uri()` | ✅ 支持 |
| HTML 内容 | `set_data()` | ✅ 支持 |
| URL 加载 | `load_uri()` | ✅ 支持 |
| JS 执行 | `evaluate_js()` | ✅ 支持 |
| 历史导航 | `go_back()/go_forward()` | ✅ 支持 |

### 不支持的配置
| 配置项 | 状态 | 替代方案 |
|-------|------|---------|
| User-Agent | ❌ 不支持 | 使用系统默认，可用 JS 查看 |
| Cookie 设置 | ❌ 不直接支持 | 通过 JS `document.cookie` |
| 缓存控制 | ❌ 不支持 | 系统自动管理 |
| 下载控制 | ❌ 不支持 | - |
| 缩放设置 | ❌ 不支持 | 通过 JS 设置 viewport |
| 自定义协议 | ❌ 不支持 | - |

### 关键建议

1. **访问外部网站**: 必须调用 `allow_navigation(true)`
2. **动态网页**: 必须先调用 `allow_javascript(true)`
3. **User-Agent**: 无法自定义，使用系统默认
4. **高级功能**: 通过 `evaluate_js()` 实现大部分功能

---

## 🔗 参考资料

- Termux GUI Python 源码: `/data/data/com.termux/files/home/Documents/termux-gui-python-bindings/src/termuxgui/webview.py`
- Rust 实现: `/data/data/com.termux/files/home/termux-gui-rust-demo/src/components/web_view.rs`
- Android WebView 文档: https://developer.android.com/reference/android/webkit/WebView

---

**最后更新**: 2024
**版本**: Termux GUI Rust Demo v1.0
