use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::MaterialMesh2dBundle,
};
use bevy_asset_loader::{AssetLoader, AssetCollection};
use image::{
    GenericImageView, Rgba, RgbaImage,
    imageops::overlay,
};

use super::bounce::{Bounce, BounceEffect, EffectType};
use super::cycle::CycleMaterial;


pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(FontState::FontsLoading)
            .continue_to_state(FontState::FontsReady)
            .with_collection::<FontAssets>()
            .build(app);

        app
            .add_state(FontState::FontsLoading)
            .add_system_set(SystemSet::on_exit(FontState::FontsLoading)
                .with_system(load_fonts))
            .add_system_set(SystemSet::on_enter(FontState::FontsReady)
                .with_system(create_text))
            .add_system(render_text);
    }
}

// Bevy Asset Loader

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum FontState {
    FontsLoading,
    FontsReady,
}

enum CharMap {
    Ascii,
    TSK,
}

struct FontInfo {
    name: String,
    asset_handle: Handle<Image>,
    tile_size: Vec2,
    grid_size: Vec2,
    char_map: CharMap,
}

#[derive(Component)]
struct BitmapFont {
    info: FontInfo,
    texture_atlas_handle: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
struct FontAssets {
    #[asset(path = "fonts/32X32-FA.png")]
    bluepink: Handle<Image>,
    #[asset(path = "fonts/elecfont.png")]
    elecfont: Handle<Image>,
    #[asset(path = "fonts/fantfont.png")]
    fantfont: Handle<Image>,
    #[asset(path = "fonts/geebeeyay-8x8.png")]
    geebee: Handle<Image>,
    #[asset(path = "fonts/TSK03.png")]
    tsk: Handle<Image>,
}

fn load_fonts(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    // asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let fonts = vec![
        FontInfo {
            name: "BluePink".into(),
            asset_handle: font_assets.bluepink.clone(),
            tile_size: Vec2::new(32., 32.),
            grid_size: Vec2::new(10., 6.),
            char_map: CharMap::Ascii,
        },
        FontInfo {
            name: "Elecfont".into(),
            asset_handle: font_assets.elecfont.clone(),
            tile_size: Vec2::new(32., 32.),
            grid_size: Vec2::new(10., 6.),
            char_map: CharMap::Ascii,
        },
        FontInfo {
            name: "Fantfont".into(),
            asset_handle: font_assets.fantfont.clone(),
            tile_size: Vec2::new(32., 32.),
            grid_size: Vec2::new(10., 6.),
            char_map: CharMap::Ascii,
        },
        FontInfo {
            name: "GeeBee".into(),
            asset_handle: font_assets.geebee.clone(),
            tile_size: Vec2::new(8., 8.),
            grid_size: Vec2::new(77., 1.),
            char_map: CharMap::Ascii,
        },
        FontInfo {
            name: "TSK".into(),
            asset_handle: font_assets.tsk.clone(),
            tile_size: Vec2::new(32., 20.),
            grid_size: Vec2::new(10., 4.),
            char_map: CharMap::TSK,
        },
    ];

    for font in fonts {
        let texture_atlas = TextureAtlas::from_grid(font.asset_handle.clone(), font.tile_size,
            font.grid_size.x as usize, font.grid_size.y as usize);

        commands.spawn().insert(BitmapFont {
            info: font,
            texture_atlas_handle: texture_atlases.add(texture_atlas),
        });
    }
}

#[derive(Component)]
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

