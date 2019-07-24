
use green_moon_2d::prelude::*;

const QUIT_ITEM: u8 = 4;

pub struct MainScreen {
    state: GM_ScreenState,
    menu: GM_Menu,
    screen_size: GM_ScreenSize,
    menu_pos: GM_Position,
    menu_y_speed: u32,
}

impl MainScreen {
    pub fn new(settings: &GM_Settings) -> MainScreen {
        /*
        let position = GM_Position::new(512, 128);

        let title_font = GM_Font::load("assets/title_font.png");
        let item_font = GM_Font::load("assets/item_font.png");
        let exit_font = GM_Font::load("assets/item_font2.png");
        let fonts = vec![title_font, item_font, exit_font];

        let title = GM_Menu_Title::new("MAIN MENU", 0);

        let items = vec![
            GM_Menu_Item::new("START", 1),
            GM_Menu_Item::new("OPTIONS", 1),
            GM_Menu_Item::new("HIGH SCORE", 1),
            GM_Menu_Item::new("CREDITS", 1),
            GM_Menu_Item::new("EXIT", 2),
        ];

        let mut menu = GM_Menu::new( position, fonts, title, items );
        */

        let menu = GM_Menu::load("assets/main_menu.toml");
        let screen_size = settings.get_screen_size();

        MainScreen {
            state: GM_ScreenState::GM_Enter,
            menu,
            screen_size: screen_size.clone(),
            menu_pos: GM_Position::new(screen_size.get_width() / 2, 128),
            menu_y_speed: 16,
        }
    }
}

impl GM_Screen_T for MainScreen {
    fn enter(&mut self) {
        self.state = GM_ScreenState::GM_Enter;
        self.menu_pos = GM_Position::new(self.screen_size.get_width() / 2, self.screen_size.get_height());
        self.menu.set_position(&self.menu_pos);
    }

    fn process(&mut self, runtime: &mut GM_Runtime) {
        match self.state {
            GM_ScreenState::GM_Enter => {
                // Nothing to do for now
            }
            GM_ScreenState::GM_Normal => {
                // Only process when in normal mode
                self.menu.process(runtime);

                match runtime.get_event() {
                    GM_Event::GM_Key_Enter_Press => {
                        if self.menu.get_selected_item() == QUIT_ITEM {
                            self.state = GM_ScreenState::GM_Leave;
                        }
                    }
                    _ => {}
                }
            }
            GM_ScreenState::GM_Leave => {
                // Nothing to do for now
            }
        }
    }

    fn update(&mut self, runtime: &mut GM_Runtime) {
        match self.state {
            GM_ScreenState::GM_Enter => {
                if self.menu_pos.get_y() > 128 {
                    self.menu_pos.dec_y(self.menu_y_speed);
                } else {
                    self.menu_pos.set_y(128);
                    self.state = GM_ScreenState::GM_Normal;
                }
                self.menu.set_position(&self.menu_pos);
            }
            GM_ScreenState::GM_Normal => {
            }
            GM_ScreenState::GM_Leave => {
            }
        }
        self.menu.update(runtime);
    }

    fn draw(&mut self, runtime: &mut GM_Runtime) {
        match self.state {
            GM_ScreenState::GM_Enter => {
            }
            GM_ScreenState::GM_Normal => {
            }
            GM_ScreenState::GM_Leave => {
            }
        }
        self.menu.draw(runtime);
    }
}

