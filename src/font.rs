use bevy::prelude::*;


pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_system(render_text.system());
    }
}

struct BitmapFont {
    name: String,
    size: Vec2,
    texture_atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
struct BitmapText {
    text: String,
    font: String,
    position: Vec3,
    size: Vec2,
    padding: f32,
    background_color: Color,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let size = Vec2::new(8.0, 8.0);

    let texture_handle = asset_server.load("fonts/geebeeyay-8x8.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, size, 77, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn().insert(BitmapFont {
        name: "GeeBee".to_string(),
        size,
        texture_atlas_handle,
    });

    commands.spawn().insert(BitmapText {
        text: "Pizza!!".to_string(),
        font: "GeeBee".to_string(),
        position: Vec3::new(0., 0., 2.),
        size: Vec2::new(100., 20.),
        padding: 2.,
        background_color: Color::BLACK,
        ..Default::default()
    });
}

fn render_text(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fonts: Query<&BitmapFont>,
    texts: Query<&BitmapText, Added<BitmapText>>,
) {
    for text in texts.iter() {
        for font in fonts.iter() {
            if font.name == text.font {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.add(text.background_color.into()),
                        transform: Transform::from_translation(text.position),
                        sprite: Sprite::new(text.size),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        let offset_x = (font.size.x - text.size.x) / 2. + text.padding;
                        let offset_y = (text.size.y - font.size.y) / 2. - text.padding;

                        for (i, b) in text.text.to_uppercase().bytes().enumerate() {
                            parent.spawn_bundle(SpriteSheetBundle {
                                texture_atlas: font.texture_atlas_handle.clone(),
                                sprite: TextureAtlasSprite {
                                    index: b as u32 - 32,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    offset_x + font.size.x * i as f32,
                                    offset_y,
                                    1.),
                                ..Default::default()
                            });
                        }
                    });
            }
        }
    }
}
