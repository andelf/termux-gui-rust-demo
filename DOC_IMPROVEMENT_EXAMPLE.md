# Documentation Improvement Example

## Before and After Comparison

### Current Documentation (Good)

```rust
//! Button component

/// A Button that can be clicked
pub struct Button {
    view: View,
    aid: i64,
}

impl Button {
    /// Create a new Button
    pub fn new(activity: &mut Activity, text: &str, parent: Option<i64>) -> Result<Self> {
        // ...
    }
}
```

### Improved Documentation (Better)

```rust
//! Button component for user interaction.
//!
//! Buttons are clickable UI elements that trigger actions when pressed.
//! They are one of the most fundamental interactive components in GUI applications.
//!
//! # Examples
//!
//! Basic button creation:
//!
//! ```no_run
//! use termux_gui::{Activity, Result};
//!
//! fn main() -> Result<()> {
//!     let mut activity = Activity::new(false)?;
//!     let layout = activity.create_linear_layout(None)?;
//!     
//!     // Create a button
//!     let button = activity.create_button("Click Me!", Some(layout.id()))?;
//!     
//!     // Buttons emit "click" events that can be handled
//!     // in your event loop
//!     Ok(())
//! }
//! ```
//!
//! # Event Handling
//!
//! Buttons generate click events that should be handled in your application's
//! event loop. The event will contain the button's ID for identification.

use serde_json::json;
use crate::activity::Activity;
use crate::view::View;
use crate::error::Result;

/// A clickable button widget.
///
/// Buttons display text and respond to user clicks. They are commonly used
/// for actions like "Submit", "Cancel", "OK", etc.
///
/// # Features
///
/// - Displays text label
/// - Emits click events
/// - Supports all View operations (sizing, colors, etc.)
/// - Text is not automatically capitalized
///
/// # Layout
///
/// Buttons should typically be placed in a layout container. If no parent
/// is specified, the button becomes a top-level view.
pub struct Button {
    view: View,
    aid: i64,
}

impl Button {
    /// Creates a new Button with the specified text.
    ///
    /// # Arguments
    ///
    /// * `activity` - The activity to create the button in
    /// * `text` - The text to display on the button
    /// * `parent` - Optional parent view ID. Use `None` for top-level buttons,
    ///              or `Some(layout.id())` to add to a layout
    ///
    /// # Returns
    ///
    /// Returns a `Button` instance that can be used to access and modify
    /// the button's properties.
    ///
    /// # Examples
    ///
    /// Creating a button in a layout:
    ///
    /// ```no_run
    /// # use termux_gui::{Activity, Result};
    /// # fn main() -> Result<()> {
    /// let mut activity = Activity::new(false)?;
    /// let layout = activity.create_linear_layout(None)?;
    /// 
    /// let submit_button = activity.create_button("Submit", Some(layout.id()))?;
    /// let cancel_button = activity.create_button("Cancel", Some(layout.id()))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creating a standalone button:
    ///
    /// ```no_run
    /// # use termux_gui::{Activity, Result};
    /// # fn main() -> Result<()> {
    /// let mut activity = Activity::new(true)?; // dialog mode
    /// let button = activity.create_button("OK", None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Communication with the GUI service fails
    /// - The parent view ID is invalid (doesn't exist)
    /// - The GUI service returns an invalid response
    ///
    /// # See Also
    ///
    /// - [`TextView`] for non-clickable text
    /// - [`ToggleButton`] for buttons with on/off states
    pub fn new(activity: &mut Activity, text: &str, parent: Option<i64>) -> Result<Self> {
        let mut params = json!({
            "aid": activity.id(),
            "text": text,
            "allcaps": false
        });
        
        if let Some(parent_id) = parent {
            params["parent"] = json!(parent_id);
        }
        
        eprintln!("[DEBUG] Button::new() - sending createButton...");
        let response = activity.send_read(&json!({
            "method": "createButton",
            "params": params
        }))?;
        eprintln!("[DEBUG] Button::new() - got response: {:?}", response);
        
        let id = response
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Invalid id".to_string()))?;
        
        Ok(Button {
            view: View::new(id),
            aid: activity.id(),
        })
    }
    
