extern crate aoc17;

use aoc17::day19::*;

#[test]
fn test_trace_path() {
    assert_eq!(trace_path("     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ "), (38, String::from("ABCDEF")));
}
