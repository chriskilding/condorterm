use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::style::{Style, BOX_WIDTH};
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct VarioProps {
    pub value: Option<State<f32>>,
    pub units: String,
    /// The vario's range, from +<scale> knots to -<scale> knots
    pub scale: u32,
    pub inop: bool,
    pub style: Style,
}

#[component]
pub fn Vario(props: &VarioProps) -> impl Into<AnyElement<'static>> {
    let widths = bar_widths(
        BarSpec {
            scale: props.scale,
            widget_size: (BOX_WIDTH - 5) as u32,
        },
        props.value.map(|x| x.get()),
    );

    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Vario")
            Box(border_color: props.style.border_color, border_style: props.style.border_style, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding_top: 1, padding_left: 1, padding_right: 2) {
                Box(background_color: Color::Grey, height: 1, justify_content: widths.justification) {
                    Box(width: widths.left_padding)
                    Box(width: widths.main, background_color: Color::DarkGrey)
                    Box(width: widths.right_padding)
                }
                Text(content: format!("{:+.1}", props.value.map_or(0.0, |x| x.get())), wrap: TextWrap::NoWrap, align: TextAlign::Center)
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: props.style.units_color)
                Box(justify_content: JustifyContent::End, height: 1) {
                    Text(content: inop_text(props.inop), color: Color::DarkRed)
                }
            }
        }
    }
}

struct BarWidths {
    left_padding: u32,
    main: u32,
    right_padding: u32,
    justification: JustifyContent,
}

impl Default for BarWidths {
    fn default() -> Self {
        Self {
            left_padding: 0,
            main: 0,
            right_padding: 0,
            justification: JustifyContent::Start,
        }
    }
}

struct BarSpec {
    /// Total real width of the bar widget
    widget_size: u32,
    scale: u32,
}

fn bar_widths(spec: BarSpec, value: Option<f32>) -> BarWidths {
    let BarSpec { widget_size, scale } = spec;
    let total_width = widget_size;
    let half_width = (total_width as f32) / 2.0;

    match value {
        Some(v) => {
            let rescaled_proportion = (v.abs() / scale as f32) / 2.0;
            let bar = rescaled_proportion * (total_width as f32);
            let main = bar as u32;

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

            BarWidths {
                left_padding,
                main,
                right_padding,
                justification,
            }
        }
        None => BarWidths::default(),
    }
}

fn inop_text(inop: bool) -> String {
    if inop {
        "INOP".into()
    } else {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 21 characters wide bar
    /// -12 to +12 knot scale
    const SPEC: BarSpec = BarSpec {
        widget_size: 20,
        scale: 12,
    };

    #[test]
    fn should_have_proportional_size_with_positive_value() {
        let result = bar_widths(SPEC, Some(6.0));
        assert_eq!(result.main, 5);
    }

    #[test]
    fn should_have_justification_start_with_positive_value() {
        let result = bar_widths(SPEC, Some(6.0));
        assert_eq!(result.justification, JustifyContent::Start);
    }

    #[test]
    fn should_have_half_left_padding_with_positive_value() {
        let result = bar_widths(SPEC, Some(6.0));
        assert_eq!(result.left_padding, 10)
    }

    #[test]
    fn should_have_no_right_padding_with_positive_value() {
        let result = bar_widths(SPEC, Some(6.0));
        assert_eq!(result.right_padding, 0);
    }

    #[test]
    fn should_have_proportional_width_with_negative_value() {
        let result = bar_widths(SPEC, Some(-4.0));
        assert_eq!(result.main, 3.3 as u32);
    }

    #[test]
    fn should_have_justification_end_with_negative_value() {
        let result = bar_widths(SPEC, Some(-4.0));
        assert_eq!(result.justification, JustifyContent::End);
    }

    #[test]
    fn should_have_no_left_padding_with_negative_value() {
        let result = bar_widths(SPEC, Some(-4.0));
        assert_eq!(result.left_padding, 0);
    }

    #[test]
    fn should_have_half_right_padding_with_negative_value() {
        let result = bar_widths(SPEC, Some(-4.0));
        assert_eq!(result.right_padding, 10)
    }

    #[test]
    fn should_have_zero_width_with_zero_value() {
        let result = bar_widths(SPEC, Some(0.0));
        assert_eq!(result.main, 0);
    }

    #[test]
    fn should_have_justification_start_with_zero_value() {
        let result = bar_widths(SPEC, Some(0.0));
        assert_eq!(result.justification, JustifyContent::Start);
    }

    #[test]
    fn should_have_half_left_padding_with_zero_value() {
        let result = bar_widths(SPEC, Some(0.0));
        assert_eq!(result.left_padding, 10)
    }

    #[test]
    fn should_have_no_right_padding_with_zero_value() {
        let result = bar_widths(SPEC, Some(0.0));
        assert_eq!(result.right_padding, 0);
    }
}
