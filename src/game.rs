

pub trait GM_Game_T : GM_Process_T + GM_Update_T + GM_Draw_T {
    fn initialize(&mut self) {
    }
}

struct GreenMoon2D<U> {
    resources: GM_Resources,
    settinge: GM_Settings,
    actual_game: U,
}

impl<U> GreenMoon2D<U> {
    pub fn new<U: GM_Game_T>(actual_game: U) -> Result<GreenMoon2D, GM_Init_Error> {
        GreenMoon2D {
            resources: GM_Resources::new(),
            settings: GM_Settings::new(),
            actual_game,
        }
    }

    pub fn run(&mut self) -> Result<(), GM_Game_Error> {
        self.actual_game.initialize();

        while ! self.resources.quit {
            self.process();
            self.update();
            self.draw();
        }
    }

    fn process(&mut self) {
        self.actual_game.process(event, &mut self.resources);
    }

    fn update(&mut self) {
        self.actual_game.update(&mut self.resources);
    }

    fn draw(&mut self) {
        self.actual_game.draw(&mut self.resources);
    }
}

pub enum GM_Init_Error {
    IO,
}

pub enum GM_Game_Error {
    TODO,
}