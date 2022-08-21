use bevy::prelude::*;
use bevy::window::WindowResized;

use super::MainCamera;


#[derive(Clone, Copy, Debug)]
pub struct Display {
    pub width: f32,
    pub height: f32,
    pub camera_x: f32,
    pub camera_y: f32,
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

enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component)]
struct BlackBar(Edge);

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_startup_system(init_display)
            .add_system(switch_resolution_on_space)
            .add_system(handle_resize);
    }
}

fn init_display(
    mut commands: Commands,
) {
    commands.insert_resource(DISPLAYS[0]);

    spawn_black_bar(&mut commands, Edge::Top);
    spawn_black_bar(&mut commands, Edge::Bottom);
    spawn_black_bar(&mut commands, Edge::Left);
    spawn_black_bar(&mut commands, Edge::Right);
}

fn switch_resolution_on_space(
    mut commands: Commands,
    mut cursor_events: EventWriter<CursorMoved>,
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    display: Res<Display>,
    mut black_bars: Query<(&BlackBar, &mut Style)>,
) {
    if keys.just_released(KeyCode::Space) {
        let window = windows.get_primary_mut().expect("Window not found.");
        let mut camera = cameras.get_single_mut().expect("Camera not found.");
        let index = match DISPLAYS.iter()
            .position(|d| d.width == display.width && d.height == display.height) {
            Some(i) => (i + 1) % DISPLAYS.len(),
            None => 0,
        };

        size_window(window, &mut camera, &DISPLAYS[index], &mut black_bars);
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
    mut windows: ResMut<Windows>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    mut window_resized_events: EventReader<WindowResized>,
    mut black_bars: Query<(&BlackBar, &mut Style)>,
    display: Res<Display>,
) {
    let window = windows.get_primary_mut().expect("Window not found.");
    let mut camera = cameras.get_single_mut().expect("Camera not found.");
    if window_resized_events.iter().any(|event| event.id == window.id()) {
        size_window(window, &mut camera, &display, &mut black_bars);
    }
}

fn size_window(
    window: &mut Window,
    camera_transform: &mut Transform,
    display: &Display,
    black_bars: &mut Query<(&BlackBar, &mut Style)>,
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

    for (BlackBar(edge), mut style) in black_bars.iter_mut() {
        style.size = match edge {
            Edge::Top => Size::new(Val::Percent(100.), Val::Px(bar_height)),
            Edge::Bottom => Size::new(Val::Percent(100.), Val::Px(bar_height)),
            Edge::Left => Size::new(Val::Px(bar_width), Val::Percent(100.)),
            Edge::Right => Size::new(Val::Px(bar_width), Val::Percent(100.)),
        };
        style.position = match edge {
            Edge::Top => UiRect { top: Val::Px(0.), ..Default::default() },
            Edge::Bottom => UiRect { bottom: Val::Px(0.), ..Default::default() },
            Edge::Left => UiRect { left: Val::Px(0.), ..Default::default() },
            Edge::Right => UiRect { right: Val::Px(0.), ..Default::default() },
        };
    }
}

fn spawn_black_bar(
    commands: &mut Commands,
    edge: Edge,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(BlackBar(edge));
}
