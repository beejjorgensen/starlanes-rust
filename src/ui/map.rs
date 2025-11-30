use starlanes::map::{Map, MapCell};
use starlanes::company::Company;

pub fn display_map(m: Map) {
    for row in m.data {
        for cell in row {
            let character = match cell {
                MapCell::Space => '.',
                MapCell::Outpost => '+',
                MapCell::Star => '*',
                MapCell::Company(Company::AltairStarways) => 'A',
                MapCell::Company(Company::BetelgeuseLtd) => 'B',
                MapCell::Company(Company::CapellaFreightCo) => 'C',
                MapCell::Company(Company::DenebolaShippers) => 'D',
                MapCell::Company(Company::EridaniExpediters) => 'E',
            };
            print!(" {character} ");
        }
        println!();
    }
}
