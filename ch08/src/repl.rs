use std::collections::HashMap;
use std::io;
use std::io::Write;

use regex::Regex;

enum Action {
    AddPerson(String, String),
    ListAll,
    ListSection(String),
    None,
}

pub struct Env {
    map: HashMap<String, Vec<String>>,
}

impl Env {
    pub fn new() -> Self {
        let mut db = Env {
            map: HashMap::new(),
        };
        db.map.insert(
            String::from("Sales"),
            vec![String::from("Alice"), String::from("Bob")],
        );
        db
    }

    fn read() -> Action {
        let re_add = Regex::new(r"add (?P<name>.*) to (?P<section>.*)").unwrap();
        let re_list_all = Regex::new(r"list all").unwrap();
        let re_list_section = Regex::new(r"list (?P<section>.*)").unwrap();

        let mut input = String::new();
        print!("input command:\n> ");
        io::stdout().flush().expect("oh no");
        io::stdin().read_line(&mut input).expect("failed");

        input = input.trim().to_string();

        if re_add.is_match(input.as_str()) {
            let capture = re_add.captures(input.as_str()).unwrap();
            let name = capture.name("name").unwrap().as_str();
            let section = capture.name("section").unwrap().as_str();
            return Action::AddPerson(String::from(name), String::from(section));
        } else if re_list_all.is_match(input.as_str()) {
            return Action::ListAll;
        }
        Action::None
    }

    fn eval(&mut self, a: Action) -> String {
        match a {
            Action::AddPerson(name, section) => {
                self.map.entry(section.clone()).or_insert(vec![]);
                self.map.get_mut(&section).unwrap().push(name.clone());

                format!("added {} to {}", name, section)
            }
            Action::ListAll => {
                let mut s = String::new();
                for (k, vs) in self.map.iter() {
                    s.push_str(&format!("## section={}\n", k)[..]);
                    for v in vs {
                        s.push_str(&format!("* {}\n", v)[..]);
                    }
                }
                s
            }
            _ => format!("not implemented"),
        }
    }

    fn print(msg: String) {
        for line in msg.lines() {
            println!("| {}", line);
        }
    }

    pub fn repl(&mut self) {
        loop {
            Env::print(self.eval(Env::read()));
        }
    }
}
