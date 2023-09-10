pub mod cmd {
    pub mod command;
    pub mod manager;
    pub mod operand;
    pub mod error;
}

mod run;
mod compile;
mod read;
mod save;
mod configure;

#[cfg(test)]
mod test {
    mod cmd_test;
    mod read_test;
    mod compile_test;
    mod config_test;
}

use run::run;

fn main() {

    run()
}
