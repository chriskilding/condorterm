use iocraft::prelude::*;
use crate::components::instruments::shared::style::{BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE};
use crate::components::instruments::shared::label::Label;

#[derive(Default, Props)]
pub struct VarioProps {
    pub value: f32,
    pub units: String
}

#[component]
pub fn Vario(props: &VarioProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Vario")
            Box(border_color: INSTRUMENT_BORDER_COLOR, border_style: INSTRUMENT_BORDER_STYLE, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(background_color: Color::Grey, height: 1) {
                    Box(width: 9, border_style: BorderStyle::None) {}
                    Box(width: 3, border_style: BorderStyle::None, background_color: Color::Green) {}
                    Box(width: 1, border_style: BorderStyle::None, background_color: Color::DarkGreen) {}
                }
                Text(content: format!("+{}", props.value), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
            }
        }
    }
}