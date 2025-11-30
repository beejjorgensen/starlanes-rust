use starlanes::map::Map;

mod ui;

fn main() {
    let m = Map::new();

    ui::display_map(m);
    //println!("{:#?}", m);
}
