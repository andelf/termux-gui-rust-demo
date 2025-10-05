# WebView 快速参考

## 🚀 快速启动

### 运行演示程序

```bash
# User-Agent 和配置演示
./run_webview_config.sh
# 或
./target/release/examples/webview_config_demo

# 全屏 WebView 演示
./run_fullscreen_webview.sh
# 或
./target/release/examples/fullscreen_webview
```

---

## 📋 完整 API 列表

### 创建 WebView
```rust
let webview = activity.create_web_view(parent_id)?;
```

### 1. 允许 JavaScript ⭐
```rust
let enabled = webview.allow_javascript(&mut activity, true)?;
// 返回: bool - 用户是否允许
// 需要: 用户确认对话框
```

### 2. 允许导航 ⭐⭐⭐
```rust
webview.allow_navigation(&mut activity, true)?;
// 功能: 允许跳转到不同网站
// 重要: 访问外部网站时必须启用！
```

### 3. 允许 Content URI
```rust
webview.allow_content_uri(&mut activity, true)?;
// 功能: 访问 content:// 协议的本地内容
```

### 4. 加载 URL
```rust
webview.load_uri(&mut activity, "https://www.google.com")?;
```

### 5. 设置 HTML 内容
```rust
let html = "<html><body><h1>Hello</h1></body></html>";
webview.set_data(&mut activity, html)?;
```

### 6. 执行 JavaScript
```rust
webview.evaluate_js(&mut activity, "alert('Hello')")?;
```

### 7. 后退/前进
```rust
webview.go_back(&mut activity)?;
webview.go_forward(&mut activity)?;
```

---

## 🌐 访问外部网站标准流程

```rust
// 1. 创建 WebView
let webview = activity.create_web_view(None)?;

// 2. ⭐ 启用导航（必须！）
webview.allow_navigation(&mut activity, true)?;

// 3. 启用 JavaScript（如果需要）
webview.allow_javascript(&mut activity, true)?;

// 4. 加载网站
webview.load_uri(&mut activity, "https://github.com")?;
```

---

## 🔍 User-Agent 相关

### ❌ 不支持设置 User-Agent

Termux GUI 没有提供设置 User-Agent 的 API。

### ✅ 查看 User-Agent

```rust
webview.allow_javascript(&mut activity, true)?;
webview.evaluate_js(&mut activity, r#"
    console.log('User-Agent:', navigator.userAgent);
    // 或显示在页面上
    alert(navigator.userAgent);
"#)?;
```

### 📱 默认 User-Agent 格式

```
Mozilla/5.0 (Linux; Android 13; Device) 
AppleWebKit/537.36 (KHTML, like Gecko) 
Chrome/120.0.0.0 Mobile Safari/537.36
```

---

## 💡 实用 JavaScript 技巧

### 1. 查看完整浏览器信息
```rust
webview.evaluate_js(&mut activity, r#"
    const info = {
        userAgent: navigator.userAgent,
        platform: navigator.platform,
        language: navigator.language,
        online: navigator.onLine,
        cookieEnabled: navigator.cookieEnabled,
        screenWidth: screen.width,
        screenHeight: screen.height
    };
    alert(JSON.stringify(info, null, 2));
"#)?;
```

### 2. 夜间模式
```rust
webview.evaluate_js(&mut activity, r#"
    document.body.style.filter = 'invert(1) hue-rotate(180deg)';
"#)?;
```

### 3. 放大字体
```rust
webview.evaluate_js(&mut activity, r#"
    document.body.style.fontSize = '20px';
    const style = document.createElement('style');
    style.textContent = '* { font-size: 20px !important; }';
    document.head.appendChild(style);
"#)?;
```

