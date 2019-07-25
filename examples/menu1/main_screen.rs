
use green_moon_2d::prelude::*;

const QUIT_ITEM: u8 = 4;

pub struct MainScreen {
    state: GM_ScreenState,
    menu: GM_Menu,
    screen_size: GM_Dimension,
    menu_pos: GM_Position,
    menu_y_speed: u32,
}

impl MainScreen {
    pub fn new(settings: &GM_Settings) -> MainScreen {
        let screen_size = settings.get_screen_size();
        let menu_pos = GM_Position::new(screen_size.get_width() / 2, 128);

        let title_font = GM_BitmapFont::load("assets/title_font.png");
        let item_unselected_font = GM_BitmapFont::load("assets/item_unselected_font.png");
        let item_selected_font = GM_BitmapFont::load("assets/item_selected_font.png");
        let exit_unselected_font = GM_BitmapFont::load("assets/exit_unselected_font.png");
        let exit_selected_font = GM_BitmapFont::load("assets/exit_selected_font.png");

        let title = Box::new(GM_WaveText::new(&title_font, "MAIN MENU"));
        
        let items: Vec<Box<dyn GM_SelectableText_T>> = vec![
            Box::new(GM_SelectableText::new(&item_unselected_font, &item_selected_font, "START")),
            Box::new(GM_SelectableText::new(&item_unselected_font, &item_selected_font, "OPTIONS")),
            Box::new(GM_SelectableText::new(&item_unselected_font, &item_selected_font, "HIGHSCORE")),
            Box::new(GM_SelectableText::new(&item_unselected_font, &item_selected_font, "CREDITS")),
            Box::new(GM_SelectableText::new(&exit_unselected_font, &exit_selected_font, "EXIT")),
        ];

        let mut menu = GM_Menu::new(title, items);

        MainScreen {
            state: GM_ScreenState::GM_Enter,
            menu,
            screen_size: screen_size.clone(),
            menu_pos,
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

    fn update(&mut self, runtime: &GM_Runtime) {
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

    fn draw(&self, canvas: &mut GM_Canvas) {
        match self.state {
            GM_ScreenState::GM_Enter => {
            }
            GM_ScreenState::GM_Normal => {
            }
            GM_ScreenState::GM_Leave => {
            }
        }
        self.menu.draw(canvas);
    }
}

