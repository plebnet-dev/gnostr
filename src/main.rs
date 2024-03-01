extern crate getopts;
use getopts::Options;

use std::env;
use std::process;

extern crate gnostr_bins;

use reqwest::Url;
use std::io::Read;
use gnostr_bins::{get_relays};

use std::time::{SystemTime, UNIX_EPOCH};

extern crate libc;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}

extern {
    ///static void gnostr_sha256(int argc, const char* argv[], struct args *args)
    fn gnostr_sha256(input: libc::c_int) -> libc::c_int;
}
extern {
    ///static int copyx(unsigned char *output, const unsigned char *x32, const unsigned char *y32, void *data)
    fn copyx(input: libc::c_int) -> libc::c_int;
}
extern {
    ///static void try_subcommand(int argc, const char* argv[])
    fn try_subcommand(input: libc::c_int) -> libc::c_int;
}
extern {
    ///static void print_hex(unsigned char* data, size_t size)
    fn print_hex(input: libc::c_int) -> libc::c_int;
}

///gnostr-bins::get_relays()
pub fn relays(_program: &str, _opts: &Options) {
    let relays = get_relays();
    println!("{}", format!("{  }", relays.unwrap()));
}

pub fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
    //process::exit(0);
}

pub fn print_input(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    //REF: https://docs.rs/getopts/latest/getopts/struct.Options.html
    let mut opts = Options::new();

    opts.optopt("o", "", "set output file name", "NAME");
    opts.optopt(
        "i",
        "input",
        "Specify the maximum number of commits to show (default: 10)",
        "NUMBER",
    );

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("r", "relays", "print a json object of relays");

    if args.len() >= 1 {
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                println!("Error: {}", f.to_string());
                panic!("{}", f.to_string())
            }
        };
        if matches.opt_present("h") {
            print_usage(&program, &opts);
            process::exit(0);
        }
        if matches.opt_present("r") {
            relays(&program, &opts);
            process::exit(0);
        }

    let _output = matches.opt_str("o");
		//leave input as &Option<String>
    let _input = matches.opt_str("i");
		//deref &str
    let _value = _input.as_deref().unwrap_or("100");
    }
}
