use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    And,
    Lshift,
    Rshift,
    Not,
    Or,
}

impl Operation {
    fn run(&self, a: u16, b: u16) -> u16 {
        match self {
            Operation::And => a & b,
            Operation::Lshift => a << b,
            Operation::Rshift => a >> b,
            Operation::Not => !b,
            Operation::Or => a | b,
        }
    }

    fn from(s: &str) -> Operation {
        match s {
            "AND" => Operation::And,
            "LSHIFT" => Operation::Lshift,
            "RSHIFT" => Operation::Rshift,
            "NOT" => Operation::Not,
            "OR" => Operation::Or,
            _ => unreachable!("Unknown operation: {}", s),
        }
    }
}

#[derive(Debug)]
enum Input {
    Wire(String),
    Signal(u16),
}

impl Input {
    fn get_value(&self, circuit: &HashMap<String, Wire>) -> Option<u16> {
        match self {
            Input::Signal(i) => Some(*i),
            Input::Wire(s) =>
                match circuit.get(s) {
                    None => unreachable!("Wire '{}' should have been resolved", s),
                    Some(w) => w.get_output(circuit),
                }
        }
    }

    fn from(s: &str) -> Input {
        if let Ok(n) = s.parse::<u16>() { Input::Signal(n) } else { Input::Wire(s.to_string()) }
    }
}

#[derive(Debug)]
pub struct Wire {
    a: Option<Input>,
    b: Input,
    op: Option<Operation>,
    cached: Cell<Option<u16>>,
    override_input: Cell<Option<u16>>,
}

impl Wire {
    fn reset(&self) {
        self.cached.set(None)
    }

    fn set_override_input(&self, input: u16) {
        self.override_input.set(Some(input))
    }

    fn new(a: Option<Input>, b: Input, op: Option<Operation>) -> Self {
        Self {
            a,
            b,
            op,
            cached: Cell::new(None),
            override_input: Cell::new(None),
        }
    }

    fn get_output(&self, circuit: &HashMap<String, Wire>) -> Option<u16> {
        if self.cached.get().is_some() {
            return self.cached.get();
        }

        if self.override_input.get().is_some() {
            return self.override_input.get();
        }

        if let Some(op) = self.op {
            if self.a.is_none() {
                match op {
                    Operation::Not => (),
                    _ => unreachable!("Operation {:?} requires an input", op),
                }
            }

            let a_val = self.a.as_ref().unwrap_or(&Input::Signal(0)).get_value(circuit);
            let b_val = self.b.get_value(circuit);

            if a_val.is_none() || b_val.is_none() {
                return None;
            }

            let result = Some(op.run(a_val.unwrap(), b_val.unwrap()));
            self.cached.set(result);

            result
        } else {
            let result = self.b.get_value(circuit);
            self.cached.set(result);
            result
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Wire> {
    let mut circuit = HashMap::new();

    let re = Regex::new(
        r"^(?:(?P<a>\d{1,5}|[a-z]{1,2}) )?(?:(?P<op>[A-Z]+) )?(?:(?P<b>\d{1,5}|[a-z]{1,2})) -> (?P<out>[a-z]{1,2})"
    ).unwrap();

    for line in input.lines() {
        let caps = re.captures(line.trim()).expect("Invalid input");
        let a = caps.name("a").map(|v| Input::from(v.as_str()));
        let op = caps.name("op").map(|v| Operation::from(v.as_str()));
        let b = Input::from(caps.name("b").unwrap().as_str());
        let out = caps.name("out").unwrap().as_str();

        circuit.insert(out.to_string(), Wire::new(a, b, op));
    }

    circuit
}

#[aoc(day7, part1)]
pub fn solve_part1(circuit: &HashMap<String, Wire>) -> u16 {
    let a = circuit.get("a").unwrap().get_output(circuit).unwrap();
    a
}

#[aoc(day7, part2)]
pub fn solve_part2(circuit: &HashMap<String, Wire>) -> u16 {
    let a = circuit.get("a").unwrap().get_output(circuit).unwrap();
    for (out, w) in circuit.iter() {
        w.reset();

        if out == "b" {
            w.set_override_input(a);
        }
    }

    circuit.get("a").unwrap().get_output(circuit).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        let circuit = input_generator(input);
        assert_eq!(circuit.get("d").unwrap().get_output(&circuit).unwrap(), 72);
        assert_eq!(circuit.get("e").unwrap().get_output(&circuit).unwrap(), 507);
        assert_eq!(circuit.get("f").unwrap().get_output(&circuit).unwrap(), 492);
        assert_eq!(circuit.get("g").unwrap().get_output(&circuit).unwrap(), 114);
        assert_eq!(circuit.get("h").unwrap().get_output(&circuit).unwrap(), 65412);
        assert_eq!(circuit.get("i").unwrap().get_output(&circuit).unwrap(), 65079);
        assert_eq!(circuit.get("x").unwrap().get_output(&circuit).unwrap(), 123);
        assert_eq!(circuit.get("y").unwrap().get_output(&circuit).unwrap(), 456);
    }
}
