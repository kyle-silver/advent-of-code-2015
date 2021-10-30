use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

const INPUT: &str = include_str!("res/17.txt");

type Volume = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Container {
    id: usize,
    volume: Volume,
}

type Containers = BTreeSet<Container>;

struct Combinator {
    cache: BTreeMap<Volume, BTreeMap<Containers, BTreeSet<Containers>>>,
}

impl Combinator {
    fn new() -> Combinator {
        Combinator {
            cache: BTreeMap::new(),
        }
    }

    fn insert(&mut self, volume: Volume, containers: Containers, value: BTreeSet<Containers>) {
        match self.cache.entry(volume) {
            Entry::Vacant(entry) => {
                let mut sub_map = BTreeMap::new();
                sub_map.insert(containers, value);
                entry.insert(sub_map);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(containers, value);
            }
        };
    }

    fn combinations(&mut self, volume: Volume, containers: &Containers) -> BTreeSet<Containers> {
        if let Some(vc) = self.cache.get(&volume) {
            if let Some(combos) = vc.get(containers) {
                return combos.clone();
            }
        }
        let mut combos = BTreeSet::new();
        for container in containers {
            if container.volume > volume {
                continue;
            }
            let sub_volume = volume - container.volume;
            if sub_volume == 0 {
                let mut ans = BTreeSet::new();
                ans.insert(container.clone());
                combos.insert(ans);
                continue;
            }
            let mut remaining = containers.clone();
            remaining.remove(container);
            let sub_combos = self.combinations(sub_volume, &remaining);
            for mut sub_combo in sub_combos {
                sub_combo.insert(container.clone());
                combos.insert(sub_combo.clone());
            }
        }
        self.insert(volume, containers.clone(), combos.clone());
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
fn problem() {
    let containers: BTreeSet<_> = INPUT
        .lines()
        .map(|c| c.parse().unwrap())
        .enumerate()
        .map(|(id, volume)| Container { id, volume })
        .collect();
    let mut combinator = Combinator::new();
    let combos = combinator.combinations(150, &containers);
    println!("Day 17, part 1: {}", combos.len());
    assert_eq!(1638, combos.len());
    let min_containers = combos.iter().map(BTreeSet::len).min().unwrap();
    let min_container_count = combos
        .iter()
        .map(BTreeSet::len)
        .filter(|len| *len == min_containers)
        .count();
    println!("Day 17, part 2: {}", min_container_count);
}
