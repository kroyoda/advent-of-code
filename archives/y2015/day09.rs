struct Graph {
    nodes: Vec<String>,
    edges: Vec<(String, String, u32)>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn new_distance(&mut self, from: &str, to: &str, distance: u32) {
        self.edges.push((from.to_string(), to.to_string(), distance));
        self.edges.push((to.to_string(), from.to_string(), distance));
        if !self.nodes.contains(&from.to_string()) {
            self.nodes.push(from.to_string());
        }
        if !self.nodes.contains(&to.to_string()) {
            self.nodes.push(to.to_string());
        }
    }

    fn distance(&self, from: &str, to: &str) -> Option<u32> {
        for (f, t, d) in &self.edges {
            if f == from && t == to {
                return Some(*d);
            }
        }
        None
    }

    fn shortest_path(&self) -> Option<u32> {
        let mut shortest = None;
        for path in self.paths() {
            let mut distance = 0;
            for i in 0..path.len() - 1 {
                distance += self.distance(&path[i], &path[i + 1]).unwrap();
            }
            if shortest.is_none() || distance < shortest.unwrap() {
                shortest = Some(distance);
            }
        }
        shortest
    }

    fn longest_path(&self) -> Option<u32> {
        let mut longest = None;
        for path in self.paths() {
            let mut distance = 0;
            for i in 0..path.len() - 1 {
                distance += self.distance(&path[i], &path[i + 1]).unwrap();
            }
            if longest.is_none() || distance > longest.unwrap() {
                longest = Some(distance);
            }
        }
        longest
    }

    fn paths(&self) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        for node in &self.nodes {
            let path = vec![node.to_string()];
            paths.append(&mut self.paths_from(&path));
        }
        paths
    }

    fn paths_from(&self, path: &[String]) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        for node in &self.nodes {
            if !path.contains(node) {
                let mut new_path = path.to_vec();
                new_path.push(node.to_string());
                paths.append(&mut self.paths_from(&new_path));
            }
        }
        if paths.is_empty() {
            paths.push(path.to_vec());
        }
        paths
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut graph = Graph::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let distance: u32 = parts[1].parse().unwrap();
        let cities: Vec<&str> = parts[0].split(" to ").collect();
        graph.new_distance(cities[0], cities[1], distance);
    }
    graph.shortest_path().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut graph = Graph::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let distance: u32 = parts[1].parse().unwrap();
        let cities: Vec<&str> = parts[0].split(" to ").collect();
        graph.new_distance(cities[0], cities[1], distance);
    }
    graph.longest_path().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "London to Dublin = 464\n\
                     London to Belfast = 518\n\
                     Dublin to Belfast = 141";
        assert_eq!(solve_part1(input), 605);
    }

    #[test]
    fn test_part2() {
        let input =
            "London to Dublin = 464\n\
                     London to Belfast = 518\n\
                     Dublin to Belfast = 141";
        assert_eq!(solve_part2(input), 982);
    }
}
