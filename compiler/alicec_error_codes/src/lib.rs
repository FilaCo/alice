#[macro_export]
macro_rules! error_codes {
    ($macro:path) => {
        $macro!(
            E0001: 0001,
        )
    };
}