### 4. 注入自定义 CSS
```rust
webview.evaluate_js(&mut activity, r#"
    const style = document.createElement('style');
    style.textContent = `
        body { background: #1a1a1a !important; color: white !important; }
        a { color: #4fc3f7 !important; }
    `;
    document.head.appendChild(style);
"#)?;
```

### 5. 禁用图片（加速）
```rust
webview.evaluate_js(&mut activity, r#"
    const style = document.createElement('style');
    style.textContent = 'img { display: none !important; }';
    document.head.appendChild(style);
"#)?;
```

### 6. 自动滚动
```rust
// 滚动到底部
webview.evaluate_js(&mut activity, 
    "window.scrollTo(0, document.body.scrollHeight)")?;

// 平滑滚动
webview.evaluate_js(&mut activity, r#"
    window.scrollTo({top: document.body.scrollHeight, behavior: 'smooth'});
"#)?;
```

### 7. 获取页面标题
```rust
webview.evaluate_js(&mut activity, 
    "console.log('Title:', document.title)")?;
```

### 8. 检测页面加载完成
```rust
webview.evaluate_js(&mut activity, r#"
    if (document.readyState === 'complete') {
        console.log('Page loaded');
    } else {
        window.addEventListener('load', () => {
            console.log('Page loaded');
        });
    }
"#)?;
```

---

## ⚠️ 常见问题

### Q1: 无法跳转到其他网站？
**A**: 必须调用 `allow_navigation(true)`

```rust
webview.allow_navigation(&mut activity, true)?;
```

### Q2: HTML 中的 JavaScript 不执行？
**A**: 必须先启用 JavaScript，再设置 HTML

```rust
webview.allow_javascript(&mut activity, true)?;
webview.set_data(&mut activity, html)?;
```

### Q3: 如何自定义 User-Agent？
**A**: Termux GUI 不支持。使用系统默认 User-Agent。

### Q4: 如何设置 Cookie？
**A**: 通过 JavaScript：

```rust
webview.evaluate_js(&mut activity, 
    "document.cookie = 'name=value; path=/'")?;
```

### Q5: 如何处理下载？
**A**: Termux GUI 不支持下载功能。

---

## 📚 配置选项对比

| 功能 | Python | Rust | 说明 |
|------|--------|------|------|
| JavaScript | `allowjavascript()` | `allow_javascript()` | 需用户确认 |
| 导航 | `allownavigation()` | `allow_navigation()` | 访问外部必需 |
| Content URI | `allowcontenturi()` | `allow_content_uri()` | 本地内容 |
| 加载 URL | `loaduri()` | `load_uri()` | - |
| 设置 HTML | `setdata()` | `set_data()` | Rust 用 base64 |
| 执行 JS | `evaluatejs()` | `evaluate_js()` | - |
| 后退 | `goback()` | `go_back()` | - |
| 前进 | `goforward()` | `go_forward()` | - |

---

## 🎯 最佳实践

### 1. 初始化顺序
```rust
// 正确顺序
webview.allow_navigation(&mut activity, true)?;
webview.allow_javascript(&mut activity, true)?;
webview.load_uri(&mut activity, url)?;
```

### 2. HTML + JavaScript
```rust
// 先启用 JS，再设置 HTML
webview.allow_javascript(&mut activity, true)?;
webview.set_data(&mut activity, html_with_js)?;
```

### 3. 错误处理
```rust
match webview.allow_javascript(&mut activity, true) {
    Ok(true) => println!("JavaScript 已启用"),
    Ok(false) => println!("用户拒绝启用 JavaScript"),
    Err(e) => eprintln!("错误: {}", e),
}
```

---

## 🔗 相关文档

- [WEBVIEW_SETTINGS.md](./WEBVIEW_SETTINGS.md) - 详细配置说明
- [WEBVIEW_COMPARISON.md](./WEBVIEW_COMPARISON.md) - Python vs Rust 对比
- [examples/webview_config_demo.rs](./examples/webview_config_demo.rs) - 配置演示
- [examples/fullscreen_webview.rs](./examples/fullscreen_webview.rs) - 全屏演示

---

**提示**: 运行 `./run_webview_config.sh` 查看实际效果！
