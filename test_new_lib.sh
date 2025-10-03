#!/bin/bash
# 测试新库示例
# 使用方法: ./test_new_lib.sh [example_name]
# 不带参数则依次测试所有示例

set -e

cd "$(dirname "$0")"

# 新库示例列表
NEW_EXAMPLES=(
    "test_lib_minimal"
    "test_single_button"
    "button_demo_v2"
    "button_demo_v3_debug"
    "button_demo_fullscreen"
)

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Termux GUI 新库测试工具 ===${NC}\n"

# 如果指定了示例名称，只测试该示例
if [ ! -z "$1" ]; then
    EXAMPLE="$1"
    echo -e "${YELLOW}测试单个示例: ${EXAMPLE}${NC}\n"
    
    echo -e "${GREEN}[1/2]${NC} 编译示例..."
    cargo build --example "$EXAMPLE" --release
    
    echo -e "\n${GREEN}[2/2]${NC} 运行示例..."
    echo -e "${YELLOW}提示: 按 Ctrl+C 退出${NC}\n"
    ./target/release/examples/"$EXAMPLE"
    
    exit 0
fi

# 测试所有示例
echo -e "${BLUE}可用的新库示例:${NC}\n"
for i in "${!NEW_EXAMPLES[@]}"; do
    echo -e "  ${GREEN}$((i+1)).${NC} ${NEW_EXAMPLES[$i]}"
done

echo -e "\n${YELLOW}选择要测试的示例 (1-${#NEW_EXAMPLES[@]})，或按回车依次测试所有:${NC} "
read -r choice

if [ -z "$choice" ]; then
    # 测试所有示例
    echo -e "\n${BLUE}开始测试所有示例...${NC}\n"
    
    for EXAMPLE in "${NEW_EXAMPLES[@]}"; do
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${YELLOW}测试: ${EXAMPLE}${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
        
        echo "编译中..."
        if cargo build --example "$EXAMPLE" --release 2>&1 | tail -3; then
            echo -e "\n${GREEN}✓ 编译成功${NC}"
            echo -e "${YELLOW}提示: 在设备上测试，按返回键关闭${NC}\n"
            
            ./target/release/examples/"$EXAMPLE" || true
            
            echo -e "\n${GREEN}✓ ${EXAMPLE} 测试完成${NC}\n"
        else
            echo -e "${RED}✗ 编译失败${NC}\n"
        fi
        
        sleep 1
    done
    
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}✓ 所有测试完成！${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
else
    # 测试选定的示例
    if [ "$choice" -ge 1 ] && [ "$choice" -le "${#NEW_EXAMPLES[@]}" ]; then
        EXAMPLE="${NEW_EXAMPLES[$((choice-1))]}"
        echo -e "\n${YELLOW}测试: ${EXAMPLE}${NC}\n"
        
        echo "编译中..."
        cargo build --example "$EXAMPLE" --release
        
        echo -e "\n${GREEN}✓ 编译成功${NC}"
        echo -e "${YELLOW}提示: 按 Ctrl+C 或关闭窗口退出${NC}\n"
        
        ./target/release/examples/"$EXAMPLE"
    else
        echo -e "${RED}✗ 无效的选择${NC}"
        exit 1
    fi
fi
