
use std::any::Any;

use log::error;

pub fn error_panic(message: &str) -> ! {
    error!("{}", message);
    panic!("{}", message);
}

pub fn extract_f32_value(message: &str, data: Option<Box<dyn Any>>) -> f32 {
    if let Some(data) = data {
        if let Ok(value) = data.downcast::<f32>() {
            return *value
        }
    }

    error_panic(&format!("util::extract_f32_value(), expected f32, message: {}", message))
}
