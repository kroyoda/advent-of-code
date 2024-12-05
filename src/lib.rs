pub mod template;

pub mod grid;
pub mod printer;

pub fn add_tuples(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    (t1.0 + t2.0, t1.1 + t2.1)
}

pub fn take_middle_value(v: &[u32]) -> u32 {
    let middle = v.len() / 2;
    v[middle]
}
