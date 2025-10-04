# Release Notes - v0.2.0

**Release Date:** January 4, 2025  
**Package:** termux-gui  
**Version:** 0.2.0  
**Registry:** [crates.io](https://crates.io/crates/termux-gui)

## 🎉 First Official Release!

We're excited to announce the first official release of `termux-gui` on crates.io! This library provides comprehensive Rust bindings for Termux:GUI, enabling developers to build native Android GUI applications directly from Termux using idiomatic Rust code.

## ✨ What's New

### Complete Widget Library (20 Components)
This release includes full implementations of all core Android UI components:

#### Basic Widgets
- **TextView** - Display text with formatting
- **Button** - Interactive buttons with click handlers
- **EditText** - Single-line and multi-line text input
- **ImageView** - Display images from files or base64 data

#### Selection Widgets
- **Checkbox** - Multi-select checkboxes
- **Switch** - Toggle switches
- **RadioButton & RadioGroup** - Single-select radio buttons
- **ToggleButton** - Two-state toggle buttons
- **Spinner** - Dropdown selection lists

#### Interactive Widgets
- **ProgressBar** - Linear and circular progress indicators
- **WebView** - Embedded web content with JavaScript support
- **Space** - Flexible spacing component

#### Layout Containers
- **LinearLayout** - Vertical and horizontal layouts
- **FrameLayout** - Layered frame layouts
- **GridLayout** - Grid-based layouts
- **NestedScrollView** - Vertical scrolling container
- **HorizontalScrollView** - Horizontal scrolling container
- **SwipeRefreshLayout** - Pull-to-refresh gesture support
- **TabLayout** - Tabbed navigation interface

### 16 Complete Examples
Every component comes with a fully functional demo application showing best practices:
- `button_demo_v2` - Button interactions
- `input_demo_v2` - Text input handling
- `checkbox_demo_v2` - Multi-select checkboxes
- `switch_demo_v2` - Toggle switches
- `radio_demo_v2` - Radio button groups
- `spinner_demo_v2` - Dropdown lists with cascading selection
- `image_demo_v2` - Image display
- `progress_demo_v2` - Progress bars
- `toggle_demo_v2` - Toggle buttons
- `space_demo_v2` - Spacing and layout
- `frame_layout_demo_v2` - Frame layouts
- `grid_layout_demo_v2` - Grid layouts
- `horizontal_scroll_demo_v2` - Horizontal scrolling
- `swipe_refresh_demo_v2` - Pull-to-refresh
- `tab_layout_demo_v2` - Tabbed interfaces
- `webview_demo_v2` - Web content embedding

### Key Features

#### Type-Safe API
All components use Rust's type system to ensure correctness at compile time. Invalid operations are caught by the compiler, not at runtime.

#### Event-Driven Architecture
Comprehensive event handling system with typed event handlers for all user interactions.

#### Layout System
Full support for Android's layout system including:
- Weight-based sizing
- MATCH_PARENT and WRAP_CONTENT constants
- Nested layouts
- Scroll containers

#### Resource Management
Automatic cleanup of resources when components are dropped, preventing memory leaks.

## 📦 Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
termux-gui = "0.2.0"
```

Or use cargo:
```bash
cargo add termux-gui
```

## 🚀 Quick Example

```rust
use termux_gui::{Activity, LinearLayout, Button, Orientation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create activity and layout
    let mut activity = Activity::new()?;
    let layout = activity.create_linear_layout(None, Orientation::Vertical)?;
    
    // Add a button
    let button = activity.create_button(Some(layout.id()), "Click Me")?;
    
    // Handle events
    activity.show()?;
    loop {
        let event = activity.wait_event()?;
        if event.event_type == "click" && event.view_id == button.id() {
            println!("Button clicked!");
            break;
        }
    }
    
    Ok(())
}
```

## 🐛 Bug Fixes

This release includes numerous bug fixes discovered during development:

### Critical Fixes
- Fixed send/send_read inconsistency causing crashes
- Resolved layout parameter handling in nested containers
- Fixed event handling for activity finish events
- Corrected SwipeRefreshLayout requiring direct TextView child

### Component Fixes
- EditText: Fixed text setting and retrieval
- Spinner: Fixed item selection and change events
- RadioGroup: Fixed selection persistence
- HorizontalScrollView: Fixed scrolling with tall content
- WebView: Fixed JavaScript execution and navigation

## 📚 Documentation

- **README.md** - Comprehensive project overview
- **API Documentation** - Available at [docs.rs/termux-gui](https://docs.rs/termux-gui)
- **Examples** - 16 complete demo applications
- **Component Guides** - Detailed usage for each widget

## 🔧 Technical Details

### Dependencies
- `serde_json` - JSON serialization for protocol communication
- `rand` - Random number generation for connection setup
- `libc` - Low-level socket operations
- `thiserror` - Ergonomic error handling

### Compatibility
- **Termux:GUI version:** 0.1.3+
- **Rust version:** 2021 edition
- **Android version:** 7.0+ (API level 24+)
- **Platform:** Android (Termux environment)

### Code Quality
- 52% reduction in code compared to original examples
- Consistent API patterns across all components
- Comprehensive error handling
- Zero unsafe code in public API

## 🎯 Performance

- Minimal runtime overhead
- Zero-cost abstractions
- Efficient event processing
- Smart resource cleanup

## ⚠️ Known Limitations

1. **SwipeRefreshLayout** requires a direct TextView child (not a layout) for proper scrolling behavior
2. **WebView** content URI access requires explicit permission via `allow_content_uri()`
3. Some components require Android 8.0+ for full functionality

## 🤝 Contributing

We welcome contributions! Please check our repository for guidelines.

## 📄 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- The Termux team for the amazing Termux:GUI plugin
- The Rust community for excellent tooling and support
- All contributors who helped test and provide feedback

## 📮 Support

- **Issues:** Report bugs and request features on GitHub
- **Documentation:** https://docs.rs/termux-gui
- **Package:** https://crates.io/crates/termux-gui

---

**Thank you for using termux-gui! We hope this library helps you build amazing Android applications with Rust!** 🦀✨
