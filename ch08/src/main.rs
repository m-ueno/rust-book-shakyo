mod repl;

fn main() {
    let mut env = repl::Env::new();
    env.repl();
}
