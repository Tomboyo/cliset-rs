mod options;

use std::collections::BTreeSet;

use itertools::Itertools;

use options::Operation::*;
use options::Options;

fn main() {
    match Options::from_stdin().map(invoke) {
        Err(e) => println!("Error: {}", e),
        Ok(output) => println!("{}", output),
    }
}

fn invoke(options: Options) -> String {
    let f = match options.operation {
        Intersect() => intersect,
        SubsetTest() => subset_test,
    };
    
    f(&options.left, &options.right)
}

fn intersect(left: &BTreeSet<String>, right: &BTreeSet<String>) -> String {
    left.intersection(&right).join("\n")
}

fn subset_test(left: &BTreeSet<String>, right: &BTreeSet<String>) -> String {
    if left.is_subset(&right) {
        String::from("true")
    } else {
        String::from("false")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_files<F>(left_content: &str, right_content: &str, f: F)
    where F: Fn(&std::path::Path, &std::path::Path) -> ()
    {
        // setup
        let left_file = tempfile::NamedTempFile::new().unwrap();
        let right_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(&left_file, left_content).unwrap();
        std::fs::write(&right_file, right_content).unwrap();

        // test
        f(&left_file.path(), &right_file.path());

        // teardown
        std::fs::remove_file(left_file.path()).unwrap();
        std::fs::remove_file(right_file.path()).unwrap();
    }

    #[test]
    fn test_intersect() {
        with_files("A\nB\nC\nD\nE", "B\nD", |left, right| {
            let options = Options::from_iterable(vec![
                "intersect",
                "--left", left.to_str().unwrap(),
                "--right", right.to_str().unwrap(),
            ]).unwrap();

            assert_eq!("B\nD", invoke(options),
                "Should return the intersection");
        });
    }

    #[test]
    fn test_intersect_ignores_whitespace() {
        with_files("A\n  \n B \n\n", "  \n \nB\nA\n\n", |left, right| {
            let options = Options::from_iterable(vec![
                "intersect",
                "--left", left.to_str().unwrap(),
                "--right", right.to_str().unwrap()
            ]).unwrap();

            assert_eq!("A\nB", invoke(options),
                "Should ignore all whitespace");
        });
    }

    #[test]
    fn test_subset_test() {
        with_files("A\nB", "X\nA\nY\nB\nZ", |left, right| {
            let options = Options::from_iterable(vec![
                "subset-test",
                "--left", left.to_str().unwrap(),
                "--right", right.to_str().unwrap()
            ]).unwrap();

            assert_eq!("true", invoke(options),
                "Should return 'true' when left is a subset of right");
        })
    }

    #[test]
    fn test_subset_test_when_not_subset() {
        with_files("X", "Y", |left, right| {
            let options = Options::from_iterable(vec![
                "subset-test",
                "--left", left.to_str().unwrap(),
                "--right", right.to_str().unwrap()
            ]).unwrap();

            assert_eq!("false", invoke(options),
                "Should return 'false' when left is not a subset of right");
        })
    }

    #[test]
    fn test_subset_test_ignore_whitespace_lines() {
        with_files("A\nB\n  \n\n", "X\nA\nB\nY", |left, right| {
            let options = Options::from_iterable(vec![
                "subset-test",
                "--left", left.to_str().unwrap(),
                "--right", right.to_str().unwrap()
            ]).unwrap();

            assert_eq!("true", invoke(options),
                "Should ignore all whitespace");
        })
    }
}
