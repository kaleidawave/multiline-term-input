use std::io;

/// Reads string where if shift is pressed while new line then keeps reading.
/// Returns the length of string read in
#[cfg(target_os = "windows")]
pub fn read_string(stdin: &mut std::io::Stdin, buf: &mut String) -> usize {
    use winconsole::input::{is_key_down, KeyCode};

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

#[cfg(not(target_os = "windows"))]
pub fn read_string(stdin: &mut std::io::Stdin, buf: &mut String) -> usize {
    if let Ok(count) = stdin.read_line(buf) {
        count
    } else {
        0
    }
}
