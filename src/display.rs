use bevy::prelude::*;
use bevy::window::WindowResized;

use super::constants::*;


pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, init_display)
            .add_system(handle_resize);
    }
}

fn init_display(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_resolution(PIXEL_WIDTH * 2., PIXEL_HEIGHT * 2.);
        size_window(&mut commands, window);
    }
}

fn handle_resize(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut window_resized_events: EventReader<WindowResized>,
    mut black_bars: Query<Entity, With<BlackBar>>,
) {
    if let Some(window) = windows.get_primary_mut() {
        if window_resized_events.iter().any(|event| event.id == window.id()) {
            for black_bar in black_bars.iter_mut() {
                commands.entity(black_bar).despawn();
            }
            size_window(&mut commands, window);
        }
    }
}

#[derive(Component)]
struct BlackBar;

fn size_window(
    commands: &mut Commands,
    window: &mut Window,
) {
    let scale_x = window.physical_width() as f32 / PIXEL_WIDTH;
    let scale_y = window.physical_height() as f32 / PIXEL_HEIGHT;
    let scale = scale_x.min(scale_y).floor();

    window.set_scale_factor_override(Some(scale.into()));
    window.set_resolution(
        window.physical_width() as f32 / scale, window.physical_height() as f32 / scale);

    let bar_width = (window.width() - PIXEL_WIDTH) / 2.;
    let bar_height = (window.height() - PIXEL_HEIGHT) / 2.;

    spawn_black_bar(
        commands,
        Size::new(Val::Percent(100.), Val::Px(bar_height)),
        Rect { top: Val::Px(0.), ..Default::default() },
    );

    spawn_black_bar(
        commands,
        Size::new(Val::Percent(100.), Val::Px(bar_height)),
        Rect { bottom: Val::Px(0.), ..Default::default() },
    );

    spawn_black_bar(
        commands,
        Size::new(Val::Px(bar_width), Val::Percent(100.)),
        Rect { left: Val::Px(0.), ..Default::default() },
    );

    spawn_black_bar(
        commands,
        Size::new(Val::Px(bar_width), Val::Percent(100.)),
        Rect { right: Val::Px(0.), ..Default::default() },
    );
}

fn spawn_black_bar(
    commands: &mut Commands,
    size: Size<Val>,
    position: Rect<Val>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size,
                position_type: PositionType::Absolute,
                position,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(BlackBar);
}
