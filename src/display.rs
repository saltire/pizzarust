use bevy::prelude::*;
use bevy::window::WindowResized;

use super::MainCamera;


#[derive(Clone, Copy, Debug)]
struct Display {
    width: f32,
    height: f32,
    camera_x: f32,
    camera_y: f32,
}

const DISPLAYS: [Display; 5] = [
    Display {
        width: 320.,
        height: 180.,
        camera_x: -40.,
        camera_y: -180.,
    },
    Display {
        width: 384.,
        height: 216.,
        camera_x: -10.,
        camera_y: -162.,
    },
    Display {
        width: 480.,
        height: 270.,
        camera_x: 45.,
        camera_y: -135.,
    },
    Display {
        width: 640.,
        height: 360.,
        camera_x: 20.,
        camera_y: - 90.,
    },
    Display {
        width: 960.,
        height: 540.,
        camera_x: 0.,
        camera_y: 0.,
    },
];

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_startup_system(init_display)
            .add_system(switch_resolution)
            .add_system(handle_resize);
    }
}

fn init_display(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
) {
    let window = windows.get_primary_mut().expect("Window not found.");
    let mut camera = cameras.get_single_mut().expect("Camera not found.");
    size_window(&mut commands, window, &mut camera, &DISPLAYS[0]);
    commands.insert_resource(DISPLAYS[0]);
}

fn switch_resolution(
    mut commands: Commands,
    mut cursor_events: EventWriter<CursorMoved>,
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    display: Res<Display>,
    black_bars: Query<Entity, With<BlackBar>>,
) {
    if keys.just_released(KeyCode::Space) {
        let window = windows.get_primary_mut().expect("Window not found.");
        let mut camera = cameras.get_single_mut().expect("Camera not found.");
        let index = match DISPLAYS.iter()
            .position(|d| d.width == display.width && d.height == display.height) {
            Some(i) => (i + 1) % DISPLAYS.len(),
            None => 0,
        };

        clear_bars(&mut commands, &black_bars);
        size_window(&mut commands, window, &mut camera, &DISPLAYS[index]);
        commands.insert_resource(DISPLAYS[index]);

        if let Some(position) = window.cursor_position() {
            cursor_events.send(CursorMoved {
                id: window.id(),
                position,
            });
        }
    }
}

fn handle_resize(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    mut window_resized_events: EventReader<WindowResized>,
    black_bars: Query<Entity, With<BlackBar>>,
    display: Res<Display>,
) {
    let window = windows.get_primary_mut().expect("Window not found.");
    let mut camera = cameras.get_single_mut().expect("Camera not found.");
    if window_resized_events.iter().any(|event| event.id == window.id()) {
        clear_bars(&mut commands, &black_bars);
        size_window(&mut commands, window, &mut camera, &display);
    }
}

fn clear_bars(
    commands: &mut Commands,
    black_bars: &Query<Entity, With<BlackBar>>,
) {
    for black_bar in black_bars.iter() {
        commands.entity(black_bar).despawn();
    }
}

#[derive(Component)]
struct BlackBar;

fn size_window(
    commands: &mut Commands,
    window: &mut Window,
    camera_transform: &mut Transform,
    display: &Display,
) {
    let max_display = DISPLAYS[4];

    let max_scale_x = window.physical_width() as f32 / max_display.width;
    let max_scale_y = window.physical_height() as f32 / max_display.height;
    let max_scale = max_scale_x.min(max_scale_y).max(1.).floor();

    let pixel_scale = max_scale * max_display.width / display.width;

    window.set_scale_factor_override(Some(pixel_scale.into()));
    window.set_resolution(
        window.physical_width() as f32 / pixel_scale,
        window.physical_height() as f32 / pixel_scale,
    );

    camera_transform.translation = Vec3::new(
        display.camera_x, display.camera_y, camera_transform.translation.z);

    let bar_width = (window.width() - display.width) / 2.;
    let bar_height = (window.height() - display.height) / 2.;

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
