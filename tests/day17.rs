use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("res/17.txt");

type Volume = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Container {
    id: usize,
    volume: Volume,
}

type Containers = BTreeSet<Container>;

struct Combinator(BTreeMap<(Volume, Containers), BTreeSet<Containers>>);

impl Combinator {
    fn new() -> Combinator {
        Combinator(BTreeMap::new())
    }

    fn combinations(&mut self, volume: Volume, containers: &Containers) -> BTreeSet<Containers> {
        if volume == 0 {
            let mut ans = BTreeSet::new();
            ans.insert(BTreeSet::new());
            return ans;
        }
        // cached solution
        if let Some(combos) = self.0.get(&(volume, containers.clone())) {
            return combos.clone();
        }
        // base case
        if containers.len() == 1 {
            if volume == containers.iter().next().unwrap().volume {
                let mut ans = BTreeSet::new();
                ans.insert(containers.clone());
                return ans;
            } else {
                return BTreeSet::new();
            }
        }
        // no cached solution
        let mut combos = BTreeSet::new();
        for container in containers {
            if container.volume > volume {
                continue;
            }
            let sub_volume = volume - container.volume;
            let mut remaining = containers.clone();
            remaining.remove(container);
            let sub_combos = self.combinations(sub_volume, &remaining);
            for mut sub_combo in sub_combos {
                sub_combo.insert(container.clone());
                combos.insert(sub_combo);
            }
            // println!("Final combos: {:?}", combos);
        }
        self.0.insert((volume, containers.clone()), combos.clone());
        return combos;
    }
}

#[test]
fn base_case() {
    let mut combinator = Combinator::new();
    let available: BTreeSet<_> = vec![
        Container { id: 0, volume: 20 },
        Container { id: 1, volume: 15 },
        Container { id: 2, volume: 10 },
        Container { id: 3, volume: 5 },
        Container { id: 4, volume: 5 },
    ]
    .into_iter()
    .collect();
    let combos = combinator.combinations(25, &available);
    for combo in combos.iter() {
        println!("{:?}", combo)
    }
    println!("{:?}", combos.len());
}

#[test]
fn part1() {
    let containers: BTreeSet<_> = INPUT
        .lines()
        .map(|c| c.parse().unwrap())
        .enumerate()
        .map(|(id, volume)| Container { id, volume })
        .collect();
    let mut combinator = Combinator::new();
    let combos = combinator.combinations(150, &containers);
    println!("{:?}", combos.len());
}
