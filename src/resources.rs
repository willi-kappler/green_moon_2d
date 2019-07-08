
struct GM_Resources {
    // GFX
    canvas: GM_Canvas,

    // SFX
    sound_pool: Vec<GM_Sound>,
    music_pool: Vec<GM_Music>,

    // Misc
    settings: GM_Settings,
    quit: bool,
}

impl GM_Resources {
    fn new() -> GM_Resources {
        GM_Resources {
            canvas: GM_Canvas::new(),
            sound_pool: Vec::new(),
            music_pool: Vec::new(),
            settings: GM_Settings::new(),
            quit: false,
        }
    }

    pub fn quit_game(&mut self) {
        self.quit = true;
    }
}
