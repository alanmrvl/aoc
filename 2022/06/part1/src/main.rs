fn main() {
    let path = std::path::Path::new("input");
    let stream = std::fs::read_to_string(path).unwrap();

    let mut freq = ['0'; 3];

    for (i, curr) in stream.chars().enumerate() {
        if i > 2 && no_dupes(&freq, curr) {
            println!("{}", i + 1);
            // println!("{:?} {}", freq, curr);
            return;
        } else {
            freq.copy_within(1.., 0);
            freq[2] = curr;
        }
    }
}

fn no_dupes(freq: &[char], curr: char) -> bool {
    let mut dupes: u32 = 0; // bitwise check

    let bit_position = (curr as u8) - ('a' as u8) + 1;
    let mask: u32 = 1 << bit_position;

    dupes = dupes | mask;

    for el in freq {
        let bit_position = (*el as u8) - ('a' as u8) + 1;
        let mask: u32 = 1 << bit_position;
        let result = dupes & mask;

        if result > 0 {
            return false;
        }
        dupes = dupes | mask;
    }

    return true;
}
