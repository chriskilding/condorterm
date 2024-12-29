use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::style::{Style, BOX_WIDTH};
use iocraft::prelude::*;

#[derive(Default)]
pub struct Scale {
    min: f32,
    max: f32,
}

impl Scale {
    pub fn create(min: f32, baseline: f32, max: f32) -> Option<Self> {
        // TODO use the baseline parameter
        if (min <= baseline) && (baseline <= max) {
            let scale = Self { min, max };
            Some(scale)
        } else {
            None
        }
    }

    pub fn contains(&self, value: f32) -> bool {
        if (value >= self.min) && (value <= self.max) {
            true
        } else {
            false
        }
    }
}

#[derive(Default, Props)]
pub struct AccelerometerProps {
    pub value: Option<State<f32>>,
    pub units: String,
    pub scale: Scale,
    pub inop: bool,
    pub style: Style,
}

#[component]
pub fn Accelerometer(props: &AccelerometerProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "G-Meter")
            Box(border_style: props.style.border_style, border_color: props.style.border_color, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding_top: 1, padding_left: 1, padding_right: 1) {
                Box(background_color: Color::Grey, height: 1) {
                    Box(width: 9)
                    Box(width: 2, background_color: bar_color(props.value, &props.scale))
                }
                Text(content: format!("{:+.1}", props.value.map_or(0.0, |x| x.get())), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
                Box(justify_content: JustifyContent::End, height: 1) {
                    Text(content: inop_text(props.inop), color: props.style.inop_color)
                }
            }
        }
    }
}

fn bar_color(value: Option<State<f32>>, scale: &Scale) -> Color {
    match value.map(|x| x.get()) {
        Some(v) => {
            if scale.contains(v) {
                Color::DarkGreen
            } else {
                Color::DarkRed
            }
        }
        None => Color::Grey,
    }
}

fn inop_text(inop: bool) -> String {
    if inop {
        "INOP".into()
    } else {
        "".into()
    }
}
