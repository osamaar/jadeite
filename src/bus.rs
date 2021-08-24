#[derive(Debug)]
pub struct Bus {

}

impl Bus {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn read(&self, _addr: u16) -> u8 {
        0
    }
}