fn main() {
    let path = std::path::Path::new("input");
    let stream = std::fs::read_to_string(path).unwrap();

    let mut freq = [0; 13];

    for (i, curr) in stream.as_bytes().iter().enumerate() {
        if i > 12 && no_dupes(&freq, curr) {
            println!("{}", i + 1);
            // println!("{:?} {}", freq, curr);
            return;
        } else {
            freq.copy_within(1.., 0);
            let last = freq.last_mut().unwrap();
            *last = *curr;
        }
    }
}

fn no_dupes(freq: &[u8], curr: &u8) -> bool {
    let mut dupes: u32 = 0; // bitwise check

    let bit_position = curr - b'a' + 1;
    let mask: u32 = 1 << bit_position;

    dupes = dupes | mask;

    for el in freq {
        let bit_position = *el - b'a' + 1;
        let mask: u32 = 1 << bit_position;
        let result = dupes & mask;

        if result > 0 {
            return false;
        }
        dupes = dupes | mask;
    }

    return true;
}
