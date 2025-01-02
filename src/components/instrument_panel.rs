use std::sync::Arc;
use std::time::Duration;

use crate::components::instruments::accelerometer;
use crate::components::instruments::airspeed;
use crate::components::instruments::altimeter::Altimeter;
use crate::components::instruments::clock::Clock;
use crate::components::instruments::compass::Compass;
use crate::components::instruments::slip::TurnSlipIndicator;
use crate::components::instruments::vario;
use crate::data::client::Client;
use crate::utils::decimal_hours::into_time;
use chrono::NaiveTime;
use iocraft::prelude::*;

#[component]
pub fn InstrumentPanel(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let client = hooks.use_context::<Arc<Client>>();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);
    let mut airspeed = hooks.use_state(|| 0);
    let mut altitude = hooks.use_state(|| 0);
    let mut compass = hooks.use_state(|| 0);
    let mut g_force = hooks.use_state(|| 1.0);
    let mut slip = hooks.use_state(|| 0.0);
    let mut vario = hooks.use_state(|| 0.0);
    let mut time = hooks.use_state(|| NaiveTime::MIN);

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

    // Read from data source
    hooks.use_future({
        let c = client.clone();

        async move {
            loop {
                smol::Timer::after(Duration::from_millis(500)).await;

                let d = c.receive().await;

                if let Some(air) = d.airspeed {
                    airspeed.set(air as u32);
                }
                if let Some(alt) = d.altitude {
                    altitude.set(alt as u32)
                }
                if let Some(c) = d.compass {
                    compass.set(c as u32);
                }
                if let Some(g) = d.gforce {
                    g_force.set(g);
                }
                if let Some(s) = d.slipball {
                    slip.set(s);
                }
                if let Some(v) = d.vario {
                    vario.set(v);
                }
                if let Some(t) = d.time {
                    time.set(into_time(t));
                }
            }
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
            }
            Box(flex_direction: FlexDirection::Column, gap: Gap::Length(1), padding: 1) {
                Box(gap: Gap::Length(2)) {
                    vario::Vario(value: vario, units: "kt", scale: 12 as u32)
                    airspeed::AirSpeedIndicator(value: airspeed, units: "kt", scale: airspeed::Scale::create(84, 119, 125).unwrap())
                    Altimeter(value: altitude, units: "ft")
                }
                Box(gap: Gap::Length(2)) {
                    Compass(value: compass, units: "deg")
                    TurnSlipIndicator(value: slip)
                    accelerometer::Accelerometer(value: g_force, units: "G", scale: accelerometer::Scale::create(accelerometer::Limits { min: -4.0, max: 6.5 }, 7).unwrap())
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
