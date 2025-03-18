/// Creates a new style with specified properties
#[macro_export]
macro_rules! style {
    (
        $($field:ident : $value:expr),* $(,)?
    ) => {{
        let mut style = $crate::Style::default();
        $(
            style.$field = Some($value);
        )*
        style
    }};
}

/// Creates a parser with defined styles
#[macro_export]
macro_rules! tcss {
    (
        $(
            $name:expr => {
                $(
                    $field:ident: $value:expr
                ),* $(,)?
            }
        ),* $(,)?
    ) => {{
        let mut parser = $crate::Termio::new();
        $(
            let style = $crate::style! {
                $($field: $value),*
            };
            parser.add_style($name, style);
        )*
        parser
    }};
}

// Helper macro for testing
#[cfg(test)]
mod tests {
    use crate::border::BorderStyle;
    use crate::color::Color;
    use crate::decoration::Decoration;
    use crate::StyledText;

    #[test]
    fn test_style_macro() {
        let style = style! {
            fg: Color::Green,
            bg: Color::Black,
            decoration: vec![Decoration::Bold],
            padding: 1,
            margin: 1,
            border_color: Color::Yellow,
            border_style: BorderStyle::Rounded
        };

        assert_eq!(style.fg, Some(Color::Green));
        assert_eq!(style.bg, Some(Color::Black));
        assert_eq!(style.decoration, Some(vec![Decoration::Bold]));
        assert_eq!(style.padding, Some(1));
        assert_eq!(style.margin, Some(1));
        assert_eq!(style.border_color, Some(Color::Yellow));
        assert_eq!(style.border_style, Some(BorderStyle::Rounded));
    }

    #[test]
    fn test_tcss_macro() {
        let parser = tcss! {
            "header" => {
                fg: Color::Green,
                bg: Color::Black,
                decoration: vec![Decoration::Bold],
                padding: 1,
                margin: 1,
                border_color: Color::Yellow,
                border_style: BorderStyle::Rounded
            },
            "warning" => {
                fg: Color::Yellow,
                bg: Color::IntenseRed,
                decoration: vec![Decoration::Bold, Decoration::Italic],
                border_style: BorderStyle::Dashed,
                padding: 1
            }
        };

        let header = "Test Header".style("header", &parser);
        let warning = "Test Warning".style("warning", &parser);

        assert!(format!("{}", header).contains("\x1b[32m")); // green
        assert!(format!("{}", warning).contains("\x1b[33m")); // yellow
    }
}
