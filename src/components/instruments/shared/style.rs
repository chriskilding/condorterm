use iocraft::prelude::*;

pub const BOX_WIDTH: i32 = 23;

pub struct Style {
    pub border_color: iocraft::Color,
    pub border_style: iocraft::components::BorderStyle,
    pub inop_color: iocraft::Color,
    pub units_color: iocraft::Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            border_color: Color::DarkGrey,
            border_style: BorderStyle::Round,
            inop_color: Color::DarkRed,
            units_color: Color::DarkGrey,
        }
    }
}
