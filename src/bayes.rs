use crate::{FortuneGameParameters, LotResult};

pub struct Bayes1 {
    pub p: [f64; 6],
}

impl Bayes1 {
    pub fn uniform() -> Bayes1 {
        Bayes1 { p: [1.0 / 6.0; 6] }
    }

    pub fn new_draw(&mut self, r: LotResult) {
        let p = if r == LotResult::Win {
            [
                0.0 / 5.0 * self.p[0],
                1.0 / 5.0 * self.p[1],
                2.0 / 5.0 * self.p[2],
                3.0 / 5.0 * self.p[3],
                4.0 / 5.0 * self.p[4],
                5.0 / 5.0 * self.p[5],
            ]
        } else {
            [
                5.0 / 5.0 * self.p[0],
                4.0 / 5.0 * self.p[1],
                3.0 / 5.0 * self.p[2],
                2.0 / 5.0 * self.p[3],
                1.0 / 5.0 * self.p[4],
                0.0 / 5.0 * self.p[5],
            ]
        };
        let s = p[0] + p[1] + p[2] + p[3] + p[4] + p[5];
        self.p = [p[0] / s, p[1] / s, p[2] / s, p[3] / s, p[4] / s, p[5] / s];
    }

    pub fn submit_expect(&self, params: FortuneGameParameters) -> (usize, f64) {
        let (max, _) = self
            .p
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .unwrap();
        let mut gain = 0.0;
        for (i, &p) in self.p.iter().enumerate() {
            if i == max {
                gain += params.win_point * p;
            } else {
                gain += params.lose_point * p;
            }
        }
        (max, gain)
    }
}
