#[macro_export]
macro_rules! error {
    ($fmt:expr $(, $args:expr)*; $hint:expr) => {
        eprintln!(
            "[{}] {}",
            "\x1b[31m\x1b[1mERROR\x1b[0m",
            format!($fmt $(, $args)*)
        );
        eprintln!("\x1b[3mhint\x1b[0m: {}", $hint);
        std::process::exit(1);
    };

    ($fmt:expr $(, $args:expr)*) => {
        eprintln!(
            "[{}] {}",
            "\x1b[31m\x1b[1mERROR\x1b[0m",
            format!($fmt $(, $args)*)
        );
        std::process::exit(1);
    };
}


#[macro_export]
macro_rules! info {
    ($fmt:expr $(, $args:expr)*) => {
        eprintln!(
            "[{}] {}",
            "\x1b[32m\x1b[1mINFO\x1b[0m",
            format!($fmt $(, $args)*)
        )
    };
}
