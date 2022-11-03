
use std::rc::Rc;

use crate::bitmap_text::{GMBitmapText, GMBitmapFont};
use crate::math::{GMVec2D};
use crate::context::GMContext;

#[derive(Debug, Clone)]
pub struct GMScoreBase {
    pub value: u32,
    pub text: GMBitmapText,
    pub prefix: String,
    pub suffix: String,
    pub active: bool,
    pub visible: bool,
}

// TODO:
// - Add some transition text effects when the value changes
// - Add effects for score
// - Add ScoreBuilder

impl GMScoreBase {
    pub fn new<V: Into<GMVec2D>>(value: u32, bitmap_font: &Rc<GMBitmapFont>, position: V) -> Self {
        let position = position.into();

        let mut text = GMBitmapText::new(bitmap_font);
        text.base.position = position;
        text.base.set_text(format!("{}", value));

        Self {
            value,
            text,
            prefix: "SCORE: ".to_string(),
            suffix: "".to_string(),
            active: true,
            visible: true,
        }
    }

    fn change_text(&mut self) {
        self.text.base.set_text(format!("{}{}{}", self.prefix, self.value, self.suffix));
    }

    pub fn set_value(&mut self, value: u32) {
        if self.active {
            self.value = value;
            self.change_text();
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn add_value(&mut self, amount: u32) {
        if self.active {
            self.value += amount;
            self.change_text();
        }
    }

    pub fn sub_value(&mut self, amount: u32) {
        if self.active {
            self.value -= amount;
            self.change_text();
        }
    }

    pub fn set_position<V: Into<GMVec2D>>(&mut self, position: V) {
        self.text.base.set_position(position);
    }

    pub fn set_prefix<S: Into<String>>(&mut self, prefix: S) {
        self.prefix = prefix.into();
        self.change_text();
    }

    pub fn set_suffix<S: Into<String>>(&mut self, suffix: S) {
        self.suffix = suffix.into();
        self.change_text();
    }

    pub fn update(&mut self, context: &mut GMContext) {
        if self.active {
            self.text.update(context);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        if self.visible {
            self.text.draw(context);
        }
    }

}

#[derive(Debug, Clone)]
pub struct GMHighScoreBase {
    position: GMVec2D,
    title: GMBitmapText,
    title_offset: f32,
    score_text: GMBitmapText,
    score_offset: f32,
    separator: String,
    num_of_entries: usize,
    scores: Vec<(u32, String)>,
    texts: Vec<GMBitmapText>,
}


impl GMHighScoreBase {
    pub fn new<V: Into<GMVec2D>>(position: V, bitmap_font: &Rc<GMBitmapFont>, num_of_entries: usize) -> Self {
        let position = position.into();
        let mut title = GMBitmapText::new(bitmap_font);
        title.base.position = position;
        title.base.set_text("HIGHSCORE");

        let score_text = GMBitmapText::new(bitmap_font);

        let mut scores = Vec::with_capacity(num_of_entries + 1);
        let mut score_counter = 1000 * (num_of_entries + 1) as u32;

        for _ in 0..num_of_entries {
            scores.push((score_counter, "GREEN MOON 2D".to_string()));
            score_counter -= 1000;
        }

        let texts = Vec::new();

        let mut result = Self {
            position,
            title,
            title_offset: 16.0,
            score_text,
            score_offset: 16.0,
            separator: " - ".to_string(),
            num_of_entries,
            scores,
            texts,
        };

        result.score_changed();
        result
    }

    pub fn score_changed(&mut self) {
        todo!();
    }

    pub fn is_score_in(&self, score: u32) -> bool {
        todo!();
    }

    pub fn add_score<S: Into<String>>(&mut self, score: u32, text: S) {
        let text = text.into();
        self.scores.push((score, text));
        self.scores.sort_by(|(score1, _), (score2, _)| score1.cmp(score2) );
        self.score_changed();
    }

    pub fn set_scores(&mut self) {
        todo!();
    }

    pub fn export_csv(&self) {
        todo!();
    }

    pub fn import_csv(&mut self) {
        todo!();
    }

    pub fn set_position(&mut self) {
        todo!();
    }

    pub fn set_title(&mut self) {
        todo!();
    }

    pub fn set_title_font(&mut self) {
        todo!();
    }

    pub fn set_title_offset(&mut self) {
        todo!();
    }

    pub fn set_score_font(&mut self) {
        todo!();
    }

    pub fn set_score_offset(&mut self, score_offset: f32) {
        self.score_offset = score_offset;


    }

    pub fn set_separator<S: Into<String>>(&mut self, separator: S) {
        self.separator = separator.into();
        self.score_changed();
    }

    pub fn update(&mut self, context: &mut GMContext) {
        self.title.update(context);

        for text in self.texts.iter_mut() {
            text.update(context);
        }
    }

    pub fn draw(&self, context: &mut GMContext) {
        self.title.draw(context);

        for text in self.texts.iter() {
            text.draw(context);
        }
    }
}

