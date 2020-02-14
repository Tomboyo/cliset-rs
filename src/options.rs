extern crate clap;

use std::collections::BTreeSet;
use std::io::Error;

use clap::{App, Arg, ArgMatches};
#[cfg(test)]
use clap::AppSettings;

pub struct Options {
    pub left: BTreeSet<String>,
    pub right: BTreeSet<String>,
    pub operation: Operation,
}

pub enum Operation {
    Intersect(),
    SubsetTest(),
}

impl Options {
    pub fn from_stdin() -> Result<Self, Error> {
        Options::from_matches(create_clap_app().get_matches())
    }

    #[cfg(test)]
    pub fn from_iterable<I, T>(iter: I) -> Result<Self, Error>
    where I: IntoIterator<Item = T>,
          T: Into<std::ffi::OsString> + Clone,
    {
        Options::from_matches(
            create_clap_app()
            .setting(AppSettings::NoBinaryName)
            .get_matches_from(iter))
    }

    fn from_matches<'a>(matches: ArgMatches<'a>) -> Result<Self, Error> {
        let left = read_treeset(matches.value_of("left").unwrap())?;
        let right = read_treeset(matches.value_of("right").unwrap())?;
        let operation = match matches.value_of("operation") {
            Some("intersect") => Operation::Intersect(),
            Some("subset-test") => Operation::SubsetTest(),
            // Any match here is a programming error
            any => panic!("Unexpected operation {:?}", any),
        };

        Ok(Options { left, right, operation })
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
                .required(true),
        )
        .arg(
            Arg::with_name("right")
                .short("r")
                .long("right")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("operation")
                .required(true)
                .index(1)
                .possible_values(&["intersect", "subset-test"]),
        )
}

fn read_treeset(path: &str) -> Result<BTreeSet<String>, Error> {
    std::fs::read_to_string(path)
        .map(|s| {
            s.split("\n")
                .map(&str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        })
        .map_err(|e| Error::new(
            e.kind(),
            format!("Cannot read file '{}': {}", path, e)))
}
