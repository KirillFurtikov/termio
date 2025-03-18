use crate::border::BorderStyle;
use crate::color::Color;
use crate::decoration::Decoration;
use crate::style::Style;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Termio {
    styles: HashMap<String, Style>,
}

/// Custom error type for TCSS parsing errors
#[derive(Debug)]
pub enum ParseError {
    InvalidSyntax(String),
    DuplicateElement(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            ParseError::DuplicateElement(name) => write!(f, "Duplicate element name: {}", name),
        }
    }
}

impl Error for ParseError {}

impl From<String> for ParseError {
    fn from(s: String) -> Self {
        ParseError::InvalidSyntax(s)
    }
}

impl Termio {
    /// Creates a new Termio with an empty style map.
    pub fn new() -> Self {
        Termio {
            styles: HashMap::new(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, ParseError> {
        let mut tcss = Self::new();
        let content = std::fs::read_to_string(path).map_err(|e| ParseError::InvalidSyntax(e.to_string()))?;
        tcss.parse(&content)?;
        Ok(tcss)
    }

    /// Retrieves a style by name, returning None if not found.
    pub fn get_style(&self, name: &str) -> Option<Style> {
        self.styles.get(name).cloned()
    }

    /// Parses TCSS content and populates the style map.
    pub fn parse(&mut self, content: &str) -> Result<(), ParseError> {
        let mut lines = content.lines().peekable();
        let mut current_style = None;
        let mut current_name = None;

        while let Some(line) = lines.next() {
            let line = line.trim();
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if line.starts_with("@element") {
                if let Some(name) = current_name {
                    if self.styles.contains_key(&name) {
                        return Err(ParseError::DuplicateElement(name));
                    }
                    self.styles.insert(name, current_style.unwrap_or_default());
                }

                let name = line
                    .split('"')
                    .nth(1)
                    .ok_or_else(|| ParseError::InvalidSyntax("Missing element name".to_string()))?;
                current_name = Some(name.to_string());
                current_style = Some(Style::new());
            } else if let Some(style) = &mut current_style {
                if line == "}" {
                    if let Some(name) = current_name.take() {
                        if self.styles.contains_key(&name) {
                            return Err(ParseError::DuplicateElement(name));
                        }
                        self.styles
                            .insert(name, current_style.take().unwrap_or_default());
                    }
                } else {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() != 2 {
                        return Err(ParseError::InvalidSyntax(format!(
                            "Invalid property: {}",
                            line
                        )));
                    }

                    let property = parts[0].trim();
                    let value = parts[1].trim().trim_end_matches(';');

                    match property {
                        "color" => {
                            style.fg = Some(
                                Color::from_str(value)
                                    .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?,
                            )
                        }
                        "background" => {
                            style.bg = Some(
                                Color::from_str(value)
                                    .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?,
                            )
                        }
                        "decoration" => style.decoration = Some(self.parse_decoration(value)?),
                        "padding" => {
                            let values: Vec<&str> = value.split_whitespace().collect();
                            match values.len() {
                                1 => {
                                    let pad = values[0].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid padding value: {}",
                                            value
                                        ))
                                    })?;
                                    style.padding = Some(pad);
                                    style.padding_top = Some(pad);
                                    style.padding_bottom = Some(pad);
                                    style.padding_left = Some(pad);
                                    style.padding_right = Some(pad);
                                }
                                2 => {
                                    let v = values[0].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid padding value: {}",
                                            value
                                        ))
                                    })?;
                                    let h = values[1].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid padding value: {}",
                                            value
                                        ))
                                    })?;
                                    style.padding_top = Some(v);
                                    style.padding_bottom = Some(v);
                                    style.padding_left = Some(h);
                                    style.padding_right = Some(h);
                                }
                                4 => {
                                    let top = values[0].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid padding value: {}",
                                            value
                                        ))
                                    })?;
                                    let right = values[1].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid padding value: {}",
                                            value
                                        ))
                                    })?;
                                    let bottom = values[2].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid padding value: {}",
                                            value
                                        ))
                                    })?;
                                    let left = values[3].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid padding value: {}",
                                            value
                                        ))
                                    })?;
                                    style.padding_top = Some(top);
                                    style.padding_right = Some(right);
                                    style.padding_bottom = Some(bottom);
                                    style.padding_left = Some(left);
                                }
                                _ => {
                                    return Err(ParseError::InvalidSyntax(
                                        "Invalid padding format. Use 1, 2, or 4 values".to_string(),
                                    ))
                                }
                            }
                        }
                        "padding-top" => {
                            style.padding_top = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid padding-top value: {}",
                                    value
                                ))
                            })?)
                        }
                        "padding-bottom" => {
                            style.padding_bottom = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid padding-bottom value: {}",
                                    value
                                ))
                            })?)
                        }
                        "padding-left" => {
                            style.padding_left = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid padding-left value: {}",
                                    value
                                ))
                            })?)
                        }
                        "padding-right" => {
                            style.padding_right = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid padding-right value: {}",
                                    value
                                ))
                            })?)
                        }
                        "margin" => {
                            let values: Vec<&str> = value.split_whitespace().collect();
                            match values.len() {
                                1 => {
                                    let margin = values[0].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid margin value: {}",
                                            value
                                        ))
                                    })?;
                                    style.margin = Some(margin);
                                    style.margin_top = Some(margin);
                                    style.margin_bottom = Some(margin);
                                    style.margin_left = Some(margin);
                                    style.margin_right = Some(margin);
                                }
                                2 => {
                                    let v = values[0].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid margin value: {}",
                                            value
                                        ))
                                    })?;
                                    let h = values[1].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid margin value: {}",
                                            value
                                        ))
                                    })?;
                                    style.margin_top = Some(v);
                                    style.margin_bottom = Some(v);
                                    style.margin_left = Some(h);
                                    style.margin_right = Some(h);
                                }
                                4 => {
                                    let top = values[0].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid margin value: {}",
                                            value
                                        ))
                                    })?;
                                    let right = values[1].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid margin value: {}",
                                            value
                                        ))
                                    })?;
                                    let bottom = values[2].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid margin value: {}",
                                            value
                                        ))
                                    })?;
                                    let left = values[3].parse().map_err(|_| {
                                        ParseError::InvalidSyntax(format!(
                                            "Invalid margin value: {}",
                                            value
                                        ))
                                    })?;
                                    style.margin_top = Some(top);
                                    style.margin_right = Some(right);
                                    style.margin_bottom = Some(bottom);
                                    style.margin_left = Some(left);
                                }
                                _ => {
                                    return Err(ParseError::InvalidSyntax(
                                        "Invalid margin format. Use 1, 2, or 4 values".to_string(),
                                    ))
                                }
                            }
                        }
                        "margin-top" => {
                            style.margin_top = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid margin-top value: {}",
                                    value
                                ))
                            })?)
                        }
                        "margin-bottom" => {
                            style.margin_bottom = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid margin-bottom value: {}",
                                    value
                                ))
                            })?)
                        }
                        "margin-left" => {
                            style.margin_left = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid margin-left value: {}",
                                    value
                                ))
                            })?)
                        }
                        "margin-right" => {
                            style.margin_right = Some(value.parse().map_err(|_| {
                                ParseError::InvalidSyntax(format!(
                                    "Invalid margin-right value: {}",
                                    value
                                ))
                            })?)
                        }
                        "border-color" => {
                            style.border_color = Some(
                                Color::from_str(value)
                                    .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?,
                            )
                        }
                        "border-style" => {
                            style.border_style = Some(self.parse_border_style(value)?)
                        }
                        "border" => {
                            let (s, c) = value.split_once(" ").unwrap();
                            style.border_style = Some(self.parse_border_style(s)?);
                            style.border_color = Some(
                                Color::from_str(c)
                                    .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?,
                            );
                        }
                        _ => {
                            return Err(ParseError::InvalidSyntax(format!(
                                "Unknown property: {}",
                                property
                            )))
                        }
                    }
                }
            }
        }

        // Handle the last style if exists
        if let Some(name) = current_name {
            if self.styles.contains_key(&name) {
                return Err(ParseError::DuplicateElement(name));
            }
            self.styles.insert(name, current_style.unwrap_or_default());
        }

        Ok(())
    }

    /// Parses a decoration string into a vector of decorations
    fn parse_decoration(&self, value: &str) -> Result<Vec<Decoration>, ParseError> {
        value
            .split_whitespace()
            .map(|d| Decoration::from_str(d).map_err(|e| ParseError::InvalidSyntax(e.to_string())))
            .collect()
    }

    /// Parses a border style string into a BorderStyle
    fn parse_border_style(&self, value: &str) -> Result<BorderStyle, ParseError> {
        BorderStyle::from_str(value).map_err(|e| ParseError::InvalidSyntax(e.to_string()))
    }

    pub fn add_style(&mut self, name: &str, style: Style) {
        self.styles.insert(name.to_string(), style);
    }
}
