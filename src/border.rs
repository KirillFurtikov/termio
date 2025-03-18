use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BorderStyle {
    Solid,   // ┌─┐ │ └─┘
    Dashed,  // ┌┈┐ ┊ └┈┘
    Rounded, // ╭─╮ │ ╰─╯
    Double,  // ╔═╗ ║ ╚═╝
}

impl FromStr for BorderStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "solid" => Ok(BorderStyle::Solid),
            "dashed" => Ok(BorderStyle::Dashed),
            "rounded" => Ok(BorderStyle::Rounded),
            "double" => Ok(BorderStyle::Double),
            _ => Err(format!("Unknown border style: {}", s)),
        }
    }
}
