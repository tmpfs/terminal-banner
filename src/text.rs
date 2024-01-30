#[cfg(feature = "color")]
use colored::Color;
use std::borrow::Cow;

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
pub struct Text<'a> {
    /// Content for the text.
    pub content: Cow<'a, str>,
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

impl<'a> Text<'a> {
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

impl From<String> for Text<'_> {
    fn from(value: String) -> Self {
        Text {
            content: Cow::Owned(value),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(value: &'a str) -> Self {
        Text {
            content: Cow::Borrowed(value),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a String> for Text<'a> {
    fn from(value: &'a String) -> Self {
        Text {
            content: Cow::Borrowed(value),
            ..Default::default()
        }
    }
}

impl<'a> From<Cow<'a, str>> for Text<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Text {
            content: value,
            ..Default::default()
        }
    }
}
