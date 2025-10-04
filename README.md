# Termux:GUI Rust Bindings

A Rust library and collection of examples for building Android GUI applications using Termux:GUI. This project provides idiomatic Rust bindings for the Termux:GUI plugin, making it easy to create native Android interfaces from the Termux terminal environment.

## Features

✅ **Complete Termux:GUI Protocol Implementation** - Full support for the low-level communication protocol  
✅ **Rich Widget Library** - Buttons, text inputs, checkboxes, switches, radio buttons, spinners, and more  
✅ **Advanced Layouts** - Linear, Grid, Frame, and Tab layouts with nested support  
✅ **Image Support** - Display images from files or base64 data  
✅ **WebView Integration** - Embed web content in your applications  
✅ **Scroll Views** - Both horizontal and vertical scrolling containers  
✅ **Event Handling** - Comprehensive event system with typed handlers  
✅ **Type Safety** - Leverages Rust's type system for compile-time correctness  
✅ **Memory Safety** - Zero-cost abstractions with guaranteed memory safety  

## Project Structure

```
termux-gui-rust-demo/
├── src/
│   ├── lib.rs              # Main library with all widgets and layouts
│   ├── core/               # Core communication protocols
│   │   ├── connection.rs   # Socket connection management
│   │   ├── activity.rs     # Activity lifecycle
│   │   └── events.rs       # Event handling
│   └── widgets/            # Widget implementations
├── examples/               # Comprehensive widget examples
│   ├── button_demo_v2.rs
│   ├── input_demo_v2.rs
│   ├── checkbox_demo_v2.rs
│   └── ... (many more)
└── Cargo.toml
```

## Quick Start

### Installation

Add this library to your Rust project:

```bash
cargo add termux-gui
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
termux-gui = "0.2.0"
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
use termux_gui::{Activity, LinearLayout, Button};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create activity
    let mut activity = Activity::new()?;
    
    // Create vertical layout
    let mut root = LinearLayout::new(&mut activity, true)?;
    
    // Add a button
    let mut button = Button::new(&mut activity, "Click Me!", &mut root)?;
    
    // Handle click events
    activity.wait_event(|event| {
        if event.is_click(&button) {
            println!("Button clicked!");
        }
        false  // Continue event loop
    })?;
    
    Ok(())
}
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

- [ ] Add more layout types (ConstraintLayout, RelativeLayout)
- [ ] Implement gesture recognizers
- [ ] Add animation support
- [ ] Improve error messages and recovery
- [ ] Create async/await API variant
- [ ] Add comprehensive documentation
- [ ] Publish to crates.io

## Resources

- [Termux:GUI Official Repository](https://github.com/termux/termux-gui)
- [Python Bindings](https://github.com/tareksander/termux-gui-python-bindings)
- [Termux:GUI Protocol Documentation](https://github.com/termux/termux-gui/blob/main/Protocol.md)

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Version**: 0.2.0  
**Last Updated**: 2024
