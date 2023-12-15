fn part1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let nums: Vec<u32> = line
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect();
        
        total += (nums.first().unwrap()) * 10 + nums.last().unwrap();   
    }
    total
}  

fn part2(input: &str) -> u32 {
    let changes = vec![("one", "one1one"), 
    ("two", "two2two"), 
    ("three", "three3three"), 
    ("four", "four4four"), 
    ("five", "five5five"), 
    ("six", "six6six"), 
    ("seven", "seven7seven"), 
    ("eight", "eight8eight"),
    ("nine", "nine9nine")];

    let mut input = input.to_string();

    for (from, to) in changes {
        input = input.replace(from, to);
    }
    
    part1(&input) 
}
