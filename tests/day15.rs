use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("res/15.txt");

#[derive(Debug)]
struct ComboSummator(HashMap<(i64, usize), Vec<Vec<i64>>>);

impl ComboSummator {
    fn new() -> ComboSummator {
        ComboSummator(HashMap::new())
    }

    fn combos(&mut self, target: i64, n: usize) -> Vec<Vec<i64>> {
        // base cases
        if n == 0 {
            return vec![vec![]];
        }
        if n == 1 {
            return self
                .0
                .entry((target, n))
                .or_insert_with(|| vec![vec![target]])
                .clone();
        }
        // memoized solution
        if let Some(combos) = self.0.get(&(target, n)) {
            return combos.clone();
        }
        // construct new case
        let combos: Vec<Vec<i64>> = (1..target)
            .map(|i| {
                self.combos(target - i, n - 1)
                    .into_iter()
                    .map(|mut seq| {
                        seq.push(i);
                        seq
                    })
                    .collect_vec()
            })
            .flat_map(|f| f.into_iter())
            .collect_vec();
        self.0.insert((target, n), combos);
        self.0.get(&(target, n)).unwrap().clone()
    }
}

#[derive(Debug)]
struct Ingredient([i64; 5]);

impl Ingredient {
    fn new() -> Ingredient {
        Ingredient([0; 5])
    }

    fn parse(input: &str) -> Ingredient {
        let mut tokens = input.split_ascii_whitespace();
        Ingredient([
            tokens.nth(2).unwrap().parse().unwrap(),
            tokens.nth(1).unwrap().parse().unwrap(),
            tokens.nth(1).unwrap().parse().unwrap(),
            tokens.nth(1).unwrap().parse().unwrap(),
            tokens.nth(1).unwrap().parse().unwrap(),
        ])
    }

    fn score(&self) -> i64 {
        self.0[..4]
            .iter()
            .map(|v| if *v < 0 { 0 } else { *v })
            .product()
    }

    fn calories(&self) -> i64 {
        self.0[4]
    }
}

fn fold_ingredients(recipe: &[(&Ingredient, i64)]) -> Ingredient {
    let (dish, _): (Ingredient, i64) = recipe.iter().fold(
        (Ingredient::new(), 0),
        |(mut acc, _), (ingredient, quantity)| {
            acc.0
                .iter_mut()
                .zip(ingredient.0.iter())
                .for_each(|(acc_prop, ing_prop)| *acc_prop += ing_prop * quantity);
            (acc, 0)
        },
    );
    return dish;
}

// how do we get all quartets of numbers that sum to 100?
#[test]
fn part1() {
    let ingredients = INPUT.lines().map(Ingredient::parse).collect_vec();
    let mut combo_summator = ComboSummator::new();
    let ans = combo_summator
        .combos(100, ingredients.len())
        .iter()
        .map(|recipe| {
            recipe
                .iter()
                .enumerate()
                .map(|(i, n)| (&ingredients[i], *n))
                .collect_vec()
        })
        .map(|recipe| fold_ingredients(&recipe))
        .map(|recipe| recipe.score())
        .max()
        .unwrap();
    println!("Day 15, part 1: {}", ans);
    assert_eq!(13882464, ans);
}

#[test]
fn part2() {
    let ingredients = INPUT.lines().map(Ingredient::parse).collect_vec();
    let mut combo_summator = ComboSummator::new();
    let ans = combo_summator
        .combos(100, ingredients.len())
        .iter()
        .map(|recipe| {
            recipe
                .iter()
                .enumerate()
                .map(|(i, n)| (&ingredients[i], *n))
                .collect_vec()
        })
        .map(|recipe| fold_ingredients(&recipe))
        .filter(|recipe| recipe.calories() == 500)
        .map(|recipe| recipe.score())
        .max()
        .unwrap();
    println!("Day 15, part 2: {}", ans);
    assert_eq!(11171160, ans);
}
