use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Num {
    I32(i32),
    F64(f64),
}

use Num::*;

impl Add for Num {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match self {
            I32(i) => {
                if let I32(j) = other {
                    I32(i + j)
                } else {
                    panic!("oh no");
                }
            }
            F64(i) => {
                if let F64(j) = other {
                    F64(i + j)
                } else {
                    panic!("noo")
                }
            }
        }
    }
}

fn sum(xs: &Vec<Num>) -> Num {
    if xs.len() == 0 {
        F64(0.)
    } else {
        xs.clone().into_iter().reduce(|a, b| a + b).unwrap()
    }
}

// fn assert_almost_equal<T: PartialEq + Into<f64>>(x: T, y: T) -> bool {
//     (x.into() - y.into()).abs() < 1e-6
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let cases: Vec<(Vec<Num>, Num)> = vec![
            (vec![], Num::F64(0.)),
            (
                vec![1, 2, 3, 4]
                    .iter()
                    .map(|&n| Num::I32(n))
                    .collect::<Vec<Num>>(),
                Num::I32(10),
            ),
            (
                vec![1.0, 0.5, 0.25]
                    .into_iter()
                    .map(|n| Num::F64(n))
                    .collect::<Vec<Num>>(),
                Num::F64(1.75),
            ),
            //
        ];
        for case in cases {
            let (got, want) = (sum(&case.0), case.1);
            assert_eq!(got, want);
        }
    }
}
