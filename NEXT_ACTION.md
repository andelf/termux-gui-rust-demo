# 下一步行动计划

## 📅 更新时间: 2025-01-04 (最新)

## ✅ 当前状态 - 里程碑达成！

### 核心库完成度
- ✅ 核心库创建完成 (src/lib.rs, src/activity.rs, src/connection.rs, src/view.rs)
- ✅ 8个基础组件封装完成并优化
- ✅ 所有send/send_read问题修复
- ✅ 布局参数支持 (WRAP_CONTENT, MATCH_PARENT, weight)
- ✅ 所有6个核心demo迁移完成！

### 已完成的Demo (v2版本)
- ✅ button_demo_v2 - 按钮交互演示
- ✅ checkbox_demo_v2 - 复选框多选演示
- ✅ input_demo_v2 - 文本输入演示
- ✅ switch_demo_v2 - 开关切换演示
- ✅ radio_demo_v2 - 单选按钮演示
- ✅ spinner_demo_v2 - 下拉列表演示（含级联选择）

### 已实现的组件
1. **TextView** - 文本显示
2. **Button** - 按钮
3. **EditText** - 文本输入（单行/多行）
4. **Checkbox** - 复选框
5. **Switch** - 开关
6. **RadioButton** - 单选按钮
7. **RadioGroup** - 单选组
8. **Spinner** - 下拉列表
9. **LinearLayout** - 线性布局（垂直/水平）
10. **NestedScrollView** - 嵌套滚动视图

---

## 🎯 下一阶段目标

### 阶段一：补充缺失的基础组件（优先级：高）

从Python框架对比，我们还缺少以下常用组件：

#### 1. ImageView - 图像显示 ⭐⭐⭐⭐⭐
- **用途**: 显示图片（PNG、JPEG等）
- **Python实现**: ✅ 已有
- **方法**: setimage(), setbuffer()
- **优先级**: 非常高，图片显示是基础功能

#### 2. ProgressBar - 进度条 ⭐⭐⭐⭐
- **用途**: 显示加载/进度状态
- **Python实现**: ✅ 已有
- **方法**: setprogress()
- **优先级**: 高，用户体验必需

#### 3. ToggleButton - 切换按钮 ⭐⭐⭐
- **用途**: 类似Switch，但样式不同
- **Python实现**: ✅ 已有
- **继承自**: CompoundButton

#### 4. Space - 空白间隔 ⭐⭐⭐
- **用途**: 在布局中添加空白空间
- **Python实现**: ✅ 已有
- **优先级**: 中，布局辅助工具

#### 5. FrameLayout - 帧布局 ⭐⭐⭐⭐
- **用途**: 简单的层叠布局
- **Python实现**: ✅ 已有
- **优先级**: 高，基础布局组件

#### 6. GridLayout - 网格布局 ⭐⭐⭐
- **用途**: 网格排列组件
- **Python实现**: ✅ 已有

#### 7. HorizontalScrollView - 水平滚动视图 ⭐⭐
- **用途**: 水平滚动容器
- **Python实现**: ✅ 已有

#### 8. TabLayout - 标签页布局 ⭐⭐⭐⭐
- **用途**: 多标签页界面
- **Python实现**: ✅ 已有
- **优先级**: 高，现代应用常用

#### 9. SwipeRefreshLayout - 下拉刷新 ⭐⭐⭐
- **用途**: 下拉刷新功能
- **Python实现**: ✅ 已有

#### 10. WebView - 网页视图 ⭐⭐⭐⭐⭐
- **用途**: 显示网页内容
- **Python实现**: ✅ 已有
- **方法**: loadurl(), evaluatejs(), etc.
- **优先级**: 非常高，功能强大

---

## 📋 实施计划

### 本周任务（Week 1）

#### Day 1: ImageView + ProgressBar
- [ ] 创建 `src/components/image_view.rs`
- [ ] 实现 ImageView::new(), set_image()
- [ ] 创建 `src/components/progress_bar.rs`
- [ ] 实现 ProgressBar::new(), set_progress()
- [ ] 添加 Activity 便捷方法
- [ ] 创建演示程序

#### Day 2: FrameLayout + Space
- [ ] 创建 `src/components/frame_layout.rs`
- [ ] 创建 `src/components/space.rs`
- [ ] 添加 Activity 便捷方法
- [ ] 创建演示程序

#### Day 3: ToggleButton
- [ ] 创建 `src/components/toggle_button.rs`
- [ ] 实现类似 Switch 的功能
- [ ] 创建演示程序

#### Day 4-5: TabLayout（复杂）
- [ ] 研究 Python TabLayout 实现
- [ ] 创建 `src/components/tab_layout.rs`
- [ ] 实现标签页切换逻辑
- [ ] 创建演示程序

