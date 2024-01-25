use colored::Color;
use terminal_banner::{Banner, BoxSymbols, Padding, Text, TextAlign};

fn main() {
    let banner = Banner::new()
        .width(82)
        .symbols(BoxSymbols::default())
        .text(Text::from("DEFAULT"))
        .text(Text::from("LEFT").align(TextAlign::LEFT).color(Color::Green))
        .text(Text::from("CENTER").align(TextAlign::CENTER).color(Color::Blue))
        .text(Text::from("RIGHT").align(TextAlign::RIGHT))
        .padding(Padding::one())
        .divider()
        .text(Text::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."))
        .render();
    println!("{}", banner);
}
