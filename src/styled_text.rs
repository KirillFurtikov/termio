//! Termio - A simple styling system for terminal text
//!
//! This module provides functionality for styling terminal text using CSS-like syntax.
//! It supports colors, decorations, borders, padding, and margins.

use crate::border::BorderStyle;
use crate::color::Color;
use crate::decoration::Decoration;
use crate::style::Style;
use crate::termio::Termio;
use std::fmt;
use unicode_width::UnicodeWidthStr;

/// A trait for text that can be styled using Termio.
///
/// This trait provides methods for applying styles to text elements.
/// It is implemented for `String` and `&str`.
///
/// # Examples
///
/// ```
/// use terminal_css::StyledText;
/// use terminal_css::Termio;
///
/// let tcss = Termio::from_file("examples/styles.tcss").unwrap();
/// let text = "Hello, World!".style("header", &tcss);
///
/// ```
pub trait StyledText {
    /// Applies a style from the TCSS parser by name.
    ///
    /// # Arguments
    ///
    /// * `style_name` - The name of the style to apply
    /// * `tcss` - The TCSS parser containing the styles
    ///
    /// # Returns
    ///
    /// A new `StyledString` instance with the applied style
    fn style(self, style_name: &str, tcss: &Termio) -> StyledString;

    /// Set text color
    fn color(self, color: Color) -> StyledString;

    /// Set background color
    fn bg(self, color: Color) -> StyledString;

    /// Add decoration
    fn decoration(self, decoration: Decoration) -> StyledString;

    /// Set padding (all sides)
    fn padding(self, padding: u8) -> StyledString;

    /// Set padding for top, right, bottom, left
    fn padding_trbl(self, top: u8, right: u8, bottom: u8, left: u8) -> StyledString;

    /// Set margin (all sides)
    fn margin(self, margin: u8) -> StyledString;

    /// Set border style
    fn border(self, style: BorderStyle) -> StyledString;

    /// Set border color
    fn border_color(self, color: Color) -> StyledString;
}

/// A string with applied TCSS styles.
///
/// This struct holds the text content and its associated style.
#[derive(Clone)]
pub struct StyledString {
    text: String,
    style: Style,
}

impl StyledString {
    /// Creates a new styled string with the given text and style.
    fn new(text: String, style: Style) -> Self {
        StyledString { text, style }
    }

    /// Gets the foreground color
    pub fn get_fg(&self) -> Option<Color> {
        self.style.fg
    }

    /// Gets the background color
    pub fn get_bg(&self) -> Option<Color> {
        self.style.bg
    }

    /// Gets the text decoration
    pub fn get_decoration(&self) -> Option<Vec<Decoration>> {
        self.style.decoration.clone()
    }

    /// Gets the text content
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Set text color
    pub fn color(mut self, color: Color) -> Self {
        self.style.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.style.bg = Some(color);
        self
    }

    /// Add decoration
    pub fn decoration(mut self, decoration: Decoration) -> Self {
        match &mut self.style.decoration {
            Some(decs) => decs.push(decoration),
            None => self.style.decoration = Some(vec![decoration]),
        }
        self
    }

    /// Set padding (all sides)
    pub fn padding(mut self, padding: u8) -> Self {
        self.style.padding = Some(padding);
        self
    }

    /// Set padding for specific sides
    pub fn padding_trbl(mut self, top: u8, right: u8, bottom: u8, left: u8) -> Self {
        self.style.padding_top = Some(top);
        self.style.padding_right = Some(right);
        self.style.padding_bottom = Some(bottom);
        self.style.padding_left = Some(left);
        self
    }

    /// Set margin (all sides)
    pub fn margin(mut self, margin: u8) -> Self {
        self.style.margin = Some(margin);
        self
    }

    /// Set border style
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.style.border_style = Some(style);
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.style.border_color = Some(color);
        self
    }
}

impl StyledText for String {
    fn style(self, style_name: &str, tcss: &Termio) -> StyledString {
        let style = tcss.get_style(style_name).unwrap_or_default();
        StyledString::new(self, style)
    }

    fn color(self, color: Color) -> StyledString {
        let mut style = Style::new();
        style.fg = Some(color);
        StyledString::new(self, style)
    }

    fn bg(self, color: Color) -> StyledString {
        let mut style = Style::new();
        style.bg = Some(color);
        StyledString::new(self, style)
    }

    fn decoration(self, decoration: Decoration) -> StyledString {
        let mut style = Style::new();
        style.decoration = Some(vec![decoration]);
        StyledString::new(self, style)
    }

    fn padding(self, padding: u8) -> StyledString {
        let mut style = Style::new();
        style.padding = Some(padding);
        StyledString::new(self, style)
    }

    fn padding_trbl(self, top: u8, right: u8, bottom: u8, left: u8) -> StyledString {
        let mut style = Style::new();
        style.padding_top = Some(top);
        style.padding_right = Some(right);
        style.padding_bottom = Some(bottom);
        style.padding_left = Some(left);
        StyledString::new(self, style)
    }

