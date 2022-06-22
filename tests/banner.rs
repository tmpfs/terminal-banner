use std::borrow::Cow;
use terminal_banner::*;

#[test]
fn default() {
    let expected = include_str!("default.txt").trim_end_matches('\n');
    let result = Banner::new()
        .width(82)
        .text(Cow::from("LIPSUM"))
        .text(Cow::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")).render();
    assert_eq!(expected, &result);
}

#[test]
fn strong() {
    let expected = include_str!("strong.txt").trim_end_matches('\n');
    let result = Banner::new()
        .width(82)
        .symbols(BoxSymbols::strong())
        .padding(Padding::one())
        .text(Cow::from("LIPSUM"))
        .text(Cow::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")).render();

    assert_eq!(expected, &result);
}
