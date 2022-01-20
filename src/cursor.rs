use bevy::{
    prelude::*,
    window::CursorMoved,
};

use super::constants::*;


pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_startup_system(create_cursor)
            .add_system(move_cursor);
    }
}

#[derive(Component)]
struct Cursor;

fn create_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
                    image: asset_server.load("hand-cursor.png").into(),
                    ..Default::default()
                })
                .insert(Cursor);
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
