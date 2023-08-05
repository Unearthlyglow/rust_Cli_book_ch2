use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Award Malisi <awardmalisi@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();

    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

    println!("{:#?}", matches);
}

//Line 23 -
//Values: An iterator for getting multiple values out of an argument.

//I will use the ArgMatches::values_of_lossy function to get an Option<Vec<String>>. The Option::unwrap function will take the value out of Some<T> to get at the payload T. Because the text argument is required by clap, I know it will be impossible to have None; therefore, I can safely call Option::unwrap to get the Vec<String> value. The reason "text" is used with values_of_lossy is because of: Arg::with_name("text").
