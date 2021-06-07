use crate::animation::GMAnimationT;
use crate::font::GMFontT;

use std::collections::HashMap;

pub struct GMContext {
    fonts: HashMap<String, Box<dyn GMFontT>>,
    animation: HashMap<String, Box<dyn GMAnimationT>>,
}