fn create_text(
    mut commands: Commands
) {
    // commands.spawn().insert(BitmapText {
    //     text: "Nemo et voluptas et cumque ipsum cumque inventore. Eveniet soluta odio sint aut asperiores et. Maxime unde cupiditate sunt dolor corporis nihil.".to_string(),
    //     font: "GeeBee".to_string(),
    //     position: Vec3::new(-100., 0., 2.),
    //     box_size: Vec2::new(160., 200.),
    //     padding: 4.,
    //     background_color: Color::rgb(0.1, 0.1, 0.2),
    //     ..Default::default()
    // });

    commands.spawn()
        .insert(BitmapText {
            text: "Pizza!".into(),
            font: "Elecfont".into(),
            padding: 6.,
            // background_color: Color::rgb(0.1, 0.1, 0.2),
            position: Vec3::new(0., 0., 2.),
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

fn wrap_lines(text: &str, maxlen: usize) -> Vec<&str> {
    let mut lines: Vec<&str> = vec![];
    if maxlen <= 0 {
        lines.push(&text);
    } else {
        let mut linestart = 0;
        let mut lineend = 0;
        for (i, char) in text.chars().enumerate() {
            if char == ' ' {
                if i - linestart <= maxlen {
                    // Line not full yet; mark this space and keep going.
                    lineend = i + 1;
                } else if lineend == linestart {
                    // Line full with a single word; push everything up to here.
                    lineend = i + 1;
                    lines.push(&text[linestart..lineend - 1]);
                    linestart = lineend;
                } else {
                    // Line full with more than one word;
                    // push up to the end of the last one.
                    lines.push(&text[linestart..lineend - 1]);
                    linestart = lineend;
                    lineend = i + 1;
                }
            }
        }
        if lineend > linestart && text.len() - linestart > maxlen {
            // End of text with more than one word remaining
            // and longer than a line; push up to the end of the last word.
            lines.push(&text[linestart..lineend - 1]);
            linestart = lineend;
        }
        // Push the remaining text.
        lines.push(&text[linestart..]);
    }

    lines
}

fn get_char_index(char: char, char_map: &CharMap) -> Option<usize> {
    match char_map {
        CharMap::Ascii => " !\"#$%&'()*+'-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        CharMap::TSK => "ABCDEFGHIJKLMNOPQRSTUVWXYZ.,()0123456789",
    }.find(char)
}

fn render_text(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CycleMaterial>>,
    fonts: Query<&BitmapFont>,
    texts: Query<(&BitmapText, Entity), Added<BitmapText>>,
) {
    for (text, entity) in texts.iter() {
        let font = fonts.iter().find(|f| f.info.name == text.font).expect("Font not found.");

        let lines = wrap_lines(&text.text,
            ((text.box_size.x - text.padding * 2.) / font.info.tile_size.x) as usize);

        let box_size = if text.box_size != Vec2::ZERO { text.box_size } else {
            let clen = lines.iter().map(|line| line.len()).max().unwrap_or(0) as f32;
            Vec2::new(
                clen * font.info.tile_size.x + text.padding * 2.,
                font.info.tile_size.y + text.padding * 2.,
            )
        };

        // Create a new image with the given dimensions and background colour.
        let mut image = RgbaImage::from_pixel(
            box_size.x as u32,
            box_size.y as u32,
            Rgba([
                (text.background_color.r() * 255.) as u8,
                (text.background_color.g() * 255.) as u8,
                (text.background_color.b() * 255.) as u8,
                (text.background_color.a() * 255.) as u8,
            ]),
        );

        // Overlay each character from the texture atlas onto the image.
        let texture_atlas = texture_atlases.get(&font.texture_atlas_handle)
            .expect("Texture atlas not found.");
        let atlas_tex = images.get(&texture_atlas.texture)
            .expect("Texture atlas texture not found.");
        let atlas_image = RgbaImage::from_raw(
            texture_atlas.size.x as u32,
            texture_atlas.size.y as u32,
            atlas_tex.data.clone(),
        ).expect("Atlas image not found.");

        for (y, line) in lines.iter().enumerate() {
            let offset_y = text.padding + font.info.tile_size.y * y as f32;

            for (x, c) in line.to_uppercase().char_indices() {
                if let Some(index) = get_char_index(c, &font.info.char_map) {
                    if let Some(rect) = texture_atlas.textures.get(index) {
                        let view = atlas_image.view(
                            rect.min.x as u32,
                            rect.min.y as u32,
                            rect.width() as u32,
                            rect.height() as u32,
                        );

                        overlay(&mut image, &view,
                            (text.padding + font.info.tile_size.x * x as f32) as u32,
                            offset_y as u32);
                        }
                }
            }
        }

        // Convert the image to a texture asset.
        let texture = Image::new(
            Extent3d {
                width: box_size.x as u32,
                height: box_size.y as u32,
                ..Default::default()
            },
            TextureDimension::D2,
            image.into_raw(),
            TextureFormat::Rgba8UnormSrgb,
        );

        // Add texture as a 2D mesh onto the existing BitmapText entity.
        commands.entity(entity)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    scale: Vec3::from((box_size, 1.)),
                    translation: text.position,
                    ..Default::default()
                },
                material: materials.add(CycleMaterial {
                    image: Some(images.add(texture)),
                }),
                ..Default::default()
            });
    }
}
