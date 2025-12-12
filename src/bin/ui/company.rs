//! Company UI.
use crate::ui;

/// A list of company names used.
pub const COMPANY_NAMES: [&str; 5] = [
    "ALTAIR STARWAYS",
    "BETELGEUSE, LTD.",
    "CAPELLA FREIGHT CO.",
    "DENEBOLA SHIPPERS",
    "ERIDANI EXPEDITERS",
];

/// Print a new-company-formed messsage.
pub fn company_formed(co_num: usize) {
    println!("A NEW SHIPPING COMPANY HAS BEEN FORMED!");
    println!("IT'S NAME IS {}", ui::company_name(co_num));
}

/// Return a company name for a given index.
pub fn company_name(co_num: usize) -> &'static str {
    ui::COMPANY_NAMES[co_num]
}
