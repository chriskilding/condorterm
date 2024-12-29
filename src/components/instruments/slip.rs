use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::style::{
    BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE,
};
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct TurnSlipIndicatorProps {
    /// A value between -1 and 1
    pub value: Option<State<f32>>,
}

#[component]
pub fn TurnSlipIndicator(props: &TurnSlipIndicatorProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Slip")
            Box(border_style: INSTRUMENT_BORDER_STYLE, border_color: INSTRUMENT_BORDER_COLOR, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(background_color: Color::Grey, height: 1) {
                    Box(width: left_padding_width(props.value), border_style: BorderStyle::None)
                    Box(background_color: Color::DarkGrey, width: 1, border_style: BorderStyle::None)
                }
                Text(content: format!("{:+.2}", props.value.map_or(0.0, |x| x.get())), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Box(height: 1)
            }
        }
    }
}

fn left_padding_width(value: Option<State<f32>>) -> u32 {
    // TODO remove the constant reference
    let total_width = (BOX_WIDTH - 2) as f32;

    match value.map(|x| x.get()) {
        Some(v) => {
            let rescaled_proportion = (v + 1.0) / 2.0;
            let bar = rescaled_proportion * total_width;
            bar as u32
        }
        None => 0,
    }
}
