fn to_latin(input: &str) -> String {
    match first_vowel(input) {
        Some(0) => input.to_string() + "hay",
        Some(idx) => {
            let xs: Vec<char> = input.chars().collect();
            let s1 = xs[..idx].iter().collect::<String>();
            let s2 = xs[idx..].iter().collect::<String>();

            format!("{}{}ay", s2, s1)
        }
        None => input.to_string(),
        _ => panic!("oh no"),
    }
}

fn first_vowel(input: &str) -> Option<usize> {
    input.find(&['a', 'o', 'e', 'i', 'u'][..])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_latin() {
        let cases = vec![
            ("first", "irstfay"),
            ("apple", "applehay"),
            //
            ("", ""),
            ("aaa", "aaahay"),
            ("str", "str"),
            ("mess", "essmay"),
            ("star", "arstay"),
            ("bitch", "itchbay"),
        ];
        for case in cases {
            let (got, want) = (to_latin(case.0), case.1);
            assert_eq!(got, want); // assert_eqが&strとString型変換してくれる
        }
    }
}
