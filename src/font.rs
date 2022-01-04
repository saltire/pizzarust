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
        text: "Nemo et voluptas et cumque ipsum cumque inventore. Eveniet soluta odio sint aut asperiores et. Maxime unde cupiditate sunt dolor corporis nihil.".to_string(),
        font: "GeeBee".to_string(),
        position: Vec3::new(-100., 0., 2.),
        size: Vec2::new(160., 200.),
        padding: 4.,
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
                        let maxlen = ((text.size.x - text.padding * 2.) / font.size.x) as usize;

                        let mut lines: Vec<&str> = vec![];
                        if maxlen <= 0 {
                            lines.push(&text.text);
                        } else {
                            let mut linestart = 0;
                            let mut lineend = 0;
                            for (i, char) in text.text.chars().enumerate() {
                                if char == ' ' {
                                    if i - linestart <= maxlen {
                                        // Line not full yet; mark this space and keep going.
                                        lineend = i + 1;
                                    } else if lineend == linestart {
                                        // Line full with a single word; push everything up to here.
                                        lineend = i + 1;
                                        lines.push(&text.text[linestart..lineend - 1]);
                                        linestart = lineend;
                                    } else {
                                        // Line full with more than one word;
                                        // push up to the end of the last one.
                                        lines.push(&text.text[linestart..lineend - 1]);
                                        linestart = lineend;
                                        lineend = i + 1;
                                    }
                                }
                            }
                            if lineend > linestart && text.text.len() - linestart > maxlen {
                                // End of text with more than one word remaining
                                // and longer than a line; push up to the end of the last word.
                                lines.push(&text.text[linestart..lineend - 1]);
                                linestart = lineend;
                            }
                            // Push the remaining text.
                            lines.push(&text.text[linestart..]);
                        }

                        let offset_x = (font.size.x - text.size.x) / 2. + text.padding;
                        for (y, line) in lines.iter().enumerate() {
                            let offset_y = (text.size.y - font.size.y) / 2. - text.padding
                                - font.size.y * y as f32;

                            for (x, b) in line.to_uppercase().bytes().enumerate() {
                                parent.spawn_bundle(SpriteSheetBundle {
                                    texture_atlas: font.texture_atlas_handle.clone(),
                                    sprite: TextureAtlasSprite {
                                        index: b as u32 - 32,
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(
                                        offset_x + font.size.x * x as f32,
                                        offset_y,
                                        1.),
                                    ..Default::default()
                                });
                            }
                        }
                    });
            }
        }
    }
}
