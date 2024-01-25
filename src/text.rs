use colored::Color;

#[derive(Default, Copy, Clone)]
pub enum TextAlign {
    /// text align left
    #[default]
    LEFT,
    /// text align right
    RIGHT,
    /// text align center
    CENTER,
}

/// text content
#[derive(Default, Clone)]
pub struct Text {
    pub content: String,
    pub style: TextStyle,
}

#[derive(Clone)]
pub struct TextStyle {
    pub align: TextAlign,
    pub color: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            align: Default::default(),
            color: Color::BrightWhite,
        }
    }
}

impl Text {
    pub fn align(mut self, align: TextAlign) -> Self {
        self.style.align = align;
        self
    }

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
