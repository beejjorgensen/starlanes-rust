use starlanes::map::Map;
use starlanes::map::MapCell::*;

use crate::ui;

fn company_to_char(id: u32) -> char {
    match id {
        0..5 => (b'A' + id as u8) as char,
        _ => panic!("unknown company id: {}", id),
    }
}

pub fn display_map(m: &Map) {
    ui::formfeed();

    println!("{}MAP OF THE GALAXY", ui::tab(22));
    println!("{}*******************", ui::tab(21));
    println!("{} A  B  C  D  E  F  G  H  I  J  K  L", ui::tab(12));

    for (i, row) in m.data.iter().enumerate() {
        print!("{} {} ", ui::tab(9), i + 1);
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
