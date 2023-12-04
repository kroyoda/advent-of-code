use itertools::Itertools;

#[derive(Clone, Debug)]
struct Interaction {
    name: String,
    neighbor: String,
    happiness: i32,
}

impl Interaction {
    fn from_line(line: &str) -> Self {
        // Split whitespace and remove the last period
        let mut words = line.trim_end_matches('.').split_ascii_whitespace();
        let name = words.next().unwrap().to_string();

        words.next(); // Skip "would"

        let happiness = match words.next().unwrap() {
            "gain" => words.next().unwrap().parse::<i32>().unwrap(),
            "lose" => -words.next().unwrap().parse::<i32>().unwrap(),
            _ => panic!("Invalid input"),
        };

        let neighbor = words.last().unwrap().to_string();

        Self {
            name,
            neighbor,
            happiness,
        }
    }
}

#[derive(Clone, Debug)]
struct Table {
    seats: Vec<String>,
    interactions: Vec<Interaction>,
    total_happiness: i32,
}

impl Table {
    fn from_lines(input: &str) -> Self {
        let interactions: Vec<Interaction> = input.lines().map(Interaction::from_line).collect();
        let mut seats: Vec<String> = interactions
            .iter()
            .map(|seat| seat.name.clone())
            .collect();
        seats.sort();
        seats.dedup();

        let total_happiness = 0;
        let mut table = Table {
            seats,
            interactions,
            total_happiness,
        };

        table.calculate_total_happiness();
        table
    }

    fn get_seat_happiness(&self, seat_index: usize) -> i32 {
        // Consider the seat to the left and right of the current seat
        let left_index = if seat_index == 0 { self.seats.len() - 1 } else { seat_index - 1 };
        let right_index = (seat_index + 1) % self.seats.len();

        let seat = &self.seats[seat_index];
        let left_neighbor = &self.seats[left_index];
        let right_neighbor = &self.seats[right_index];

        let mut happiness = 0;
        for interaction in &self.interactions {
            if &interaction.name == seat && &interaction.neighbor == left_neighbor {
                happiness += interaction.happiness;
            }
            if &interaction.name == seat && &interaction.neighbor == right_neighbor {
                happiness += interaction.happiness;
            }
        }

        happiness
    }

    fn calculate_total_happiness(&mut self) {
        let mut total_happiness = 0;
        for (i, _) in self.seats.iter().enumerate() {
            total_happiness += self.get_seat_happiness(i);
        }
        self.total_happiness = total_happiness;
    }

    fn rearrange_seats(&mut self, plan: Vec<&String>) {
        // Rearrange the seats according to the plan
        let mut new_seats = vec![];
        for seat in plan {
            new_seats.push(seat.clone());
        }
        self.seats = new_seats;
        self.calculate_total_happiness();
    }

    fn add_guest(&mut self, name: &str, interactions: Vec<Interaction>) {
        // Add a new guest to the table
        self.seats.push(name.to_string());
        self.interactions.extend(interactions);
        self.calculate_total_happiness();
    }
}

#[aoc(day13, part1)]
fn solve_part1(input: &str) -> i32 {
    // Find the best permutation of seats
    let table = Table::from_lines(input);
    let mut best_table = table.clone();

    for permutation in table.seats.iter().permutations(table.seats.len()) {
        let mut new_table = table.clone();
        new_table.rearrange_seats(permutation);
        if new_table.total_happiness > best_table.total_happiness {
            best_table = new_table;
        }
    }

    best_table.total_happiness
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> i32 {
    // Add myself to the table
    let mut table = Table::from_lines(input);
    let interactions = table.seats
        .iter()
        .map(|seat| Interaction {
            name: "Me".to_string(),
            neighbor: seat.clone(),
            happiness: 0,
        })
        .collect();
    table.add_guest("Me", interactions);

    // Find the best permutation of seats
    let mut best_table = table.clone();

    for permutation in table.seats.iter().permutations(table.seats.len()) {
        let mut new_table = table.clone();
        new_table.rearrange_seats(permutation);
        if new_table.total_happiness > best_table.total_happiness {
            best_table = new_table;
        }
    }

    best_table.total_happiness
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_to_seat() {
        let line = "Alice would gain 54 happiness units by sitting next to Bob.";
        let seat = Interaction::from_line(line);
        assert_eq!(seat.name, "Alice");
        assert_eq!(seat.neighbor, "Bob");
        assert_eq!(seat.happiness, 54);

        let line = "Alice would lose 79 happiness units by sitting next to Carol.";
        let seat = Interaction::from_line(line);
        assert_eq!(seat.name, "Alice");
        assert_eq!(seat.neighbor, "Carol");
        assert_eq!(seat.happiness, -79);
    }

    #[test]
    fn test_solve_part1() {
        let input =
            "Alice would gain 54 happiness units by sitting next to Bob.
                     Alice would lose 79 happiness units by sitting next to Carol.
                     Alice would lose 2 happiness units by sitting next to David.
                     Bob would gain 83 happiness units by sitting next to Alice.
                     Bob would lose 7 happiness units by sitting next to Carol.
                     Bob would lose 63 happiness units by sitting next to David.
                     Carol would lose 62 happiness units by sitting next to Alice.
                     Carol would gain 60 happiness units by sitting next to Bob.
                     Carol would gain 55 happiness units by sitting next to David.
                     David would gain 46 happiness units by sitting next to Alice.
                     David would lose 7 happiness units by sitting next to Bob.
                     David would gain 41 happiness units by sitting next to Carol.";
        assert_eq!(solve_part1(input), 330);
    }
}
