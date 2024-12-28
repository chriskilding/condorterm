use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct LabelProps {
    pub content: String,
}

#[component]
pub fn Label(props: &LabelProps) -> impl Into<AnyElement<'static>> {
    element! {
        Box() {
            Box(width: 2)
            Text(content: format!("{}", props.content), wrap: TextWrap::NoWrap, align: TextAlign::Center, color: Color::Blue)
        }
    }
}
