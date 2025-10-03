# 示例程序说明

本目录包含多个 Termux:GUI Rust 演示程序，展示不同的功能和使用方法。

## 📁 示例列表

### 1. button_demo.rs - 按钮交互演示

**功能**:
- ✅ 计数器应用
- ✅ 三个按钮：增加、减少、重置
- ✅ 动态颜色变化
- ✅ LinearLayout 布局

**运行**:
```bash
cargo run --example button_demo --release
```

**界面**:
```
┌───────────────────────────┐
│    计数器演示 🦀          │
│                           │
│    点击次数: 0            │
│    (动态变色)             │
│                           │
│  [➕ 增加]  [➖ 减少]     │
│                           │
│      [🔄 重置]            │
└───────────────────────────┘
```

**学习要点**:
- Button 创建和点击事件
- LinearLayout 布局（垂直和水平）
- TextView 动态更新
- 颜色设置

---

### 2. input_demo.rs - 输入框交互演示 🆕

**功能**:
- ✅ EditText 文本输入（单行）
- ✅ EditText 数字输入
- ✅ EditText 多行文本输入
- ✅ getText() 读取输入内容
- ✅ setText() 动态设置内容
- ✅ 输入验证和反馈
- ✅ 三个操作按钮

**运行**:
```bash
cargo run --example input_demo --release
```

**界面**:
```
┌───────────────────────────────┐
│    文本输入演示 📝            │
│                               │
│ 在下方输入你的名字:           │
│ [________________]            │
│                               │
│ 输入一个数字:                 │
│ [0_______________]            │
│                               │
│ 输入多行消息:                 │
│ [________________]            │
│ [________________]            │
│ [________________]            │
│                               │
│ [✅ 提交] [🗑️ 清空] [🧪 测试]│
│                               │
│ ━━━━━━━━━━━━━━━━━━━━━        │
│                               │
│ 结果将显示在这里...           │
└───────────────────────────────┘
```

**学习要点**:
- EditText 创建（text, number, textMultiLine）
- getText() 读取输入
- setText() 设置内容
- 输入验证
- 复杂布局组合

**操作说明**:
1. **输入内容**: 在三个输入框中输入内容
2. **提交**: 点击 ✅ 按钮，显示输入的内容和处理结果
3. **清空**: 点击 🗑️ 按钮，清除所有输入
4. **测试**: 点击 🧪 按钮，自动填充测试数据

**输出示例**:
```bash
=== 输入框交互演示 ===

✓ 连接建立
✓ 界面创建完成

━━━━━━━━━━━━━━━━━━━━━━
提示:
  • 在输入框中输入内容
  • 点击 '提交' 查看输入的内容
  • 点击 '清空' 清除所有输入
  • 点击 '测试' 填充测试数据
━━━━━━━━━━━━━━━━━━━━━━

📨 提交按钮被点击
   姓名: 张三
   数字: 42
   消息: 这是一条测试消息

🗑️ 清空按钮被点击

🧪 测试按钮被点击
```

---

## 🎯 功能对比

| 功能 | button_demo | input_demo |
|------|-------------|------------|
| Button | ✅ | ✅ |
| TextView | ✅ | ✅ |
| EditText | ❌ | ✅ |
| getText() | ❌ | ✅ |
| setText() | ✅ | ✅ |
| LinearLayout | ✅ | ✅ |
| 颜色设置 | ✅ | ✅ |
| 文本输入 | ❌ | ✅ |
| 数字输入 | ❌ | ✅ |
| 多行输入 | ❌ | ✅ |

---

## 📊 复杂度对比

| 示例 | 代码行数 | 控件数量 | 难度 |
|------|---------|---------|------|
| button_demo | ~180行 | 7个 | ⭐⭐ |
| input_demo | ~340行 | 12个 | ⭐⭐⭐⭐ |

---

## 🔧 技术细节

### EditText 创建参数

