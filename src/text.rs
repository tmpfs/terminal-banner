#[cfg(feature = "color")]
use colored::Color;

/// Variants for text alignment.
#[derive(Default, Copy, Clone)]
pub enum TextAlign {
    /// Text align left
    #[default]
    Left,
    /// Text align right
    Right,
    /// Text align center
    Center,
}

/// Text content.
#[derive(Default, Clone)]
pub struct Text {
    /// Content for the text.
    pub content: String,
    /// Styling information.
    pub style: TextStyle,
}

/// Text style.
#[derive(Clone)]
pub struct TextStyle {
    /// Alignment.
    pub align: TextAlign,
    /// Color information.
    #[cfg(feature = "color")]
    pub color: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            align: Default::default(),
            #[cfg(feature = "color")]
            color: Color::BrightWhite,
        }
    }
}

impl Text {
    /// Set the text alignment.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.style.align = align;
        self
    }
    
    /// Set the text color.
    #[cfg(feature = "color")]
    pub fn color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text {
            content: value,
            ..Default::default()
        }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text {
            content: value.to_string(),
            ..Default::default()
        }
    }
}
