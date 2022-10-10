use crossterm::{terminal::{enable_raw_mode}, event::{read, Event, KeyCode, KeyModifiers}};



fn main() {
    let _raw = enable_raw_mode();

    loop {
        if let Ok(Event::Key(key)) = read() {
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
                break;
            } else if key.modifiers.intersects(KeyModifiers::all())  {
                println!("{:?}\r", key.modifiers)
            } else {
                println!("{:?}\r", key.code)
            }
        }
    }
}
