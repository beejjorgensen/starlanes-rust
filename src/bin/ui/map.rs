//! Map-related UI functionality

use starlanes::map::Map;
use starlanes::map::MapCell::*;

use crate::ui;

/// Map a company number to a map character
fn company_to_char(id: u32) -> char {
    match id {
        0..5 => (b'A' + id as u8) as char,
        _ => panic!("unknown company id: {}", id),
    }
}

/// Display the map
pub fn display_map(m: &Map) {
    ui::formfeed();

    println!("\n{}MAP OF THE GALAXY", ui::tab(22));
    println!("{}*******************", ui::tab(21));
    println!("{} A  B  C  D  E  F  G  H  I  J  K  L", ui::tab(12));

    for r in 0..m.height {
        print!("{} {} ", ui::tab(9), r + 1);
        for c in 0..m.width {
            let cell = m.get(r, c);

            let character = match cell {
                Space => '.',
                Outpost => '+',
                Star => '*',
                Company(id) => company_to_char(id),
            };

            print!(" {character} ");
        }
        println!();
    }
}
