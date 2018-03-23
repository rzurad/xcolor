use clap::{App, Arg};

pub fn get_cli() -> App<'static, 'static> {
    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("format")
             .short("f")
             .long("format")
             .takes_value(true)
             .value_name("NAME")
             .help("Output format (defaults to hex)")
             .possible_values(&["hex", "HEX", "plain", "rgb"])
             .conflicts_with("custom"))
        .arg(Arg::with_name("custom")
             .short("c")
             .long("custom")
             .takes_value(true)
             .value_name("FORMAT")
             .help("Custom output format")
             .conflicts_with("format"))
        .arg(Arg::with_name("selection")
             .short("s")
             .long("selection")
             .takes_value(true)
             .value_name("SELECTION")
             .min_values(0)
             .max_values(1)
             .possible_values(&["primary", "secondary"])
             .help("Output to selection (defaults to primary)"));

    if cfg!(debug_assertions) {
        app = app.arg(Arg::with_name("foreground")
                .short("F")
                .long("foreground")
                .requires("selection")
                .help("Stay in the foreground"));
    }

    app
}
