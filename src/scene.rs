use bevy::prelude::*;

use super::constants::*;


pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(black_bars.after("init"))
            .add_startup_system(scene.after("init"));
    }
}

fn scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("grid-480x270.png"),
        ..Default::default()
    });
}

fn black_bars(
    mut commands: Commands,
    displays: Query<&super::Display>,
) {
    for display in displays.iter() {
        let bar_width = (display.window_size.x / display.scale - PIXEL_WIDTH) / 2.;
        let bar_height = (display.window_size.y / display.scale - PIXEL_HEIGHT) / 2.;

        commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(bar_height)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        });

        commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(bar_height)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        });

        commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(bar_width), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        });

        commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(bar_width), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        });
    }
}
