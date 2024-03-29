//! Tiny utility to render a boxed banner at the width of the terminal.
//!
//! Use the `color` feature to enable support for terminal colors,
//! see the examples for usage.
//!
//! ```
//! use terminal_banner::Banner;
//! let banner = Banner::new()
//!     .text("LIPSUM".into())
//!     .text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".into())
//!     .render();
//! println!("{}", banner);
//! ```
//!
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

#[cfg(feature = "color")]
use colored::Colorize;
use textwrap::core::display_width;
use textwrap::{termwidth, wrap, Options};

mod text;

pub use text::{Text, TextAlign, TextStyle};

#[cfg(feature = "color")]
pub use colored;

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

enum Line<'a> {
    Text(Text<'a>),
    Divider(char),
}

/// Render a terminal banner.
#[derive(Default)]
pub struct Banner<'a> {
    symbols: BoxSymbols,
    lines: Vec<Line<'a>>,
    padding: Padding,
    width: Option<usize>,
}

impl<'a> Banner<'a> {
    /// Create a new banner.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set an explicit width for the banner.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
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
    pub fn text(mut self, text: Text<'a>) -> Self {
        self.lines.push(Line::Text(text));
        self
    }

    /// Append a divider with customize char.
    pub fn divider_with(mut self, character: char) -> Self {
        self.lines.push(Line::Divider(character));
        self
    }

    /// Append a divider rule.
    pub fn divider(mut self) -> Self {
        self.lines.push(Line::Divider(self.symbols.h));
        self
    }

    /// Append a divider with space.
    pub fn divider_with_space(mut self) -> Self {
        self.lines.push(Line::Divider(' '));
        self
    }

    /// Append a newline rule.
    pub fn newline(self) -> Self {
        self.divider_with_space()
    }

    /// Render the banner.
    pub fn render(&self) -> String {
        let width = self.width.unwrap_or_else(termwidth);
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

        for (_, line) in self.lines.iter().enumerate() {
            match line {
                Line::Text(text) => {
                    let repeat = if text.content.len() > width {
                        0
                    } else {
                        match text.style.align {
                            TextAlign::Left => 0,
                            TextAlign::Right => {
                                width
                                    - 2
                                    - self.padding.right as usize
                                    - text.content.len()
                            }
                            TextAlign::Center => {
                                (width - 2 - text.content.len()) / 2
                            }
                        }
                    };
                    let mut context =
                        String::from(' ').repeat(if repeat > 0 {
                            repeat - self.padding.right as usize
                        } else {
                            repeat
                        });
                    context.push_str(text.content.as_ref());
                    let lines = wrap(context.as_str(), &options);
                    let length = lines.len();

                    for (index, line) in lines.into_iter().enumerate() {
                        #[cfg(feature = "color")]
                        let line = {
                            let mut line_text = String::from(self.symbols.v);
                            let line = &line[self.symbols.v.len_utf8()..]
                                .color(text.style.color)
                                .to_string();
                            line_text.push_str(&line);
                            line_text
                        };
                        message.push_str(&line);
                        let fill_width = width - display_width(&line) - 1;
                        let filler = String::from(' ').repeat(fill_width);
                        message.push_str(&filler);
                        message.push(self.symbols.v);
                        if index < length - 1 {
                            message.push('\n');
                        }
                    }

                    message.push('\n');
                }
                Line::Divider(character) => {
                    let mut divider = String::from(&indent);
                    divider.push_str(&*character.to_string().repeat(
                        width
                            - 2
                            - self.padding.left as usize
                            - self.padding.right as usize,
                    ));
                    divider.push_str(
                        &String::from(' ').repeat(self.padding.right.into()),
                    );
                    divider.push(self.symbols.v);
                    divider.push('\n');
                    message.push_str(&divider);
                }
            }
        }

        for _ in 0..self.padding.bottom {
            message.push_str(&spacer);
        }
        message.push_str(&end_line);
        message
    }
}
