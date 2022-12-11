
pub fn matches_help(args: &Vec<String>) -> bool {
    args.iter()
        .filter(|&x| x == &"--help".to_string() || x == &"-h".to_string())
        .count()
        > 0
}
