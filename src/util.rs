
use log::error;

pub fn error_panic(message: &str) -> ! {
    error!("{}", message);
    panic!("{}", message);
}