    fn margin(self, margin: u8) -> StyledString {
        let mut style = Style::new();
        style.margin = Some(margin);
        StyledString::new(self, style)
    }

    fn border(self, style: BorderStyle) -> StyledString {
        let mut s = Style::new();
        s.border_style = Some(style);
        StyledString::new(self, s)
    }

    fn border_color(self, color: Color) -> StyledString {
        let mut style = Style::new();
        style.border_color = Some(color);
        StyledString::new(self, style)
    }
}

impl StyledText for &str {
    fn style(self, style_name: &str, tcss: &Termio) -> StyledString {
        let style = tcss.get_style(style_name).unwrap_or_default();
        StyledString::new(self.to_string(), style)
    }

    fn color(self, color: Color) -> StyledString {
        let mut style = Style::new();
        style.fg = Some(color);
        StyledString::new(self.to_string(), style)
    }

    fn bg(self, color: Color) -> StyledString {
        let mut style = Style::new();
        style.bg = Some(color);
        StyledString::new(self.to_string(), style)
    }

    fn decoration(self, decoration: Decoration) -> StyledString {
        let mut style = Style::new();
        style.decoration = Some(vec![decoration]);
        StyledString::new(self.to_string(), style)
    }

    fn padding(self, padding: u8) -> StyledString {
        let mut style = Style::new();
        style.padding = Some(padding);
        StyledString::new(self.to_string(), style)
    }

    fn padding_trbl(self, top: u8, right: u8, bottom: u8, left: u8) -> StyledString {
        let mut style = Style::new();
        style.padding_top = Some(top);
        style.padding_right = Some(right);
        style.padding_bottom = Some(bottom);
        style.padding_left = Some(left);
        StyledString::new(self.to_string(), style)
    }

    fn margin(self, margin: u8) -> StyledString {
        let mut style = Style::new();
        style.margin = Some(margin);
        StyledString::new(self.to_string(), style)
    }

    fn border(self, style: BorderStyle) -> StyledString {
        let mut s = Style::new();
        s.border_style = Some(style);
        StyledString::new(self.to_string(), s)
    }

    fn border_color(self, color: Color) -> StyledString {
        let mut style = Style::new();
        style.border_color = Some(color);
        StyledString::new(self.to_string(), style)
    }
}

impl fmt::Display for StyledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        // Split text into lines
        let lines: Vec<&str> = self.text.lines().collect();
        // Using UnicodeWidthStr for correct width calculation
        let max_width = lines.iter().map(|line| line.width_cjk()).max().unwrap_or(0);
        // Calculate spacing
        let (padding, margin) = self.calculate_spacing();
        // Calculate dimensions
        let (content_width, _) = self.calculate_dimensions(max_width, lines.len(), &padding);
        // Build styles
        let text_style = self.build_text_style();
        let border_style = self.build_border_style();
        let border_chars = self.get_border_chars();
        // Draw the complete element
        self.draw_element(
            &mut result,
            &lines,
            &text_style,
            &border_style,
            &border_chars,
            padding,
            margin,
            content_width,
        );
        write!(f, "{}", result)
    }
}

impl StyledString {
    /// Calculates padding and margin values from the style
    fn calculate_spacing(&self) -> (Padding, Margin) {
        let padding = self.style.padding.unwrap_or(0);
        let margin = self.style.margin.unwrap_or(0);

        (
            Padding {
                top: self.style.padding_top.unwrap_or(padding) as usize,
                bottom: self.style.padding_bottom.unwrap_or(padding) as usize,
                left: self.style.padding_left.unwrap_or(padding) as usize,
                right: self.style.padding_right.unwrap_or(padding) as usize,
            },
            Margin {
                top: self.style.margin_top.unwrap_or(margin) as usize,
                bottom: self.style.margin_bottom.unwrap_or(margin) as usize,
                left: self.style.margin_left.unwrap_or(margin) as usize,
                right: self.style.margin_right.unwrap_or(margin) as usize,
            },
        )
    }

    /// Calculates total dimensions including padding
    fn calculate_dimensions(
        &self,
        max_width: usize,
        text_height: usize,
        padding: &Padding,
    ) -> (usize, usize) {
        let content_width = max_width + padding.left + padding.right;
        let total_height = text_height + padding.top + padding.bottom;
        (content_width, total_height)
    }

    /// Builds the text style string including colors and decorations
    fn build_text_style(&self) -> String {
        let mut style = String::new();

        // Apply foreground color
        if let Some(color) = &self.style.fg {
            style.push_str(&color.to_ansi_foreground());
        }

        // Apply background color
        if let Some(color) = &self.style.bg {
            style.push_str(&color.to_ansi_background());
        }

        // Apply decorations
        if let Some(decorations) = &self.style.decoration {
            for decoration in decorations {
                style.push_str(&decoration.to_ansi());
            }
        }

        style
    }

