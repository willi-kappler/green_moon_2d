use crate::resource_manager::GMName;
use crate::error::GMError;

use std::{collections::HashMap, hash::BuildHasherDefault};

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams};
use macroquad::color::colors;
use macroquad::math::Rect;
use macroquad::file::load_string;

use fnv::{FnvHashMap, FnvHasher};

pub struct GMBitmapFont {
    pub(crate) name: String,
    pub(crate) spacing_x: f32,
    pub(crate) spacing_y: f32,
    pub(crate) mapping: HashMap<char, Rect, BuildHasherDefault<FnvHasher>>,
    pub(crate) unknown: Rect,
    pub(crate) data: Texture2D,
}

impl GMBitmapFont {
    pub async fn from_img_file(name: &str, spacing_x: f32, spacing_y: f32, char_mapping: Vec<(char, Rect)>, unknown: Rect, file_name: &str) -> Result<GMBitmapFont, GMError> {
        let data = load_texture(file_name).await?;
        let mut mapping = FnvHashMap::with_capacity_and_hasher(char_mapping.len(), Default::default());

        for (c, r) in char_mapping.iter() {
            mapping.insert(*c, *r);
        }

        let font = GMBitmapFont {
            name: name.to_string(),
            spacing_x,
            spacing_y,
            mapping,
            unknown,
            data
        };

        Ok(font)
    }

    pub async fn from_config_file(file_name: &str) -> Result<GMBitmapFont, GMError> {
        let file_content = load_string(file_name).await?;
        GMBitmapFont::parse_config(&file_content)
    }

    fn parse_config(input: &str) -> Result<GMBitmapFont, GMError> {
        // Format:
        // font_name: font name
        // file_name: name of bitmap file (png, jpeg, ..)
        // spacing_x: spacing after each char in x direction
        // spacing_y: spacing after each char in y direction
        // unknown: x y w h of unknown char
        // mapping: [
        //     A 0 0 32 32
        //     B 32 0 32 32
        //     C 64 0 32 32
        //     ...
        // ]

        let mut font_name: String;
        let mut file_name: String;
        let mut spacing_x: u32;
        let mut spacing_y: u32;

        let mapping_mode = false;
        let mut mapping: Vec<(char, Rect)> = Vec::new();


        for (line_no, line) in input.split('\n').enumerate() {
            let line_no2 = line_no + 1;

            if mapping_mode {
                let items = line.split(char::is_whitespace).collect::<Vec<&str>>();
                let num_of_items = items.len();

                match num_of_items {
                    1 => {
                        if items[0] == "]" {
                            // End of mapping mode, exit loop
                            break
                        } else {
                            return Err(GMError::ParseFont(format!("Expected end of mapping ']', got: {} (line {})", items[0], line_no2)))
                        }
                    }
                    5 => {
                        let c = items[0].chars().next().unwrap();
                        let x: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping x coordinate as u32 , got: {} (line {})", items[1], line_no2)))?;
                        let y: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping y coordinate as u32 , got: {} (line {})", items[2], line_no2)))?;
                        let w: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping width as u32 , got: {} (line {})", items[3], line_no2)))?;
                        let h: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping height as u32 , got: {} (line {})", items[4], line_no2)))?;

                        mapping.push((c, Rect::new(x as f32, y as f32, w as f32, h as f32)));
                    }
                    _ => {
                        return Err(GMError::ParseFont(format!("Expected char mapping, got: {} (line {})", line, line_no2)))
                    }
                }
            } else {
                if line.starts_with("font_name:") {
                    font_name = line[10..].trim().to_string();
                } else if line.starts_with("file_name:") {
                    file_name = line[10..].trim().to_string();
                } else if line.starts_with("spacing_x:") {
                    spacing_x = line[10..].trim().parse().map_err(|_| GMError::ParseFont(format!("Expected spacing_x as u32 , got: {} (line {})", line, line_no2)))?;
                } else if line.starts_with("spacing_x:") {
                    spacing_y = line[10..].trim().parse().map_err(|_| GMError::ParseFont(format!("Expected spacing_y as u32 , got: {} (line {})", line, line_no2)))?;
                } else if line.starts_with("unknown:") {
                    let items = line[8..].split(char::is_whitespace);
                }
            }
        }

        todo!()
    }

    pub fn draw_char(&self, c: char, x: f32, y: f32) -> (f32, f32) {
        let rect = self.source_rect(c);
        let source = Some(rect);
        let params = DrawTextureParams {
            source, .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::BLANK, params);

        let spacing_x = rect.w + self.spacing_x;
        let spacing_y = rect.h + self.spacing_y;
        (spacing_x, spacing_y)
    }

    fn source_rect(&self, c: char) -> Rect {
        match self.mapping.get(&c) {
            Some(rect) => {
                *rect
            }
            None => {
                self.unknown
            }
        }
    }
}

impl GMName for GMBitmapFont {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }

    fn has_prefix(&self, name: &str) -> bool {
        self.name.starts_with(name)
    }
}
