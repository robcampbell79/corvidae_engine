extern crate rand;

use rand::Rng;

pub struct Location {
    pub country: String,
    pub state: String,
    pub town: String,
}

pub struct Protagonist {
    pub firstName: String,
    pub lastName: String,
    pub occupation: String,
}