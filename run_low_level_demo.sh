#!/bin/bash
# Run the low-level protocol demo
# This demonstrates direct Unix socket communication with Termux:GUI service
cd /data/data/com.termux/files/home/termux-gui-rust-demo
echo "🔧 启动底层协议演示..."
./target/release/examples/low_level_protocol_demo
