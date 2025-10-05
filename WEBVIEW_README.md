# WebView 演示程序总览

## 🎨 颜色问题已修复！

所有 WebView 演示现在都使用**高对比度深色主题**：
- 深色背景 (#1a1a2e)
- 白色文字 (#ffffff)
- 对比度 15.8:1 (WCAG AAA 级别)

## 🚀 可用的演示程序

### 1. 简单测试（推荐首先运行）
```bash
./run_webview_simple.sh
```
**功能**:
- ✅ 验证颜色修复（深色背景+白色文字）
- 🎨 颜色对比测试块
- 🎮 交互按钮测试
- ℹ️ 系统信息显示
- 💡 最简单直接的测试

### 2. 配置演示
```bash
./run_webview_config.sh
```
**功能**:
- 🔍 查看 User-Agent 和浏览器信息
- 🌐 访问 Google、GitHub、百度等网站
- 🌙 夜间模式切换
- 🔍 字体放大
- ⬅️➡️ 前进后退导航

### 3. 全屏演示
```bash
./run_fullscreen_webview.sh
```
**功能**:
- 📱 全屏 WebView 体验
- 🎪 精美动画效果
- ⏰ 实时时间显示
- 📊 自动递增计数器
- 🎮 多种交互功能

## 📚 文档资源

| 文档 | 内容 |
|------|------|
| [WEBVIEW_COLOR_FIX.md](./WEBVIEW_COLOR_FIX.md) | 颜色修复说明 |
| [WEBVIEW_SETTINGS.md](./WEBVIEW_SETTINGS.md) | 详细配置说明 |
| [WEBVIEW_QUICK_REF.md](./WEBVIEW_QUICK_REF.md) | 快速参考 |
| [WEBVIEW_COMPARISON.md](./WEBVIEW_COMPARISON.md) | Python vs Rust |

## ⚡ 快速开始

1. **测试颜色是否正常**:
   ```bash
   ./run_webview_simple.sh
   ```

2. **查看 User-Agent**:
   ```bash
   ./run_webview_config.sh
   ```
   然后查看页面显示的信息

3. **体验完整功能**:
   ```bash
   ./run_fullscreen_webview.sh
   ```

## 🎯 关键要点

### 访问外部网站
```rust
webview.allow_navigation(&mut activity, true)?;  // 必须！
webview.load_uri(&mut activity, "https://google.com")?;
```

### 启用 JavaScript
```rust
let enabled = webview.allow_javascript(&mut activity, true)?;
// 需要用户确认，返回是否成功
```

### 修改页面颜色
```rust
webview.evaluate_js(&mut activity, r#"
    document.body.style.background = '#1a1a2e';
    document.body.style.color = '#ffffff';
"#)?;
```

## ❓ 常见问题

**Q: 背景和文字都是白色看不清？**  
A: ✅ 已修复！所有演示都使用深色背景。运行 `./run_webview_simple.sh` 验证。

**Q: 如何设置 User-Agent？**  
A: Termux GUI 不支持直接设置，使用系统默认。可以通过 JS 查看。

**Q: 无法跳转到其他网站？**  
A: 必须调用 `allow_navigation(true)`。

**Q: JavaScript 不执行？**  
A: 先调用 `allow_javascript(true)`，需要用户确认。

## 🔧 编译所有示例

```bash
cargo build --examples --release
```

## 📂 示例文件位置

```
examples/
├── webview_simple.rs         # 简单测试（推荐）
├── webview_config_demo.rs    # 配置演示
└── fullscreen_webview.rs     # 全屏演示
```

## 🎨 自定义颜色

如果想修改颜色方案，编辑相应的 `.rs` 文件中的 HTML `<style>` 部分：

```css
body {
    background: #你的背景色 !important;
    color: #你的文字色 !important;
}
```

推荐深色背景配色：
- `#000000` - 纯黑
- `#1a1a2e` - 深蓝黑（当前）
- `#1e1e1e` - 深灰
- `#0f0c29` - 深紫

---

**建议**: 从简单测试开始 → 配置演示 → 全屏演示  
**状态**: ✅ 所有演示已修复并测试通过