    /// Builds the border style string
    fn build_border_style(&self) -> String {
        let mut style = String::new();
        if let Some(color) = &self.style.border_color {
            style.push_str(&color.to_ansi_foreground());
        }
        style
    }

    /// Gets border characters based on style
    fn get_border_chars(&self) -> BorderChars {
        match self.style.border_style {
            Some(BorderStyle::Solid) => BorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
            },
            Some(BorderStyle::Dashed) => BorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '┈',
                vertical: '┊',
            },
            Some(BorderStyle::Rounded) => BorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
            },
            Some(BorderStyle::Double) => BorderChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
            },
            None => BorderChars {
                top_left: ' ',
                top_right: ' ',
                bottom_left: ' ',
                bottom_right: ' ',
                horizontal: ' ',
                vertical: ' ',
            },
        }
    }

    /// Builds the background style string
    fn build_background_style(&self) -> String {
        let mut style = String::new();
        if let Some(color) = &self.style.bg {
            style.push_str(&color.to_ansi_background());
        }
        style
    }

    /// Draws the complete element with all its components
    fn draw_element(
        &self,
        output: &mut String,
        lines: &[&str],
        text_style: &str,
        border_style: &str,
        border_chars: &BorderChars,
        padding: Padding,
        margin: Margin,
        content_width: usize,
    ) {
        // Add top margin
        for _ in 0..margin.top {
            output.push('\n');
        }

        let margin_left = " ".repeat(margin.left);
        let bg_style = self.build_background_style();

        // Draw top border if border style is set
        if self.style.border_style.is_some() {
            output.push_str(&margin_left);
            output.push_str(border_style);
            output.push(border_chars.top_left);
            output.push_str(&border_chars.horizontal.to_string().repeat(content_width));
            output.push(border_chars.top_right);
            output.push_str(&Decoration::reset());
            output.push('\n');
        }

        // Draw top padding
        for _ in 0..padding.top {
            if self.style.border_style.is_some() {
                output.push_str(&margin_left);
                output.push_str(border_style);
                output.push(border_chars.vertical);
                output.push_str(&bg_style);
                output.push_str(&" ".repeat(content_width));
                output.push_str(&Decoration::reset());
                output.push_str(border_style);
                output.push(border_chars.vertical);
                output.push_str(&Decoration::reset());
            }
            output.push('\n');
        }

        // Draw text lines with padding
        lines.iter().enumerate().for_each(|(i, line)| {
            output.push_str(&margin_left);
            if self.style.border_style.is_some() {
                output.push_str(border_style);
                output.push(border_chars.vertical);
                output.push_str(&Decoration::reset());
            }
            output.push_str(&bg_style);
            // Left padding
            output.push_str(&" ".repeat(padding.left));
            // Text content with style
            output.push_str(text_style);
            output.push_str(line);
            // Using UnicodeWidthStr for width calculation
            let line_width = line.width();
            let extra_width = line.width_cjk() - line.width();
            let padding_after_text = content_width - line_width - padding.left - padding.right + extra_width;
            
            output.push_str(&" ".repeat(padding_after_text));
            output.push_str(&Decoration::reset());
            // Right padding
            output.push_str(&bg_style);
            output.push_str(&" ".repeat(padding.right));
            output.push_str(&Decoration::reset());
            if self.style.border_style.is_some() {
                output.push_str(border_style);
                output.push(border_chars.vertical);
                output.push_str(&Decoration::reset());
            }
            // Add a new line if it's not the last line and border style is set
            if i < lines.len() && self.style.border_style.is_some() {
                output.push('\n');
            }
        });

        // Draw bottom padding
        for _ in 0..padding.bottom {
            if self.style.border_style.is_some() {
                output.push_str(&margin_left);
                output.push_str(border_style);
                output.push(border_chars.vertical);
                output.push_str(&bg_style);
                output.push_str(&" ".repeat(content_width));
                output.push_str(&Decoration::reset());
                output.push_str(border_style);
                output.push(border_chars.vertical);
                output.push_str(&Decoration::reset());
            }
            output.push('\n');
        }

        // Draw bottom border if border style is set
        if self.style.border_style.is_some() {
            output.push_str(&margin_left);
            output.push_str(border_style);
            output.push(border_chars.bottom_left);
            output.push_str(&border_chars.horizontal.to_string().repeat(content_width));
            output.push(border_chars.bottom_right);
            output.push_str(&Decoration::reset());
        }

        // Add bottom margin
        for _ in 0..margin.bottom {
            output.push('\n');
        }
    }
}

/// Represents padding values for all sides
#[derive(Debug, Clone)]
struct Padding {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

/// Represents margin values for all sides
#[derive(Debug, Clone)]
struct Margin {
    top: usize,
    bottom: usize,
    left: usize,
    #[allow(dead_code)]
    right: usize,
}

/// Represents border characters for all sides
#[derive(Debug, Clone)]
struct BorderChars {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
}
