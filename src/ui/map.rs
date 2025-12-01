use starlanes::company::Company::*;
use starlanes::map::Map;
use starlanes::map::MapCell::*;

pub fn display_map(m: &Map) {
    for row in &m.data {
        for cell in row {
            let character = match cell {
                Space => '.',
                Outpost => '+',
                Star => '*',
                Company(AltairStarways) => 'A',
                Company(BetelgeuseLtd) => 'B',
                Company(CapellaFreightCo) => 'C',
                Company(DenebolaShippers) => 'D',
                Company(EridaniExpediters) => 'E',
            };
            print!(" {character} ");
        }
        println!();
    }
}
