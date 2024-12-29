use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::string::s;
use crate::components::instruments::shared::style::{
    BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE,
};
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct CompassProps {
    pub value: Option<State<u32>>,
    pub units: String,
}

#[component]
pub fn Compass(props: &CompassProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Compass")
            Box(border_style: INSTRUMENT_BORDER_STYLE, border_color: INSTRUMENT_BORDER_COLOR, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(height: 1, justify_content: JustifyContent::Center) {
                    Box(background_color: Color::Grey, height: 1, width: 5) {
                        Box(width: 2, border_style: BorderStyle::None) {}
                        Text(content: props.value.map_or(String::from(""), |t| get_nearest_arrow_glyph(t.get() as i32)), color: Color::Red, weight: Weight::Bold, wrap: TextWrap::NoWrap)
                    }
                }
                Text(content: s(props.value), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
            }
        }
    }
}

fn get_nearest_arrow_glyph(degrees: i32) -> String {
    let arr = [0, 45, 90, 135, 180, 225, 270, 315, 360];

    let mut current = arr[0];

    for val in arr {
        if (degrees - val).abs() < (degrees - current).abs() {
            current = val;
        }
    }

    return get_arrow_glyph(current);
}

fn get_arrow_glyph(degrees: i32) -> String {
    let glyph = match degrees {
        0 => "↑",
        45 => "↗",
        90 => "→",
        135 => "↘",
        180 => "↓",
        225 => "↙",
        270 => "←",
        315 => "↖",
        360 => "↑",
        _ => "",
    };

    glyph.to_owned()
}
