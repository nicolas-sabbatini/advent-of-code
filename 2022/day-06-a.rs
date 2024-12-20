use helpers::load_input;

mod helpers;

const PACKET_MARKER_SIZE: usize = 4;

fn is_marker(buff: &[char]) -> bool {
    for i in 0..PACKET_MARKER_SIZE - 1 {
        for j in i + 1..PACKET_MARKER_SIZE {
            if buff[i] == buff[j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    // Load input
    let input = load_input();
    let datastream_buffer = input[0].chars().collect::<Vec<char>>();
    let mut i = PACKET_MARKER_SIZE;
    while i < datastream_buffer.len() {
        if is_marker(&datastream_buffer[i - PACKET_MARKER_SIZE..i]) {
            break;
        }
        i += 1;
    }
    println!("{i:?}");
}
