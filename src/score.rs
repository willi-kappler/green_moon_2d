
// TODO:
// - draw()
// - update()
// - event()
// - add font
// - add text
// - add title
// - add button (OK or BACK)
// - add load()
// - add save()

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GMScore {
    points: u32,
    name: String,
}

impl GMScore {
    pub fn new(points: u32, name: &str) -> Self {
        Self {
            points,
            name: name.to_string(),
        }
    }
}

pub struct GMScoreTable {
    scores: Vec<GMScore>,
    max_entries: usize,
}

impl GMScoreTable {
    pub fn new(max_entries: usize) -> Self {
        Self {
            scores: Vec::with_capacity(max_entries),
            max_entries,
        }
    }
    pub fn new_from_vec(scores: Vec<(u32, String)>) -> Self {
        let mut scores2: Vec<GMScore> = scores.into_iter().map(|(s, n)| GMScore::new(s, &n)).collect();
        scores2.sort();

        let max_entries = scores2.len();

        Self {
            scores: scores2,
            max_entries,
        }
    }
    pub fn add_score(&mut self, points: u32, name: &str) {
        if self.enough_points(points) {
            self.scores.push(GMScore::new(points, name));
            self.scores.sort();
            self.scores.truncate(self.max_entries);
        }
    }
    pub fn enough_points(&self, points: u32) -> bool {
        if self.scores.len() == self.max_entries {
            let last = self.scores.last().unwrap();
            points > last.points
        } else {
            true
        }
    }
    pub fn draw(&self) {

    }
    pub fn update(&mut self) {

    }
    pub fn event(&self) -> bool {
        false
    }
}
