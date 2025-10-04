//! WebView 网页视图演示 (新库版本)
//!
//! 演示 WebView 组件的使用：
//! - 显示HTML内容
//! - 加载网页URL
//! - 执行JavaScript代码
//! - 网页导航（前进/后退）

use termux_gui::{Activity, WebView, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("=== WebView 网页视图演示 (新库版本) ===\n");
    
    // 创建 Activity
    let mut activity = Activity::new(false)?;
    println!("✓ 连接建立\n");
    
    // 创建根布局 (垂直)
    let root = activity.create_linear_layout(None)?;
    
    // 标题文本
    let title = activity.create_text_view("🌐 WebView 演示", Some(root.id()))?;
    title.set_text_size(&mut activity, 20)?;
    title.view().set_height_wrap_content(&mut activity)?;
    title.view().set_margin(&mut activity, 16)?;
    title.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 提示文本
    let hint = activity.create_text_view(
        "━━━━━━━━━━━━━━━━━━━━━━\n\
        演示步骤:\n\
        1️⃣ 显示HTML内容 (3秒)\n\
        2️⃣ 加载Google网页 (3秒)\n\
        3️⃣ 请求JavaScript权限\n\
        4️⃣ 执行JavaScript代码\n\
        ━━━━━━━━━━━━━━━━━━━━━━", 
        Some(root.id()))?;
    hint.set_text_size(&mut activity, 14)?;
    hint.view().set_height_wrap_content(&mut activity)?;
    hint.view().set_margin(&mut activity, 16)?;
    hint.view().set_linear_layout_params(&mut activity, 0, None)?;
    
    // 创建 WebView
    let webview = activity.create_web_view(Some(root.id()))?;
    webview.view().set_linear_layout_params(&mut activity, 1, None)?;  // 占据剩余空间
    
    println!("✓ 界面创建完成\n");
    
    // 演示流程
    demo_workflow(&mut activity, &webview)?;
    
    println!("\n📌 等待用户关闭窗口...");
    
    // 事件循环
    loop {
        let event = read_message(activity.event_stream())?;
        let event_type = event["type"].as_str().unwrap_or("");
        
        match event_type {
            "destroy" => {
                println!("\n✓ Activity 已关闭");
                return Ok(());
            },
            _ => {}
        }
    }
}

/// 演示 WebView 的各种功能
fn demo_workflow(activity: &mut Activity, webview: &WebView) -> Result<()> {
    use std::thread;
    use std::time::Duration;
    
    // 步骤1: 显示 HTML 内容
    println!("1️⃣ 显示 HTML 内容...");
    println!("   📝 设置 HTML 文档...");
    
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: Arial, sans-serif;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            text-align: center;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }
        h1 {
            font-size: 2.5em;
            margin-bottom: 20px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        p {
            font-size: 1.3em;
            line-height: 1.8;
            margin: 10px 0;
        }
        .emoji {
            font-size: 4em;
            margin: 20px 0;
            animation: bounce 2s infinite;
        }
        @keyframes bounce {
            0%, 100% { transform: translateY(0); }
            50% { transform: translateY(-20px); }
        }
        .box {
            background: rgba(255,255,255,0.2);
            padding: 30px;
            border-radius: 15px;
            backdrop-filter: blur(10px);
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <div class="emoji">🚀</div>
    <h1>Hello from HTML!</h1>
    <div class="box">
        <p>这是通过 <strong>setData()</strong> 设置的 HTML 内容</p>
        <p>包含样式、布局和动画</p>
        <p style="margin-top:20px; font-size:1em;">⏰ 3秒后将跳转到 Google</p>
    </div>
</body>
</html>"#;
    
    webview.set_data(activity, html)?;
    println!("   ✓ HTML 内容已设置");
    println!("   👀 请查看 WebView - 应该能看到紫色渐变背景和火箭动画");
    
    // 等待3秒
    println!("   ⏰ 等待 3 秒...");
    thread::sleep(Duration::from_secs(3));
    
    // 步骤2: 加载网页
    println!("\n2️⃣ 加载 Google 网页...");
    println!("   🌐 正在加载 https://www.google.com ...");
    webview.load_uri(activity, "https://www.google.com")?;
    println!("   ✓ URL 加载指令已发送");
    println!("   👀 请查看 WebView - Google 搜索页面");
    
    // 等待3秒
    println!("   ⏰ 等待 3 秒...");
    thread::sleep(Duration::from_secs(3));
    
    // 步骤3: 请求 JavaScript 权限
    println!("\n3️⃣ 请求 JavaScript 权限...");
    println!("   📱 将弹出确认对话框，请点击允许");
    println!("   ⏳ 等待用户确认...");
    
    match webview.allow_javascript(activity, true) {
        Ok(enabled) => {
            if enabled {
                println!("   ✅ JavaScript 已启用");
                
                // 步骤4: 执行 JavaScript
                println!("\n4️⃣ 执行 JavaScript 代码...");
                println!("   💻 使用 JS 动态创建页面...");
                let js_code = r#"
                    document.body.innerHTML = `
                        <div style="
                            font-family: Arial, sans-serif;
                            padding: 20px;
                            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
                            color: white;
                            text-align: center;
                            min-height: 100vh;
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                        ">
                            <div style="font-size: 5em; margin-bottom: 20px; animation: spin 3s linear infinite;">✨</div>
                            <h1 style="font-size: 2.5em; margin-bottom: 20px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">
                                Hello from JavaScript!
                            </h1>
                            <div style="background: rgba(255,255,255,0.2); padding: 30px; border-radius: 15px; backdrop-filter: blur(10px);">
                                <p style="font-size: 1.5em; line-height: 1.6; margin: 10px 0;">
                                    这个页面是通过 <strong>evaluateJS()</strong> 动态创建的
                                </p>
                                <p style="font-size: 1.3em; margin-top: 20px;">
                                    🎉 JavaScript 执行成功！
                                </p>
                                <p style="font-size: 1em; margin-top: 15px; opacity: 0.9;">
                                    当前时间: ${new Date().toLocaleString('zh-CN')}
                                </p>
                            </div>
                        </div>
                        <style>
                            @keyframes spin {
                                from { transform: rotate(0deg); }
                                to { transform: rotate(360deg); }
                            }
                        </style>
                    `;
                "#;
                webview.evaluate_js(activity, js_code)?;
                println!("   ✓ JavaScript 已执行");
                println!("   👀 请查看 WebView - 粉色渐变背景，带旋转动画的星星");
            } else {
                println!("   ⚠️  JavaScript 未启用（用户拒绝或系统限制）");
                println!("   ℹ️  跳过 JavaScript 执行步骤");
            }
        },
        Err(e) => {
            println!("   ❌ 启用 JavaScript 失败: {}", e);
            println!("   ℹ️  跳过 JavaScript 执行步骤");
        }
    }
    
    println!("\n✅ 演示流程完成！");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 总结:");
    println!("  • setData() - 设置 HTML 内容");
    println!("  • loadURI() - 加载网页 URL");
    println!("  • allowJavascript() - 请求 JavaScript 权限");
    println!("  • evaluateJS() - 执行 JavaScript 代码");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}
