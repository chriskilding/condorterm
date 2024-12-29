use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::string::s;
use crate::components::instruments::shared::style::{Style, BOX_WIDTH};
use chrono::NaiveTime;
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct ClockProps {
    pub value: Option<State<NaiveTime>>,
    pub timezone: String,
    pub style: Style,
}

#[component]
pub fn Clock(props: &ClockProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Time")
            Box(border_style: props.style.border_style, border_color: props.style.border_color, flex_direction: FlexDirection::Column, width: BOX_WIDTH, align_content: JustifyContent::Center, padding: 1) {
                Box(height: 1, justify_content: JustifyContent::Center) {
                    Box(background_color: Color::Grey, height: 1, width: 10, justify_content: JustifyContent::Center) {
                        Text(content: s(props.value), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::Black)
                    }
                }
                Text(content: format!("{}", props.timezone), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: props.style.units_color)
                Box(height: 1)
            }
        }
    }
}
