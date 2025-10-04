# Spinner Event Handling - Important Notes

## Issue Discovery (2024-01-04)

During development, we discovered a critical difference between expected and actual Spinner event behavior in Termux:GUI.

## Key Finding

**The `itemselected` event returns the selected TEXT as a string, NOT an index number.**

### Event Structure

When a Spinner item is selected, the event structure is:

```json
{
  "aid": 0,
  "id": 215766365,
  "selected": "Option Text"
}
```

Note that:
- The field name is `"selected"` (not `"index"`)
- The value is a **string** containing the selected item's text
- There is NO index field in the event

### Correct Event Handling

```rust
// ✅ CORRECT - Match against string value
"itemselected" => {
    let view_id = event_value["id"].as_i64().unwrap_or(-1);
    let selected_text = event_value["selected"].as_str().unwrap_or("");
    
    match selected_text {
        "Apple" => println!("Apple selected"),
        "Orange" => println!("Orange selected"),
        _ => {}
    }
}
```

### Incorrect Event Handling

```rust
// ❌ WRONG - This will always return 0 or None
"itemselected" => {
    let index = event_value["index"].as_i64().unwrap_or(0);  // Always 0!
    
    if index == 1 {  // This will never be true
        println!("Second item selected");
    }
}
```

## Impact on Cascading Spinners

This discovery was critical for implementing cascading Spinner functionality:

```rust
// First Spinner - Fruit selection
if view_id == fruit_spinner.id() {
    match selected_text {
        "Apple" => {
            // Update second Spinner with apple varieties
            variety_spinner.set_list(&mut activity, &["Fuji", "Gala", "Granny Smith"])?;
        },
        "Orange" => {
            // Update second Spinner with orange varieties
            variety_spinner.set_list(&mut activity, &["Navel", "Blood", "Valencia"])?;
        },
        _ => {}
    }
}
```

## Comparison with Python Implementation

The Python bindings correctly document this behavior:

```python
# From termux-gui-python-bindings/tutorial/inputs2.py
if ev.type == tg.Event.itemselected and ev.value["id"] == spinner:
    # for itemselected events, selected is the selected value as a string
    print("Spinner selected: ", ev.value["selected"])
```

## Resolution

All Spinner examples have been updated to:
1. Use `event_value["selected"].as_str()` instead of `event_value["index"]`
2. Match against string values instead of numeric indices
3. Document this behavior in code comments and API documentation

## Related Files Updated

- `src/components/spinner.rs` - Added comprehensive doc comments
- `examples/spinner_demo_v2.rs` - Fixed to use string matching
- `examples/spinner_test_simple.rs` - Test case demonstrating correct usage

## Testing

To verify Spinner behavior, run:

```bash
cargo run --release --example spinner_test_simple
```

This simple two-Spinner cascade demonstrates proper event handling and dynamic list updates.
