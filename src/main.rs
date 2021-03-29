struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        s
    }
}


fn main() {
    ["owoztneoer", "fviefuro"]
        .iter()
        .map(|s| s.to_string())
        .map(Solution::original_digits)
        .for_each(|r| println!("{}", r))
}


// zero one two three four five six seven eight nine
// z: zero
// w: two 
// u: four
// x: six 
// g: eight

// zero two four six eight 
// one three five seven nine
