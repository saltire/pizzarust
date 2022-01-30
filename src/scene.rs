use bevy::{
    input::mouse::{MouseButton, MouseButtonInput},
    math::Vec3Swizzles,
    prelude::*,
};

use super::MainCamera;
use super::cursor;


pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ToppingClickEvent>()
            .add_startup_system(scene)
            .add_system(click_toppings);
    }
}

#[derive(Component, Debug)]
struct Pizza;

#[derive(Clone, Debug)]
pub enum Placement {
    Cover,
    Scatter,
}

#[derive(Component, Clone, Debug)]
pub struct Topping {
    pub name: String,
    pub color: Color,
    pub placement: Placement,
}

#[derive(Component, Debug)]
struct Container {
    topping: Topping,
}

#[derive(Debug)]
pub struct ToppingClickEvent(pub Option<Topping>);

fn scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Background
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("grid-960x540.png"),
        ..Default::default()
    });

    // Conveyor belt
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.1, 0.2, 0.3),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(960., 90., 1.),
            translation: Vec3::new(0., -225., 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    let toppings = vec![
        Topping {
            name: "Tomato Sauce".into(),
            color: Color::rgb(0.5, 0.2, 0.1),
            placement: Placement::Cover,
        },
        Topping {
            name: "Mozzarella".into(),
            color: Color::rgb(0.9, 0.9, 0.7),
            placement: Placement::Cover,
        },
        Topping {
            name: "Pepperoni".into(),
            color: Color::rgb(0.6, 0.2, 0.1),
            placement: Placement::Scatter,
        },
        Topping {
            name: "Green Peppers".into(),
            color: Color::rgb(0.4, 0.5, 0.1),
            placement: Placement::Scatter,
        },
    ];

    for (x, topping) in toppings.iter().enumerate() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: topping.color,
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::new(40., 40., 1.),
                    translation: Vec3::new(-60. + (x as f32 * 50.), -140., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Container {
                topping: (*topping).clone(),
            });
    }
}

fn click_toppings(
    windows: Res<Windows>,
    cameras: Query<&Transform, With<MainCamera>>,
    mut click_events: EventReader<MouseButtonInput>,
    mut topping_events: EventWriter<ToppingClickEvent>,
    containers: Query<(&Container, &Transform)>,
) {
    if let Some(position) = cursor::get_world_coords(windows, cameras) {
        if click_events.iter().any(|e| e.button == MouseButton::Left) {
            if let Some(container) = containers.iter().find_map(|(c, t)| {
                let diff = t.translation.xy() - position;
                if diff.x.abs() < 20. && diff.y.abs() < 20. { Some(c) } else { None }
            }) {
                topping_events.send(ToppingClickEvent(Some(container.topping.clone())));
            } else {
                topping_events.send(ToppingClickEvent(None));
            }
        }
    }
}
