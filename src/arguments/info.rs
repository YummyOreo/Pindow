pub fn matches_help(args: &[String]) -> bool {
    args.iter()
        .filter(|&x| x == &"--help".to_string() || x == &"-h".to_string())
        .count()
        > 0
}

pub fn matches_path(args: &[String]) -> bool {
    args.iter()
        .filter(|&x| x == &"--get-path".to_string())
        .count()
        > 0
}
