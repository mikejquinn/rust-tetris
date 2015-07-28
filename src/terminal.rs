use libc::{c_ulong, c_int, c_uchar};

// While this code is pretty generic, I've pulled much of this code from another Rust Tetris implementation:
// https://github.com/jankes/tetris1/blob/master/tetris1.rs

// Linux specifc termios structure definition
//
// Since we don't actually access any of the fields individually, and instead just
// pass around termios as a "black box", this will probably work for other platforms
// as long their struct termios is smaller than Linux's. For example, Mac OS omits the
// c_line field and only has 20 control characters.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(C)]
struct termios {
    c_iflag:  c_ulong,        // input flags
    c_oflag:  c_ulong,        // output flags
    c_cflag:  c_ulong,        // control flags
    c_lflag:  c_ulong,        // local flags
    c_cc:    [c_uchar; 20],   // control chars
    c_ispeed: c_ulong,        // input speed
    c_ospeed: c_ulong,        // output speed
}

extern {
    fn tcgetattr(filedes: c_int, termptr: *mut termios) -> c_int;
    fn tcsetattr(filedes: c_int, opt: c_int, termptr: *const termios) -> c_int;
    fn cfmakeraw(termptr: *mut termios);
}

fn get_terminal_attr() -> (termios, c_int) {
    unsafe {
        let ios = &mut termios {
            c_iflag:  0,
            c_oflag:  0,
            c_cflag:  0,
            c_lflag:  0,
            c_cc:     [0; 20],
            c_ispeed: 0,
            c_ospeed: 0
        };

        // first parameter is file descriptor number, 0 ==> standard input
        let err = tcgetattr(0, ios as *mut termios);

        return (*ios, err);
    }
}

fn make_raw(ios: &termios) -> termios {
    unsafe {
        let mut ios = *ios;
        cfmakeraw(&mut ios);
        return ios;
    }
}

fn set_terminal_attr(ios: &termios) -> c_int {
    unsafe {
        // first paramter is file descriptor number, 0 ==> standard input
        // second paramter is when to set, 0 ==> now
        return tcsetattr(0, 0, ios as *const termios);
    }
}

pub struct TerminalRestorer {
    ios: termios
}

impl Drop for TerminalRestorer {
    fn drop(&mut self) {
        set_terminal_attr(&self.ios);
    }
}

pub fn set_terminal_raw_mode() -> TerminalRestorer {
    let (original_ios, err) = get_terminal_attr();
    if err != 0 {
        panic!("failed to get terminal settings");
    }

    let raw_ios = make_raw(&original_ios);
    let err = set_terminal_attr(&raw_ios);
    if err != 0 {
        panic!("failed to switch terminal to raw mode");
    }

    TerminalRestorer {
        ios: original_ios
    }
}
