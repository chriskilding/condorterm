use iocraft::prelude::*;
use crate::components::instruments::clock::Clock;
use crate::components::instruments::compass::Compass;
use crate::components::instruments::vario::Vario;
use crate::components::instruments::accelerometer::Accelerometer;
use crate::components::instruments::altimeter::Altimeter;
use crate::components::instruments::airspeed::AirSpeedIndicator;
use crate::components::instruments::slip::TurnSlipIndicator;


#[derive(Default, Props)]
pub struct InstrumentPanelProps {
    pub host: String,
    pub port: String
}


#[component]
pub fn InstrumentPanel(mut hooks: Hooks, props: &InstrumentPanelProps) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char('q') => should_exit.set(true),
                    _ => {}
                }
            }
            _ => {}
        }
    });

    if should_exit.get() {
        system.exit();
    }

    element! {
        Box(flex_direction: FlexDirection::Column) {
            Box(background_color: Color::DarkGrey, flex_direction: FlexDirection::Column) {
                Box() {
                    Box(width: 1)
                    Text(content: "condorterm", color: Color::White, wrap: TextWrap::NoWrap, weight: Weight::Bold, align: TextAlign::Center)
                }
                Box() {
                    Box(width: 1)
                    Box() {
                        Text(content: "Host", color: Color::White, wrap: TextWrap::NoWrap, weight: Weight::Bold)
                        Box(width: 1)
                        Text(content: format!("{}", props.host), wrap: TextWrap::NoWrap, color: Color::White)
                        Box(width: 1)
                        Text(content: "Port", color: Color::White, wrap: TextWrap::NoWrap, weight: Weight::Bold)
                        Box(width: 1)
                        Text(content: format!("{}", props.port), wrap: TextWrap::NoWrap, color: Color::White)
                    }
                }
            }
            Box(flex_direction: FlexDirection::Column, gap: Gap::Length(1), padding: 1) {
                Box(gap: Gap::Length(2)) {
                    Vario(value: 4.0, units: "kt")
                    AirSpeedIndicator(value: 101, units: "kt")
                    Altimeter(value: 2641, units: "ft")
                }
                Box(gap: Gap::Length(2)) {
                    Compass(value: 241, units: "deg")
                    TurnSlipIndicator(value: 0.0)
                    Accelerometer(value: 1.5, units: "G")
                }
                Box(gap: Gap::Length(2), justify_content: JustifyContent::Center) {
                    Clock(timezone: "UTC")
                }
            }
            Box(background_color: Color::DarkGrey) {
                Box(width: 1)
                Box() {
                    Text(content: "Press [", wrap: TextWrap::NoWrap, color: Color::White)
                    Text(content: "Q", color: Color::White, wrap: TextWrap::NoWrap, weight: Weight::Bold)
                    Text(content: "] to quit", color: Color::White, wrap: TextWrap::NoWrap)
                }
            }
        }
    }
}