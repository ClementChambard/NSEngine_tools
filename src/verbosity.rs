pub fn calculate_verbosity(matches: &clap::ArgMatches) -> u8 {
    if matches.get_flag("quiet") {
        0u8
    } else {
        matches.get_one::<u8>("verbose").unwrap() + 1
    }
}
