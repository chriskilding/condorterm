use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::style::{
    BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE,
};
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct AccelerometerProps {
    pub value: Option<State<f32>>,
    pub units: String,
}

#[component]
pub fn Accelerometer(props: &AccelerometerProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "G-Meter")
            Box(border_style: INSTRUMENT_BORDER_STYLE, border_color: INSTRUMENT_BORDER_COLOR, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(background_color: Color::Grey, height: 1) {
                    Box(width: 9, border_style: BorderStyle::None) {}
                    Box(width: 1, border_style: BorderStyle::None, background_color: Color::Green) {}
                    Box(width: 1, border_style: BorderStyle::None, background_color: Color::DarkGreen) {}
                }
                Text(content: format!("{:+.1}", props.value.map_or(0.0, |x| x.get())), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
            }
        }
    }
}
