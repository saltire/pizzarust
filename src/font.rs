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

struct BitmapText {
    text: String,
    font: String,
    transform: Transform,
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
        transform: Transform::from_xyz(-220., -115., 2.),
    });
}

fn render_text(
    mut commands: Commands,
    fonts: Query<&BitmapFont>,
    texts: Query<&BitmapText, Added<BitmapText>>,
) {
    for text in texts.iter() {
        for font in fonts.iter() {
            if font.name == text.font {
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: text.transform,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        for (i, b) in text.text.to_uppercase().bytes().enumerate() {
                            parent.spawn_bundle(SpriteSheetBundle {
                                texture_atlas: font.texture_atlas_handle.clone(),
                                sprite: TextureAtlasSprite {
                                    index: b as u32 - 32,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    font.size.x * (i as f32 + 0.5),
                                    font.size.y * 0.5,
                                    0.),
                                ..Default::default()
                            });
                        }
                    });
            }
        }
    }
}
