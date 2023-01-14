use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
enum LogItem {
    CmdChangeDirectory(String),
    CmdListDirectory,
    Directory(String),
    File(String, u32),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLogItemError {
    line: String,
}

impl FromStr for LogItem {
    type Err = ParseLogItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();

        return if parts[0] == "$" && parts[1] == "cd" {
            Ok(LogItem::CmdChangeDirectory(parts[2].to_string()))
        } else if parts[0] == "$" && parts[1] == "ls" {
            Ok(LogItem::CmdListDirectory)
        } else if parts[0] == "dir" {
            Ok(LogItem::Directory(parts[1].to_string()))
        } else if let Ok(num) = parts[0].parse::<u32>() {
            Ok(LogItem::File(parts[1].to_string(), num))
        } else {
            Err(ParseLogItemError {
                line: s.to_string(),
            })
        };
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let max = 100_000;
    let mut total = 0;

    let mut dir: Vec<u32> = vec![0];

    for line in lines.skip(1) {
        let line = line.unwrap();
        let log_item = line.parse::<LogItem>().unwrap();

        match log_item {
            LogItem::CmdChangeDirectory(dir_name) => {
                if dir_name == ".." {
                    // Since apparently this is just DFS, we can append all the results
                    // to the parent when we go back up a directory, as we will never need
                    // to go to this directory again.
                    let curr = dir.pop().unwrap();
                    let parent = dir.last_mut().unwrap();
                    *parent += curr;

                    if curr < max {
                        total += curr;
                    }
                } else {
                    // We're going _down into_ directories exactly once (because DFS),
                    // so we can just assume all we need to do is create a new zeroed
                    // count for the directory.
                    dir.push(0);
                }
            }
            LogItem::File(_, size) => {
                let curr = dir.last_mut().unwrap();
                *curr += size;
            }
            _ => {}
        }
    }

    // cleanup remaining directories
    while !dir.is_empty() {
        let last = dir.pop().unwrap();

        if last < max {
            total += last;
        }

        if let Some(parent) = dir.last_mut() {
            *parent += last;
        }
    }

    println!("total {}", total);
}
