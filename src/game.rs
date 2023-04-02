use crate::{FFFortuneMachine, FFortuneMachine, FortuneMachine, LotResult};
use rand::{distributions::Distribution, Rng};

#[derive(Debug, Copy, Clone)]
pub struct FortuneGameParameters {
    pub max_turn: u32,
    pub win_point: f64,
    pub lose_point: f64,
}

impl Default for FortuneGameParameters {
    fn default() -> Self {
        FortuneGameParameters {
            max_turn: 300,
            win_point: 1.0,
            lose_point: -3.0,
        }
    }
}

pub struct FortuneGame {
    ff_machine: FFortuneMachine,
    pub params: FortuneGameParameters,
    pub current_turn: u32,
    pub current_point: f64,
    lottery: Option<Lottery>,
}

pub struct Lottery {
    f_machine: FortuneMachine,
    pub total_count: u32,
    pub win_count: u32,
}

impl FortuneGame {
    pub fn new<R: Rng>(
        rng: &mut R,
        fff_machine: &mut FFFortuneMachine,
        params: FortuneGameParameters,
    ) -> FortuneGame {
        FortuneGame {
            ff_machine: fff_machine.sample(rng),
            params,
            current_turn: 0,
            current_point: 0.0,
            lottery: None,
        }
    }

    pub fn lottery(&self) -> Option<&Lottery> {
        self.lottery.as_ref()
    }

    pub fn draw<R: Rng>(&mut self, rng: &mut R) -> LotResult {
        assert!(self.current_turn < self.params.max_turn);
        let lottery = self.lottery.get_or_insert_with(|| Lottery {
            f_machine: self.ff_machine.sample(rng),
            total_count: 0,
            win_count: 0,
        });
        self.current_turn += 1;
        lottery.total_count += 1;
        let r = lottery.f_machine.sample(rng);
        if r == LotResult::Win {
            lottery.win_count += 1;
        }
        r
    }

    pub fn submit(&mut self, p: f64) -> Option<(bool, f64)> {
        if let Some(lottery) = self.lottery.take() {
            let r = lottery.f_machine.p == p;
            if r {
                self.current_point += self.params.win_point;
            } else {
                self.current_point += self.params.lose_point;
            }
            Some((r, lottery.f_machine.p))
        } else {
            None
        }
    }
}
