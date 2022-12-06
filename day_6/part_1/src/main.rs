use std::collections::{HashMap};
use std::collections::hash_map::{Entry};
use std::io::BufRead;
use utils::get_file_reader;

fn main() -> std::io::Result<()> {
    let reader = get_file_reader("src/assets/input.txt");

    let mut current_unique_chars_in_buf_sequence = 0;
    let mut total_count_to_find_start_of_packet_sequence = 0;

    const UNIQUE_CHARS_NEEDED_FOR_START_OF_PACKET_MARKER: usize = 4;

    let mut packet_marker_detected = false;

    let mut cache = HashMap::new();

    for line in reader.lines() {
        for character in line.unwrap().chars() {
            let buf_is_primed = total_count_to_find_start_of_packet_sequence > (UNIQUE_CHARS_NEEDED_FOR_START_OF_PACKET_MARKER - 1);

            if buf_is_primed {
                match cache.entry(character) {
                    Entry::Occupied(_) => {
                        cache.clear();
                        current_unique_chars_in_buf_sequence = 0;
                    }
                    Entry::Vacant(e) => {
                        e.insert(character);
                        current_unique_chars_in_buf_sequence += 1;
                        if current_unique_chars_in_buf_sequence
                            == UNIQUE_CHARS_NEEDED_FOR_START_OF_PACKET_MARKER {
                            packet_marker_detected = true;
                            break;
                        }
                    }
                };
            } else {
                cache.insert(character, character);
                current_unique_chars_in_buf_sequence += 1;
            }
            total_count_to_find_start_of_packet_sequence += 1;
        }
    }

    if packet_marker_detected {
        println!("`Start-of-Packet` marker detected at byte: {}", total_count_to_find_start_of_packet_sequence);
    } else {
        println!("`Start-of-Packet` marker not found");
    }
    Ok(())
}
