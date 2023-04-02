use fortune_bayes::{Bayes1, FFFortuneMachine, FortuneGame, FortuneGameParameters};
use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;
use std::io::{stdin, stdout, BufRead, Write};

enum UserInput {
    Draw,
    Submit(usize),
}

fn read_input<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> UserInput {
    loop {
        write!(writer, "Draw[default] or Submit[0-5]: ").unwrap();
        writer.flush().unwrap();
        let mut buf = String::new();
        reader.read_line(&mut buf).expect("Fail to read stdin");
        return match buf.trim().to_lowercase().as_str() {
            "d" | "draw" | "" => UserInput::Draw,
            "0" => UserInput::Submit(0),
            "1" => UserInput::Submit(1),
            "2" => UserInput::Submit(2),
            "3" => UserInput::Submit(3),
            "4" => UserInput::Submit(4),
            "5" => UserInput::Submit(5),
            _ => {
                writeln!(writer, "bad input").unwrap();
                continue;
            }
        };
    }
}

fn main() {
    let mut rng = Mcg128Xsl64::from_entropy();
    let params = FortuneGameParameters::default();
    let mut fff_m = FFFortuneMachine::AlwaysUniform;
    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    let mut game = FortuneGame::new(&mut rng, &mut fff_m, params);
    let mut bayes = Bayes1::uniform();
    while game.current_turn < game.params.max_turn {
        let r = game.draw(&mut rng);
        let lottery = game.lottery().unwrap();
        writeln!(stdout, "Draw = {:?}", r).unwrap();
        writeln!(
            stdout,
            "current draw stats: {}/{} = {:.3}%",
            lottery.win_count,
            lottery.total_count,
            100.0 * lottery.win_count as f64 / lottery.total_count as f64,
        )
        .unwrap();
        bayes.new_draw(r);
        let (i, gain) = bayes.submit_expect(params);
        writeln!(stdout, "Bayes say: {} is best, get {}", i, gain).unwrap();
        writeln!(stdout, "Bayes internal: {:?}", bayes.p).unwrap();

        match read_input(&mut stdin, &mut stdout) {
            UserInput::Draw => (),
            UserInput::Submit(p) => {
                let (r, p) = game.submit(p as f64 / 5.0).unwrap();
                if r {
                    writeln!(stdout, "WIN").unwrap();
                } else {
                    writeln!(stdout, "LOSE, ans is {}/5", (p * 5.0).round()).unwrap();
                }
                writeln!(stdout, "point = {}", game.current_point).unwrap();
                writeln!(stdout).unwrap();

                bayes = Bayes1::uniform();
            }
        }
    }
}
