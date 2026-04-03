use crate::re::{NiPoint2, PlayerControlsData, ThumbstickEvent};

use super::access::player_controls;

pub const DEFAULT_DIRECTION_THRESHOLD: f32 = 0.25;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl InputDirection {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
            Self::UpLeft => "up_left",
            Self::UpRight => "up_right",
            Self::DownLeft => "down_left",
            Self::DownRight => "down_right",
        }
    }

    #[inline(always)]
    pub const fn is_cardinal(self) -> bool {
        matches!(self, Self::Up | Self::Down | Self::Left | Self::Right)
    }

    #[inline(always)]
    pub const fn is_diagonal(self) -> bool {
        !self.is_cardinal()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalogInputSnapshot {
    current: NiPoint2,
    previous: NiPoint2,
}

impl AnalogInputSnapshot {
    #[inline(always)]
    pub const fn new(current: NiPoint2, previous: NiPoint2) -> Self {
        Self { current, previous }
    }

    #[inline(always)]
    pub const fn from_current(current: NiPoint2) -> Self {
        Self::new(current, NiPoint2::new(0.0, 0.0))
    }

    #[inline(always)]
    pub const fn current(self) -> NiPoint2 {
        self.current
    }

    #[inline(always)]
    pub const fn previous(self) -> NiPoint2 {
        self.previous
    }

    #[inline(always)]
    pub fn delta(self) -> NiPoint2 {
        self.current - self.previous
    }

    #[inline(always)]
    pub fn magnitude(self) -> f32 {
        self.current.length()
    }

    #[inline(always)]
    pub fn previous_magnitude(self) -> f32 {
        self.previous.length()
    }

    #[inline(always)]
    pub fn has_input(self, threshold: f32) -> bool {
        direction_from_vector(self.current, threshold).is_some()
    }

    #[inline(always)]
    pub fn previous_has_input(self, threshold: f32) -> bool {
        direction_from_vector(self.previous, threshold).is_some()
    }

    #[inline(always)]
    pub fn direction(self, threshold: f32) -> Option<InputDirection> {
        direction_from_vector(self.current, threshold)
    }

    #[inline(always)]
    pub fn previous_direction(self, threshold: f32) -> Option<InputDirection> {
        direction_from_vector(self.previous, threshold)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalInputSnapshot {
    movement: AnalogInputSnapshot,
    look: AnalogInputSnapshot,
    auto_move: bool,
    running: bool,
    remap_mode: bool,
}

impl DirectionalInputSnapshot {
    #[inline(always)]
    pub const fn new(
        movement: AnalogInputSnapshot,
        look: AnalogInputSnapshot,
        auto_move: bool,
        running: bool,
        remap_mode: bool,
    ) -> Self {
        Self {
            movement,
            look,
            auto_move,
            running,
            remap_mode,
        }
    }

    #[inline(always)]
    pub const fn from_data(data: PlayerControlsData) -> Self {
        Self::new(
            AnalogInputSnapshot::new(data.move_input_vec, data.prev_move_vec),
            AnalogInputSnapshot::new(data.look_input_vec, data.prev_look_vec),
            data.auto_move,
            data.running,
            data.remap_mode,
        )
    }

    #[inline(always)]
    pub const fn movement(self) -> AnalogInputSnapshot {
        self.movement
    }

    #[inline(always)]
    pub const fn look(self) -> AnalogInputSnapshot {
        self.look
    }

    #[inline(always)]
    pub const fn auto_move(self) -> bool {
        self.auto_move
    }

    #[inline(always)]
    pub const fn running(self) -> bool {
        self.running
    }

    #[inline(always)]
    pub const fn remap_mode(self) -> bool {
        self.remap_mode
    }
}

#[inline(always)]
const fn sanitize_direction_threshold(threshold: f32) -> f32 {
    if threshold.is_finite() && threshold > 0.0 {
        threshold
    } else {
        DEFAULT_DIRECTION_THRESHOLD
    }
}

#[inline(always)]
pub fn direction_from_vector(vector: NiPoint2, threshold: f32) -> Option<InputDirection> {
    let threshold = sanitize_direction_threshold(threshold);
    let horizontal = if vector.x >= threshold {
        1
    } else if vector.x <= -threshold {
        -1
    } else {
        0
    };
    let vertical = if vector.y >= threshold {
        1
    } else if vector.y <= -threshold {
        -1
    } else {
        0
    };

    match (horizontal, vertical) {
        (0, 0) => None,
        (0, 1) => Some(InputDirection::Up),
        (0, -1) => Some(InputDirection::Down),
        (-1, 0) => Some(InputDirection::Left),
        (1, 0) => Some(InputDirection::Right),
        (-1, 1) => Some(InputDirection::UpLeft),
        (1, 1) => Some(InputDirection::UpRight),
        (-1, -1) => Some(InputDirection::DownLeft),
        (1, -1) => Some(InputDirection::DownRight),
        _ => None,
    }
}

#[inline(always)]
pub fn thumbstick_input_snapshot(event: &ThumbstickEvent) -> AnalogInputSnapshot {
    AnalogInputSnapshot::from_current(NiPoint2::new(event.x_value, event.y_value))
}

#[inline(always)]
pub fn thumbstick_direction(event: &ThumbstickEvent, threshold: f32) -> Option<InputDirection> {
    thumbstick_input_snapshot(event).direction(threshold)
}

#[inline(always)]
pub fn directional_snapshot() -> DirectionalInputSnapshot {
    DirectionalInputSnapshot::from_data(player_controls().data)
}

#[inline(always)]
pub fn movement_input_snapshot() -> AnalogInputSnapshot {
    directional_snapshot().movement()
}

#[inline(always)]
pub fn look_input_snapshot() -> AnalogInputSnapshot {
    directional_snapshot().look()
}

#[cfg(test)]
mod tests {
    use crate::re::{NiPoint2, PlayerControlsData};

    use super::{
        AnalogInputSnapshot, DirectionalInputSnapshot, InputDirection, direction_from_vector,
    };

    #[test]
    fn direction_from_vector_respects_dead_zone_and_diagonals() {
        assert_eq!(direction_from_vector(NiPoint2::new(0.0, 0.0), 0.25), None);
        assert_eq!(
            direction_from_vector(NiPoint2::new(0.9, 0.0), 0.25),
            Some(InputDirection::Right)
        );
        assert_eq!(
            direction_from_vector(NiPoint2::new(-0.8, 0.0), 0.25),
            Some(InputDirection::Left)
        );
        assert_eq!(
            direction_from_vector(NiPoint2::new(0.4, 0.9), 0.25),
            Some(InputDirection::UpRight)
        );
        assert_eq!(
            direction_from_vector(NiPoint2::new(-0.6, -0.7), 0.25),
            Some(InputDirection::DownLeft)
        );
    }

    #[test]
    fn analog_snapshot_reports_delta_magnitude_and_direction() {
        let snapshot = AnalogInputSnapshot::new(NiPoint2::new(0.8, 0.1), NiPoint2::new(0.2, 0.1));

        assert!(snapshot.has_input(0.25));
        assert_eq!(snapshot.direction(0.25), Some(InputDirection::Right));
        assert_eq!(snapshot.delta(), NiPoint2::new(0.6, 0.0));
        assert!(snapshot.magnitude() > snapshot.previous_magnitude());
    }

    #[test]
    fn directional_snapshot_tracks_move_look_and_modes() {
        let snapshot = DirectionalInputSnapshot::from_data(PlayerControlsData {
            move_input_vec: NiPoint2::new(0.0, 1.0),
            look_input_vec: NiPoint2::new(-1.0, 0.0),
            prev_move_vec: NiPoint2::new(0.0, 0.5),
            prev_look_vec: NiPoint2::new(-0.5, 0.0),
            auto_move: true,
            running: true,
            remap_mode: false,
            ..Default::default()
        });

        assert_eq!(
            snapshot.movement().direction(0.25),
            Some(InputDirection::Up)
        );
        assert_eq!(snapshot.look().direction(0.25), Some(InputDirection::Left));
        assert!(snapshot.auto_move());
        assert!(snapshot.running());
        assert!(!snapshot.remap_mode());
    }
}
