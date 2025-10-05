# Termux:GUI Rust Bindings

[![Crates.io](https://img.shields.io/crates/v/termux-gui.svg)](https://crates.io/crates/termux-gui)
[![Documentation](https://docs.rs/termux-gui/badge.svg)](https://docs.rs/termux-gui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for building Android GUI applications using Termux:GUI. This project provides idiomatic Rust bindings for the Termux:GUI plugin, making it easy to create native Android interfaces from the Termux terminal environment.

## Features

✅ **Complete Termux:GUI Protocol Implementation** - Full support for the low-level communication protocol  
✅ **Rich Widget Library** - 12+ UI components (Button, TextView, EditText, WebView, etc.)  
✅ **Advanced Layouts** - 7 layout types including Linear, Grid, Frame, Tab, and Scroll layouts  
✅ **WebView Integration** - Full web content support with JavaScript execution  
✅ **Type Safety** - Leverages Rust's type system for compile-time correctness  
✅ **Memory Safety** - Zero-cost abstractions with guaranteed memory safety  
✅ **Comprehensive Examples** - 21 working examples covering all components  
✅ **International Ready** - 100% English documentation, ready for worldwide use

## Quick Start

### Installation

Add this library to your Rust project:

```bash
cargo add termux-gui
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
termux-gui = "0.3.0"
```

### Prerequisites

1. Install [Termux](https://termux.com/) on your Android device
2. Install the [Termux:GUI](https://github.com/termux/termux-gui) plugin from F-Droid
3. Install Rust in Termux:
```bash
pkg install rust
```

### Building from Source

```bash
cd termux-gui-rust-demo
cargo build --release
```

### Running Examples

Run any of the v2 examples (which use the new library API):

```bash
# Button demo
cargo run --example button_demo_v2

# Input fields demo
cargo run --example input_demo_v2

# Complex layout demo
cargo run --example grid_layout_demo_v2

# WebView demo
cargo run --example webview_demo_v2
```

Or run the pre-built binaries:

```bash
./target/release/examples/button_demo_v2
```

## Available Widgets

### Basic Widgets
- **Button** - Clickable buttons with text and styling
- **TextView** - Display static or dynamic text
- **EditText** - Single-line and multi-line text input
- **Checkbox** - Toggle checkboxes with state management
- **Switch** - Toggle switches
- **RadioButton/RadioGroup** - Mutually exclusive option selection
- **Spinner** - Dropdown selection lists
- **ToggleButton** - Two-state toggle buttons

### Display Widgets
- **ImageView** - Display images from files or base64 data
- **ProgressBar** - Show progress (determinate or indeterminate)
- **Space** - Flexible spacing for layouts
- **WebView** - Embed web content with full JavaScript support

### Container Widgets
- **LinearLayout** - Vertical or horizontal linear layouts
- **GridLayout** - Grid-based layouts
- **FrameLayout** - Stack-based layouts
- **HorizontalScrollView** - Horizontal scrolling container
- **NestedScrollView** - Vertical scrolling container
- **SwipeRefreshLayout** - Pull-to-refresh pattern
- **TabLayout** - Tabbed interface

## Example: Simple Button App

```rust
use termux_gui::{Activity, Result};

fn main() -> Result<()> {
    // Create a full-screen activity
    let mut activity = Activity::new(false)?;
    
    // Create vertical layout
    let layout = activity.create_linear_layout(None)?;
    
    // Add a button
    let button = activity.create_button("Click Me!", Some(layout.id()))?;
    
    // Button will generate click events
    // Handle them in your event loop
    
    Ok(())
}
```

For complete working examples, see the `examples/` directory.

## Library Structure

```
src/
├── lib.rs                 # Public API and exports
├── error.rs               # Error types
├── connection.rs          # Socket communication
├── activity.rs            # Activity management
├── view.rs                # Base view operations
└── components/            # UI components (19 files)
    ├── button.rs
    ├── text_view.rs
    ├── edit_text.rs
    ├── checkbox.rs
    ├── switch.rs
    ├── radio.rs
    ├── spinner.rs
    ├── layout.rs          # All layout types
    ├── image_view.rs
    ├── progress_bar.rs
    ├── toggle_button.rs
    ├── space.rs
    └── web_view.rs
```

## Core Architecture

### Communication Protocol

The library implements the Termux:GUI protocol using:
- **Dual Socket Architecture** - Separate channels for commands and events
- **Binary Protocol** - 4-byte big-endian length prefix + JSON payload
- **Abstract Unix Domain Sockets** - Uses Linux abstract namespace for zero-filesystem-impact communication
- **Async Event Handling** - Non-blocking event loop with thread-based handling

### Type Safety

Rust's type system ensures:
- Widget IDs are properly typed and cannot be confused
- Event handlers are type-safe
- Resource cleanup is automatic via RAII
- No null pointer exceptions or data races

## Documentation

Each widget has comprehensive examples in the `examples/` directory. The examples demonstrate:
- Basic usage
- Event handling
- Layout composition
- State management
- Common patterns and best practices

## Development

### Running with Debug Output

```bash
# Enable debug logging
RUST_LOG=debug cargo run --example button_demo_v2
```

### Testing Individual Components

```bash
# Test a specific example
cargo build --release --example checkbox_demo_v2
./target/release/examples/checkbox_demo_v2
```

## Known Limitations

- **SwipeRefreshLayout Event Handling**: Due to Termux:GUI plugin limitations, complex nested layouts inside SwipeRefreshLayout may cause connection drops. Keep layouts simple when using SwipeRefreshLayout.
- **ScrollView Interaction**: Nested ScrollViews require careful layout management to ensure proper touch event propagation.

## Comparison with Python Bindings

| Feature | Python | Rust |
|---------|--------|------|
| Type Safety | Runtime | Compile-time |
| Memory Safety | Runtime | Compile-time |
| Performance | Good | Excellent |
| Error Handling | Exceptions | Result<T, E> |
| Deployment | Interpreter required | Single binary |
| Learning Curve | Easy | Moderate |
| Concurrency | GIL limitations | Fearless concurrency |

## Roadmap

- [x] Complete widget library (12+ components)
- [x] All layout types (7 layouts)
- [x] WebView with JavaScript support
- [x] Comprehensive documentation
- [x] International ready (English)
- [x] Published to crates.io
- [ ] Add more layout types (ConstraintLayout)
- [ ] Implement gesture recognizers
- [ ] Add animation support
- [ ] Create async/await API variant

## What's New in 0.3.0

- ✨ Complete WebView implementation with JavaScript support
- ✨ All components fully documented in English
- ✨ Clean library structure (src/ contains only library code)
- ✨ 21 comprehensive examples
- ✨ Internationalization complete
- 🐛 Fixed layout viewport filling
- 📚 Enhanced documentation with usage guidelines

## Resources

- [Termux:GUI Official Repository](https://github.com/termux/termux-gui)
- [Python Bindings](https://github.com/tareksander/termux-gui-python-bindings)
- [Termux:GUI Protocol Documentation](https://github.com/termux/termux-gui/blob/main/Protocol.md)
- [Project Structure](./PROJECT_STRUCTURE.md)
- [WebView Documentation](./WEBVIEW_README.md)

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Version**: 0.3.0  
**Status**: Production Ready  
**Last Updated**: 2024-10-05
