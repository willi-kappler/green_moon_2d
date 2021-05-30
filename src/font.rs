use crate::resource_manager::GMName;
use crate::error::GMError;

use std::{collections::HashMap};

use macroquad::texture::{Texture2D, draw_texture_ex, load_texture, DrawTextureParams, Image};
use macroquad::color::colors;
use macroquad::math::Rect;
use macroquad::file::load_string;

#[derive(Clone, Debug, PartialEq)]
pub struct GMBitmapFontConfig {
    pub(crate) name: String,
    pub(crate) spacing_x: f32,
    pub(crate) spacing_y: f32,
    pub(crate) mapping: HashMap<char, Rect>,
    pub(crate) unknown: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GMBitmapFont {
    pub(crate) config: GMBitmapFontConfig,
    pub(crate) data: Texture2D,
}

impl GMBitmapFont {
    pub async fn from_img_file(config: GMBitmapFontConfig, file_name: &str) -> Result<GMBitmapFont, GMError> {
        let data = load_texture(file_name).await?;

        let font = GMBitmapFont { config, data };

        Ok(font)
    }

    pub async fn from_config_file(file_name: &str) -> Result<GMBitmapFont, GMError> {
        let file_content = load_string(file_name).await?;
        let (config, file_name) = GMBitmapFont::parse_config(&file_content)?;
        GMBitmapFont::from_img_file(config, &file_name).await
    }

    fn parse_config(input: &str) -> Result<(GMBitmapFontConfig, String), GMError> {
        // Format:
        // font_name: font name
        // file_name: name of bitmap file (png, jpeg, ..)
        // spacing_x: spacing after each char in x direction
        // spacing_y: spacing after each char in y direction
        // unknown: x y w h of unknown char
        // mapping: (x y w h for each line)
        //     A 0 0 32 32
        //     B 32 0 32 32
        //     C 64 0 32 32
        //     ...
        // end_mapping

        let mut font_name = String::new();
        let mut file_name = String::new();
        let mut spacing_x: u32 = 0;
        let mut spacing_y: u32 = 0;
        let mut unknown = Rect::new(0.0, 0.0, 0.0, 0.0);

        let mut mapping_mode = false;
        let mut mapping: HashMap<char, Rect> = HashMap::new();

        for (line, line_no) in input.split('\n').zip(1..) {
            if mapping_mode {
                let items = line.split(char::is_whitespace).collect::<Vec<&str>>();
                let num_of_items = items.len();

                match num_of_items {
                    1 => {
                        if items[0] == "end_mapping" {
                            // End of mapping mode, exit loop
                            break
                        } else {
                            return Err(GMError::ParseFont(format!("Expected 'end_mapping', got: {} (line {})", items[0], line_no)))
                        }
                    }
                    5 => {
                        let c = items[0].chars().next().unwrap();

                        if mapping.contains_key(&c) {
                            return Err(GMError::ParseFont(format!("Character already defined: {} (line {})", line, line_no)))
                        }

                        let x: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping x coordinate as u32, got: {} (line {})", items[1], line_no)))?;
                        let y: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping y coordinate as u32, got: {} (line {})", items[2], line_no)))?;
                        let w: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping width as u32, got: {} (line {})", items[3], line_no)))?;
                        let h: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected mapping height as u32, got: {} (line {})", items[4], line_no)))?;

                        let rect = Rect::new(x as f32, y as f32, w as f32, h as f32);

                        mapping.insert(c, rect);
                    }
                    _ => {
                        return Err(GMError::ParseFont(format!("Expected char mapping, got: {} (line {})", line, line_no)))
                    }
                }
            } else {
                if line.starts_with("font_name:") {
                    font_name = line[10..].trim().to_string();
                } else if line.starts_with("file_name:") {
                    file_name = line[10..].trim().to_string();
                } else if line.starts_with("spacing_x:") {
                    spacing_x = line[10..].trim().parse().map_err(|_| GMError::ParseFont(format!("Expected spacing_x as u32 , got: {} (line {})", line, line_no)))?;
                } else if line.starts_with("spacing_x:") {
                    spacing_y = line[10..].trim().parse().map_err(|_| GMError::ParseFont(format!("Expected spacing_y as u32 , got: {} (line {})", line, line_no)))?;
                } else if line.starts_with("unknown:") {
                    let items = line[8..].split(char::is_whitespace).collect::<Vec<&str>>();

                    if items.len() != 4 {
                        return Err(GMError::ParseFont(format!("Expecting 4 u32 integer values for unknown char, got: {} (line: {})", line, line_no)))
                    }

                    let x: u32 = items[0].parse().map_err(|_| GMError::ParseFont(format!("Expected x coordinate as u32 for unknown char, got: {} (line {})", items[0], line_no)))?;
                    let y: u32 = items[1].parse().map_err(|_| GMError::ParseFont(format!("Expected y coordinate as u32 for unknown char, got: {} (line {})", items[1], line_no)))?;
                    let w: u32 = items[2].parse().map_err(|_| GMError::ParseFont(format!("Expected width as u32 for unknown char, got: {} (line {})", items[2], line_no)))?;
                    let h: u32 = items[3].parse().map_err(|_| GMError::ParseFont(format!("Expected height as u32 for unknown char, got: {} (line {})", items[3], line_no)))?;

                    unknown = Rect::new(x as f32, y as f32, w as f32, h as f32);
                } else if line.starts_with("mapping:") {
                    mapping_mode = true;
                }
            }
        }

        if font_name.len() == 0 {
            return Err(GMError::ParseFont("Font name is missing".to_string()))
        }

        if file_name.len() == 0 {
            return Err(GMError::ParseFont("File name is missing".to_string()))
        }

        if unknown.w == 0.0 {
            return Err(GMError::ParseFont("Unknown char width is missing or invalid".to_string()))
        }

        if unknown.h == 0.0 {
            return Err(GMError::ParseFont("Unknown char height is missing or invalid".to_string()))
        }

        if mapping.len() == 0 {
            return Err(GMError::ParseFont("Mapping is missing".to_string()))
        }

        let config = GMBitmapFontConfig{
            name: font_name,
            spacing_x: spacing_x as f32,
            spacing_y: spacing_y as f32,
            mapping,
            unknown,

        };

        Ok((config, file_name))
    }

