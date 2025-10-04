//! Activity management

use serde_json::{json, Value};
use std::os::unix::net::UnixStream;

use crate::connection::Connection;
use crate::error::Result;
use crate::components::*;

/// Represents a GUI Activity (window)
pub struct Activity {
    conn: Connection,
    aid: i64,
}

impl Activity {
    /// Create a new Activity
    /// 
    /// # Arguments
    /// * `dialog` - If true, creates a dialog-style window; if false, creates a full-screen activity
    pub fn new(dialog: bool) -> Result<Self> {
        eprintln!("[DEBUG] Activity::new() - creating connection...");
        let mut conn = Connection::new()?;
        
        eprintln!("[DEBUG] Activity::new() - sending newActivity...");
        let response = conn.send_read(&json!({
            "method": "newActivity",
            "params": {
                "dialog": dialog,
                "canceloutside": !dialog
            }
        }))?;
        
        eprintln!("[DEBUG] Activity::new() - got response: {:?}", response);
        // Response is an array, aid is at index 0
        let aid = response[0]
            .as_i64()
            .ok_or_else(|| crate::error::GuiError::InvalidResponse("Missing aid".to_string()))?;
        
        eprintln!("[DEBUG] Activity::new() - aid = {}", aid);
        Ok(Activity { conn, aid })
    }
    
    /// Get the Activity ID
    pub fn id(&self) -> i64 {
        self.aid
    }
    
    /// Send a message and read response
    pub fn send_read(&mut self, msg: &Value) -> Result<Value> {
        self.conn.send_read(msg)
    }
    
    /// Send a message without waiting for response
    pub fn send(&mut self, msg: &Value) -> Result<()> {
        self.conn.send(msg)
    }
    
    /// Get mutable reference to event stream
    pub fn event_stream(&mut self) -> &mut UnixStream {
        self.conn.event_stream()
    }
    
    /// Create a LinearLayout
    pub fn create_linear_layout(&mut self, parent: Option<i64>) -> Result<LinearLayout> {
        LinearLayout::new(self, parent)
    }
    
    /// Create a LinearLayout with specified orientation
    pub fn create_linear_layout_horizontal(&mut self, parent: Option<i64>) -> Result<LinearLayout> {
        LinearLayout::new_with_orientation(self, parent, false)
    }
    
    /// Create a NestedScrollView
    pub fn create_nested_scroll_view(&mut self, parent: Option<i64>) -> Result<NestedScrollView> {
        NestedScrollView::new(self, parent)
    }
    
    /// Create a TextView
    pub fn create_text_view(&mut self, text: &str, parent: Option<i64>) -> Result<TextView> {
        TextView::new(self, text, parent)
    }
    
    /// Create a Button
    pub fn create_button(&mut self, text: &str, parent: Option<i64>) -> Result<Button> {
        Button::new(self, text, parent)
    }
    
    /// Create an EditText
    pub fn create_edit_text(&mut self, text: &str, parent: Option<i64>) -> Result<EditText> {
        EditText::new(self, text, parent)
    }
    
    /// Create a multi-line EditText
    pub fn create_edit_text_multiline(&mut self, text: &str, parent: Option<i64>) -> Result<EditText> {
        EditText::new_multiline(self, text, parent)
    }
    
    /// Create a Checkbox
    pub fn create_checkbox(&mut self, text: &str, parent: Option<i64>) -> Result<Checkbox> {
        Checkbox::new(self, text, parent)
    }
    
    /// Create a Checkbox with initial checked state
    pub fn create_checkbox_checked(&mut self, text: &str, parent: Option<i64>, checked: bool) -> Result<Checkbox> {
        Checkbox::new_with_checked(self, text, parent, checked)
    }
    
    /// Create a Switch
    pub fn create_switch(&mut self, text: &str, parent: Option<i64>) -> Result<Switch> {
        Switch::new(self, text, parent)
    }
    
    /// Create a Switch with specified checked state
    pub fn create_switch_checked(&mut self, text: &str, parent: Option<i64>, checked: bool) -> Result<Switch> {
        Switch::new_with_checked(self, text, parent, checked)
    }
    
    /// Create a RadioButton
    pub fn create_radio_button(&mut self, text: &str, parent: Option<i64>) -> Result<RadioButton> {
        RadioButton::new(self, text, parent)
    }
    
    /// Create a RadioButton with specified checked state
    pub fn create_radio_button_checked(&mut self, text: &str, parent: Option<i64>, checked: bool) -> Result<RadioButton> {
        RadioButton::new_with_checked(self, text, parent, checked)
    }
    
    /// Create a RadioGroup
    pub fn create_radio_group(&mut self, parent: Option<i64>) -> Result<RadioGroup> {
        RadioGroup::new(self, parent)
    }
    
    /// Create a Spinner
    pub fn create_spinner(&mut self, parent: Option<i64>) -> Result<Spinner> {
        Spinner::new(self, parent)
    }
    
    /// Create an ImageView
    pub fn create_image_view(&mut self, parent: Option<i64>) -> Result<ImageView> {
        ImageView::new(self, parent)
    }
    
    /// Create a ProgressBar
    pub fn create_progress_bar(&mut self, parent: Option<i64>) -> Result<ProgressBar> {
        ProgressBar::new(self, parent)
    }
    
    /// Create a ToggleButton
    pub fn create_toggle_button(&mut self, text: &str, parent: Option<i64>) -> Result<ToggleButton> {
        ToggleButton::new(self, text, parent)
    }
    
    /// Create a ToggleButton with specified checked state
    pub fn create_toggle_button_checked(&mut self, text: &str, parent: Option<i64>, checked: bool) -> Result<ToggleButton> {
        ToggleButton::new_with_checked(self, text, parent, checked)
    }
    
    /// Create a Space (empty space for layout)
    pub fn create_space(&mut self, parent: Option<i64>) -> Result<Space> {
        Space::new(self, parent)
    }
    
    /// Create a FrameLayout
    pub fn create_frame_layout(&mut self, parent: Option<i64>) -> Result<FrameLayout> {
        FrameLayout::new(self, parent)
    }
    
    /// Set the Activity title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.send(&json!({
            "method": "setTheme",
            "params": {
                "aid": self.aid,
                "statusBarColor": 0,
                "colorPrimary": 0,
                "windowBackground": 0,
                "textColor": 0,
                "colorAccent": 0
            }
        }))?;
        
        self.send_read(&json!({
            "method": "setTaskDescription",
            "params": {
                "aid": self.aid,
                "label": title
            }
        }))?;
        
        Ok(())
    }
    
    /// Finish (close) the Activity
    pub fn finish(&mut self) -> Result<()> {
        self.send_read(&json!({
            "method": "finishActivity",
            "params": {
                "aid": self.aid,
                "finishing": true
            }
        }))?;
        Ok(())
    }
}
