use crate::components::instruments::shared::label::Label;
use crate::components::instruments::shared::string::s;
use crate::components::instruments::shared::style::{
    BOX_WIDTH, INSTRUMENT_BORDER_COLOR, INSTRUMENT_BORDER_STYLE,
};
use iocraft::prelude::*;

#[derive(Props, Default)]
pub struct AltimeterProps {
    pub value: Option<State<u32>>,
    pub units: String,
    pub inop: bool,
}

#[component]
pub fn Altimeter(props: &AltimeterProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box(flex_direction: FlexDirection::Column) {
            Label(content: "Altimeter")
            Box(border_color: INSTRUMENT_BORDER_COLOR, border_style: INSTRUMENT_BORDER_STYLE, flex_direction: FlexDirection::Column, width: BOX_WIDTH, padding_top: 1, padding_left: 1, padding_right: 1) {
                Box(background_color: Color::Grey, height: 1, justify_content: JustifyContent::End) {
                    Text(content: s(props.value), wrap: TextWrap::NoWrap, align: TextAlign::Right, color: Color::Black)
                    Box(width: 1, border_style: BorderStyle::None) {}
                }
                Text(content: format!("{}", props.units), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::DarkGrey)
                Box(height: 1)
                Box(justify_content: JustifyContent::End, height: 1) {
                    Text(content: inop_text(props.inop), color: Color::DarkRed)
                }
            }
        }
    }
}

fn inop_text(inop: bool) -> String {
    if inop {
        "INOP".into()
    } else {
        "".into()
    }
}
