#[aoc(day4, part1)]
pub fn solve_part1(input: &[u8]) -> String {
    md5_hash_solver_with_zeroes(input, 5)
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[u8]) -> String {
    md5_hash_solver_with_zeroes(input, 6)
}

fn md5_hash_solver_with_zeroes(input: &[u8], n_zeros: usize) -> String {
    // Find the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces a MD5 hash which, in hexadecimal, start with at least five zeroes
    let mut i = 0;

    loop {
        let mut context = md5::Context::new();

        context.consume(input);
        context.consume(i.to_string().as_bytes());

        let hash = context.compute();
        let hash_str = format!("{:x}", hash);

        if hash_str.starts_with(&"0".repeat(n_zeros)) {
            return i.to_string();
        }

        i += 1;
    }
}
