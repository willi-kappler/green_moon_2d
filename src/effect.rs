
#[macro_export]
macro_rules! gen_effect_trait {
    ($trait:ident, $base:ty) => {
        pub trait $trait {
            fn update(&mut self, _text_base: &mut $base, _context: &mut GMContext) {
            }

            fn draw(&self, _text_base: &$base, _context: &mut GMContext) {
            }

            fn send_message(&mut self, _message: &str) {
            }
        }
    };
}

#[macro_export]
macro_rules! gen_effect_impl_for_type {
    ($type:ty) => {
        impl GMUpdateT for $type {
            fn update(&mut self, context: &mut GMContext) {
                if self.active {
                    for effect in self.effects.iter_mut() {
                        effect.update(&mut self.base, context);
                    }
                }
            }
        }

        impl GMDrawT for $type {
            fn draw(&self, context: &mut GMContext) {
                if self.visible {
                    if self.base.draw_text_first {
                        self.base.draw(context);

                        for effect in self.effects.iter() {
                            effect.draw(&self.base, context);
                        }
                    } else {
                        for effect in self.effects.iter() {
                            effect.draw(&self.base, context);
                        }

                        self.base.draw(context);
                    }
                }
            }
        }

        gen_impl_active!($type);

        gen_impl_visible!($type);

    };
}

#[macro_export]
macro_rules! gen_type_effect_methods {
    ($base:ty, $trait:ident) => {
        pub fn get_base(&self) -> &$base {
            &self.base
        }

        pub fn get_base_mut(&mut self) -> &mut $base {
            &mut self.base
        }

        pub fn add_effect<T: 'static + $trait>(&mut self, effect: T) {
            debug!("$base::add_effect()");
            self.effects.push(Box::new(effect));
        }

        pub fn add_effect2(&mut self, effect: Box<dyn $trait>) {
            debug!("$base::add_effect2()");
            self.effects.push(effect);
        }

        pub fn remove_effect(&mut self, index: usize) {
            debug!("$base::remove_effect(), index: {}", index);
            self.effects.remove(index);
        }

        pub fn set_effects(&mut self, effects: Vec<Box<dyn $trait>>) {
            debug!("$base::set_effects()");
            self.effects = effects;
        }

        pub fn replace_effect(&mut self, index: usize, effect: Box<dyn $trait>) {
            debug!("$base:replace_effect(), index: {}", index);
            self.effects[index] = effect;
        }

        pub fn clear_effects(&mut self) {
            debug!("$base::clear_effects()");
            self.effects.clear();
        }

        pub fn swap_effects(&mut self, index1: usize, index2: usize) {
            debug!("$base::swap_effect(), index1: {}, index2: {}", index1, index2);
            self.effects.swap(index1, index2);
        }

        pub fn send_effect_message(&mut self, index: usize, message: &str) {
            self.effects[index].send_message(message);
        }

        pub fn send_effect_message_all(&mut self, message: &str) {
            for effect in self.effects.iter_mut() {
                effect.send_message(message);
            }
        }

        pub fn get_effects(&self) -> &Vec<Box<dyn $trait>> {
            &self.effects
        }

        pub fn get_effect(&self, index: usize) -> &Box<dyn $trait> {
            &self.effects[index]
        }

        pub fn get_effects_mut(&mut self) -> &mut Vec<Box<dyn $trait>> {
            &mut self.effects
        }

        pub fn get_effect_mut(&mut self, index: usize) -> &mut Box<dyn $trait> {
            &mut self.effects[index]
        }
    }
}
