use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// *** Test cases ***

// *** Macros ****
#[macro_export]
macro_rules! print {
  //The $crate variable ensures that the macro also works from outside the std crate by expanding to std when it's used in other crates.
  ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
  () => ($crate::print!("\n"));
  ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}


#[test_case]
fn test_println_output() {
  let s = "Some test string that fits on a single line";
  println!("{}", s);
  for (i, c) in s.chars().enumerate() {
    let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
    assert_eq!(char::from(screen_char.ascii_character), c);
  }
}

#[test_case]
fn test_println_simple() {
  println!("test_println_simple output");
  // if not panci => success
}

#[test_case]
fn test_println_many() {
  for i in 0..200 {
    println!("test_println_many({}) output", i);
  }
}


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use core::fmt::Write;
  WRITER.lock().write_fmt(args).unwrap();
}

// Lazy_static will initialize the code inside once when it's accessed the first time
lazy_static! { 
  // Global interface for Writer
  // In order to ensure we will not encouter data race while accessing the writer
  // we often use mutex to lock and unlock. But our OS doesn't support any kind of blocking or
  // threading. So we're going to use a very basic kind of mutex called : spinlock that requires
  // no OS features.
  // Spinlock will just a loop and burns CPU time and constantly check if the lock is unlocked.
  // The it'll return to the original process
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    return ColorCode((background as u8) << 4 | (foreground as u8));
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


#[repr(transparent)]
struct Buffer {
  // rust will automatically discard the Buffer bc we only write to it and doesn't read from it
  // It's from the optimize feature of Rust. In order to avoid it we use Volatile
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// *** Writer ***
pub struct Writer {
  column_position: usize,
  color_code: ColorCode,
  // 'static to denote the lifetime of buffer is valid for the whole program run time
  buffer: &'static mut Buffer,
}

impl Writer {
  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // printable ASCII byte or newline
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // not part of printable ASCII range
        _ => self.write_byte(0xfe),
      }

    }
  }

  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar{
          ascii_character: byte,
          color_code,
        });

        self.column_position += 1;
      }
    }
  }

  fn new_line(&mut self) {
    // shift everything one line up
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH { 
        let char = self.buffer.chars[row][col].read();
        self.buffer.chars[row-1][col].write(char);
      }
    }
    self.clear_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
  }

  fn clear_row(&mut self, row: usize) {
    let blank = ScreenChar{
      ascii_character: b' ',
      color_code: self.color_code,
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }
}

// implt Write trait for writer
// this way we can use write!, writeln! from fmt
impl fmt::Write for Writer{
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    return Ok(());
  }
}

