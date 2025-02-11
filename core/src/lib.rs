//! The core library of [Iced].
//!
//! This library holds basic types that can be reused and re-exported in
//! different runtime implementations.
//!
//! ![The foundations of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/foundations.png?raw=true)
//!
//! [Iced]: https://github.com/iced-rs/iced
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
pub mod alignment;
pub mod animation;
pub mod border;
pub mod clipboard;
pub mod event;
pub mod font;
pub mod gradient;
pub mod image;
pub mod input_method;
pub mod keyboard;
pub mod layout;
pub mod mouse;
pub mod overlay;
pub mod padding;
pub mod renderer;
pub mod svg;
pub mod text;
pub mod theme;
pub mod time;
pub mod touch;
pub mod widget;
pub mod window;

mod angle;
mod background;
mod color;
mod content_fit;
mod element;
mod length;
mod pixels;
mod point;
mod rectangle;
mod rotation;
mod settings;
mod shadow;
mod shell;
mod size;
mod transformation;
mod vector;

pub use alignment::Alignment;
pub use angle::{Degrees, Radians};
pub use animation::Animation;
pub use background::Background;
pub use border::Border;
pub use clipboard::Clipboard;
pub use color::Color;
pub use content_fit::ContentFit;
pub use element::Element;
pub use event::Event;
pub use font::Font;
pub use gradient::Gradient;
pub use image::Image;
pub use input_method::InputMethod;
pub use layout::Layout;
pub use length::Length;
pub use overlay::Overlay;
pub use padding::Padding;
pub use pixels::Pixels;
pub use point::Point;
pub use rectangle::Rectangle;
pub use renderer::Renderer;
pub use rotation::Rotation;
pub use settings::Settings;
pub use shadow::Shadow;
pub use shell::Shell;
pub use size::Size;
pub use svg::Svg;
pub use text::Text;
pub use theme::Theme;
pub use transformation::Transformation;
pub use vector::Vector;
pub use widget::Widget;

pub use smol_str::SmolStr;

/// A function that can _never_ be called.
///
/// This is useful to turn generic types into anything
/// you want by coercing them into a type with no possible
/// values.
pub fn never<T>(never: std::convert::Infallible) -> T {
    match never {}
}

/// Applies the given prefix value to the provided closure and returns
/// a new closure that takes the other argument.
///
/// This lets you partially "apply" a function—equivalent to currying,
/// but it only works with binary functions. If you want to apply an
/// arbitrary number of arguments, use the [`with!`] macro instead.
///
/// # When is this useful?
/// Sometimes you will want to identify the source or target
/// of some message in your user interface. This can be achieved through
/// normal means by defining a closure and moving the identifier
/// inside:
///
/// ```rust
/// # let element: Option<()> = Some(());
/// # enum Message { ButtonPressed(u32, ()) }
/// let id = 123;
///
/// # let _ = {
/// element.map(move |result| Message::ButtonPressed(id, result))
/// # };
/// ```
///
/// That's quite a mouthful. [`with()`] lets you write:
///
/// ```rust
/// # use iced_core::with;
/// # let element: Option<()> = Some(());
/// # enum Message { ButtonPressed(u32, ()) }
/// let id = 123;
///
/// # let _ = {
/// element.map(with(Message::ButtonPressed, id))
/// # };
/// ```
///
/// Effectively creating the same closure that partially applies
/// the `id` to the message—but much more concise!
pub fn with<T, R, O>(
    mut f: impl FnMut(T, R) -> O,
    prefix: T,
) -> impl FnMut(R) -> O
where
    T: Clone,
{
    move |result| f(prefix.clone(), result)
}

/// Applies the given prefix values to the provided closure in the first
/// argument and returns a new closure that takes its last argument.
///
/// This is variadic version of [`with()`] which works with any number of
/// arguments.
#[macro_export]
macro_rules! with {
    ($f:expr, $($x:expr),+ $(,)?) => {
        move |result| $f($($x),+, result)
    };
}
