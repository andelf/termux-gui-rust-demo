//! 简单的 WebView 测试 - 确保颜色对比清晰

use termux_gui::{Activity, WebView, Result};
use termux_gui::connection::read_message;

fn main() -> Result<()> {
    println!("🌐 简单 WebView 测试");
    
    let mut activity = Activity::new(false)?;
    let root = activity.create_linear_layout(None)?;
    let webview = activity.create_web_view(Some(root.id()))?;
    webview.view().set_linear_layout_params(&mut activity, 1, None)?;
    
    println!("✓ WebView 已创建");
    println!("📱 请求 JavaScript 权限（请允许）...");
    
    webview.allow_javascript(&mut activity, true)?;
    
    let html = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebView 测试</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: Arial, sans-serif;
            background: #1a1a2e !important;
            color: #ffffff !important;
            padding: 30px;
            min-height: 100vh;
        }
        
        h1 {
            font-size: 2.5em;
            margin-bottom: 30px;
            color: #00ff88 !important;
            text-align: center;
            text-shadow: 0 0 10px rgba(0, 255, 136, 0.5);
        }
        
        .card {
            background: #2a2a4e;
            padding: 25px;
            margin: 20px 0;
            border-radius: 15px;
            border: 2px solid #00ff88;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
        }
        
        .card h2 {
            color: #ffeb3b !important;
            margin-bottom: 15px;
            font-size: 1.5em;
        }
        
        .info {
            font-size: 1.2em;
            line-height: 1.8;
            color: #ffffff !important;
        }
        
        .highlight {
            color: #00ff88 !important;
            font-weight: bold;
        }
        
        .emoji {
            font-size: 3em;
            text-align: center;
            margin: 20px 0;
        }
        
        button {
            background: #00ff88;
            color: #1a1a2e !important;
            border: none;
            padding: 15px 30px;
            font-size: 1.2em;
            border-radius: 10px;
            cursor: pointer;
            margin: 10px 5px;
            font-weight: bold;
            box-shadow: 0 4px 10px rgba(0, 255, 136, 0.3);
        }
        
        button:active {
            transform: scale(0.95);
        }
        
        .status {
            background: rgba(0, 255, 136, 0.1);
            padding: 15px;
            border-radius: 10px;
            margin: 15px 0;
            border-left: 4px solid #00ff88;
        }
        
        .test-colors {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 15px;
            margin: 20px 0;
        }
        
        .color-box {
            padding: 20px;
            text-align: center;
            border-radius: 10px;
            font-weight: bold;
        }
        
        .color-white { background: #ffffff; color: #000000 !important; }
        .color-black { background: #000000; color: #ffffff !important; }
        .color-red { background: #ff5252; color: #ffffff !important; }
        .color-green { background: #00ff88; color: #000000 !important; }
        .color-blue { background: #2196f3; color: #ffffff !important; }
        .color-yellow { background: #ffeb3b; color: #000000 !important; }
    </style>
</head>
<body>
    <div class="emoji">✅</div>
    <h1>WebView 颜色测试成功！</h1>
    
    <div class="card">
        <h2>📋 当前配置</h2>
        <div class="info">
            <p>✅ 背景色: <span class="highlight">深色 (#1a1a2e)</span></p>
            <p>✅ 文字颜色: <span class="highlight">白色 (#ffffff)</span></p>
            <p>✅ 强调色: <span class="highlight">绿色 (#00ff88)</span></p>
            <p>✅ JavaScript: <span class="highlight">已启用</span></p>
        </div>
    </div>
    
    <div class="card">
        <h2>🎨 颜色对比测试</h2>
        <div class="test-colors">
            <div class="color-box color-white">白色背景</div>
            <div class="color-box color-black">黑色背景</div>
            <div class="color-box color-red">红色背景</div>
            <div class="color-box color-green">绿色背景</div>
            <div class="color-box color-blue">蓝色背景</div>
            <div class="color-box color-yellow">黄色背景</div>
        </div>
    </div>
    
    <div class="card">
        <h2>🎮 交互测试</h2>
        <div style="text-align: center;">
            <button onclick="changeBackground()">🎨 随机背景</button>
            <button onclick="showAlert()">💬 显示提示</button>
            <button onclick="updateTime()">⏰ 显示时间</button>
        </div>
        <div class="status" id="status">
            点击按钮测试交互功能
        </div>
    </div>
    
    <div class="card">
        <h2>ℹ️ 系统信息</h2>
        <div class="info" id="system-info">
            加载中...
        </div>
    </div>
    
    <script>
        // 显示系统信息
        const info = `
            <p>🌐 User-Agent: ${navigator.userAgent.substring(0, 50)}...</p>
            <p>📱 平台: ${navigator.platform}</p>
            <p>🌍 语言: ${navigator.language}</p>
            <p>📊 屏幕: ${screen.width} × ${screen.height}</p>
            <p>🔌 在线: ${navigator.onLine ? '是' : '否'}</p>
        `;
        document.getElementById('system-info').innerHTML = info;
        
        // 背景颜色数组
        const backgrounds = [
            'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)',
            'linear-gradient(135deg, #0f0c29 0%, #302b63 50%, #24243e 100%)',
            'linear-gradient(135deg, #232526 0%, #414345 100%)',
            'linear-gradient(135deg, #000000 0%, #434343 100%)',
            'linear-gradient(135deg, #1e3c72 0%, #2a5298 100%)',
            'linear-gradient(135deg, #134e5e 0%, #71b280 100%)'
        ];
        
        function changeBackground() {
            const randomBg = backgrounds[Math.floor(Math.random() * backgrounds.length)];
            document.body.style.background = randomBg;
            document.getElementById('status').innerHTML = 
                '<span class="highlight">✓</span> 背景已更改';
        }
        
        function showAlert() {
            alert('🎉 JavaScript 工作正常！\n\n这证明 WebView 已正确配置。');
            document.getElementById('status').innerHTML = 
                '<span class="highlight">✓</span> 提示框已显示';
        }
        
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
            document.getElementById('status').innerHTML = 
                '<span class="highlight">⏰</span> 当前时间: ' + timeStr;
        }
        
        console.log('✅ WebView 页面加载完成');
        console.log('背景色: #1a1a2e (深色)');
        console.log('文字色: #ffffff (白色)');
    </script>
</body>
</html>"#;
    
    webview.set_data(&mut activity, html)?;
    println!("✓ 页面已加载");
    println!("\n🎨 颜色方案:");
    println!("  背景: 深蓝黑色 (#1a1a2e)");
    println!("  文字: 白色 (#ffffff)");
    println!("  强调: 绿色 (#00ff88)");
    println!("\n按 Ctrl+C 或关闭窗口退出...\n");
    
    // 等待关闭
    loop {
        let event = read_message(activity.event_stream())?;
        if let Some(event_type) = event.get("type").and_then(|v| v.as_str()) {
            if event_type == "destroy" {
                println!("✓ 已关闭");
                return Ok(());
            }
        }
    }
}
