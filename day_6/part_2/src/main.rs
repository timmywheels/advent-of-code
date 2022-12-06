use std::collections::{HashSet};
use std::io::BufRead;
use utils::get_file_reader;

fn main() -> std::io::Result<()> {
    let reader = get_file_reader("src/assets/input.txt");

    let mut total_count_to_find_start_of_packet_sequence = 0;

    const UNIQUE_CHARS_NEEDED_FOR_START_OF_PACKET_MARKER: usize = 14;

    let mut packet_marker_detected = false;

    let mut cache = HashSet::new();

    for line in reader.lines() {
        let curr_line = line.unwrap();
        // use fast & slow pointers
        for (slow, character) in curr_line.chars().enumerate() {
            // add initial char to cache
            cache.insert(character);

            // set length to `1` to account for initial character
            // let mut length_of_sequence = 1;
            // look ahead with fast pointer to start building up sequence
            for fast in (slow + 1)..curr_line.chars().count() {
                let (_, fast_char) = curr_line.char_indices().nth(fast).unwrap();
                match cache.contains(&fast_char) {
                    true => {
                        // dupe value found in cache
                        // so reset cache and running count and goto next iteration
                        cache.clear();
                        break;
                    }
                    false => {
                        cache.insert(fast_char);
                        if cache.len() == UNIQUE_CHARS_NEEDED_FOR_START_OF_PACKET_MARKER {
                            packet_marker_detected = true;
                            break;
                        }
                    }
                }
            }

            if packet_marker_detected {
                break;
            }

            total_count_to_find_start_of_packet_sequence += 1;
        }
    }

    if packet_marker_detected {
        let byte_where_packet_sequence_completed = total_count_to_find_start_of_packet_sequence + UNIQUE_CHARS_NEEDED_FOR_START_OF_PACKET_MARKER;
        println!("`Start-of-Message` marker detected at byte: {}", byte_where_packet_sequence_completed);
    } else {
        println!("`Start-of-Message` marker not found");
    }
    Ok(())
}
