use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::style::{Style, BOX_WIDTH};
use iocraft::prelude::*;

#[derive(Default, Copy, Clone)]
pub struct Limits {
    pub min: f32,
    pub max: f32,
}

impl Limits {
    fn contains(&self, value: f32) -> bool {
        if (value >= self.min) && (value <= self.max) {
            true
        } else {
            false
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Scale {
    limits: Limits,
    /// The magnitude of the scale displayed (which is wider than the limits)
    range: u32,
}

impl Scale {
    pub fn create(limits: Limits, range: u32) -> Option<Self> {
        if (limits.min.abs().ceil() as u32 <= range) && (limits.max.abs().ceil() as u32 <= range) {
            if limits.min < limits.max {
                let scale = Self { limits, range };
                return Some(scale);
            }
        }

        return None;
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
    let bar = bar(
        BarSpec {
            widget_size: (BOX_WIDTH - 5) as u32,
            scale: props.scale,
        },
        props.value.map(|x| x.get()),
    );

    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "G-Meter")
            Box(border_style: props.style.border_style, border_color: props.style.border_color, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding_top: 1, padding_left: 1, padding_right: 2) {
                Box(background_color: Color::Grey, height: 1, justify_content: bar.justification) {
                    Box(width: bar.left_padding)
                    Box(width: bar.main, background_color: bar.color)
                    Box(width: bar.right_padding)
                }
                Text(content: format!("{:+.1}", props.value.map_or(0.0, |x| x.get())), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: props.style.units_color)
                Box(justify_content: JustifyContent::End, height: 1) {
                    Text(content: inop_text(props.inop), color: props.style.inop_color)
                }
            }
        }
    }
}

fn bar_color(value: f32, limits: &Limits) -> Color {
    if limits.contains(value) {
        Color::DarkGreen
    } else {
        Color::DarkRed
    }
}

fn inop_text(inop: bool) -> String {
    if inop {
        "INOP".into()
    } else {
        "".into()
    }
}

struct Bar {
    left_padding: u32,
    main: u32,
    right_padding: u32,
    color: Color,
    justification: JustifyContent,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            left_padding: 0,
            main: 0,
            right_padding: 0,
            color: Color::Green,
            justification: JustifyContent::Start,
        }
    }
}

struct BarSpec {
    /// Total real width of the bar widget
    widget_size: u32,
    scale: Scale,
}

fn bar(spec: BarSpec, value: Option<f32>) -> Bar {
    let BarSpec { widget_size, scale } = spec;
    let Scale { limits, range } = scale;
    let total_width = widget_size;
    let half_width = (total_width as f32) / 2.0;

    match value {
        Some(v) => {
            let rescaled_proportion = (v.abs() / range as f32) / 2.0;
            let bar = rescaled_proportion * (total_width as f32);
            let main = bar as u32;

            let color: Color = bar_color(v, &limits);

            let left_padding: u32;
            let right_padding: u32;
            let justification: JustifyContent;

            if v >= 0.0 {
                left_padding = half_width as u32;
                right_padding = 0;
                justification = JustifyContent::Start;
            } else {
                left_padding = 0;
                right_padding = half_width as u32;
                justification = JustifyContent::End;
            }

            Bar {
                left_padding,
                main,
                right_padding,
                justification,
                color,
            }
        }
        None => Bar::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: BarSpec = BarSpec {
        widget_size: 20,
        scale: Scale {
            range: 20,
            limits: Limits {
                min: -4.0,
                max: 6.5,
            },
        },
    };

    #[test]
    fn should_have_green_color_if_value_within_limits() {
        let result = bar(SPEC, Some(1.0));

        assert_eq!(Color::DarkGreen, result.color);
    }

    #[test]
    fn should_have_red_color_if_value_greater_than_max_limit() {
        let result = bar(SPEC, Some(7.0));

        assert_eq!(Color::DarkRed, result.color);
    }

    #[test]
    fn should_have_red_color_if_value_less_than_min_limit() {
        let result = bar(SPEC, Some(-5.0));

        assert_eq!(Color::DarkRed, result.color);
    }
}
