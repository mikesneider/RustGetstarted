

pub fn main() {
    let config_max = Some(3u16);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
