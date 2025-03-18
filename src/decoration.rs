use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Decoration {
    None,
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,
    RapidBlink,
    Reverse,
    Conceal,
    CrossedOut,
    DoubleUnderline,
    Overline,
    Hidden,
    Strikethrough,
    Framed,
    Encircled,
}

impl Decoration {
    /// Converts the decoration to an ANSI escape sequence
    pub fn to_ansi(&self) -> String {
        let code = match self {
            Decoration::None => "0",
            Decoration::Bold => "1",
            Decoration::Faint => "2",
            Decoration::Italic => "3",
            Decoration::Underline => "4",
            Decoration::Blink => "5",
            Decoration::RapidBlink => "6",
            Decoration::Reverse => "7",
            Decoration::Conceal => "8",
            Decoration::CrossedOut => "9",
            Decoration::DoubleUnderline => "21",
            Decoration::Overline => "53",
            Decoration::Hidden => "8",
            Decoration::Strikethrough => "9",
            Decoration::Framed => "51",
            Decoration::Encircled => "52",
        };
        format!("\x1b[{}m", code)
    }

    /// Resets all decorations
    pub fn reset() -> String {
        "\x1b[0m".to_string()
    }
}

impl FromStr for Decoration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Decoration::None),
            "bold" => Ok(Decoration::Bold),
            "faint" => Ok(Decoration::Faint),
            "italic" => Ok(Decoration::Italic),
            "underline" => Ok(Decoration::Underline),
            "blink" => Ok(Decoration::Blink),
            "rapid-blink" => Ok(Decoration::RapidBlink),
            "reverse" => Ok(Decoration::Reverse),
            "conceal" => Ok(Decoration::Conceal),
            "crossed-out" => Ok(Decoration::CrossedOut),
            "double-underline" => Ok(Decoration::DoubleUnderline),
            "overline" => Ok(Decoration::Overline),
            "hidden" => Ok(Decoration::Hidden),
            "strikethrough" => Ok(Decoration::Strikethrough),
            "framed" => Ok(Decoration::Framed),
            "encircled" => Ok(Decoration::Encircled),
            _ => Err(format!("Unknown decoration: {}", s)),
        }
    }
}
