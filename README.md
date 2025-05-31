# Moved to https://codeberg.org/icewind/contrast

# Contrast

Calculate contrast between two colors as specified by [WCAG 2](http://www.w3.org/TR/WCAG20#contrast-ratiodef)

# Usage

```rust
use contrast::contrast;
use rgb::RGB8;
///
fn main() {
    let contrast: f32 = contrast(RGB8::from([255, 255, 255]), RGB8::from([255, 255, 0]));
    assert_eq!(contrast, 1.0738392);
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
