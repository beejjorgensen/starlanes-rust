#[derive(Debug)]
pub struct Company {
    pub in_use: bool,
    pub size: usize,
    //share_price: usize,
}

impl Company {
    pub fn new() -> Self {
        Company {
            in_use: false,
            size: 0,
            //share_price: 100,
        }
    }
}

impl Default for Company {
    fn default() -> Self {
        Self::new()
    }
}
