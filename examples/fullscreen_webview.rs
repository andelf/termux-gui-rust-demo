//! 全屏 WebView 演示应用
//!
//! 这个演示创建一个占据整个屏幕的 WebView，展示其完整功能：
//! - 全屏显示
//! - 加载网页
//! - JavaScript 支持
//! - 导航功能

use termux_gui::{Activity, WebView, LinearLayout, Result};
use termux_gui::connection::read_message;
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<()> {
    println!("🌐 全屏 WebView 演示应用");
    println!("━━━━━━━━━━━━━━━━━━━━━━");
    
    // 创建 Activity
    let mut activity = Activity::new(false)?;
    println!("✓ Activity 已创建");
    
    // 创建全屏 LinearLayout
    let root = activity.create_linear_layout(None)?;
    println!("✓ 根布局已创建");
    
    // 创建 WebView（占据整个屏幕）
    let webview = activity.create_web_view(Some(root.id()))?;
    
    // 设置 WebView 占据所有可用空间
    webview.view().set_linear_layout_params(&mut activity, 1, None)?;
    println!("✓ 全屏 WebView 已创建");
    
    // 启用 JavaScript
    println!("\n📱 正在请求 JavaScript 权限...");
    println!("   请在弹出的对话框中点击【允许】");
    
    let js_enabled = webview.allow_javascript(&mut activity, true)?;
    
    if js_enabled {
        println!("✅ JavaScript 已启用");
        
        // 加载一个精美的 HTML 页面
        let html = create_demo_html();
        webview.set_data(&mut activity, &html)?;
        println!("✓ 已加载演示页面");
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━");
        println!("📋 可用命令:");
        println!("  1 - 显示演示 HTML 页面（当前）");
        println!("  2 - 加载 Google 搜索");
        println!("  3 - 加载 Bing 搜索");
        println!("  4 - 加载 GitHub");
        println!("  5 - 执行 JavaScript 动画");
        println!("  6 - 修改页面背景色");
        println!("  b - 后退");
        println!("  f - 前进");
        println!("  q - 退出");
        println!("━━━━━━━━━━━━━━━━━━━━━━\n");
        
        // 启动交互式命令循环
        interactive_mode(&mut activity, &webview)?;
    } else {
        println!("⚠️  JavaScript 未启用（用户拒绝）");
        println!("   仍可以加载网页，但无法执行 JavaScript");
        
        // 不启用 JavaScript 的情况下，加载简单的 HTML
        let simple_html = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {
            margin: 0;
            padding: 20px;
            font-family: Arial, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%) !important;
            color: #ffffff !important;
            text-align: center;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
        }
        h1 { font-size: 2.5em; margin-bottom: 20px; color: #ffffff !important; }
        p { font-size: 1.3em; line-height: 1.6; color: #ffffff !important; }
    </style>
</head>
<body>
    <h1>🌐 WebView 演示</h1>
    <p>这是一个全屏 WebView 应用</p>
    <p>JavaScript 未启用</p>
    <p>但仍可以显示静态 HTML 内容</p>
</body>
</html>
        "#;
        webview.set_data(&mut activity, simple_html)?;
        
        println!("\n按 Ctrl+C 或关闭窗口退出...");
        wait_for_destroy(&mut activity)?;
    }
    
    Ok(())
}

/// 创建演示 HTML 页面
fn create_demo_html() -> String {
    r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebView 演示</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #0f0c29 0%, #302b63 50%, #24243e 100%) !important;
            color: #ffffff !important;
            min-height: 100vh;
            overflow-x: hidden;
        }
        
        body *, h1, h2, h3, p, span, div {
            color: #ffffff !important;
        }
        
        .container {
            padding: 40px 20px;
            max-width: 800px;
            margin: 0 auto;
        }
        
        .header {
            text-align: center;
            margin-bottom: 40px;
        }
        
        .logo {
            font-size: 5em;
            margin-bottom: 20px;
            animation: float 3s ease-in-out infinite;
        }
        
        @keyframes float {
            0%, 100% { transform: translateY(0px); }
            50% { transform: translateY(-20px); }
        }
        
        h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        
        .subtitle {
            font-size: 1.2em;
            opacity: 0.9;
        }
        
        .card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 20px;
            padding: 30px;
            margin: 20px 0;
            border: 1px solid rgba(255, 255, 255, 0.2);
            transition: transform 0.3s ease;
        }
        
        .card:hover {
            transform: translateY(-5px);
        }
        
        .feature {
            display: flex;
            align-items: center;
            margin: 15px 0;
            font-size: 1.1em;
        }
        
        .feature-icon {
            font-size: 2em;
            margin-right: 15px;
        }
        
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 20px;
            margin: 30px 0;
        }
        
        .stat-card {
            background: rgba(255, 255, 255, 0.15);
            padding: 20px;
            border-radius: 15px;
            text-align: center;
        }
        
        .stat-number {
            font-size: 2.5em;
            font-weight: bold;
            display: block;
            margin-bottom: 5px;
        }
        
        .stat-label {
            font-size: 0.9em;
            opacity: 0.9;
        }
        
        .time-display {
            background: rgba(255, 255, 255, 0.2);
            padding: 20px;
            border-radius: 15px;
            text-align: center;
            font-size: 1.5em;
            margin: 20px 0;
        }
        
        .buttons {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin: 20px 0;
        }
        
        button {
            flex: 1;
            min-width: 120px;
            padding: 15px 25px;
            font-size: 1em;
            background: rgba(255, 255, 255, 0.2);
            border: 2px solid rgba(255, 255, 255, 0.3);
            border-radius: 10px;
            color: white;
            cursor: pointer;
            transition: all 0.3s ease;
        }
        
        button:hover {
            background: rgba(255, 255, 255, 0.3);
            transform: scale(1.05);
        }
        
        button:active {
            transform: scale(0.95);
        }
        
        .footer {
            text-align: center;
            margin-top: 40px;
            padding-top: 20px;
            border-top: 1px solid rgba(255, 255, 255, 0.2);
            opacity: 0.8;
            font-size: 0.9em;
        }
        
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
        
        .pulse {
            animation: pulse 2s ease-in-out infinite;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="logo">🚀</div>
            <h1>WebView 全屏演示</h1>
            <p class="subtitle">Termux GUI - Rust 版本</p>
        </div>
        
        <div class="card">
            <h2 style="margin-bottom: 20px;">✨ 功能特性</h2>
            <div class="feature">
                <span class="feature-icon">📱</span>
                <span>全屏显示，完美适配</span>
            </div>
            <div class="feature">
                <span class="feature-icon">⚡</span>
                <span>JavaScript 支持，动态交互</span>
            </div>
            <div class="feature">
                <span class="feature-icon">🎨</span>
                <span>CSS3 动画和特效</span>
            </div>
            <div class="feature">
                <span class="feature-icon">🌐</span>
                <span>加载外部网页</span>
            </div>
            <div class="feature">
                <span class="feature-icon">🔄</span>
                <span>前进/后退导航</span>
            </div>
        </div>
        
        <div class="stats">
            <div class="stat-card">
                <span class="stat-number" id="counter">0</span>
                <span class="stat-label">计数器</span>
            </div>
            <div class="stat-card">
                <span class="stat-number">100%</span>
                <span class="stat-label">功能完整</span>
            </div>
            <div class="stat-card">
                <span class="stat-number">∞</span>
                <span class="stat-label">可能性</span>
            </div>
        </div>
        
        <div class="time-display" id="time">
            加载中...
        </div>
        
        <div class="card">
            <h3 style="margin-bottom: 15px;">🎮 交互按钮</h3>
            <div class="buttons">
                <button onclick="changeColor()">🎨 变色</button>
                <button onclick="startAnimation()">🎪 动画</button>
                <button onclick="incrementCounter()">➕ 计数</button>
                <button onclick="showAlert()">💬 提示</button>
            </div>
        </div>
        
        <div class="footer">
            <p>Termux GUI Rust Demo</p>
            <p class="pulse">● 运行中</p>
        </div>
    </div>
    
    <script>
        // 计数器
        let counter = 0;
        
        function incrementCounter() {
            counter++;
            document.getElementById('counter').textContent = counter;
        }
        
        // 随机颜色生成
        function getRandomColor() {
            const colors = [
                'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
                'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
                'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
                'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
                'linear-gradient(135deg, #fa709a 0%, #fee140 100%)',
                'linear-gradient(135deg, #30cfd0 0%, #330867 100%)',
                'linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)',
                'linear-gradient(135deg, #ff9a9e 0%, #fecfef 100%)'
            ];
            return colors[Math.floor(Math.random() * colors.length)];
        }
        
        function changeColor() {
            document.body.style.background = getRandomColor();
        }
        
        function startAnimation() {
            const logo = document.querySelector('.logo');
            logo.style.animation = 'none';
            setTimeout(() => {
                logo.style.animation = 'float 3s ease-in-out infinite, spin 2s linear infinite';
            }, 10);
            
            // 添加旋转动画
            const style = document.createElement('style');
            style.textContent = `
                @keyframes spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
            `;
            document.head.appendChild(style);
        }
        
        function showAlert() {
            alert('👋 你好！这是一个全屏 WebView 应用\n\n使用 Termux GUI + Rust 构建');
        }
        
        // 更新时间
        function updateTime() {
            const now = new Date();
            const timeStr = now.toLocaleString('zh-CN', {
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                hour: '2-digit',
                minute: '2-digit',
                second: '2-digit',
                hour12: false
            });
            document.getElementById('time').textContent = '🕐 ' + timeStr;
        }
        
        // 初始化
        updateTime();
        setInterval(updateTime, 1000);
        
        // 自动增加计数器
        setInterval(() => {
            incrementCounter();
        }, 3000);
        
        console.log('✅ WebView 演示页面加载完成');
    </script>
</body>
</html>"#.to_string()
}

