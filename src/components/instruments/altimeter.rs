use iocraft::prelude::*;
use crate::components::instruments::shared::style::{BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE};
use crate::components::instruments::shared::label::Label;

#[derive(Default, Props)]
pub struct AltimeterProps {
    pub value: i32,
    pub units: String
}

#[component]
pub fn Altimeter(props: &AltimeterProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Altimeter")
            Box(border_color: INSTRUMENT_BORDER_COLOR, border_style: INSTRUMENT_BORDER_STYLE, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(background_color: Color::Grey, height: 1, justify_content: JustifyContent::End) {
                    Text(content: format!("{}", props.value), wrap: TextWrap::NoWrap, align: TextAlign::Right, color: Color::Black)
                    Box(width: 1, border_style: BorderStyle::None) {}
                }
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
                Box(height: 1)
            }
        }
    }
}