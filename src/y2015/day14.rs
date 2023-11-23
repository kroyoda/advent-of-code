struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,

    current_distance: u32,
    current_fly_time: u32,
    current_rest_time: u32,

    resting: bool,

    score: u32,
}

impl Reindeer {
    fn from_line(line: &str) -> Self {
        let mut words = line.trim_end_matches('.').split_whitespace();
        let name = words.next().unwrap().to_string();

        let speed = words.nth(2).unwrap().parse::<u32>().unwrap();
        let fly_time = words.nth(2).unwrap().parse::<u32>().unwrap();

        let rest_time = words.nth(6).unwrap().parse::<u32>().unwrap();

        Self {
            name,
            speed,
            fly_time,
            rest_time,

            current_distance: 0,
            current_fly_time: 0,
            current_rest_time: 0,

            resting: false,
            score: 0,
        }
    }

    fn tick(&mut self) {
        if self.resting {
            self.current_rest_time += 1;
            if self.current_rest_time == self.rest_time {
                self.resting = false;
                self.current_rest_time = 0;
            }
        } else {
            self.current_fly_time += 1;
            self.current_distance += self.speed;
            if self.current_fly_time == self.fly_time {
                self.resting = true;
                self.current_fly_time = 0;
            }
        }
    }
}

struct Race {
    elapsed_time: u32,
    reindeers: Vec<Reindeer>,
}

impl Race {
    fn from_input(input: &str) -> Self {
        Self {
            elapsed_time: 0,
            reindeers: input.lines().map(Reindeer::from_line).collect(),
        }
    }

    fn tick(&mut self) {
        self.elapsed_time += 1;
        for reindeer in self.reindeers.iter_mut() {
            reindeer.tick();
        }
        self.increment_score();
    }

    fn tick_n(&mut self, n: u32) {
        for _ in 0..n {
            self.tick();
        }
    }

    fn get_lead(&self) -> &Reindeer {
        self.reindeers
            .iter()
            .max_by_key(|r| r.current_distance)
            .unwrap()
    }

    fn increment_score(&mut self) {
        let lead = self.get_lead().current_distance;
        for reindeer in self.reindeers.iter_mut() {
            if reindeer.current_distance == lead {
                reindeer.score += 1;
            }
        }
    }

    fn get_best_score(&self) -> &Reindeer {
        self.reindeers
            .iter()
            .max_by_key(|r| r.score)
            .unwrap()
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut race = Race::from_input(input);
    race.tick_n(2503);

    let winner = race.get_lead();

    println!("{} won with {} km", winner.name, winner.current_distance);
    winner.current_distance
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut race = Race::from_input(input);
    race.tick_n(2503);

    let winner = race.get_best_score();

    println!("{} won with {} km", winner.name, winner.current_distance);
    winner.score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
                 Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_part1() {
        let mut race = Race::from_input(TEST_INPUT);
        race.tick_n(1000);
        assert_eq!(race.get_lead().current_distance, 1120);
    }

    #[test]
    fn test_part2() {
        let mut race = Race::from_input(TEST_INPUT);
        race.tick_n(1000);
        assert_eq!(race.get_best_score().score, 689);
    }
}
