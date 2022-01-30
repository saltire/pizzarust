use bevy::{
    math::Vec4Swizzles,
    prelude::*,
    window::CursorMoved,
};

use super::MainCamera;
use super::constants::*;
use super::scene::ToppingClickEvent;


pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_startup_system(create_cursor)
            .add_system(move_cursor)
            .add_system(on_click_topping);
    }
}

#[derive(Component)]
struct Cursor {
    pub default_image: Handle<Image>,
    pub topping_image: Handle<Image>,
}

fn create_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let default_image = asset_server.load("cursor-hand.png");
    let topping_image = asset_server.load("cursor-circle.png");

    // Put cursor inside a blank parent node as this bumps its z-index above the black bars.
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Percent(50.),
                            bottom: Val::Percent(50.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    image: default_image.clone().into(),
                    ..Default::default()
                })
                .insert(Cursor {
                    default_image,
                    topping_image,
                });
        });
}

fn move_cursor(
    mut cursors: Query<&mut Style, With<Cursor>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in cursor_moved_events.iter() {
        for mut style in cursors.iter_mut() {
            style.position = Rect {
                left: Val::Px(event.position.x.floor() - CURSOR_HOTSPOT_X),
                bottom: Val::Px(event.position.y.ceil() - CURSOR_HOTSPOT_Y - 1.),
                ..Default::default()
            };
        }
    }
}

fn on_click_topping(
    mut commands: Commands,
    mut cursors: Query<(Entity, &Cursor)>,
    mut topping_events: EventReader<ToppingClickEvent>,
) {
    if let Ok((entity, cursor)) = cursors.get_single_mut() {
        let mut entity_commands = commands.entity(entity);

        for event in topping_events.iter() {
            entity_commands
                .remove::<UiImage>()
                .remove::<UiColor>();

            if let ToppingClickEvent(Some(topping)) = event {
                entity_commands
                    .insert(UiImage(cursor.topping_image.clone().into()))
                    .insert(UiColor(topping.color.clone()));
            } else {
                entity_commands
                    .insert(UiImage(cursor.default_image.clone().into()))
                    .insert(UiColor(Color::WHITE));
            }
        }
    }
}

pub fn get_world_coords(
    windows: Res<Windows>,
    camera_transforms: Query<&Transform, With<MainCamera>>,
) -> Option<Vec2> {
    let window = windows.get_primary().expect("Window not found.");
    let camera_transform = camera_transforms.get_single().expect("Camera not found.");
    if let Some(position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let world_position = camera_transform.compute_matrix()
            * (position - size / 2.0).extend(0.).extend(1.);
        return Some(world_position.xy());
    }
    None
}
