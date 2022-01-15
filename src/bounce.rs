use bevy::prelude::*;
use std::f32::consts::TAU;


pub struct BouncePlugin;

impl Plugin for BouncePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bounce);
    }
}

pub enum EffectType {
    Bounce,
    HorizontalWave,
}
impl Default for EffectType {
    fn default() -> Self { EffectType::Bounce }
}

#[derive(Default)]
pub struct BounceEffect {
    pub effect_type: EffectType,
    pub distance: f32,
    pub period: f64,
    pub current_offset: Vec2,
}

#[derive(Component)]
pub struct Bounce {
    pub effects: Vec<BounceEffect>,
}

fn bounce(
    mut objects: Query<(&mut Bounce, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut bounce, mut transform) in objects.iter_mut() {
        for mut effect in bounce.effects.iter_mut() {
            let phase = (time.seconds_since_startup() / effect.period) as f32 % 1.;

            let offset = match effect.effect_type {
                EffectType::Bounce => Vec2::new(0., (phase * TAU / 2.).sin().abs() * effect.distance),
                EffectType::HorizontalWave => Vec2::new((phase * TAU).sin() * effect.distance, 0.),
            };

            transform.translation = Vec3::new(
                transform.translation.x - effect.current_offset.x + offset.x,
                transform.translation.y - effect.current_offset.y + offset.y,
                transform.translation.z);

            effect.current_offset = offset;
        }
    }
}
