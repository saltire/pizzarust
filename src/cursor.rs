use bevy::{
    prelude::*,
    window::CursorMoved,
};

use super::constants::*;
use super::Display;


pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build (&self, app: &mut AppBuilder) {
        app
            .add_startup_system(create_cursor.system())
            .add_system(move_cursor.system());
    }
}

#[derive(Debug)]
struct Cursor;

fn create_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cursor_handle = asset_server.load("hand-cursor.png");

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(cursor_handle.into()),
            ..Default::default()
        })
        .insert(Cursor);
}

fn move_cursor(
    mut cursors: Query<(&Cursor, &mut Transform)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    displays: Query<&Display>,
) {
    for display in displays.iter() {
        for event in cursor_moved_events.iter() {
            for (_cursor, mut transform) in cursors.iter_mut() {
                transform.translation = Vec3::new(
                    ((event.position.x - display.offset.x) / display.scale)
                        .floor()
                        - PIXEL_WIDTH / 2.
                        + CURSOR_WIDTH / 2.
                        - CURSOR_HOTSPOT_X,
                    ((event.position.y - display.offset.y) / display.scale)
                        .floor()
                        - PIXEL_HEIGHT / 2.
                        - CURSOR_HEIGHT / 2.
                        + CURSOR_HOTSPOT_Y
                        + 1.,
                    10.);
            }
        }
    }
}
