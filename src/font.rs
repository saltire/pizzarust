use bevy::prelude::*;
use std::path::Path;

use super::bounce::{Bounce, BounceEffect, EffectType};


pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(load_fonts.system())
            .add_startup_system(create_text.system())
            .add_system(render_text.system());
    }
}

struct FontInfo {
    name: String,
    file_name: String,
    tile_size: Vec2,
    grid_size: Vec2,
}

struct BitmapFont {
    info: FontInfo,
    texture_atlas_handle: Handle<TextureAtlas>,
}

struct BitmapText {
    text: String,
    font: String,
    position: Vec3,
    box_size: Vec2,
    padding: f32,
    background_color: Color,
}
impl Default for BitmapText {
    fn default() -> Self {
        BitmapText {
            text: "".into(),
            font: "".into(),
            position: Vec3::ZERO,
            box_size: Vec2::ZERO,
            padding: 0.,
            background_color: Color::NONE,
        }
    }
}

fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let fonts = vec![
        FontInfo {
            name: "GeeBee".into(),
            file_name: "geebeeyay-8x8".into(),
            tile_size: Vec2::new(8., 8.),
            grid_size: Vec2::new(77., 1.),
        },
        FontInfo {
            name: "BluePink".into(),
            file_name: "32X32-FA".into(),
            tile_size: Vec2::new(32., 32.),
            grid_size: Vec2::new(10., 6.),
        },
    ];

    for font in fonts {
        let texture_handle = asset_server.load(Path::new(&format!("fonts/{}.png", font.file_name)));
        let texture_atlas = TextureAtlas::from_grid(texture_handle, font.tile_size,
            font.grid_size.x as usize, font.grid_size.y as usize);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn().insert(BitmapFont {
            info: font,
            texture_atlas_handle,
        });
    }
}

fn create_text(mut commands: Commands) {
    // commands.spawn().insert(BitmapText {
    //     text: "Nemo et voluptas et cumque ipsum cumque inventore. Eveniet soluta odio sint aut asperiores et. Maxime unde cupiditate sunt dolor corporis nihil.".to_string(),
    //     font: "GeeBee".to_string(),
    //     position: Vec3::new(-100., 0., 2.),
    //     box_size: Vec2::new(160., 200.),
    //     padding: 4.,
    //     background_color: Color::BLACK,
    //     ..Default::default()
    // });

    commands.spawn()
        .insert(BitmapText {
            text: "Pizza!".into(),
            font: "BluePink".into(),
            padding: 6.,
            ..Default::default()
        })
        .insert(Bounce {
            effects: vec![
                BounceEffect {
                    effect_type: EffectType::Bounce,
                    distance: 30.,
                    period: 0.75,
                    ..Default::default()
                },
                BounceEffect {
                    effect_type: EffectType::HorizontalWave,
                    distance: 50.,
                    period: 2.,
                    ..Default::default()
                },
            ],
        });
}

fn render_text(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    fonts: Query<&BitmapFont>,
    texts: Query<(&BitmapText, Entity), Added<BitmapText>>,
) {
    for (text, entity) in texts.iter() {
        for font in fonts.iter() {
            if font.info.name == text.font {
                let maxlen = ((text.box_size.x - text.padding * 2.) / font.info.tile_size.x)
                    as usize;

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

                let box_size = if text.box_size != Vec2::ZERO { text.box_size } else {
                    let clen = lines.iter().map(|line| line.len()).max().unwrap_or(0) as f32;
                    Vec2::new(clen * font.info.tile_size.x + text.padding * 2.,
                        font.info.tile_size.y + text.padding * 2.)
                };

                // Add sprites onto the existing BitmapText entity.
                commands.entity(entity)
                    .insert_bundle(SpriteBundle {
                        material: materials.add(text.background_color.into()),
                        transform: Transform::from_translation(text.position),
                        sprite: Sprite::new(box_size),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        let offset_x = (font.info.tile_size.x - box_size.x) / 2. + text.padding;
                        for (y, line) in lines.iter().enumerate() {
                            let offset_y = (box_size.y - font.info.tile_size.y) / 2.
                                - text.padding - font.info.tile_size.y * y as f32;

                            for (x, b) in line.to_uppercase().bytes().enumerate() {
                                parent.spawn_bundle(SpriteSheetBundle {
                                    texture_atlas: font.texture_atlas_handle.clone(),
                                    sprite: TextureAtlasSprite {
                                        index: b as u32 - 32,
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(
                                        offset_x + font.info.tile_size.x * x as f32,
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
