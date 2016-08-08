#![feature(box_syntax)]

extern crate grabinput;
extern crate iter_ord;

mod divisors;
mod fraction;

use fraction::Fraction;

fn main() {
    let fractions = grabinput::by_lines(std::env::args().nth(1))
        .filter_map(|line| line.trim().parse::<Fraction>().ok());

    for fraction in fractions {
        println!("{}", fraction.reduce());
    }
}
