use num_traits::{cast, Bounded, Float, NumCast};
use rgb::RGB;
use std::ops::Div;

/// Convert sRGB color to perceptual luminance
///
/// See [Wikipedia: Converting color to Grayscale](https://en.wikipedia.org/wiki/Grayscale#Converting_color_to_grayscale)
pub fn luminance<T: Bounded + NumCast, F: Float + NumCast + Div>(color: RGB<T>) -> F {
    let channels: [T; 3] = color.into();
    let [r, g, b] = channels;
    scale_channel::<T, F>(r) * cast(0.2126).unwrap()
        + scale_channel::<T, F>(g) * cast(0.7152).unwrap()
        + scale_channel::<T, F>(b) * cast(0.0722).unwrap()
}

fn scale_channel<T: Bounded + NumCast, F: Float + NumCast + Div>(channel: T) -> F {
    let float: F = cast(channel).expect("Failed to convert rgb channel into float");
    let relative =
        float / cast(T::max_value()).expect("Max value to rgb channel doesn't fit into float");
    if relative < cast(0.03928).unwrap() {
        relative / cast(12.92).unwrap()
    } else {
        F::powf(
            (relative + cast(0.055).unwrap()) / cast(1.055).unwrap(),
            cast(2.4).unwrap(),
        )
    }
}

/// Calculate contrast between two colors as specified by [WCAG 2](http://www.w3.org/TR/WCAG20#contrast-ratiodef)
///
/// # Usage
///
/// ```rust
/// use contrast::contrast;
/// use rgb::RGB8;
///
/// fn main() {
///     let contrast: f32 = contrast(RGB8::from([255, 255, 255]), RGB8::from([255, 255, 0]));
///     assert_eq!(contrast, 1.0738392);
/// }
/// ```
pub fn contrast<T: Bounded + NumCast, F: Float + NumCast + Div>(a: RGB<T>, b: RGB<T>) -> F {
    let luminance_a: F = luminance::<T, F>(a) + cast::<_, F>(0.05).unwrap();
    let luminance_b: F = luminance::<T, F>(b) + cast::<_, F>(0.05).unwrap();

    if luminance_a > luminance_b {
        luminance_a / luminance_b
    } else {
        luminance_b / luminance_a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rgb::RGB8;

    #[test]
    fn test_contrast() {
        let white = RGB8::from([255, 255, 255]);
        let black = RGB8::from([0, 0, 0]);
        let red = RGB8::from([255, 0, 0]);
        let green = RGB8::from([0, 255, 0]);
        let blue = RGB8::from([0, 0, 255]);
        let yellow = RGB8::from([255, 255, 0]);

        assert_eq!(luminance::<_, f32>(white), 1.0);
        assert_eq!(luminance::<_, f32>(black), 0.0);

        assert_eq!(contrast::<_, f32>(white, white), 1.0);
        assert_eq!(contrast::<_, f32>(white, black), 20.999998);
        assert_eq!(contrast::<_, f32>(black, white), 20.999998);

        assert_eq!(contrast::<_, f32>(white, red), 3.9984765);
        assert_eq!(contrast::<_, f32>(white, green), 1.3721902);
        assert_eq!(contrast::<_, f32>(white, blue), 8.592471);
        assert_eq!(contrast::<_, f32>(white, yellow), 1.0738392);
        assert_eq!(contrast::<_, f32>(black, yellow), 19.556);
    }
}
