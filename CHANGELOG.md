# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2024-10-04

### Added
- ✨ Initial release of Termux:GUI Rust bindings
- 🎨 9 complete working examples:
  - `main.rs` - Basic Hello World
  - `button_demo.rs` - Button with click counter
  - `input_demo.rs` - EditText input handling
  - `checkbox_demo.rs` - Checkbox state management
  - `switch_demo.rs` - Switch toggle control
  - `radio_demo.rs` - RadioGroup with RadioButtons
  - `spinner_demo.rs` - Dropdown selector with cascading options
  - `test_events.rs` - Event handling demonstration
  - And more test variants
- 📚 Comprehensive documentation in both English and Chinese
- 🔧 Proper event handling with destroy event support
- 🎯 Complete API examples for common UI components

### Components Implemented
- TextView - Display text with customizable size and color
- EditText - Text input field
- Button - Clickable buttons
- Checkbox - Checkable box
- Switch - Toggle switch
- RadioButton & RadioGroup - Single-choice selection
- Spinner - Dropdown list selector
- LinearLayout - Linear layout container
- NestedScrollView - Scrollable container

### Fixed
- 🐛 Fixed Spinner display issue by correctly using `send_message` vs `send_and_read`
- 🐛 Proper layout management with NestedScrollView for scrollable content
- 🐛 Correct event handling patterns
- 🐛 Activity destroy event handling

### Documentation
- Complete README with quick start guide
- Component implementation progress tracking
- Troubleshooting guides (故障排查.md)
- Example program overview (示例程序总览.md)
- Architecture comparison with Python bindings (架构对比.md)

[0.1.0]: https://github.com/andelf/termux-gui-rust-demo/releases/tag/v0.1.0
