use iocraft::prelude::*;
use crate::components::instruments::style::{BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE};
use crate::components::instruments::label::Label;

#[component]
pub fn SlipIndicator() -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Slip")
            Box(border_style: INSTRUMENT_BORDER_STYLE, border_color: INSTRUMENT_BORDER_COLOR, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(background_color: Color::Grey, height: 1, justify_content: JustifyContent::Center) {
                    Box(background_color: Color::DarkGrey, width: 1, border_style: BorderStyle::None)
                }
                Text(content: "0.0", wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: "rad", wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
            }
        }
    }
}