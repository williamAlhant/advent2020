use std::process;

pub fn exit_with_error_message<ROk, RErr>(e: RErr) -> ROk
    where RErr: std::fmt::Display {
    println!("{}", e);
    process::exit(-1);
}

pub trait ResultOkPrintErrExt<T> {
    fn ok_or_print_err(self, msg: &str) -> Option<T>;
}

impl<T, E> ResultOkPrintErrExt<T> for Result<T, E>
where
    E: ::std::fmt::Debug,
{
    fn ok_or_print_err(self, msg: &str) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("ERR {}: {:?}", msg, e);
                None
            }
        }
    }
}

impl<T> ResultOkPrintErrExt<T> for Option<T>
{
    fn ok_or_print_err(self, msg: &str) -> Option<T> {
        match self {
            Some(v) => Some(v),
            None => {
                eprintln!("ERR {}", msg);
                None
            }
        }
    }
}