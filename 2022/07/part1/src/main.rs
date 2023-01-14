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
            Err(ParseLogItemError { line: s.to_string() })            
        };
    }
}

fn main() {
    let lines = std::fs::read_to_string("input").unwrap();

    for line in lines.lines() {
        let log_item = line.parse::<LogItem>().unwrap();
        dbg!(log_item);
    }
}
