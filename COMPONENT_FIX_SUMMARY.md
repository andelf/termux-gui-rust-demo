# 组件库 send/send_read 修复总结

## 问题描述

所有组件的 `set` 方法错误地使用了 `send_read()` 而非 `send()`，导致程序在设置属性时永久阻塞，因为这些方法不返回响应。

## 核心规则

```
create 方法 → send_read() (等待响应，返回ID)
set 方法    → send()      (不等待响应)
get 方法    → send_read() (等待响应，返回值)
```

## 已修复的组件

### 1. EditText
- ✅ `set_text()` - 从 send_read 改为 send
- ✅ `set_hint()` - 从 send_read 改为 send
- ✅ 新增 `get_text()` - 使用 send_read (正确)
- ✅ 新增必需参数: singleline, line, blockinput, type
- ✅ 新增 `new_multiline()` 方法

### 2. Checkbox
- ✅ `set_text()` - 从 send_read 改为 send
- ✅ `set_checked()` - 从 send_read 改为 send

### 3. Switch
- ✅ `set_text()` - 从 send_read 改为 send
- ✅ `set_checked()` - 从 send_read 改为 send

### 4. RadioButton
- ✅ `set_text()` - 从 send_read 改为 send
- ✅ `set_checked()` - 从 send_read 改为 send

### 5. Spinner
- ✅ `set_list()` - 从 send_read 改为 send
- ✅ `refresh()` - 从 send_read 改为 send

### 6. TextView (之前已修复)
- ✅ `set_text()` - 使用 send
- ✅ `set_text_size()` - 使用 send
- ✅ `set_text_color()` - 使用 send

### 7. Button (之前已修复)
- ✅ `set_text()` - 使用 send

### 8. View (之前已修复)
- ✅ `set_width()` - 使用 send
- ✅ `set_height()` - 使用 send
- ✅ `set_margin()` - 使用 send
- ✅ `set_linear_layout_params()` - 使用 send

## 相关文档

- `FIX_SEND_READ_ISSUE.md` - 最初发现问题的文档
- `DEBUG_BUTTON_ISSUE.md` - Button 问题调试过程

## 测试验证

所有组件的 demo 现在应该都能正常工作:
- ✅ button_demo_v2 - 按钮演示
- ✅ checkbox_demo_v2 - 复选框演示
- ✅ input_demo_v2 - 输入框演示
- 待迁移: switch_demo_v2, radio_demo_v2, spinner_demo_v2

## Git 提交记录

1. `fix: 修复 checkbox_demo_v2 退出问题` (e9f7579)
2. `feat: 完成 input_demo_v2 迁移到新库` (1742138)
3. `fix: 修复所有组件的 send_read 误用问题` (5f9b9d9)

## 经验教训

1. **严格遵循协议规范** - Termux:GUI 协议明确定义了哪些方法有响应，哪些没有
2. **一次性检查所有组件** - 发现一个问题后应该立即检查所有类似代码
3. **添加调试信息有助于快速定位** - DEBUG 日志帮助我们发现程序卡在 send_read
4. **参考已知可工作的代码** - 对比老版本 demo 的参数设置非常重要

## 下一步

继续迁移剩余的 demo 到新库:
- [ ] switch_demo_v2
- [ ] radio_demo_v2  
- [ ] spinner_demo_v2

现在所有基础组件都已正确实现，迁移工作应该会更顺利。
