use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    IntenseBlack,   // Bright black
    IntenseRed,     // Bright red
    IntenseGreen,   // Bright green
    IntenseYellow,  // Bright yellow
    IntenseBlue,    // Bright blue
    IntenseMagenta, // Bright magenta
    IntenseCyan,    // Bright cyan
    IntenseWhite,   // Bright white
    Code(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    /// Creates a custom RGB color.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Converts the color to an ANSI foreground color code
    pub fn to_ansi_foreground(&self) -> String {
        match self {
            Color::Black => "\x1b[30m".to_string(),
            Color::Red => "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),
            Color::IntenseBlack => "\x1b[90m".to_string(),
            Color::IntenseRed => "\x1b[91m".to_string(),
            Color::IntenseGreen => "\x1b[92m".to_string(),
            Color::IntenseYellow => "\x1b[93m".to_string(),
            Color::IntenseBlue => "\x1b[94m".to_string(),
            Color::IntenseMagenta => "\x1b[95m".to_string(),
            Color::IntenseCyan => "\x1b[96m".to_string(),
            Color::IntenseWhite => "\x1b[97m".to_string(),
            Color::Code(u) => format!("\x1b[38;5;{u}m"),
            Color::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }

    /// Converts the color to an ANSI background color code
    pub fn to_ansi_background(&self) -> String {
        match self {
            Color::Black => "\x1b[40m".to_string(),
            Color::Red => "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),
            Color::IntenseBlack => "\x1b[100m".to_string(),
            Color::IntenseRed => "\x1b[101m".to_string(),
            Color::IntenseGreen => "\x1b[102m".to_string(),
            Color::IntenseYellow => "\x1b[103m".to_string(),
            Color::IntenseBlue => "\x1b[104m".to_string(),
            Color::IntenseMagenta => "\x1b[105m".to_string(),
            Color::IntenseCyan => "\x1b[106m".to_string(),
            Color::IntenseWhite => "\x1b[107m".to_string(),
            Color::Code(u) => format!("\x1b[48;5;{u}m"),
            Color::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }
}

impl FromStr for Color {
    type Err = String;

    /// Parses a string into a Color variant.
    /// Supports color names (e.g., "black", "i-red"), and RGB (e.g., "rgb(255, 0, 0)").
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle named colors
        match s.to_lowercase().as_str() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "i-black" | "intense-black" => Ok(Color::IntenseBlack),
            "i-red" | "intense-red" => Ok(Color::IntenseRed),
            "i-green" | "intense-green" => Ok(Color::IntenseGreen),
            "i-yellow" | "intense-yellow" => Ok(Color::IntenseYellow),
            "i-blue" | "intense-blue" => Ok(Color::IntenseBlue),
            "i-magenta" | "intense-magenta" => Ok(Color::IntenseMagenta),
            "i-cyan" | "intense-cyan" => Ok(Color::IntenseCyan),
            "i-white" | "intense-white" => Ok(Color::IntenseWhite),
            _ => {
                // Handle RGB colors
                if s.starts_with("rgb(") && s.ends_with(")") {
                    let rgb_str = &s[4..s.len() - 1];
                    let parts: Vec<&str> = rgb_str.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 3 {
                        let r = parts[0]
                            .parse::<u8>()
                            .map_err(|_| "Invalid RGB value".to_string())?;
                        let g = parts[1]
                            .parse::<u8>()
                            .map_err(|_| "Invalid RGB value".to_string())?;
                        let b = parts[2]
                            .parse::<u8>()
                            .map_err(|_| "Invalid RGB value".to_string())?;
                        return Ok(Color::Rgb(r, g, b));
                    }
                }
                // Handle 8-bit color codes
                if let Ok(code) = s.parse::<u8>() {
                    return Ok(Color::Code(code));
                }
                Err(format!("Unknown color: {}", s))
            }
        }
    }
}
