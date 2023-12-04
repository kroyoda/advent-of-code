use std::collections::HashMap;

use regex::Regex;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,

    calories: i32,
}

struct Recipe {
    ingredients: HashMap<String, Ingredient>,
    max_quantity: usize,
}

impl Recipe {
    fn from_input(input: &str) -> Self {
        let mut ingredients = HashMap::new();
        let regex = Regex::new(
            r"(\w+): capacity ([-]?\d+), durability ([-]?\d+), flavor ([-]?\d+), texture ([-]?\d+), calories (\d+)"
        ).expect("Invalid regex");
        for line in input.lines() {
            let captures = regex.captures(line).expect("Invalid input");
            let name = captures.get(1).unwrap().as_str().to_string();
            let capacity = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let durability = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let flavor = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let texture = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();
            let calories = captures.get(6).unwrap().as_str().parse::<i32>().unwrap();

            ingredients.insert(name, Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            });
        }

        Self {
            ingredients,
            max_quantity: 100,
        }
    }

    /// Considering all the possible combinations of ingredients, bake all the cookies
    /// And return their score and their calories
    fn bake(&self) -> Vec<(i32, i32)> {
        let ingredients = self.ingredients.iter().collect::<Vec<_>>();

        let mut cookies = Vec::new();
        for combination in numeric_split(self.max_quantity, ingredients.len()) {
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;

            for (ingredient, quantity) in ingredients.iter().zip(combination) {
                capacity += ingredient.1.capacity * (quantity as i32);
                durability += ingredient.1.durability * (quantity as i32);
                flavor += ingredient.1.flavor * (quantity as i32);
                texture += ingredient.1.texture * (quantity as i32);
                calories += ingredient.1.calories * (quantity as i32);
            }

            if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                continue;
            }

            cookies.push((capacity * durability * flavor * texture, calories));
        }

        cookies
    }
}

/// Split a number into `parts` parts, where each part is a positive integer.
fn numeric_split(max: usize, parts: usize) -> Vec<Vec<usize>> {
    if parts == 1 {
        return vec![vec![max]];
    }

    (0..=max)
        .flat_map(|x| {
            let remaining = max - x;
            numeric_split(remaining, parts - 1)
                .into_iter()
                .map(move |mut combination| {
                    combination.push(x);
                    combination
                })
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let recipe = Recipe::from_input(input);
    let cookies = recipe.bake();

    cookies
        .iter()
        .map(|x| x.0)
        .max()
        .expect("No cookies")
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let recipe = Recipe::from_input(input);
    let cookies = recipe.bake();

    cookies
        .iter()
        .filter(|x| x.1 == 500)
        .map(|x| x.0)
        .max()
        .expect("No cookies")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(solve_part1(input), 62842880);
    }

    #[test]
    fn test_part2() {
        let input =
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(solve_part2(input), 57600000);
    }
}
