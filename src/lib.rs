//! Tiny utility to render a boxed banner at the width of the terminal.
//!
//! ```
//! use std::borrow::Cow;
//! use terminal_banner::Banner;
//! let banner = Banner::new()
//!     .text(Cow::from("LIPSUM"))
//!     .text(Cow::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."));
//! let result = banner.render();
//! println!("{}", result);
//! ```
#![deny(missing_docs)]
use std::borrow::Cow;
use textwrap::{core::display_width, termwidth, wrap, Options};

/// Collection of box drawing symbols used to draw the banner outline.
pub struct BoxSymbols {
    tl: char,
    tr: char,
    bl: char,
    br: char,
    h: char,
    v: char,
}

impl Default for BoxSymbols {
    fn default() -> Self {
        Self {
            tl: '┌',
            tr: '┐',
            bl: '└',
            br: '┘',
            h: '─',
            v: '│',
        }
    }
}

impl BoxSymbols {
    /// Creates box symbols with a strong line.
    pub fn strong() -> Self {
        Self {
            tl: '┏',
            tr: '┓',
            bl: '┗',
            br: '┛',
            h: '━',
            v: '┃',
        }
    }
}

/// Padding inside the banner outline.
#[derive(Default)]
pub struct Padding {
    /// Top padding.
    pub top: u8,
    /// Right padding.
    pub right: u8,
    /// Bottom padding.
    pub bottom: u8,
    /// Left padding.
    pub left: u8,
}

impl Padding {
    /// Creates padding with a value of one for each edge.
    pub fn one() -> Self {
        Self {
            top: 1,
            right: 1,
            bottom: 1,
            left: 1,
        }
    }
}

/// Render a terminal banner.
pub struct Banner<'a> {
    symbols: BoxSymbols,
    text: Vec<Cow<'a, str>>,
    padding: Padding,
}


impl<'a> Default for Banner<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Banner<'a> {
    /// Create a new banner.
    pub fn new() -> Self {
        Self {
            symbols: Default::default(),
            text: Vec::new(),
            padding: Default::default(),
        }
    }

    /// Set the banner padding.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Set the banner box symbols.
    pub fn symbols(mut self, symbols: BoxSymbols) -> Self {
        self.symbols = symbols;
        self
    }

    /// Append a block of text to wrap inside the banner.
    pub fn text(mut self, text: Cow<'a, str>) -> Self {
        self.text.push(text);
        self
    }

    /// Render the banner.
    pub fn render(&self) -> String {
        let width = termwidth();
        let mut indent = String::from(self.symbols.v);

        indent.push_str(&String::from(' ').repeat(self.padding.left.into()));

        let right_offset = self.padding.right + 1;

        let options = Options::new(width - right_offset as usize)
            .initial_indent(&indent)
            .subsequent_indent(&indent);
        let horizontal = String::from(self.symbols.h).repeat(width - 2);
        let mut start_line = String::from(self.symbols.tl);
        start_line.push_str(&horizontal);
        start_line.push(self.symbols.tr);
        start_line.push('\n');

        let mut end_line = String::from(self.symbols.bl);
        end_line.push_str(&horizontal);
        end_line.push(self.symbols.br);

        let mut spacer = String::from(self.symbols.v);
        spacer.push_str(&String::from(' ').repeat(width - 2));
        spacer.push(self.symbols.v);
        spacer.push('\n');

        let mut message = start_line;
        for _ in 0..self.padding.top {
            message.push_str(&spacer);
        }

        let text_length = self.text.len();
        for (index, text) in self.text.iter().enumerate() {
            let lines = wrap(text.as_ref(), &options);
            let length = lines.len();

            for (index, line) in lines.into_iter().enumerate() {
                message.push_str(&line);
                let fill_width = width - (display_width(&line) + 1);
                let filler = String::from(' ').repeat(fill_width);
                message.push_str(&filler);
                message.push(self.symbols.v);
                if index < length - 1 {
                    message.push('\n');
                }
            }

            message.push('\n');

            if index < text_length - 1 {
                message.push_str(&spacer);
            }
        }

        for _ in 0..self.padding.bottom {
            message.push_str(&spacer);
        }
        message.push_str(&end_line);
        message
    }
}

