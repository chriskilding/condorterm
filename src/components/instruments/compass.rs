use iocraft::prelude::*;
use crate::components::instruments::style::{BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE};
use crate::components::instruments::label::Label;

#[derive(Default, Props)]
pub struct CompassProps {
    pub value: i32,
    pub units: String
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
                        Text(content: "↙", color: Color::Red, weight: Weight::Bold, wrap: TextWrap::NoWrap)
                    }
                }
                Text(content: format!("{}", props.value), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
            }
        }
    }
}