/// 交互模式 - 接受用户命令
fn interactive_mode(activity: &mut Activity, webview: &WebView) -> Result<()> {
    use std::sync::mpsc;
    use std::thread;
    use std::os::unix::net::UnixStream;
    
    // 创建通道用于接收用户输入
    let (tx, rx) = mpsc::channel();
    
    // 启动输入线程
    thread::spawn(move || {
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if stdin.read_line(&mut input).is_ok() {
                let cmd = input.trim().to_string();
                if tx.send(cmd).is_err() {
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
                    println!("📄 加载演示 HTML 页面...");
                    let html = create_demo_html();
                    webview.set_data(activity, &html)?;
                    println!("✓ 已加载");
                }
                "2" => {
                    println!("🔍 加载 Google 搜索...");
                    webview.load_uri(activity, "https://www.google.com")?;
                    println!("✓ 已发送加载请求");
                }
                "3" => {
                    println!("🔍 加载 Bing 搜索...");
                    webview.load_uri(activity, "https://www.bing.com")?;
                    println!("✓ 已发送加载请求");
                }
                "4" => {
                    println!("💻 加载 GitHub...");
                    webview.load_uri(activity, "https://github.com")?;
                    println!("✓ 已发送加载请求");
                }
                "5" => {
                    println!("🎪 执行 JavaScript 动画...");
                    let js = r#"
                        document.querySelector('.logo').style.animation = 
                            'float 3s ease-in-out infinite, spin 1s linear infinite';
                        document.body.style.background = 
                            'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)';
                    "#;
                    webview.evaluate_js(activity, js)?;
                    println!("✓ JavaScript 已执行");
                }
                "6" => {
                    println!("🎨 修改页面背景色...");
                    let js = r#"
                        const colors = [
                            'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
                            'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
                            'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
                            'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)'
                        ];
                        document.body.style.background = 
                            colors[Math.floor(Math.random() * colors.length)];
                    "#;
                    webview.evaluate_js(activity, js)?;
                    println!("✓ 背景已更改");
                }
                "b" => {
                    println!("⬅️  后退...");
                    webview.go_back(activity)?;
                    println!("✓ 已后退");
                }
                "f" => {
                    println!("➡️  前进...");
                    webview.go_forward(activity)?;
                    println!("✓ 已前进");
                }
                "q" => {
                    println!("👋 退出应用...");
                    return Ok(());
                }
                "" => {} // 空输入，忽略
                _ => {
                    println!("❌ 未知命令: {}", cmd);
                    println!("   输入 1-6、b、f 或 q");
                }
            }
        }
        
        // 检查 Activity 事件（非阻塞）
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
                println!("✓ Activity 已关闭");
                return Ok(());
            }
        }
    }
}
