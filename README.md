# Terminal Banner

Tiny Rust library for rendering a boxed banner at the current terminal width wrapping each paragraph of text.

```rust
use std::borrow::Cow;
use terminal_banner::Banner;

fn main() {
    let banner = Banner::new()
        .text(Cow::from("LIPSUM"))
        .text(Cow::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")).render();
    println!("{}", banner);
}
```

See the examples for alternative rendering.

## License

MIT or Apache-2.0
