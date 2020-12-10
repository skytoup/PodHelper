use structopt::StructOpt;

/// pod helper command
#[allow(non_camel_case_types)]
#[derive(StructOpt, Debug, PartialEq)]
pub enum Command {
    #[structopt(name = "check", about = "check podfile.lock update")]
    Check {
        #[structopt(flatten)]
        opt: CheckOpts,
    },
}

/// check opts
#[derive(StructOpt, Debug, PartialEq)]
pub struct CheckOpts {
    /// podfile.lock path
    #[structopt(name = "FILE_PATH")]
    pub file_path: String,
    /// request max concurrent size
    #[structopt(short = "c", default_value = "5")]
    pub request_concurrent: usize,
    /// connect timeout sec
    #[structopt(long, default_value = "10")]
    pub connect_timeout: u64,
    /// request timeout sec
    #[structopt(long, default_value = "20")]
    pub request_timeout: u64,
    /// output is show no update
    #[structopt(long)]
    pub show_no_update: bool,
}
