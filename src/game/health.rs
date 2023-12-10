use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    pub fn with_current(mut self, current: f32) -> Self {
        self.current = current.min(self.max);
        self
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn missing(&self) -> f32 {
        (self.max - self.current).max(0.0)
    }

    /// Returns how much health was actually lost.
    pub fn lose_health(&mut self, amount: f32) -> f32 {
        let lost = amount.min(self.current);
        self.current -= lost;
        lost
    }
}
