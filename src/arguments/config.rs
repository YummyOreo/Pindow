fn get_start_config(args: &Vec<String>, pointer: usize) -> usize {
    if let Ok(num) = args[pointer + 1].parse::<usize>() {
        return num - 1;
    }
    panic!("\"{}\" is not a number.", args[pointer + 1].clone())
}

pub fn matches_start(args: &Vec<String>) -> Option<usize> {
    if args
        .iter()
        .filter(|&x| x == &"-c".to_string() || x == &"--config".to_string())
        .count()
        == 0
    {
        return None;
    }

    let mut pointer = 0;
    while pointer < args.iter().count() {
        if args[pointer] == "-c".to_string() || args[pointer] == "--config".to_string() {
            return Some(get_start_config(args, pointer));
        }

        pointer += 1;
        continue;
    }
    None
}

pub fn matches_path(args: &Vec<String>) -> Option<String> {
    if args
        .iter()
        .filter(|&x| x == &"-p".to_string() || x == &"--path".to_string())
        .count()
        == 0
    {
        return None;
    }

    let mut pointer = 0;
    while pointer < args.iter().count() {
        if args[pointer] == "-p".to_string() || args[pointer] == "--path".to_string() {
            return Some(args[pointer + 1].clone());
        }
        pointer += 1;
    }
    None
}
