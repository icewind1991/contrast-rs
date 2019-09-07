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