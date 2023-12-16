#[derive(Default)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
    fn pow(&self) -> u32 {
        (self.red * self.green * self.blue) as u32
    }
}

fn parse(input: &str) -> Vec<Vec<Round>> {
    let mut games: Vec<Vec<Round>> = Vec::new();
    for line in input.lines() {
        let (_, rounds) = line.split_once(": ").unwrap();
        let rounds = rounds.split("; ").collect::<Vec<_>>();
        let mut round_list = Vec::new();
        for r in rounds{
            let cubes = r.split(", ").collect::<Vec<_>>();
            let mut round = Round::default();
            for c in cubes {
                let (amount, color) = c.split_once(" ").unwrap();
                let amount: usize = amount.parse().unwrap();
                match &color.chars().nth(0).unwrap() {
                    'r' => round.red = amount,
                    'g' => round.green = amount,
                    'b' => round.blue = amount,
                    _ => (),
                }
            }
            round_list.push(round);
        }
        games.push(round_list);
    }
    games
}

fn part1(games: &Vec<Vec<Round>>) -> u32 {
    let mut sum_id = 0;
    'next_game: for (index, game) in games.iter().enumerate() {
        for round in game {
            if !round.is_valid() {
                continue 'next_game;
            }
        }
        sum_id += (index as u32) + 1;
    }
    sum_id
}  

fn part2(games: &Vec<Vec<Round>>) -> u32 {
    let mut sum_pow = 0;
    for game in games.iter() {
        let mut min_round = Round::default();
        for round in game {
            min_round.red = min_round.red.max(round.red);
            min_round.green = min_round.green.max(round.green);
            min_round.blue = min_round.blue.max(round.blue);
        }
        sum_pow += min_round.pow();
    }
    sum_pow
} 
