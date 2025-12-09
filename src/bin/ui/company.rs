use crate::ui;

pub const COMPANY_NAMES: [&str; 5] = [
    "ALTAIR STARWAYS",
    "BETELGEUSE, LTD.",
    "CAPELLA FREIGHT CO.",
    "DENEBOLA SHIPPERS",
    "ERIDANI EXPEDITERS",
];

pub fn company_formed(co_num: usize) {
    println!("A NEW SHIPPING COMPANY HAS BEEN FORMED!");
    println!("IT'S NAME IS {}", ui::COMPANY_NAMES[co_num]);
}
