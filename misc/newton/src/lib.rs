pub mod newton;

#[cfg(test)]
mod tests {

    use super::newton;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_draw() {
        newton::draw();
    }
}
