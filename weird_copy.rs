use std::io::prelude::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::io::{self};
use std::env;
use std::io::{Error, ErrorKind};

static O_RDONLY: i32 = 0x0000;
static O_WRONLY: i32 = 0x0001;

static O_CREAT: i32 = 0o100;
static O_TRUNC: i32	= 0o1000;

extern "C" {
    fn open(pathname: *const c_char, flags: i32, mode: i32) -> i32;
    fn close(fd: i32) -> i32;
}

/*
 * The idea of this program is to rewire stdin and stdout to files and to use this to copy the file content with println
 */
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Too few arguments"));
    }

    if args.len() < 3 {
        println!("Usage: {} <src> <dst>", args[0]);
        return Err(Error::new(ErrorKind::InvalidInput, "Too few arguments"));
    }
    if args.len() > 3 {
        println!("Usage: {} <src> <dst>", args[0]);
        return Err(Error::new(ErrorKind::InvalidInput, "Too many arguments"));
    }

    let source_path = CString::new(args[1].clone()).expect("CString::new failed");
    let dest_path = CString::new(args[2].clone()).expect("CString::new failed");

    unsafe {
        // Rewire destination file to stdout (fd = 1)
        close(1);
        let fd : i32 = open(dest_path.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, 0o660);
        if fd < 0 {
            eprintln!("ERROR: Could not open destination file");
            std::process::exit(2);
        } else if fd != 1 {
            eprintln!("ERROR: Could not connect destination file to stdout");
            std::process::exit(2);
        }

        // Rewire source file to stdin (fd = 0)
        close(0);
        let fd : i32 = open(source_path.as_ptr(), O_RDONLY, 0);
        if fd < 0 {
            eprintln!("ERROR: Could not open source file");
            std::process::exit(2);
        } else if fd != 0 {
            eprintln!("ERROR: Could not connect source file to stdin");
            std::process::exit(2);
        }
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

    return Ok(());
}