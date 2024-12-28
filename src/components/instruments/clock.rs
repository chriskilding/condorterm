use iocraft::prelude::*;
use crate::components::instruments::shared::style::{BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE};
use crate::components::instruments::shared::label::Label;

#[derive(Default, Props)]
pub struct ClockProps {
    pub timezone: String
}

#[component]
pub fn Clock(props: &ClockProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Time")
            Box(border_style: INSTRUMENT_BORDER_STYLE, border_color: INSTRUMENT_BORDER_COLOR, flex_direction: FlexDirection::Column, width: BOX_WIDTH, align_content: JustifyContent::Center, padding: 1) {
                Box(height: 1, justify_content: JustifyContent::Center) {
                    Box(background_color: Color::Grey, height: 1, width: 10, justify_content: JustifyContent::Center) {
                        Text(content: "12:35:00", wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::Black)
                    }
                }
                Text(content: format!("{}", props.timezone), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
                Box(height: 1)
            }
        }
    }
}