use green_moon_2d::{util};


fn main() {
    println!("Green Moon 2D Menu Example");

    let gm = util::init("configuration.toml");

    gm.run();
}
