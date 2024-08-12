use std::collections::{HashMap, HashSet};

use quicksilver::prelude::*;

struct HVector(Vector);

impl std::hash::Hash for HVector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
    }
}

impl PartialEq for HVector {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

impl Eq for HVector {}

impl From<Vector> for HVector {
    fn from(vec: Vector) -> Self {
        HVector(vec)
    }
}

fn main() {
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };

    run::<Game>("Quicksilver Rougelike", Vector::new(800, 600), settings);
}

struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    square_font_info: Asset<Image>,

    map_size: Vector,
    map: Vec<Tile>,
    entities: Vec<Entity>,
    player_id: usize,

    tileset: Asset<HashMap<char, Image>>,
    tile_size_px: Vector,
}

impl State for Game {
    /// Load the assets and initialize the game
    fn new() -> Result<Self> {

        let font_mononoki = "mononoki-Regular.ttf";
        let title = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "Quicksilver Rougelike",
                &FontStyle::new(72.0, Color::BLACK)
            )
        }));
        let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
                &FontStyle::new(20.0, Color::BLACK)
            )
        }));
        let square_font_info = Asset::new(Font::load(font_mononoki).and_then(move |font| {
            font.render(
                "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        let map_size = Vector::new(20, 15);
        let map = generate_map(map_size);
        let (player_id, entities) = generate_entities();

        // Game Glyphs
        let font_square = "square.ttf";
        let game_glyphs = "#@g.%";
        let tile_size_px = Vector::new(24, 24);

        let tileset = Asset::new(Font::load(font_square).and_then(move |font| {
            let tiles = font
                .render(game_glyphs, &FontStyle::new(tile_size_px.y, Color::WHITE))
                .expect("Could not render the font tileset.");
            
            let mut tileset = HashMap::new();
            for (index, glyph) in game_glyphs.chars().enumerate() {
                let pos = (index as i32 * tile_size_px.x as i32, 0);
                let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
                tileset.insert(glyph, tile);
            }

            Ok(tileset)
        }));

        Ok(Self {
            title,
            mononoki_font_info,
            square_font_info,

            map_size,
            map,
            entities,
            player_id,

            tileset,
            tile_size_px,
        })
    }

    /// Logical
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;

        let player = &mut self.entities[self.player_id];
        if window.keyboard()[Key::Left] == Pressed {
            player.pos.x -= 1.0;
        }
        if window.keyboard()[Key::Right] == Pressed {
            player.pos.x += 1.0;
        }
        if window.keyboard()[Key::Up] == Pressed {
            player.pos.y -= 1.0;
        }
        if window.keyboard()[Key::Down] == Pressed {
            player.pos.y += 1.0;
        }
        if window.keyboard()[Key::Escape] == Pressed {
            window.close();
        }

        Ok(())
    }

    /// Rendering
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;
        self.mononoki_font_info.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 - 20)),
                Img(&image),
            );
            Ok(())
        })?;
        self.square_font_info.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 - 50)),
                Img(&image),
            );
            Ok(())
        })?;


        let tileset = &mut self.tileset;
        let tile_size_px = self.tile_size_px;
        let map = &self.map;
        let entities = &self.entities;
        let offset_px = Vector::new(
            window.screen_size().x as i32 / 2 - (self.map_size.x * self.tile_size_px.x) as i32 / 2,
            120
        );

        let mut is_pos_used = HashSet::new();
        let mut draw_tile = |image: &Image, pos: Vector, color: Color| {
            let hvpos = HVector::from(pos);
            if is_pos_used.contains(&hvpos) {
                return;
            }
            is_pos_used.insert(hvpos);
            let pos_px = pos.times(tile_size_px) + offset_px;
            window.draw(
                &Rectangle::new(pos_px, image.area().size()),
                Blended(&image, color),
            );
        };

        tileset.execute(|tileset| {
            for entity in entities.iter() {
                if let Some(image) = tileset.get(&entity.glyph) {
                    draw_tile(image, entity.pos, entity.color);
                }
            }
            Ok(())
        })?;

        tileset.execute(|tileset| {
            for tile in map.iter() {
                if let Some(image) = tileset.get(&tile.glyph) {
                    draw_tile(image, tile.pos, tile.color);
                }
            }
            Ok(())
        })?;

        // health bar
        let player = &self.entities[self.player_id];
        let full_health_width_px = 100.0;
        let current_health_width_px =
            (player.hp as f32 / player.max_hp as f32) * full_health_width_px;
        
        let map_size_px = self.map_size.times(tile_size_px);
        let health_bar_pos_px = offset_px + Vector::new(map_size_px.x, 0.0);
        // full health
        window.draw(
            &Rectangle::new(health_bar_pos_px, (full_health_width_px, tile_size_px.y)),
            Col(Color::RED.with_alpha(0.5)),
        );
        // current health
        window.draw(
            &Rectangle::new(health_bar_pos_px, (current_health_width_px, tile_size_px.y)),
            Col(Color::RED),
        );

        Ok(())
    }
}

// Tiles

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
}

fn generate_map(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;

    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: if x == 0 || x == width - 1 || y == 0 || y == height - 1 { '#' } else { '.' },
                color: Color::BLACK,
            };

            map.push(tile);
        }
    }

    map
}

// Entity

#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    glyph: char,
    color: Color,
    hp: i32,
    max_hp: i32,
}

fn generate_entities() -> (usize, Vec<Entity>) {
    (0, vec![
        Entity {
            pos: Vector::new(5, 3),
            glyph: '@',
            color: Color::BLUE,
            hp: 3,
            max_hp: 5,
        },
        Entity {
            pos: Vector::new(9, 6),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(2, 4),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(7, 5),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(4, 8),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        },
    ])
}