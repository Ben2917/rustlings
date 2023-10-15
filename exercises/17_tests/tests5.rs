// tests5.rs
//
// When writing tests, we want to make sure we are as thorough as possible. To help
// us achieve this we can use a coverage tool such as llvm-cov. The coverage tool
// will give us a good idea of how many lines of our source code are actually being 
// run by our tests. 
//
// When you run this exercise you will see a line by line print out of the exercise
// code which includes a number for each line to indicate how many times it has been
// run.
// 
// There are two parts to this exercise:
// 1. Get the code coverage up to 100%
// 2. Resolve the bug in our code that was missed because of incomplete coverage!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

enum Unit {
    Millimeters,
    Centimeters,
    Meters
}

#[allow(dead_code)]
fn convert_kilometers_to_unit(kilometers : i32, output_unit : Unit) -> i32{
    match output_unit {
        Unit::Millimeters => kilometers * 1000000,
        Unit::Centimeters => kilometers * 10000,
        Unit::Meters => kilometers * 1000
    }
}

#[test]
fn can_convert_kilometers_to_meters() {
    assert_eq!(convert_kilometers_to_unit(6, Unit::Meters), 6000)
}