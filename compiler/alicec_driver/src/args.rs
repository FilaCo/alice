pub fn raw_args() -> Vec<String> {
    let mut args = vec![];

    for (_i, arg) in std::env::args_os().enumerate() {
        match arg.into_string() {
            Ok(arg) => args.push(arg),
            Err(_) => todo!(),
        }
    }

    args
}
