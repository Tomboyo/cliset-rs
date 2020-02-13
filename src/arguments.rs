extern crate clap;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use clap::{App, Arg, ArgMatches};

pub enum Operation {
    Intersect(),
    SubsetTest(),
}

pub struct Options {
    pub left: HashSet<String>,
    pub right: HashSet<String>,
    pub operation: Operation,
}

pub fn parse_args() -> Options {
    let args = parse_raw_args();

    Options {
        left: args.value_of("left").map(file_to_hashset).unwrap(),
        right: args.value_of("right").map(file_to_hashset).unwrap(),
        operation: match args.value_of("operation") {
            Some("intersect") => Operation::Intersect(),
            Some("subset-test") => Operation::SubsetTest(),
            // Any match here is a programming error
            any => panic!("Unexpected operation {:?}", any),
        },
    }
}

fn parse_raw_args<'a>() -> ArgMatches<'a> {
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
        .get_matches()
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
