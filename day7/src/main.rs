use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

type Path = Vec<String>;

fn main() {
    let result_part1 = run_part1("input.txt").unwrap();
    let result_part2 = run_part2("input.txt").unwrap();
    println!("Part 1: {result_part1} - Part 2: {result_part2}");
}

fn run_part1(filename: &str) -> io::Result<usize> {
    run(filename)
}

fn run_part2(filename: &str) -> io::Result<usize> {
    run(filename)
}

fn run(filename: &str) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut cwd: Path = Vec::new();
    let mut fs: HashMap<Path, usize> = HashMap::new();

    reader.lines().flatten().for_each(|cmd| match cmd {
        cd if cd.starts_with("$ cd") => {
            println!("{cd}");
            let dir = &cd[5..];
            if dir == ".." {
                cwd.pop();
            } else {
                cwd.push(dir.to_string());
            }
            println!("{:?}", cwd);
        }
        file if file.chars().next().unwrap().is_ascii_digit() => {
            let size = file
                .split(' ')
                .next()
                .expect("Missing file size")
                .parse::<usize>()
                .expect("Invalid file size");
            let entry = fs.entry(cwd.clone());

            // update size of current folder
            entry.and_modify(|s| *s += size).or_insert(size);

            // update size of parent folders
            for i in 1..cwd.len() {
                fs.entry(Vec::from(&cwd[0..cwd.len() - i]))
                    .and_modify(|s| *s += size).or_insert(size);
            }
            ()
        }
        _ => (),
    });

    println!("{:?}", fs);

    Ok(fs
        .iter()
        .filter_map(|(_, size)| if *size <= 100000 { Some(size) } else { None })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1("input_test.txt").unwrap(), 95437);
    }

    #[test]
    fn test_input_2_part1() {
        assert_eq!(run_part1("input_test_2.txt").unwrap(), 99999);
    }

    #[test]
    fn test_real_input_part1() {
        assert!(run_part2("input.txt").unwrap() > 1624690);
    }
}
