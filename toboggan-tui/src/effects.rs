use ratatui::layout::Rect;
use ratatui::style::Color;
use tachyonfx::{Effect, EffectManager, Interpolation, Motion, fx};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) enum EffectKey {
    #[default]
    SlideTransition,
    StepReveal,
    DialogOverlay,
    Blink,
    ConnectionPulse,
    StateTransition,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct LayoutAreas {
    pub(crate) title_bar: Rect,
    pub(crate) current_slide: Rect,
    pub(crate) content: Rect,
}

pub(crate) type Effects = EffectManager<EffectKey>;

pub(crate) fn blink_effect() -> Effect {
    fx::sequence(&[
        fx::lighten(Some(0.8), Some(0.8), (100, Interpolation::QuadIn)),
        fx::darken(Some(0.8), Some(0.8), (200, Interpolation::QuadOut)),
    ])
}

pub(crate) fn slide_transition_effect(area: Rect) -> Effect {
    let timer = (500, Interpolation::CubicOut);
    fx::parallel(&[
        fx::coalesce(timer),
        fx::fade_from_fg(Color::DarkGray, timer),
    ])
    .with_area(area)
}

pub(crate) fn step_reveal_effect(area: Rect) -> Effect {
    fx::fade_from_fg(Color::DarkGray, (300, Interpolation::SineOut)).with_area(area)
}

pub(crate) fn dialog_open_effect(area: Rect) -> Effect {
    fx::sweep_in(
        Motion::DownToUp,
        3,
        0,
        Color::DarkGray,
        (300, Interpolation::CubicOut),
    )
    .with_area(area)
}

pub(crate) fn init_to_running_effect(area: Rect) -> Effect {
    fx::coalesce((700, Interpolation::CubicOut)).with_area(area)
}

pub(crate) fn running_to_done_effect() -> Effect {
    fx::parallel(&[
        fx::hsl_shift(
            Some([30.0, 0.2, 0.15]),
            Some([30.0, 0.2, 0.15]),
            (800, Interpolation::BounceOut),
        ),
        fx::ping_pong(fx::lighten(
            Some(0.2),
            Some(0.1),
            (800, Interpolation::SineInOut),
        )),
    ])
}

pub(crate) fn connection_pulse_effect(area: Rect, connected: bool) -> Effect {
    let color = if connected { Color::Green } else { Color::Red };
    fx::ping_pong(fx::fade_from_fg(color, (400, Interpolation::SineInOut))).with_area(area)
}
