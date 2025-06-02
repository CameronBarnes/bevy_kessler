use std::f32::consts::PI;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Orbit>();
    app.register_type::<Drift>();
    app.add_systems(Update, (handle_drift, move_things_with_orbits).chain());
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct Orbit {
    radius: f32,
    degrees: f32,
    speed: f32,
    period: f32,
}

fn period_from_speed_radius(radius: f32, speed: f32) -> f32 {
    let distance = 2. * PI * radius;
    distance / speed
}

impl Orbit {
    pub fn new(radius: f32, degrees: f32, speed: f32) -> Self {
        Self {
            radius,
            degrees,
            speed,
            period: period_from_speed_radius(radius, speed),
        }
    }

    pub fn update_speed(&mut self, delta_speed: f32) {
        self.speed += delta_speed;
        self.update_period();
    }

    pub fn update_radius(&mut self, delta_radius: f32) {
        self.radius += delta_radius;
        self.update_period();
    }

    fn update_period(&mut self) {
        self.period = period_from_speed_radius(self.radius, self.speed);
    }

    fn increment_orbit(&mut self, delta_time: f32) {
        self.degrees += 360. * (delta_time / self.period);
    }

    pub fn to_xy(&self) -> Vec2 {
        Vec2::new(
            self.radius * self.degrees.to_radians().cos(),
            self.radius * self.degrees.to_radians().sin(),
        )
    }
}

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Drift(f32);

fn handle_drift(time: Res<Time>, mut query: Query<(&mut Orbit, &Drift)>) {
    for (mut orbit, drift) in &mut query {
        orbit.update_radius(drift.0 * time.delta_secs());
    }
}

fn move_things_with_orbits(
    time: Res<Time>,
    mut orbit_query: Query<(&mut Transform, &mut Orbit), With<Orbit>>,
) {
    for (mut transform, mut orbit) in &mut orbit_query {
        orbit.increment_orbit(time.delta_secs());
        let pos = orbit.to_xy();
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
