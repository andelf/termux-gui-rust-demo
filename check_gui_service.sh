#!/bin/bash
# 检查并修复Termux GUI服务

echo "=== Termux GUI 服务诊断 ==="
echo

echo "1. 检查残留进程..."
ps aux | grep -E "termux-gui|spinner|button|test_" | grep -v grep
echo

echo "2. 清理残留进程..."
pkill -9 -f "spinner_demo"
pkill -9 -f "button_demo"
pkill -9 -f "test_lib"
pkill -9 -f "test_spinner"
echo "✓ 已清理"
echo

echo "3. 检查Termux GUI应用状态..."
# 尝试获取GUI服务包信息
pm list packages | grep termux.gui
echo

echo "4. 测试Python版本（验证服务是否正常）..."
cd /data/data/com.termux/files/home/Documents/termux-gui-python-bindings
timeout 5 python tutorial/helloworld.py 2>&1 &
PY_PID=$!
sleep 3
if ps -p $PY_PID > /dev/null 2>&1; then
    echo "⚠️  Python版本也卡住了，服务可能有问题"
    kill -9 $PY_PID 2>/dev/null
else
    echo "✓ Python版本可以运行"
fi
echo

echo "5. 建议操作："
echo "   - 如果Python版本也不工作，请在Android中打开Termux:GUI应用"
echo "   - 或者尝试: am force-stop com.termux.gui"
echo "   - 最后手段: 重启Android设备"
echo

echo "=== 诊断完成 ==="
