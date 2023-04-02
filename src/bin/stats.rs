use clap::Parser;
use fortune_bayes::{Bayes1, FFFortuneMachine, FortuneGame, FortuneGameParameters};
use ordered_float::OrderedFloat;
use rand_pcg::Mcg128Xsl64;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    submit_threshold: f64,
    #[clap(long, short, default_value = "1000000")]
    samples: usize,
}

fn play_game(
    rng: &mut Mcg128Xsl64,
    fff_m: &mut FFFortuneMachine,
    params: FortuneGameParameters,
    threshold: f64,
) -> f64 {
    let mut game = FortuneGame::new(rng, fff_m, params);
    let mut bayes = Bayes1::uniform();
    while game.current_turn < game.params.max_turn {
        let r = game.draw(rng);
        bayes.new_draw(r);
        let (i, gain) = bayes.submit_expect(params);
        if gain > threshold {
            game.submit(i as f64 / 5.0);
            bayes = Bayes1::uniform();
        }
    }
    if game.lottery().is_some() {
        let (i, _) = bayes.submit_expect(params);
        game.submit(i as f64 / 5.0);
    }
    game.current_point
}

fn main() {
    let args = Args::parse();
    let mut rng = Mcg128Xsl64::new(1);
    let params = FortuneGameParameters::default();
    let mut fff_m = FFFortuneMachine::AlwaysUniform;

    let mut count = HashMap::new();
    let mut total_point = 0.0;
    for _ in 0..args.samples {
        let point = play_game(&mut rng, &mut fff_m, params, args.submit_threshold);
        total_point += point;
        *count.entry(OrderedFloat(point)).or_insert(0) += 1;
    }
    let mut points = count.keys().copied().collect::<Vec<_>>();
    points.sort();
    for point in points {
        println!("{} {}", point, count[&point]);
    }
    eprintln!("avg: {}", total_point as f64 / args.samples as f64);
}
