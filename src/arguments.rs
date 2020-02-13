extern crate clap;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use clap::{App, AppSettings, Arg, ArgMatches};

pub struct Options {
    pub left: HashSet<String>,
    pub right: HashSet<String>,
    pub operation: Operation,
}

pub enum Operation {
    Intersect(),
    SubsetTest(),
}

impl Options {
    pub fn from_stdin() -> Self {
        Options::from_matches(create_clap_app().get_matches())
    }

    #[cfg(test)]
    pub fn from_iterable<I, T>(iter: I) -> Self
    where I: IntoIterator<Item = T>,
          T: Into<std::ffi::OsString> + Clone,
    {
        Options::from_matches(
            create_clap_app()
            .setting(AppSettings::NoBinaryName)
            .get_matches_from(iter))
    }

    fn from_matches<'a>(matches: ArgMatches<'a>) -> Self {
        Options {
            left: matches.value_of("left").map(file_to_hashset).unwrap(),
            right: matches.value_of("right").map(file_to_hashset).unwrap(),
            operation: match matches.value_of("operation") {
                Some("intersect") => Operation::Intersect(),
                Some("subset-test") => Operation::SubsetTest(),
                // Any match here is a programming error
                any => panic!("Unexpected operation {:?}", any),
            },
        }
    }
}

fn create_clap_app<'a, 'b>() -> App<'a, 'b>{
    App::new("sets")
        .version(clap::crate_version!())
        .arg(
            Arg::with_name("left")
                .short("l")
                .long("left")
                .takes_value(true)
                .validator(validate_is_file)
                .required(true),
        )
        .arg(
            Arg::with_name("right")
                .short("r")
                .long("right")
                .takes_value(true)
                .validator(validate_is_file)
                .required(true),
        )
        .arg(
            Arg::with_name("operation")
                .required(true)
                .index(1)
                .possible_values(&["intersect", "subset-test"]),
        )
}

fn validate_is_file(value: String) -> Result<(), String> {
    if Path::new(&value).exists() {
        Ok(())
    } else {
        // TODO: report missing v. permission denied
        Err(format!("'{}' is not a regular file", &value))
    }
}

// read file into hashset or else panic
fn file_to_hashset(path: &str) -> HashSet<String> {
    File::open(path)
        .map(BufReader::new)
        .map(BufRead::lines)
        .map(|iter| iter.map(Result::unwrap).collect())
        .unwrap()
}
