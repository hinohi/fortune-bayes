use rand::{distributions::Distribution, Rng};

/// くじ引き機
///
/// ## Example
/// ```rust
/// use fortune_bayes::{FortuneMachine, LotResult};
/// use rand::distributions::Distribution;
/// # use rand_pcg::Mcg128Xsl64;
/// # let mut rng = Mcg128Xsl64::new(1);
/// let fm = FortuneMachine::new(1.0);
/// for _ in 0..100 {
///     assert_eq!(fm.sample(&mut rng), LotResult::Win);
/// }
/// let fm = FortuneMachine::new(0.0);
/// for _ in 0..100 {
///     assert_eq!(fm.sample(&mut rng), LotResult::Lose);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FortuneMachine {
    pub p: f64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LotResult {
    Win,
    Lose,
}

impl FortuneMachine {
    pub const fn new(p: f64) -> FortuneMachine {
        FortuneMachine { p }
    }
}

impl Distribution<LotResult> for FortuneMachine {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LotResult {
        if rng.gen_bool(self.p) {
            LotResult::Win
        } else {
            LotResult::Lose
        }
    }
}

/// くじ引き機を生成するくじ引き機
///
/// 確率 $p=0/5, 1/5, 2/5, 3/5, 4/5, 5/5$ で当たりが出るくじ引き機を生成する。
///
/// ## Example
/// ```rust
/// use fortune_bayes::{FFortuneMachine, LotResult};
/// use rand::distributions::Distribution;
/// # use rand_pcg::Mcg128Xsl64;
/// # let mut rng = Mcg128Xsl64::new(1);
/// let ffm = FFortuneMachine::new([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
/// for _ in 0..100 {
///     let fm = ffm.sample(&mut rng);
///     assert_eq!(fm.sample(&mut rng), LotResult::Win);
/// }
#[derive(Debug, Clone)]
pub struct FFortuneMachine {
    pub dist: [f64; 6],
}

impl FFortuneMachine {
    pub fn new(dist: [f64; 6]) -> FFortuneMachine {
        let s = dist[0] + dist[1] + dist[2] + dist[3] + dist[4] + dist[5];
        assert!((s - 1.0).abs() <= f64::EPSILON * 2.0);
        FFortuneMachine { dist }
    }
}

impl Distribution<FortuneMachine> for FFortuneMachine {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FortuneMachine {
        let r: f64 = rng.gen();
        let mut s = 0.0;
        for (i, &q) in self.dist.iter().enumerate() {
            s += q;
            if r <= s {
                let p = i as f64 / 5.0;
                return FortuneMachine::new(p);
            }
        }
        FortuneMachine::new(1.0)
    }
}

/// (くじ引き機を生成する)*2 くじ引き機
#[derive(Debug, Clone)]
pub enum FFFortuneMachine {
    AlwaysUniform,
}

impl Distribution<FFortuneMachine> for FFFortuneMachine {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> FFortuneMachine {
        match self {
            FFFortuneMachine::AlwaysUniform => FFortuneMachine::new([1.0 / 6.0; 6]),
        }
    }
}
