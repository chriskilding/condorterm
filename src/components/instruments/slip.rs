use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::style::{
    BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE,
};
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct TurnSlipIndicatorProps {
    pub value: f32,
}

#[component]
pub fn TurnSlipIndicator(props: &TurnSlipIndicatorProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Slip")
            Box(border_style: INSTRUMENT_BORDER_STYLE, border_color: INSTRUMENT_BORDER_COLOR, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(background_color: Color::Grey, height: 1, justify_content: JustifyContent::Center) {
                    Box(background_color: Color::DarkGrey, width: 1, border_style: BorderStyle::None)
                }
                Text(content: format!("{}", props.value), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: "rad", wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
            }
        }
    }
}
