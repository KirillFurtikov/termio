mod border;
mod color;
mod decoration;
#[macro_use]
mod macros;
mod termio;
mod style;
mod styled_text;
pub mod prelude;

pub use border::BorderStyle;
pub use color::Color;
pub use decoration::Decoration;
pub use termio::Termio;
pub use style::Style;
pub use styled_text::{StyledString, StyledText};
