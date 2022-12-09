fn is_marker(packet:&str,criteria: usize)->bool{
    let set = packet.chars().collect::<std::collections::HashSet<char>>();
    match set.len() {
        length if length == criteria => true,
        _ => false
    }
}
fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let num_chars = content.len();
    let mut found_marker_indices = Vec::new();
    for i in 0..=num_chars-4 {
        let packet = content.get(i..i+4).unwrap();
        if is_marker(packet,4) {
            found_marker_indices.push(i);
        }
    }
    let characters_processed = found_marker_indices.first().unwrap() + 4;
    println!("part one: {}", characters_processed);

    let mut found_msg_indices = Vec::new();
    for i in 0..=num_chars-14 {
        let packet = content.get(i..i+14).unwrap();
        if is_marker(packet,14) {
            found_msg_indices.push(i);
        }
    }
    let characters_processed = found_msg_indices.first().unwrap() + 14;
    println!("part two: {}", characters_processed);
}