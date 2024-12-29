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
    let widths = bar_widths(props.scale, props.value);

    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Vario")
            Box(border_color: props.style.border_color, border_style: props.style.border_style, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding_top: 1, padding_left: 1, padding_right: 1) {
                Box(background_color: Color::Grey, height: 1) {
                    Box(width: widths.left_padding)
                    Box(width: widths.main, background_color: Color::DarkGrey)
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

#[derive(Default)]
struct BarWidths {
    left_padding: u32,
    main: u32,
}

fn bar_widths(scale: u32, value: Option<State<f32>>) -> BarWidths {
    // TODO remove the constant reference
    let total_width = BOX_WIDTH - 2;
    let half_width = (total_width as f32) / 2.0;

    match value.map(|x| x.get()) {
        Some(v) => {
            let rescaled_proportion = (v.abs() / scale as f32) / 2.0;
            let bar = rescaled_proportion * (total_width as f32);
            let main = bar as u32;

            // TODO change this
            let left_padding = half_width as u32;

            BarWidths { left_padding, main }
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
