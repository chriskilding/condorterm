use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::string::s;
use crate::components::instruments::shared::style::{
    BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE,
};
use iocraft::prelude::*;

#[derive(Default)]
pub struct Scale {
    va: u32,
    vne: u32,
    vd: u32,
}

impl Scale {
    pub fn create(va: u32, vne: u32, vd: u32) -> Option<Scale> {
        if (va < vne) && (vne < vd) {
            let scale = Scale { va, vne, vd };
            return Some(scale);
        }

        None
    }

    pub fn color(&self, v: u32) -> Color {
        if v < self.va {
            Color::DarkGreen
        } else if v < self.vne {
            Color::DarkYellow
        } else {
            Color::DarkRed
        }
    }

    /// How big the value is, as a proportion of the scale (between 0 and 1)
    pub fn proportion(&self, value: u32) -> f32 {
        let v = value as f32;
        let range = self.vd as f32;
        return v / range;
    }
}

#[derive(Default, Props)]
pub struct AirSpeedIndicatorProps {
    pub value: Option<State<u32>>,
    pub units: String,
    pub scale: Scale,
}

#[component]
pub fn AirSpeedIndicator(props: &AirSpeedIndicatorProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Airspeed")
            Box(border_color: INSTRUMENT_BORDER_COLOR, border_style: INSTRUMENT_BORDER_STYLE, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding: 1) {
                Box(background_color: Color::Grey, height: 1) {
                    Box(width: bar_width(&props.scale, props.value), border_style: BorderStyle::None, background_color: bar_color(&props.scale, props.value))
                }
                Text(content: s(props.value), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
            }
        }
    }
}

fn bar_color(scale: &Scale, value: Option<State<u32>>) -> Color {
    match value.map(|x| x.get()) {
        Some(v) => scale.color(v),
        None => Color::Grey,
    }
}

fn bar_width(scale: &Scale, value: Option<State<u32>>) -> u32 {
    // TODO remove the constant reference
    let total_width = (BOX_WIDTH - 2) as f32;

    match value.map(|x| x.get()) {
        Some(v) => {
            let proportion = scale.proportion(v);
            let bar = proportion * total_width;
            bar as u32
        }
        None => 0,
    }
}
