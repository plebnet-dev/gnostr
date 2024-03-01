extern crate libc;

extern "C" {
    pub fn gnostr_get_relays(input: libc::c_int) -> libc::c_int;
}

#[allow(dead_code)]
fn get_relays() {
    let input = 128;
    let _output = unsafe { gnostr_get_relays(input) };
    //println!("{} * 2 = {}", input, output);
}
