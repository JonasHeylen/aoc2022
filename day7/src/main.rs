use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

type Path = Vec<String>;
type Fs = HashMap<Path, usize>;

const TOTAL_DISK_SPACE: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

fn main() -> io::Result<()> {
    let fs = build_filesystem("input.txt")?;
    let result_part1 = run_part1(&fs);
    let result_part2 = run_part2(&fs);
    println!("Part 1: {result_part1} - Part 2: {result_part2}");
    Ok(())
}

fn run_part1(fs: &Fs) -> usize {
    fs.iter()
        .filter_map(|(_, size)| if *size <= 100000 { Some(size) } else { None })
        .sum()
}

fn run_part2(fs: &Fs) -> usize {
    let used_disk_space = fs.get(&vec!["/".to_string()]).unwrap();
    let free_disk_space = TOTAL_DISK_SPACE - used_disk_space;
    let space_to_delete = SPACE_NEEDED - free_disk_space;
    let mut large_enough_dir_sizes: Vec<_> = fs.values().filter(|&s| *s >= space_to_delete).collect();
    large_enough_dir_sizes.sort();
    *large_enough_dir_sizes[0]
}

fn build_filesystem(filename: &str) -> io::Result<Fs> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut cwd: Path = Vec::new();
    let mut fs: Fs = HashMap::new();

    reader.lines().flatten().for_each(|cmd| match cmd {
        cd if cd.starts_with("$ cd") => {
            let dir = &cd[5..];
            if dir == ".." {
                cwd.pop();
            } else {
                cwd.push(dir.to_string());
            }
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
                    .and_modify(|s| *s += size)
                    .or_insert(size);
            }
            ()
        }
        _ => (),
    });

    Ok(fs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let fs = build_filesystem("input_test.txt").unwrap();
        assert_eq!(run_part1(&fs), 95437);
    }

    #[test]
    fn test_input_part2() {
        let fs = build_filesystem("input_test.txt").unwrap();
        assert_eq!(run_part2(&fs), 24933642);
    }
}
