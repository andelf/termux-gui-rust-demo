//! WebView 配置演示 - User-Agent 和其他设置
//!
//! 演示如何查看 User-Agent 以及配置 WebView 访问外部网站

use termux_gui::{Activity, WebView, Result};
use termux_gui::connection::read_message;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("🌐 WebView 配置演示");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 创建 Activity
    let mut activity = Activity::new(false)?;
    println!("✓ Activity 已创建");
    
    // 创建布局
    let root = activity.create_linear_layout(None)?;
    println!("✓ 布局已创建");
    
    // 创建 WebView
    let webview = activity.create_web_view(Some(root.id()))?;
    webview.view().set_linear_layout_params(&mut activity, 1, None)?;
    println!("✓ WebView 已创建\n");
    
    // 配置 WebView
    println!("📋 配置 WebView...");
    
    // 1. 启用导航
    println!("  1️⃣ 启用导航...");
    webview.allow_navigation(&mut activity, true)?;
    println!("     ✓ 导航已启用（允许跳转到其他网站）");
    
    // 2. 启用 JavaScript
    println!("  2️⃣ 请求 JavaScript 权限...");
    println!("     📱 请在弹出的对话框中点击【允许】");
    let js_enabled = webview.allow_javascript(&mut activity, true)?;
    
    if !js_enabled {
        println!("     ⚠️  JavaScript 未启用（用户拒绝）");
        println!("     ℹ️  将无法查看 User-Agent 和执行 JS");
        println!("\n按 Ctrl+C 退出...");
        wait_for_destroy(&mut activity)?;
        return Ok(());
    }
    
    println!("     ✅ JavaScript 已启用\n");
    
    // 3. 加载测试页面
    println!("🎨 加载检测页面...");
    let html = create_detection_page();
    webview.set_data(&mut activity, &html)?;
    println!("   ✓ 页面已加载\n");
    
    // 等待页面渲染
    thread::sleep(Duration::from_secs(2));
    
    // 4. 查看 User-Agent
    println!("🔍 检测 User-Agent...");
    webview.evaluate_js(&mut activity, r#"
        // 获取 User-Agent
        const ua = navigator.userAgent;
        
        // 在控制台输出
        console.log('User-Agent:', ua);
        
        // 在页面上显示
        document.getElementById('ua-value').textContent = ua;
        
        // 显示其他信息
        document.getElementById('platform').textContent = navigator.platform;
        document.getElementById('language').textContent = navigator.language;
        document.getElementById('online').textContent = navigator.onLine ? '在线' : '离线';
        document.getElementById('cookieEnabled').textContent = navigator.cookieEnabled ? '启用' : '禁用';
        
        // 显示屏幕信息
        document.getElementById('screenWidth').textContent = screen.width;
        document.getElementById('screenHeight').textContent = screen.height;
        document.getElementById('windowWidth').textContent = window.innerWidth;
        document.getElementById('windowHeight').textContent = window.innerHeight;
    "#)?;
    println!("   ✓ 信息已显示在页面上\n");
    
    // 5. 演示访问外部网站
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 交互选项:");
    println!("  按 1 - 访问 Google");
    println!("  按 2 - 访问 GitHub");
    println!("  按 3 - 访问 百度");
    println!("  按 4 - 显示检测页面");
    println!("  按 5 - 夜间模式切换");
    println!("  按 6 - 放大字体");
    println!("  按 b - 后退");
    println!("  按 f - 前进");
    println!("  按 q - 退出");
    println!("━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // 交互循环
    interactive_loop(&mut activity, &webview)?;
    
    Ok(())
}

/// 创建检测页面
fn create_detection_page() -> String {
    r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebView 信息检测</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%) !important;
            color: #ffffff !important;
            padding: 20px;
            line-height: 1.6;
            min-height: 100vh;
        }
        
        body * {
            color: inherit !important;
        }
        
        .container {
            max-width: 800px;
            margin: 0 auto;
        }
        
        h1 {
            text-align: center;
            margin-bottom: 30px;
            font-size: 2em;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        
        .card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 15px;
            padding: 20px;
            margin-bottom: 20px;
            border: 1px solid rgba(255, 255, 255, 0.2);
        }
        
        .card h2 {
            margin-bottom: 15px;
            font-size: 1.3em;
            border-bottom: 2px solid rgba(255, 255, 255, 0.3);
            padding-bottom: 10px;
        }
        
        .info-row {
            display: flex;
            margin: 10px 0;
            padding: 10px;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 8px;
        }
        
        .label {
            font-weight: bold;
            min-width: 150px;
            color: #ffd700;
        }
        
        .value {
            flex: 1;
            word-break: break-all;
            font-family: monospace;
        }
        
        .emoji {
            font-size: 3em;
            text-align: center;
            margin: 20px 0;
        }
        
        .status-badge {
            display: inline-block;
            padding: 5px 15px;
            background: rgba(0, 255, 0, 0.3);
            border-radius: 20px;
            font-size: 0.9em;
            margin-top: 10px;
        }
        
        .grid {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 10px;
            margin-top: 10px;
        }
        
        .grid-item {
            background: rgba(255, 255, 255, 0.05);
            padding: 15px;
            border-radius: 8px;
            text-align: center;
        }
        
        .grid-item .big {
            font-size: 1.8em;
            font-weight: bold;
            color: #ffd700;
        }
        
        .grid-item .small {
            font-size: 0.9em;
            opacity: 0.9;
            margin-top: 5px;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="emoji">🔍</div>
        <h1>WebView 信息检测</h1>
        
        <div class="card">
            <h2>📱 User-Agent</h2>
            <div class="info-row">
                <div class="value" id="ua-value">检测中...</div>
            </div>
            <div class="status-badge">✓ JavaScript 已启用</div>
        </div>
        
        <div class="card">
            <h2>🌐 浏览器信息</h2>
            <div class="info-row">
                <span class="label">平台:</span>
                <span class="value" id="platform">-</span>
            </div>
            <div class="info-row">
                <span class="label">语言:</span>
                <span class="value" id="language">-</span>
            </div>
            <div class="info-row">
                <span class="label">网络状态:</span>
                <span class="value" id="online">-</span>
            </div>
            <div class="info-row">
                <span class="label">Cookie:</span>
                <span class="value" id="cookieEnabled">-</span>
            </div>
        </div>
        
        <div class="card">
            <h2>📐 屏幕信息</h2>
            <div class="grid">
                <div class="grid-item">
                    <div class="big" id="screenWidth">-</div>
                    <div class="small">屏幕宽度</div>
                </div>
                <div class="grid-item">
                    <div class="big" id="screenHeight">-</div>
                    <div class="small">屏幕高度</div>
                </div>
                <div class="grid-item">
                    <div class="big" id="windowWidth">-</div>
                    <div class="small">窗口宽度</div>
                </div>
                <div class="grid-item">
                    <div class="big" id="windowHeight">-</div>
                    <div class="small">窗口高度</div>
                </div>
            </div>
        </div>
        
        <div class="card">
            <h2>ℹ️ 说明</h2>
            <p style="margin: 10px 0;">
                此页面展示了 Android WebView 的系统信息。
            </p>
            <p style="margin: 10px 0;">
                <strong>注意</strong>: Termux GUI 的 WebView 不支持自定义 User-Agent，
                显示的是 Android 系统默认的 WebView User-Agent。
            </p>
        </div>
    </div>
</body>
</html>"#.to_string()
}

