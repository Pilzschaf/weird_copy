use std::{io, env};
use std::os::raw::c_char;
use std::ffi::CString;
use std::process;

static O_RDONLY: i32 = 0x0000;
static O_WRONLY: i32 = 0x0001;

static O_CREAT: i32 = 0o100;
static O_TRUNC: i32	= 0o1000;

extern "C" {
    fn open(pathname: *const c_char, flags: i32, mode: i32) -> i32;
    fn close(fd: i32) -> i32;
}

/*
 * The idea of this program is to rewire stdin and stdout to files and to use
 * this to copy the file content with println.
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: {} source_file target_file", args[0]);
        process::exit(1);
    }

    let source_path = CString::new(args[1].clone()).unwrap_or_else(|err| {
        eprintln!("{}: CString::new(“{}”): {}", args[0], args[1], err);
        process::exit(1);
    });
    let dest_path = CString::new(args[2].clone()).unwrap_or_else(|err| {
        eprintln!("{}: CString::new(“{}”): {}", args[0], args[2], err);
        process::exit(1);
    });

    unsafe {
        close(0);
        let fd: i32 = open(source_path.as_ptr(), O_RDONLY, 0);
        if fd < 0 {
            eprintln!("{}: can not open file “{}”", args[0], args[1]);
            std::process::exit(1);
        }

        close(1);
        let fd: i32 = open(dest_path.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC,
                           0o660);
        if fd < 0 {
            eprintln!("{}: can not open file “{}”", args[0], args[2]);
            process::exit(1);
        }
    }

    let stdin = io::stdin();
    let mut input = String::new();
    while stdin.read_line(&mut input).unwrap_or_else(|err| {
        eprintln!("{}: io::stdin().read_line(): {}", args[0], err);
        process::exit(1);
    }) != 0 {
        print!("{}", input);
        input.clear();
    }
}
