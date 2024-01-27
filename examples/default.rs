use colored::Color;
use terminal_banner::{Banner, BoxSymbols, Padding, Text, TextAlign};

fn main() {
    let banner = Banner::new()
        .width(82)
        .symbols(BoxSymbols::default())
        .text(Text::from("DEFAULT"))
        .newline()
        .text(Text::from("Left").align(TextAlign::Left).color(Color::BrightGreen))
        .text(Text::from("Center").align(TextAlign::Center).color(Color::BrightBlue))
        .text(Text::from("Right").align(TextAlign::Right).color(Color::BrightRed))
        .padding(Padding::one())
        .divider()
        .text(Text::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.").color(Color::BrightCyan))
        .render();
    println!("{}", banner);
}
