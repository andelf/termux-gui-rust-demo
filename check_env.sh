#!/data/data/com.termux/files/usr/bin/bash

# Termux:GUI Rust Demo 环境检查脚本

echo "╔══════════════════════════════════════════════════════════╗"
echo "║     Termux:GUI Rust Demo - 环境检查                      ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

ERRORS=0
WARNINGS=0

# 1. 检查 Rust 环境
echo "━━━ 1. Rust 环境 ━━━"
if command -v rustc &> /dev/null; then
    echo "✅ Rust: $(rustc --version)"
else
    echo "❌ Rust 未安装"
    ERRORS=$((ERRORS + 1))
fi

if command -v cargo &> /dev/null; then
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Cargo 未安装"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# 2. 检查 Termux:GUI 插件
echo "━━━ 2. Termux:GUI 插件 ━━━"
if pm list packages | grep -q com.termux.gui; then
    echo "✅ Termux:GUI 已安装"
    VERSION=$(pm dump com.termux.gui | grep versionName | head -1 | cut -d= -f2)
    if [ -n "$VERSION" ]; then
        echo "   版本: $VERSION"
    fi
else
    echo "❌ Termux:GUI 未安装"
    echo "   请从以下渠道安装:"
    echo "   - F-Droid: https://f-droid.org/packages/com.termux.gui/"
    echo "   - GitHub: https://github.com/termux/termux-gui/releases"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# 3. 检查项目位置
echo "━━━ 3. 项目位置 ━━━"
CURRENT_DIR=$(pwd)
echo "当前目录: $CURRENT_DIR"
if [[ "$CURRENT_DIR" == /data/data/com.termux/files/home* ]]; then
    echo "✅ 在 HOME 目录"
else
    echo "⚠️  不在 HOME 目录"
    echo "   建议移动到: ~/termux-gui-rust-demo"
    WARNINGS=$((WARNINGS + 1))
fi
echo ""

# 4. 检查编译产物
echo "━━━ 4. 编译产物 ━━━"
BIN_PATH="$HOME/termux-gui-rust-demo/target/release/termux-gui-rust-demo"
if [ -f "$BIN_PATH" ]; then
    echo "✅ 二进制文件存在"
    SIZE=$(ls -lh "$BIN_PATH" | awk '{print $5}')
    echo "   大小: $SIZE"
    
    # 检查大小是否合理
    SIZE_BYTES=$(stat -c%s "$BIN_PATH" 2>/dev/null || stat -f%z "$BIN_PATH")
    if [ "$SIZE_BYTES" -gt 2000000 ]; then
        echo "⚠️  文件较大 (>2MB)，可能是 debug 版本"
        echo "   建议运行: cargo build --release"
        WARNINGS=$((WARNINGS + 1))
    fi
else
    echo "❌ 二进制文件不存在: $BIN_PATH"
    echo "   需要编译: cd ~/termux-gui-rust-demo && cargo build --release"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# 5. 检查执行权限
echo "━━━ 5. 文件权限 ━━━"
if [ -f "$BIN_PATH" ]; then
    if [ -x "$BIN_PATH" ]; then
        echo "✅ 可执行权限正确"
    else
        echo "❌ 缺少可执行权限"
        echo "   运行: chmod +x $BIN_PATH"
        ERRORS=$((ERRORS + 1))
    fi
else
    echo "⏭️  跳过（文件不存在）"
fi
echo ""

# 6. 检查必要工具
echo "━━━ 6. 系统工具 ━━━"
if command -v termux-am &> /dev/null || command -v am &> /dev/null; then
    echo "✅ Android broadcast 工具可用"
else
    echo "⚠️  broadcast 工具不可用"
    echo "   这可能影响与插件的通信"
    WARNINGS=$((WARNINGS + 1))
fi
echo ""

# 7. 检查依赖
echo "━━━ 7. 项目依赖 ━━━"
if [ -f "$HOME/termux-gui-rust-demo/Cargo.toml" ]; then
    echo "✅ Cargo.toml 存在"
    echo "   依赖:"
    grep "^[a-z]" "$HOME/termux-gui-rust-demo/Cargo.toml" | grep "=" | sed 's/^/   - /'
else
    echo "❌ Cargo.toml 不存在"
    ERRORS=$((ERRORS + 1))
fi
echo ""

# 8. 系统信息
echo "━━━ 8. 系统信息 ━━━"
echo "Android 版本: $(getprop ro.build.version.release)"
echo "架构: $(uname -m)"
echo "内核: $(uname -r)"
echo ""

# 总结
echo "╔══════════════════════════════════════════════════════════╗"
echo "║                    检查结果                              ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo "🎉 环境检查通过！可以运行程序了。"
    echo ""
    echo "运行命令:"
    echo "  cd ~/termux-gui-rust-demo"
    echo "  ./target/release/termux-gui-rust-demo"
    echo ""
    echo "或使用自动模式:"
    echo "  AUTO_MODE=1 ./target/release/termux-gui-rust-demo"
elif [ $ERRORS -eq 0 ]; then
    echo "⚠️  有 $WARNINGS 个警告，但应该可以运行。"
else
    echo "❌ 发现 $ERRORS 个错误，需要修复后才能运行。"
    echo ""
    echo "请查看上面的错误信息并修复。"
    echo "详细故障排查请参考: 故障排查.md"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

exit $ERRORS
