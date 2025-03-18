#[warn(dead_code)]
use crate::border::BorderStyle;
use crate::color::Color;
use crate::decoration::Decoration;

#[derive(Clone, Debug)]
pub struct Style {
    pub fg: Option<Color>,                   // Foreground color
    pub bg: Option<Color>,                   // Background color
    pub decoration: Option<Vec<Decoration>>, // Text decoration (bold, italic, etc.)
    pub padding: Option<u8>,                 // Padding
    pub padding_top: Option<u8>,             // Padding top
    pub padding_bottom: Option<u8>,          // Padding bottom
    pub padding_left: Option<u8>,            // Padding left
    pub padding_right: Option<u8>,           // Padding right
    pub margin: Option<u8>,                  // Margin
    pub margin_top: Option<u8>,              // Margin top
    pub margin_bottom: Option<u8>,           // Margin bottom
    pub margin_left: Option<u8>,             // Margin left
    pub margin_right: Option<u8>,            // Margin right
    pub border_color: Option<Color>,         // Border color
    pub border_style: Option<BorderStyle>,   // Border style
}

impl Style {
    /// Creates a new default style.
    pub fn new() -> Self {
        Style {
            fg: None,
            bg: None,
            decoration: None,
            padding: None,
            margin: None,
            margin_top: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            border_color: None,
            border_style: None,
            padding_top: None,
            padding_bottom: None,
            padding_left: None,
            padding_right: None,
        }
    }

    /// Sets the foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Sets the text decoration.
    pub fn decoration(mut self, deco: Vec<Decoration>) -> Self {
        self.decoration = Some(deco);
        self
    }

    /// Sets the padding (spaces inside the border).
    pub fn padding(mut self, pad: u8) -> Self {
        self.padding = Some(pad);
        self
    }

    /// Sets the top padding.
    pub fn padding_top(mut self, pad: u8) -> Self {
        self.padding_top = Some(pad);
        self
    }

    /// Sets the bottom padding.
    pub fn padding_bottom(mut self, pad: u8) -> Self {
        self.padding_bottom = Some(pad);
        self
    }

    /// Sets the left padding.
    pub fn padding_left(mut self, pad: u8) -> Self {
        self.padding_left = Some(pad);
        self
    }

    /// Sets the right padding.
    pub fn padding_right(mut self, pad: u8) -> Self {
        self.padding_right = Some(pad);
        self
    }

    /// Sets the margin (lines outside the border).
    pub fn margin(mut self, margin: u8) -> Self {
        self.margin = Some(margin);
        self
    }

    /// Sets the top margin.
    pub fn margin_top(mut self, margin: u8) -> Self {
        self.margin_top = Some(margin);
        self
    }

    /// Sets the bottom margin.
    pub fn margin_bottom(mut self, margin: u8) -> Self {
        self.margin_bottom = Some(margin);
        self
    }

    /// Sets the left margin.
    pub fn margin_left(mut self, margin: u8) -> Self {
        self.margin_left = Some(margin);
        self
    }

    /// Sets the right margin.
    pub fn margin_right(mut self, margin: u8) -> Self {
        self.margin_right = Some(margin);
        self
    }

    /// Sets the border color.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Sets the border style.
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = Some(style);
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::new()
    }
}
