pub fn matches(args: &Vec<String>) -> Option<usize> {
    if args.iter().filter(|&x| x == &"-c".to_string() || x == &"--config".to_string()).count() == 0  {
        return None;
    }
    let mut pointer = 0;
    while pointer < args.iter().count() {
        if args[pointer] == "-c".to_string() || args[pointer] == "--config".to_string() {
            return Some(args[pointer + 1].parse::<usize>().unwrap() - 1);
        }
        pointer += 1;
    }
    None
}
