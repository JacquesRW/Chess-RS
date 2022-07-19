use crate::model::defs::*;
use crate::draw::draw_square;

impl Board {
    pub fn engine_move(&mut self) -> Option<bool> {
        println!("AI Moving.");
        let m = self.analyse(5);
        self.make_move(m)
    }

    pub fn draw(&self, screen: &mut [u8]) {
        for i in 0..8 {
            for j in 0..8 {
                draw_square(self, i, j, screen);
            }
        }
    }
}
