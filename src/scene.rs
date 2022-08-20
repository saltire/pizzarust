use bevy::{
    input::{
        ButtonState,
        mouse::{MouseButton, MouseButtonInput},
    },
    math::Vec3Swizzles,
    prelude::*,
    sprite::Sprite,
};
use rand::{thread_rng, Rng};
use std::f32::consts::TAU;

use super::MainCamera;
use super::cursor;
use super::display::Display;


pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HeldTopping(None))
            .add_event::<ToppingClickEvent>()
            .add_startup_system(scene)
            .add_system(move_pizzas)
            .add_system(click_toppings);
    }
}

const CONVEYOR_Y: f32 = -225.;
const CONVEYOR_Z: f32 = 1.;
const CONVEYOR_SPEED: f32 = 20.;
const PIZZA_Z: f32 = 2.;
const PIZZA_SPAWN_MARGIN: f32 = 40.;
const TOPPING_Z: f32 = 3.;

#[derive(Component)]
struct Conveyor {
    timer: Timer,
}

#[derive(Component, Debug, Default)]
struct Pizza {
    toppings: Vec<Topping>,
}

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

#[derive(Component)]
struct HeldTopping(Option<Topping>);

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
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.2, 0.3),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(960., 90., 1.),
                translation: Vec3::new(0., CONVEYOR_Y, CONVEYOR_Z),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Conveyor {
            timer: Timer::from_seconds(1. / CONVEYOR_SPEED, true),
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

fn move_pizzas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display: Res<Display>,
    time: Res<Time>,
    mut pizzas: Query<(Entity, &mut Transform), With<Pizza>>,
    mut conveyors: Query<&mut Conveyor>,
) {
    let spawn_x = display.camera_x - display.width / 2. - PIZZA_SPAWN_MARGIN;
    let despawn_x = display.camera_x + display.width / 2. + PIZZA_SPAWN_MARGIN;

    if let Ok(mut conveyor) = conveyors.get_single_mut() {
        if conveyor.timer.tick(time.delta()).just_finished() {
            for (entity, mut transform) in pizzas.iter_mut() {
                // Move pizzas.
                transform.translation.x += conveyor.timer.times_finished_this_tick() as f32;

                // Remove any pizzas that have moved past the right edge.
                if transform.translation.x >= despawn_x {
                    commands.entity(entity).despawn_recursive();
                }
            }

            // If there is enough free space to the left, create a new pizza off screen.
            let min_clearance_x = spawn_x + 160.;
            let leftmost_pizza_x = pizzas.iter().map(|(_e, t)| t.translation.x).reduce(f32::min);
            if leftmost_pizza_x.unwrap_or(f32::MAX) > min_clearance_x {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load("pizzacircle.png"),
                        transform: Transform::from_translation(
                            Vec3::new(spawn_x, CONVEYOR_Y, PIZZA_Z)),
                        ..Default::default()
                    })
                    .insert(Pizza::default());
            }
        }
    }
}

fn click_toppings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    held: Res<HeldTopping>,
    windows: Res<Windows>,
    cameras: Query<&Transform, With<MainCamera>>,
    mut click_events: EventReader<MouseButtonInput>,
    mut topping_events: EventWriter<ToppingClickEvent>,
    containers: Query<(&Container, &Transform)>,
    mut pizzas: Query<(Entity, &mut Pizza, &Transform)>,
) {
    if let Some(position) = cursor::get_world_coords(windows, cameras) {
        if click_events.iter().any(|e| {
            e.button == MouseButton::Left && e.state == ButtonState::Released
        }) {
            // If click was on a topping container, change the cursor to that topping.
            if let Some(container) = containers.iter().find_map(|(c, t)| {
                let diff = (t.translation.xy() - position).abs();
                if diff.x < 20. && diff.y < 20. { Some(c) } else { None }
            }) {
                topping_events.send(ToppingClickEvent(Some(container.topping.clone())));
                commands.insert_resource(HeldTopping(Some(container.topping.clone())));
            }

            else if let HeldTopping(Some(held_topping)) = held.into_inner() {
                if let Some((entity, mut pizza)) = pizzas.iter_mut().find_map(|(e, p, t)| {
                    let diff = t.translation.xy() - position;
                    if diff.length() < 40. { Some((e, p)) } else { None }
                }) {
                    pizza.toppings.push(held_topping.clone());
                    let topping_z = TOPPING_Z + pizza.toppings.len() as f32;

                    let mut rng = thread_rng();

                    commands.entity(entity).with_children(|parent| {
                        match held_topping.placement {
                            Placement::Cover => {
                                parent.spawn_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        color: held_topping.color,
                                        ..Default::default()
                                    },
                                    transform: Transform {
                                        translation: Vec3::new(0., 0., topping_z),
                                        rotation: Quat::from_rotation_z(
                                            rng.gen_range(0..4) as f32 * 90.),
                                        ..Default::default()
                                    },
                                    texture: asset_server.load("pizzaspread.png"),
                                    ..Default::default()
                                });
                            }
                            Placement::Scatter => {
                                let texture = asset_server.load("circle10.png");
                                let count = rng.gen_range(10..=16);

                                for _ in 0..count {
                                    let angle = rng.gen_range(0.0..TAU);
                                    let r: f32 = rng.gen();
                                    let radius = r.sqrt() * 33.;
                                    // let r2: f32 = rng.gen();
                                    // let u = r + r2;
                                    // let radius = if u > 1. { 2. - u } else { u } * 33.;

                                    parent.spawn_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            color: held_topping.color,
                                            ..Default::default()
                                        },
                                        transform: Transform {
                                            translation: Vec3::new(
                                                (angle.cos() * radius).round(),
                                                (angle.sin() * radius).round(),
                                                topping_z),
                                            ..Default::default()
                                        },
                                        texture: texture.clone(),
                                        ..Default::default()
                                    });
                                }
                            }
                        }
                    });

                    topping_events.send(ToppingClickEvent(None));
                    commands.insert_resource(HeldTopping(None));
                }
            }
        }
    }
}
