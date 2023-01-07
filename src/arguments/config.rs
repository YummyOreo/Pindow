fn get_start_config(args: &[String], pointer: usize) -> usize {
    if let Ok(num) = args[pointer + 1].parse::<usize>() {
        return num - 1;
    }
    panic!("\"{}\" is not a number.", args[pointer + 1])
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
    while pointer < args.len() {
        if args[pointer] == *"-c" || args[pointer] == *"--config" {
            return Some(get_start_config(args, pointer));
        }

        pointer += 1;
        continue;
    }
    None
}

pub fn matches_path(args: &[String]) -> Option<String> {
    if args
        .iter()
        .filter(|&x| x == &"-p".to_string() || x == &"--path".to_string())
        .count()
        == 0
    {
        return None;
    }

    let mut pointer = 0;
    while pointer < args.len() {
        if args[pointer] == *"-p" || args[pointer] == *"--path" {
            return Some(args[pointer + 1].clone());
        }
        pointer += 1;
    }
    None
}
