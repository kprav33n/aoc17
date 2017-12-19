extern crate aoc17;

use aoc17::day18::*;

#[test]
fn test_last_recovered_freq() {
    assert_eq!(last_recovered_freq("
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
"), 4);
}
