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
            .add_startup_system(create_cursor.system().after("init"))
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

    // Put cursor inside a blank parent node as this bumps its z-index above the black bars.
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    material: materials.add(cursor_handle.into()),
                    ..Default::default()
                })
                .insert(Cursor);
        });
}

fn move_cursor(
    mut cursors: Query<(&Cursor, &mut Style)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    displays: Query<&Display>,
) {
    for display in displays.iter() {
        for event in cursor_moved_events.iter() {
            for (_cursor, mut style) in cursors.iter_mut() {
                style.position = Rect {
                    left: Val::Px((event.position.x / display.scale).floor()
                        - CURSOR_HOTSPOT_X),
                    bottom: Val::Px(((event.position.y - 1.) / display.scale).floor()
                        - CURSOR_HOTSPOT_Y),
                    ..Default::default()
                };
            }
        }
    }
}
