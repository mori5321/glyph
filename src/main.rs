use libc::STDIN_FILENO;
use std::io::{self, Write};
use termios::*;

fn enable_raw_mode() {
    let mut termios = Termios::from_fd(STDIN_FILENO).unwrap();
    tcgetattr(STDIN_FILENO, &mut termios).unwrap();

    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
    termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_oflag &= !(OPOST);
    termios.c_cflag |= CS8;
    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;

    match tcsetattr(STDIN_FILENO, TCSANOW, &termios) {
        Ok(_) => return,
        Err(err) => panic!("Failed: {:?}", err),
    }
}

fn main() {
    enable_raw_mode();

    let mut writer = io::BufWriter::new(io::stdout());

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();

        writer.write(&buffer.as_bytes()).unwrap();
        println!("{}", buffer);
    }
}
