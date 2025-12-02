use starlanes::map::Map;
use starlanes::map::MapCell::*;

fn company_to_char(id: u32) -> char {
    match id {
        0..5 => (b'A' + id as u8) as char,
        _ => panic!("unknown company id: {}", id),
    }
}

pub fn display_map(m: &Map) {
    for row in &m.data {
        for cell in row {
            let character = match cell {
                Space => '.',
                Outpost => '+',
                Star => '*',
                Company(id) => company_to_char(*id),
            };
            print!(" {character} ");
        }
        println!();
    }
}