    pub fn draw_char(&self, c: char, x: f32, y: f32) -> (f32, f32) {
        let rect = self.source_rect(c);
        let source = Some(rect);
        let params = DrawTextureParams {
            source, .. Default::default()
        };

        draw_texture_ex(self.data, x, y, colors::BLANK, params);

        let spacing_x = rect.w + self.config.spacing_x;
        let spacing_y = rect.h + self.config.spacing_y;
        (spacing_x, spacing_y)
    }

    pub fn get_extend(&self, c: char) -> (f32, f32) {
        let rect = self.source_rect(c);
        let spacing_x = rect.w + self.config.spacing_x;
        let spacing_y = rect.h + self.config.spacing_y;
        (spacing_x, spacing_y)
    }

    pub fn source_rect(&self, c: char) -> Rect {
        match self.config.mapping.get(&c) {
            Some(rect) => {
                *rect
            }
            None => {
                self.config.unknown
            }
        }
    }

    fn get_image(&self, c: char) -> Image {
        // Maybe return the char as image ?
        // https://docs.rs/macroquad/0.3.4/macroquad/texture/struct.Image.html
        todo!()
    }
}

impl GMName for GMBitmapFont {
    fn get_name(&self) -> &str {
        &self.config.name
    }

    fn set_name(&mut self, name: &str) {
        self.config.name = name.to_string();
    }

    fn has_name(&self, name: &str) -> bool {
        self.config.name == name
    }

    fn has_prefix(&self, name: &str) -> bool {
        self.config.name.starts_with(name)
    }
}
