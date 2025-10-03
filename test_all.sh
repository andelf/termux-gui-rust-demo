#!/data/data/com.termux/files/usr/bin/bash

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     Termux:GUI Rust Demo - 测试所有示例                   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

cd ~/termux-gui-rust-demo

echo "1️⃣  测试基础 Hello World"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "运行命令: AUTO_MODE=1 ./target/release/termux-gui-rust-demo"
echo "预期: 显示 Hello World，5秒后更新，再5秒后退出"
echo ""
read -p "按 Enter 运行..."
AUTO_MODE=1 ./target/release/termux-gui-rust-demo
echo ""

echo "2️⃣  测试按钮交互"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "运行命令: cargo run --example button_demo --release"
echo "预期: 计数器界面，三个按钮可点击"
echo ""
read -p "按 Enter 运行（手动关闭界面）..."
cargo run --example button_demo --release
echo ""

echo "3️⃣  测试输入框交互 🆕"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "运行命令: cargo run --example input_demo --release"
echo "预期: 输入框界面，可输入和提交"
echo ""
read -p "按 Enter 运行（手动关闭界面）..."
cargo run --example input_demo --release
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                   所有测试完成！                           ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "如果所有示例都正常运行，恭喜你！"
echo "可以开始基于这些示例开发自己的应用了！"
echo ""

