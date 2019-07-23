
use green_moon_2d::prelude::*;

struct MainScreen {
    state: GM_Screen_State,
    menu: GM_Menu,
}

impl MainScreen {
    fn new() -> MainScreen {
        let menu = GM_Menu::new();

        MainScreen {
            state: GM_Screen_State::GM_Enter,
            menu,
        }
    }
}

impl GM_Screen_T for MainScreen {
    fn enter(&mut self) {
        self.state = GM_Screen_State::GM_Enter;
    }

    fn process(&mut self, runtime: &mut GM_Runtime) {
        match self.state {
            GM_Screen_State::GM_Enter => {
                // Nothing to do for now
            }
            GM_Screen_State::GM_Normal => {
                // Only process when in normal mode
                self.menu.process(runtime);
            }
            GM_Screen_State::GM_Leave => {
                // Nothing to do for now
            }
        }
    }

    fn update(&mut self, runtime: &mut GM_Runtime) {
        self.menu.update(runtime);

        match self.state {
            GM_Screen_State::GM_Enter => {
            }
            GM_Screen_State::GM_Normal => {
            }
            GM_Screen_State::GM_Leave => {
            }
        }
    }

    fn draw(&mut self, runtime: &mut GM_Runtime) {
        self.menu.draw(runtime);

        match self.state {
            GM_Screen_State::GM_Enter => {
            }
            GM_Screen_State::GM_Normal => {
            }
            GM_Screen_State::GM_Leave => {
            }
        }
    }
}


fn main() {
    let settings = GM_Settings::new();
    let mut menu_example = GreenMoon2D::new();

    menu_example.set_settings(settings);

    menu_example.add_screen(MainScreen::new());

    menu_example.run();
}
