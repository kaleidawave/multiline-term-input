use std::io;
use winconsole::input::{is_key_down, KeyCode};

/// Reads string where if shift is pressed while new line then keeps app
pub fn read_string(stdin: &mut io::Stdin, buf: &mut String) -> usize {
    let mut total_count = 0;
    while let Ok(count) = stdin.read_line(buf) {
        total_count += count;
        if is_key_down(KeyCode::Shift) {
            print!("... ");
            io::Write::flush(&mut io::stdout()).unwrap();
        } else {
            break;
        }
    }
    total_count
}
