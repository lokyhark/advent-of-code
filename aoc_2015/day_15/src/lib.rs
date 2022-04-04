use std::cmp::max;

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 15;

pub fn part_one(input: &str) -> Result<i32> {
    let ingredients = parse_input(input.trim())?;
    let recipes = combinations(ingredients.len(), 100);
    let mut max = 0;
    for recipe in recipes {
        let cookie = cookie(&ingredients, &recipe);
        let score = score(cookie);
        if score > max {
            max = score;
        }
    }
    Ok(max)
}

pub fn part_two(input: &str) -> Result<i32> {
    let ingredients = parse_input(input.trim())?;
    let recipes = combinations(ingredients.len(), 100);
    let mut max = 0;
    for recipe in recipes {
        let cookie = cookie(&ingredients, &recipe);
        if cookie.calories == 500 {
            let score = score(cookie);
            if score > max {
                max = score;
            }
        }
    }
    Ok(max)
}

fn parse_input(input: &str) -> Result<Vec<Ingredient>> {
    let mut ingredients = Vec::new();
    for line in input.lines() {
        let (_, properties) = match line.split_once(':') {
            Some(split) => split,
            None => return Err("invalid input".into()),
        };
        let mut split = properties.split_ascii_whitespace().skip(1).step_by(2);
        let capacity = match split.next().map(|x| x.trim_end_matches(',')).map(|x| x.parse::<i32>()) {
            Some(Ok(capacity)) => capacity,
            _ => return Err("invalid input".into()),
        };
        let durability = match split.next().map(|x| x.trim_end_matches(',')).map(|x| x.parse::<i32>()) {
            Some(Ok(durability)) => durability,
            _ => return Err("invalid input".into()),
        };
        let flavor = match split.next().map(|x| x.trim_end_matches(',')).map(|x| x.parse::<i32>()) {
            Some(Ok(flavor)) => flavor,
            _ => return Err("invalid input".into()),
        };
        let texture = match split.next().map(|x| x.trim_end_matches(',')).map(|x| x.parse::<i32>()) {
            Some(Ok(texture)) => texture,
            _ => return Err("invalid input".into()),
        };
        let calories = match split.next().map(|x| x.trim_end_matches(',')).map(|x| x.parse::<i32>()) {
            Some(Ok(calories)) => calories,
            _ => return Err("invalid input".into()),
        };
        let ingredient = Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        };
        ingredients.push(ingredient);
    }
    Ok(ingredients)
}

fn combinations(len: usize, target: u32) -> Vec<Vec<u32>> {
    let mut combinations = Vec::new();
    let mut cache = vec![0; len - 1];
    let mut carry = true;
    while cache[cache.len() - 1] != target {
        let mut sum: u32 = cache.iter().sum();
        for x in cache.iter_mut() {
            if carry {
                if sum == target {
                    sum -= *x;
                    *x = 0;
                } else {
                    *x += 1;
                    carry = false;
                }
            }
        }
        carry = true;
        let mut combination = cache.to_vec();
        combination.push(target - sum - 1);
        combinations.push(combination);
    }
    let mut combination = vec![0; len - 1];
    combination.push(target);
    combinations.push(combination);
    combinations
}

fn cookie(ingredients: &[Ingredient], recipe: &[u32]) -> Ingredient {
    let mut cookie = Ingredient::default();
    for (ingredient, &teaspoons) in ingredients.iter().zip(recipe) {
        cookie.capacity += ingredient.capacity * teaspoons as i32;
        cookie.durability += ingredient.durability * teaspoons as i32;
        cookie.flavor += ingredient.flavor * teaspoons as i32;
        cookie.texture += ingredient.texture * teaspoons as i32;
        cookie.calories += ingredient.calories * teaspoons as i32;
    }
    cookie.capacity = max(0, cookie.capacity);
    cookie.durability = max(0, cookie.durability);
    cookie.flavor = max(0, cookie.flavor);
    cookie.texture = max(0, cookie.texture);
    cookie.calories = max(0, cookie.calories);
    cookie
}

fn score(cookie: Ingredient) -> i32 {
    cookie.capacity * cookie.durability * cookie.flavor * cookie.texture
}

#[derive(Debug, Default)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 62842880);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 57600000);
    Ok(())
}
