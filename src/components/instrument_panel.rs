use std::time::Duration;

use crate::components::instruments::accelerometer;
use crate::components::instruments::airspeed;
use crate::components::instruments::altimeter::Altimeter;
use crate::components::instruments::clock::Clock;
use crate::components::instruments::compass::Compass;
use crate::components::instruments::slip::TurnSlipIndicator;
use crate::components::instruments::vario;
use crate::data::client::{to_time, Datagram};
use chrono::NaiveTime;
use iocraft::prelude::*;
use smol::Timer;

#[derive(Props)]
pub struct InstrumentPanelProps {
    pub host: String,
    pub port: String,
}

impl Default for InstrumentPanelProps {
    fn default() -> Self {
        Self {
            host: String::default(),
            port: String::default(),
        }
    }
}

#[component]
pub fn InstrumentPanel(
    mut hooks: Hooks,
    props: &InstrumentPanelProps,
) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);

    let mut airspeed = hooks.use_state(|| 0);
    let mut altitude = hooks.use_state(|| 0);
    let mut compass = hooks.use_state(|| 0);
    let mut g_force = hooks.use_state(|| 1.0);
    let mut slip = hooks.use_state(|| 0.0);
    let mut vario = hooks.use_state(|| 0.0);
    let mut time: State<NaiveTime> = hooks.use_state(|| NaiveTime::MIN);

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

    // Read from Condor data source
    hooks.use_future(async move {
        loop {
            Timer::after(Duration::from_millis(500)).await;
            // TODO only update fields that have changes
            let d = Datagram::random();
            airspeed.set(d.airspeed as u32);
            altitude.set(d.altitude as u32);
            compass.set(d.compass as u32);
            g_force.set(d.gforce);
            slip.set(d.slipball);
            vario.set(d.vario);
            time.set(to_time(d.time));
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
                    vario::Vario(value: vario, units: "kt", scale: 12 as u32, inop: true)
                    airspeed::AirSpeedIndicator(value: airspeed, units: "kt", scale: airspeed::Scale::create(84, 119, 125).unwrap())
                    Altimeter(value: altitude, units: "ft")
                }
                Box(gap: Gap::Length(2)) {
                    Compass(value: compass, units: "deg")
                    TurnSlipIndicator(value: slip)
                    accelerometer::Accelerometer(value: g_force, units: "G", scale: accelerometer::Scale::create(-4.0, 1.0, 6.5).unwrap(), inop: true)
                }
                Box(gap: Gap::Length(2), justify_content: JustifyContent::Center) {
                    Clock(value: time, timezone: "UTC")
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
