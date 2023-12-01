use helpers::load_input;

mod helpers;

fn main() {
    // Load input
    let input = load_input();
    let mut line_score: Vec<usize> = Vec::new();
    for line in &input {
        let mut chunk_open: Vec<char> = Vec::new();
        let mut has_error = false;
        'line_chek: for chr in line.chars() {
            match chr {
                '(' | '[' | '{' | '<' => chunk_open.push(chr),
                _ => {
                    let prev = (chunk_open.len() - 1) as isize;
                    has_error = if prev >= 0 {
                        check_char(chr, chunk_open[prev as usize])
                    } else {
                        check_char(chr, 'n')
                    };
                    if has_error {
                        break 'line_chek;
                    }
                    chunk_open.remove(prev as usize);
                }
            }
        }
        if !has_error {
            line_score.push(complete_score(&chunk_open));
        }
    }
    let middle = line_score.len() / 2;
    line_score.sort_unstable();
    println!("{}", line_score[middle]);
}

fn check_char(c: char, target: char) -> bool {
    !matches!(
        (c, target),
        (')', '(') | (']', '[') | ('}', '{') | ('>', '<')
    )
}

fn complete_score(chunk_open: &[char]) -> usize {
    let mut res = 0;
    for chr in chunk_open.iter().rev() {
        match chr {
            '(' => res = res * 5 + 1,
            '[' => res = res * 5 + 2,
            '{' => res = res * 5 + 3,
            '<' => res = res * 5 + 4,
            _ => panic!("Error!"),
        }
    }
    res
}
