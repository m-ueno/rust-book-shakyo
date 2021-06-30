use std::collections::HashMap;
use std::hash::Hash;

fn mean<T: std::convert::Into<f64>>(xs: Vec<T>) -> f64 {
    let mut current = 0.;
    let n = xs.len() as f64;

    for x in xs {
        current += x.into() / n; // ?
    }

    current
}

fn median<T: Ord + Copy>(xs: Vec<T>) -> Option<T> {
    if xs.len() == 0 {
        return None;
    }
    let mut vec = xs.clone();
    vec.sort();

    Some(vec[vec.len() / 2])
}

fn mode<T: Eq + Hash + Copy>(xs: Vec<T>) -> Option<T> {
    if xs.len() == 0 {
        return None;
    }

    let mut map = HashMap::new();
    for x in &xs {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }

    let mut max_item = xs.first();
    let mut max_count = 0;
    for (k, v) in map.iter() {
        if *v > max_count {
            max_item = Some(k);
            max_count = *v;
        }
    }

    if let Some(v) = max_item {
        return Some(*v);
    } else {
        panic!("never reach here, but need to pass compile");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mean() {
        let cases = vec![vec![], vec![1, 2, -4, 1]];

        for case in cases {
            assert_eq!(mean(case), 0.);
        }
    }

    #[test]
    fn test_median() {
        let cases = vec![
            (vec![], None),
            (vec![1, 2, 2, 5, 1], Some(2)),
            // (vec![10., 30., 20.], 20.),
        ];

        for case in cases {
            let (got, want) = (median(case.0), case.1);
            assert_eq!(got, want);
        }
    }

    #[test]
    fn test_mode() {
        let cases = vec![
            (vec![], None),
            (vec![1, 2, 2, 5, 1, 3, 2], Some(2)),
            //
        ];
        for case in cases {
            let (got, want) = (mode(case.0), case.1);
            assert_eq!(got, want);
        }
    }
}
