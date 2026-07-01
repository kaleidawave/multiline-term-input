#[derive(Debug, Default, PartialEq, Eq)]
pub enum Mode {
	#[default]
	ReturnOnUndecoratedNewLine,
	ReturnOnShiftNewLine,
}

#[derive(Debug, Default)]
pub struct Options<'a> {
	pub prefix: &'a str,
	pub return_mode: Mode,
	pub prompt: &'a str,
}

/// Reads string where if shift is pressed while new line then keeps reading.
/// Returns the length of string read in
#[cfg(target_os = "windows")]
pub fn read_to_string(options: Options<'_>) -> std::io::Result<String> {
	use std::io::{self, Write};
	use winconsole::input::{is_key_down, KeyCode};

	let stdin = io::stdin();
	let mut buf = String::new();

	let Options { prefix, return_mode, prompt } = options;

	write!(&mut stdout, "{prompt}");
	Write::flush(&mut io::stdout())?;

	while let Ok(count) = stdin.read_line(buf) {
		if is_key_down(KeyCode::Shift) == matches!(return_mode, Mode::ReturnOnUndecoratedNewLine) {
			write!(&mut stdout, "{prefix}");
			Write::flush(&mut std::io::stdout());
		} else {
			break;
		}
	}
	Ok(buf)
}

#[cfg(not(target_os = "windows"))]
pub fn read_to_string(options: Options<'_>) -> std::io::Result<String> {
	use crossterm::terminal;
	use std::io::{self, Read, Write};

	let Options { prefix, return_mode, prompt } = options;

	let mut stdout = io::stdout();
	let mut buf = String::new();

	if !prompt.is_empty() {
		writeln!(&mut stdout, "{prompt}")?;
		Write::flush(&mut stdout)?;
	}

	if !prefix.is_empty() {
		write!(&mut stdout, "{prefix}")?;
		Write::flush(&mut stdout)?;
		buf.push_str(prefix);
	}

	terminal::enable_raw_mode()?;

	let stdin = io::stdin();
	let mut bytes = stdin.bytes();
	while let Some(byte) = bytes.next() {
		let byte = if let Ok(byte) = byte {
			byte
		} else {
			break;
		};

		match byte {
			b'\n' => {
				// print return to get to the start
				if return_mode == Mode::ReturnOnShiftNewLine {
					write!(&mut stdout, "\r\n")?;
					Write::flush(&mut stdout)?;
					break;
				}

				write!(&mut stdout, "\r\n{prefix}")?;
				Write::flush(&mut stdout)?;
				buf.push('\n');
				buf.push_str(prefix);
			}
			b'\r' => {
				if return_mode == Mode::ReturnOnUndecoratedNewLine {
					write!(&mut stdout, "\r\n")?;
					Write::flush(&mut stdout)?;
					break;
				}

				write!(&mut stdout, "\r\n{prefix}")?;
				Write::flush(&mut stdout)?;
				buf.push('\n');
				buf.push_str(prefix);
			}
			0x7f => {
				if let Some(_char) = buf.pop() {
					write!(&mut stdout, "\x08 \x08")?;
					Write::flush(&mut stdout)?;
				}
			}
			0x1b => {
				let mut sequence: Vec<_> = Vec::new();
				for _ in 0..9 {
					let byte = bytes.next();
					if let Some(Ok(byte)) = byte {
						sequence.push(byte);
					} else {
						break;
					}
				}
				// TODO more sequences
				if let b"[27;2;13~" = sequence.as_slice() {
					if return_mode == Mode::ReturnOnShiftNewLine {
						break;
					}
					write!(&mut stdout, "\r\n{prefix}")?;
					Write::flush(&mut stdout)?;
					buf.push('\n');
					buf.push_str(prefix);
				} else {
					// write!("\r\nunrecognised sequence {buf:?}\r\n");
					// Write::flush(&mut stdou?;
				}
			}
			240 => {
				let mut acc: u32 = 0b1111_0000; // byte as u32;
				for byte in bytes.by_ref() {
					let Ok(byte) = byte else {
						// write!(&mut stdout, "no byte\r\n");
						break;
					};
					acc <<= 8;
					acc |= u32::from(byte);
					if byte & 0b1000_0000 != 0b1000_0000 {
						// write!(&mut stdout, "end byte {byte:08b}\r\n");
						break;
					}
				}
				if let Some(c) = char::from_u32(acc) {
					write!(&mut stdout, "{c}")?;
					Write::flush(&mut stdout)?;
					buf.push(c);
				} else {
					// for byte in acc.to_be_bytes() {
					// 	write!(&mut stdout, "{byte:08b} ");
					// }
					// write!(&mut stdout, "\r\n");
					// Write::flush(&mut stdou?;
					// TODO
				}
			}
			// ...
			194 => {}
			byte => {
				if let Some(c) = char::from_u32(u32::from(byte)) {
					write!(&mut stdout, "{c}")?;
					Write::flush(&mut stdout)?;
					buf.push(c);
				} else {
					break;
				}
			}
		}
	}

	terminal::disable_raw_mode()?;

	Ok(buf)
}
