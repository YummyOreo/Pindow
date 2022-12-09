use crate::error::config::SetConfigArgumentError;

pub fn matches(args: &Vec<String>) -> Option<usize> {
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
            match args[pointer + 1].parse::<usize>() {
                Ok(num) => {
                    return Some(num  - 1);
                },
                _ => {
                    panic!("{}", SetConfigArgumentError{
                        problem_word: args[pointer + 1].clone()
                    })
                }
            }
        }
        pointer += 1;
    }
    None
}
