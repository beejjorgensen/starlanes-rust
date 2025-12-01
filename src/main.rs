use starlanes::starlanes::StarLanes;

mod ui;

fn main() {
    let mut g = StarLanes::new();

    g.init(3);

    ui::display_map(&g.map);
    println!("Current player: {}", g.get_current_player());
}