```rust
{
    "method": "createEditText",
    "params": {
        "aid": activity_id,          // Activity ID
        "text": "",                   // 初始文本
        "parent": parent_id,          // 父布局 ID
        "singleline": true,           // 是否单行
        "line": true,                 // 是否显示下划线
        "blockinput": false,          // 是否阻止输入
        "type": "text"                // 输入类型
    }
}
```

### 支持的输入类型

- `"text"` - 普通文本
- `"textMultiLine"` - 多行文本
- `"number"` - 数字
- `"numberDecimal"` - 小数
- `"numberSigned"` - 带符号数字
- `"phone"` - 电话号码
- `"textEmailAddress"` - 邮箱
- `"textPassword"` - 密码

### 读取输入内容

```rust
let response = send_and_read(&mut main_stream, &json!({
    "method": "getText",
    "params": {
        "aid": activity_id,
        "id": edittext_id
    }
}))?;

let text = response.as_str().unwrap_or("");
```

### 设置输入内容

```rust
send_message(&mut main_stream, &json!({
    "method": "setText",
    "params": {
        "aid": activity_id,
        "id": edittext_id,
        "text": "新内容"
    }
}))?;
```

---

## 💡 最佳实践

### 1. 输入验证

```rust
let name = name_response.as_str().unwrap_or("");

let result_text = if name.is_empty() {
    "⚠️ 请输入姓名！"
} else {
    format!("✅ 提交成功！\n姓名: {}", name)
};
```

### 2. 数字解析

```rust
let number_str = number_response.as_str().unwrap_or("0");
let number: i32 = number_str.parse().unwrap_or(0);
```

### 3. 视觉反馈

```rust
// 成功 - 绿色
let color = 0xFF4CAF50u32 as i32;

// 错误 - 红色  
let color = 0xFFF44336u32 as i32;

// 中性 - 蓝色
let color = 0xFF2196F3u32 as i32;

send_message(&mut main_stream, &json!({
    "method": "setTextColor",
    "params": {"aid": aid, "id": result_id, "color": color}
}))?;
```

---

## 🚀 扩展建议

### 基于 input_demo 可以实现:

1. **登录表单**
   - 用户名输入框
   - 密码输入框（type: "textPassword"）
   - 登录按钮

2. **计算器**
   - 两个数字输入框
   - 运算符选择
   - 计算结果显示

3. **待办事项**
   - 任务输入框
   - 添加按钮
   - 任务列表显示

4. **聊天界面**
   - 消息输入框
   - 发送按钮
   - 消息历史显示

---

## 🎓 学习路径

### 初学者
1. ✅ 运行 button_demo
2. ✅ 运行 input_demo
3. ✅ 阅读源代码注释
4. ✅ 修改文本内容重新编译

### 进阶开发
1. ✅ 理解 EditText 参数
2. ✅ 掌握 getText/setText
3. ✅ 实现输入验证
4. ✅ 添加更多输入类型测试

### 高级应用
1. ✅ 组合多种控件
2. ✅ 实现复杂交互逻辑
3. ✅ 添加数据持久化
4. ✅ 创建完整应用

---

## 📝 常见问题

### Q: input_demo 没有响应输入？

**A**: 确保：
1. EditText 创建成功（检查返回的 ID）
2. 焦点在输入框上（点击输入框）
3. 软键盘已弹出

### Q: getText() 返回空字符串？

**A**: 检查：
1. EditText ID 是否正确
2. 是否在输入框中输入了内容
3. 使用 `unwrap_or("")` 处理 None 情况

### Q: 多行输入框无法换行？

**A**: 确保：
1. `"singleline": false`
2. `"type": "textMultiLine"`

---

## 🔗 相关资源

- [Termux:GUI 官方文档](https://github.com/termux/termux-gui)
- [Python 实现参考](https://github.com/tareksander/termux-gui-python-bindings)
- [项目主 README](../README.md)
- [事件和控件实现指南](../事件和控件实现指南.md)

---

**最后更新**: 2025  
**维护者**: Demo Project  
**许可证**: 演示用途
