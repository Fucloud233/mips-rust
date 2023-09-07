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

#[cfg(test)]
mod test {
    mod cmd_test;
    mod compile_test;
}

use run::run;

fn main() {
    run()
}