/// 交互循环
fn interactive_loop(activity: &mut Activity, webview: &WebView) -> Result<()> {
    use std::sync::mpsc;
    use std::io::{self, BufRead, Write};
    
    let (tx, rx) = mpsc::channel();
    
    // 输入线程
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if handle.read_line(&mut input).is_ok() {
                if tx.send(input.trim().to_string()).is_err() {
                    break;
                }
            }
        }
    });
    
    // 事件循环
    loop {
        // 检查用户输入
        if let Ok(cmd) = rx.try_recv() {
            match cmd.as_str() {
                "1" => {
                    println!("🔍 加载 Google...");
                    webview.load_uri(activity, "https://www.google.com")?;
                }
                "2" => {
                    println!("💻 加载 GitHub...");
                    webview.load_uri(activity, "https://github.com")?;
                }
                "3" => {
                    println!("🔍 加载百度...");
                    webview.load_uri(activity, "https://www.baidu.com")?;
                }
                "4" => {
                    println!("📄 显示检测页面...");
                    let html = create_detection_page();
                    webview.set_data(activity, &html)?;
                    thread::sleep(Duration::from_millis(500));
                    webview.evaluate_js(activity, r#"
                        document.getElementById('ua-value').textContent = navigator.userAgent;
                        document.getElementById('platform').textContent = navigator.platform;
                        document.getElementById('language').textContent = navigator.language;
                        document.getElementById('online').textContent = navigator.onLine ? '在线' : '离线';
                        document.getElementById('cookieEnabled').textContent = navigator.cookieEnabled ? '启用' : '禁用';
                        document.getElementById('screenWidth').textContent = screen.width;
                        document.getElementById('screenHeight').textContent = screen.height;
                        document.getElementById('windowWidth').textContent = window.innerWidth;
                        document.getElementById('windowHeight').textContent = window.innerHeight;
                    "#)?;
                }
                "5" => {
                    println!("🌙 切换夜间模式...");
                    webview.evaluate_js(activity, r#"
                        if (document.body.style.filter === 'invert(1) hue-rotate(180deg)') {
                            document.body.style.filter = '';
                        } else {
                            document.body.style.filter = 'invert(1) hue-rotate(180deg)';
                        }
                    "#)?;
                }
                "6" => {
                    println!("🔍 放大字体...");
                    webview.evaluate_js(activity, r#"
                        document.body.style.fontSize = '20px';
                        const style = document.createElement('style');
                        style.textContent = '* { font-size: 20px !important; }';
                        document.head.appendChild(style);
                    "#)?;
                }
                "b" => {
                    println!("⬅️  后退...");
                    webview.go_back(activity)?;
                }
                "f" => {
                    println!("➡️  前进...");
                    webview.go_forward(activity)?;
                }
                "q" => {
                    println!("👋 退出...");
                    return Ok(());
                }
                "" => {}
                _ => println!("❌ 未知命令: {}", cmd),
            }
        }
        
        // 检查事件
        activity.event_stream().set_nonblocking(true).ok();
        if let Ok(event) = read_message(activity.event_stream()) {
            if let Some(event_type) = event.get("type").and_then(|v| v.as_str()) {
                if event_type == "destroy" {
                    println!("\n✓ Activity 已关闭");
                    return Ok(());
                }
            }
        }
        
        thread::sleep(Duration::from_millis(50));
    }
}

/// 等待 Activity 关闭
fn wait_for_destroy(activity: &mut Activity) -> Result<()> {
    loop {
        let event = read_message(activity.event_stream())?;
        if let Some(event_type) = event.get("type").and_then(|v| v.as_str()) {
            if event_type == "destroy" {
                return Ok(());
            }
        }
    }
}
