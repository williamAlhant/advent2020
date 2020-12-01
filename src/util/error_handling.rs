use std::process;

pub fn exit_with_error_message<ROk, RErr>(e: RErr) -> ROk
    where RErr: std::fmt::Display {
    println!("{}", e);
    process::exit(-1);
}