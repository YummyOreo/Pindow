pub fn matches(args: &[String]) -> bool {
    args.iter()
        .filter(|&x| x == &"--debug".to_string() || x == &"-d".to_string())
        .count()
        > 0
}