    /// Returns the button's unique view ID.
    ///
    /// This ID can be used to identify the button in event handlers
    /// or when performing view operations.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use termux_gui::{Activity, Result};
    /// # fn main() -> Result<()> {
    /// # let mut activity = Activity::new(false)?;
    /// let button = activity.create_button("Click", None)?;
    /// println!("Button ID: {}", button.id());
    /// # Ok(())
    /// # }
    /// ```
    pub fn id(&self) -> i64 {
        self.view.id()
    }
    
    /// Returns a reference to the underlying View.
    ///
    /// This allows access to common view operations like setting size,
    /// colors, margins, etc.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use termux_gui::{Activity, Result};
    /// # fn main() -> Result<()> {
    /// # let mut activity = Activity::new(false)?;
    /// let button = activity.create_button("Click", None)?;
    /// 
    /// // Access view methods
    /// button.view().set_width(&mut activity, 200)?;
    /// button.view().set_background_color(&mut activity, 0xFF2196F3)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn view(&self) -> &View {
        &self.view
    }
    
    /// Sets the button's text.
    ///
    /// # Arguments
    ///
    /// * `activity` - Reference to the activity
    /// * `text` - The new text to display
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use termux_gui::{Activity, Result};
    /// # fn main() -> Result<()> {
    /// # let mut activity = Activity::new(false)?;
    /// let button = activity.create_button("Submit", None)?;
    /// 
    /// // Change button text
    /// button.set_text(&mut activity, "Processing...")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if communication with the GUI service fails.
    pub fn set_text(&self, activity: &mut Activity, text: &str) -> Result<()> {
        activity.send(&json!({
            "method": "setText",
            "params": {
                "aid": self.aid,
                "id": self.view.id(),
                "text": text
            }
        }))
    }
}
```

## Key Improvements

### 1. Module Documentation
- Added comprehensive module-level docs
- Included usage examples
- Explained event handling

### 2. Type Documentation
- Explained what the type is and does
- Listed key features
- Discussed layout considerations

### 3. Method Documentation
- Added detailed `# Arguments` section
- Added `# Returns` section
- Added multiple `# Examples` showing different use cases
- Added `# Errors` section explaining failure conditions
- Added `# See Also` for related types

### 4. Code Examples
- All examples are complete and can run (with `no_run`)
- Show both common and advanced usage patterns
- Include error handling with `?`

### 5. Cross-References
- Link to related types with [`Type`] syntax
- Helps users discover related functionality

## Application to Other Files

This pattern should be applied to:

1. **All public structs** - Add type-level documentation
2. **All public methods** - Add detailed method documentation
3. **Module files** - Add comprehensive module documentation
4. **Error types** - Explain when each error occurs

## Tools for Validation

```bash
# Check documentation
cargo doc --no-deps --open

# Check for missing docs
cargo clippy -- -W missing_docs

# Check for broken links
cargo doc --no-deps 2>&1 | grep "warning:"
```

## Guidelines Summary

1. **Be explicit** - Don't assume users know Android/GUI concepts
2. **Show examples** - Code speaks louder than words
3. **Document errors** - Tell users when things can fail
4. **Cross-reference** - Help users discover related functionality
5. **Use standard format** - Follow Rust API guidelines
6. **Be concise** - But don't sacrifice clarity for brevity
7. **Think international** - Avoid idioms or cultural references
8. **Use proper English** - But don't be overly formal

## Conclusion

The current documentation is good and functional. These improvements make it excellent and professional, suitable for:

- Crates.io publication
- International audience
- API documentation generation
- IDE auto-completion and hints
- New users learning the library
