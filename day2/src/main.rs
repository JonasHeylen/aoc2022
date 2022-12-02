use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

struct Round {
    you: Shape,
    opponent: Shape,
}

impl Round {
    fn new(you: Shape, opponent: Shape) -> Self {
        Self { you, opponent }
    }

    fn play(&self) -> Outcome {
        if self.you == self.opponent {
            return Outcome::Draw;
        }
        match (&self.you, &self.opponent) {
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            _ => Outcome::Loss,
        }
    }

    fn score(&self) -> u32 {
        self.you.score() + self.play().score()
    }
}

impl FromStr for Round {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opponent = Shape::from_str(&s[0..1])?;
        let you = Shape::from_str(&s[2..3])?;
        Ok(Round::new(you, opponent))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Round2 {
    opponent: Shape,
    outcome: Outcome,
}

impl Round2 {
    fn new(opponent: Shape, outcome: Outcome) -> Self {
        Self { opponent, outcome }
    }

    fn play(&self) -> Round {
        let you = match (&self.opponent, &self.outcome) {
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Rock, Outcome::Loss) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Paper, Outcome::Loss) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
            (Shape::Scissors, Outcome::Loss) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        };
        Round::new(you, self.opponent.clone())
    }
}

impl FromStr for Round2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opponent = Shape::from_str(&s[0..1])?;
        let outcome = Outcome::from_str(&s[2..3])?;
        Ok(Round2::new(opponent, outcome))
    }
}

fn run(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .into_iter()
        .map(|line| Round::from_str(&line.unwrap()).unwrap().score())
        .sum()
}

fn run_part2(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .into_iter()
        .filter_map(|line| {
            let round2 = Round2::from_str(&line.ok()?).ok()?;
            let round = round2.play();
            let score = round.score();
            println!("{:?} you: {:?} score: {}", round2, round.you, score);
            Some(score)
        })
        .sum()
}

fn main() {
    let score = run("input.txt");
    let score2 = run_part2("input.txt");
    println!("Score: {score} - Score 2: {score2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_shape() {
        assert_eq!(Shape::Rock.score(), 1);
        assert_eq!(Shape::Paper.score(), 2);
        assert_eq!(Shape::Scissors.score(), 3);
    }

    #[test]
    fn test_score_outcome() {
        assert_eq!(Outcome::Win.score(), 6);
        assert_eq!(Outcome::Loss.score(), 0);
        assert_eq!(Outcome::Draw.score(), 3);
    }

    #[test]
    fn test_play_round() {
        assert_eq!(Round::new(Shape::Rock, Shape::Paper).play(), Outcome::Loss);
        assert_eq!(Round::new(Shape::Paper, Shape::Rock).play(), Outcome::Win);
        assert_eq!(
            Round::new(Shape::Scissors, Shape::Scissors).play(),
            Outcome::Draw
        );
    }

    #[test]
    fn test_score_round() {
        assert_eq!(Round::new(Shape::Rock, Shape::Paper).score(), 1);
        assert_eq!(Round::new(Shape::Paper, Shape::Rock).score(), 8);
        assert_eq!(Round::new(Shape::Scissors, Shape::Scissors).score(), 6);
    }

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 15);
    }

    #[test]
    fn test_run_part2() {
        assert_eq!(run_part2("input_test.txt"), 12);
    }
}
