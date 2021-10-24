use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct ComboSummator(HashMap<(u64, usize), Vec<Vec<u64>>>);

impl ComboSummator {
    fn new() -> ComboSummator {
        ComboSummator(HashMap::new())
    }

    fn combos(&mut self, target: u64, n: usize) -> Vec<Vec<u64>> {
        // base cases
        if n == 0 {
            self.0.insert((target, n), vec![vec![0]]);
            return self.0.get(&(target, n)).unwrap().clone();
        }
        if n == 1 {
            self.0.insert((target, n), vec![vec![target]]);
            return self.0.get(&(target, n)).unwrap().clone();
        }
        // memoized solution
        if let Some(combos) = self.0.get(&(target, n)) {
            return combos.clone();
        }
        // construct new case
        let combos: Vec<Vec<u64>> = (0..target)
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

#[test]
fn test() {
    let mut combo_summator = ComboSummator::new();
    println!("{:?}", combo_summator.combos(100, 4).len());
}
