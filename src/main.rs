mod arguments;

use std::collections::HashSet;

use arguments::Operation::*;

fn main() {
    let options = arguments::parse_args();
    match options.operation {
        Intersect() => intersect(options.left, options.right),
        SubsetTest() => subset_test(options.left, options.right),
    }
}

fn intersect(left: HashSet<String>, right: HashSet<String>) {
    for element in left.intersection(&right) {
        println!("{}", element);
    }
}

fn subset_test(left: HashSet<String>, right: HashSet<String>) {
    if left.is_subset(&right) {
        println!("true");
    } else {
        println!("false");
    }
}
