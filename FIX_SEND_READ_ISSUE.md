# 修复 send_read 导致的阻塞问题

## 问题描述

在使用新封装的库版本（button_demo_v2, test_lib_minimal等）时，程序会卡在创建控件后的设置操作中，界面只显示灰色半透明遮罩，或者只显示部分控件。

## 根本原因

Termux GUI协议中：
- **创建控件的方法**（如 `createTextView`, `createButton`）**需要等待响应**，返回创建的控件ID
- **设置控件属性的方法**（如 `setTextSize`, `setMargin`, `setWidth`, `setHeight`, `setLinearLayoutParams`）**不返回响应**

我们的封装代码错误地在所有View设置方法中使用了`send_read()`（等待响应），导致程序永远阻塞在等待一个不会到来的响应上。

## 修复内容

### 1. src/view.rs
将以下方法从`send_read()`改为`send()`：
- `set_width()` 
- `set_height()`
- `set_margin()`
- `set_linear_layout_params()`

### 2. src/components/text_view.rs
将以下方法从`send_read()`改为`send()`（如果存在）：
- `set_text()`
- `set_text_size()`
- `set_text_color()`

### 3. src/components/button.rs
将以下方法从`send_read()`改为`send()`（如果存在）：
- `set_text()`

## 规则总结

**简单规则：**
- **创建(create)** 方法 → 使用 `send_read()` 等待响应
- **设置(set)** 方法 → 使用 `send()` 不等待响应
- **获取(get)** 方法 → 使用 `send_read()` 等待响应

## 验证

修复后应该验证：
1. `./target/release/examples/button_demo_v2` - 应该显示完整按钮界面
2. `./target/release/examples/test_lib_minimal` - 应该显示Hello World
3. `./target/release/examples/button_demo_fullscreen` - 全屏版本应正常工作

## 后续任务

需要检查所有其他组件（Checkbox, Switch, Radio, Spinner, EditText等）的设置方法，确保它们也使用`send()`而不是`send_read()`。
