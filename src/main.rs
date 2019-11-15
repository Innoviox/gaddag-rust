use std::io;
use std::ops::Index;

struct Ixx {
    idx: usize
}

impl Index<Ixx> for String {
    type Output = Option<char>;

    fn index (&self, idx: Ixx) -> &Self::Output {
        &self.chars().nth(idx.idx)
    }
}

fn main() {
    let v = String::from("hello");
    println!("First char: {:?}", v[Ixx{idx: 0}]);
}