### 下周任务（Week 2）

#### 高级组件
- [ ] GridLayout
- [ ] WebView（最复杂，需要特别注意）
- [ ] HorizontalScrollView
- [ ] SwipeRefreshLayout

---

## 🚀 快速开始

### 查看 Python 实现参考

```bash
# ImageView
cat ~/Documents/termux-gui-python-bindings/src/termuxgui/imageview.py

# ProgressBar
cat ~/Documents/termux-gui-python-bindings/src/termuxgui/progressbar.py

# ToggleButton
cat ~/Documents/termux-gui-python-bindings/src/termuxgui/togglebutton.py

# 查看所有组件
ls ~/Documents/termux-gui-python-bindings/src/termuxgui/*.py
```

### ImageView 组件模板

```rust
//! ImageView component

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// An ImageView displays images
pub struct ImageView {
    view: View,
    aid: i64,
}

impl ImageView {
    /// Create a new ImageView
    pub fn new(activity: &mut Activity, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id()
        });
        
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        let response = activity.send_read(&json!({
            "method": "createImageView",
            "params": params
        }))?;
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(ImageView {
            view: View::new(id),
            aid: activity.id(),
        })
    }
    
    pub fn id(&self) -> i64 {
        self.view.id()
    }
    
    pub fn view(&self) -> &View {
        &self.view
    }
    
    /// Set image from base64 encoded string
    pub fn set_image(&self, activity: &mut Activity, img_base64: &str) -> Result<()> {
        activity.send(&json!({
            "method": "setImage",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "img": img_base64
            }
        }))?;
        Ok(())
    }
}
```

---

## 💡 开发规范提醒

### 组件开发 Checklist
- [ ] 创建组件文件 `src/components/xxx.rs`
- [ ] 实现 `new()` 构造函数（使用 send_read）
- [ ] 实现必要的 set 方法（使用 send）
- [ ] 实现必要的 get 方法（使用 send_read）
- [ ] 提供 `id()` 和 `view()` 访问方法
- [ ] 在 `src/components/mod.rs` 中导出
- [ ] 在 `src/activity.rs` 中添加便捷方法
- [ ] 创建演示程序 `examples/xxx_demo_v2.rs`
- [ ] 测试并修复问题
- [ ] Git commit 记录

### 代码规则（重要！）

```rust
// ✅ 创建控件 → send_read()
let response = activity.send_read(&json!({
    "method": "createXXX",
    "params": params
}))?;

// ✅ 设置属性 → send()
activity.send(&json!({
    "method": "setXXX",
    "params": params
}))?;

// ✅ 获取属性 → send_read()
let response = activity.send_read(&json!({
    "method": "getXXX",
    "params": params
}))?;
```

### 退出处理模式

```rust
match event_type {
    "destroy" => {
        println!("\n✓ Activity 已关闭");
        return Ok(());
    },
    // ... 其他事件
}
```

---

## 📚 相关文档

### 已有文档
- ✅ **MIGRATION_COMPLETE.md** - 迁移完成总结
- ✅ **COMPONENT_FIX_SUMMARY.md** - 组件修复详情
- ✅ **FIX_SEND_READ_ISSUE.md** - send/send_read问题

### 待更新文档
- [ ] README.md - 添加新组件说明
- [ ] 组件完整列表和使用示例
- [ ] API 参考文档

---

## 🎯 本周目标

### 必须完成
1. ✅ 所有6个demo迁移完成（已完成！）
2. ⏳ 实现 ImageView 组件
3. ⏳ 实现 ProgressBar 组件
4. ⏳ 创建对应的演示程序

### 期望完成
5. ⏳ 实现 FrameLayout 组件
6. ⏳ 实现 Space 组件
7. ⏳ 实现 ToggleButton 组件

### 加分项
8. ⏳ 开始 TabLayout 研究
9. ⏳ 创建组件使用手册
10. ⏳ 准备 0.3.0 版本发布

---

## 🎉 当前成就

- ✅ 核心库架构完成
- ✅ 10个基础组件实现
- ✅ 6个完整演示程序
- ✅ 代码量减少52%
- ✅ 所有已知bug修复
- ✅ 开发规范建立
- ✅ 重要里程碑达成！

---

## 📊 组件完成度

### 已完成 (10/30+)
- TextView, Button, EditText
- Checkbox, Switch, RadioButton, RadioGroup
- Spinner, LinearLayout, NestedScrollView

### 进行中 (0/30+)
- 等待开始...

### 待实现 (20/30+)
- ImageView, ProgressBar, ToggleButton
- Space, FrameLayout, GridLayout
- TabLayout, WebView
- HorizontalScrollView, SwipeRefreshLayout
- 等等...

---

**继续加油！下一个目标是补充常用组件！** 🦀✨
