#!/data/data/com.termux/files/usr/bin/bash

# Termux:GUI Rust Demo 运行脚本

echo "==================================="
echo "  Termux:GUI Rust Demo"
echo "==================================="
echo ""
echo "确保你已经安装了 Termux:GUI 插件"
echo "从 F-Droid 或 GitHub 下载安装"
echo ""
echo "按 Enter 键启动程序..."
read

# 运行程序
./target/release/termux-gui-rust-demo

echo ""
echo "程序已结束"
