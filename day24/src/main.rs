#![allow(unused)]
#![allow(dead_code)]


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn neighbors(&self) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            neighbors.push(Position::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            neighbors.push(Position::new(self.x, self.y - 1));
        }
        neighbors.push(Position::new(self.x + 1, self.y));
        neighbors.push(Position::new(self.x, self.y + 1));
        neighbors
    }
}

#[derive(Debug, Clone)]
struct Edge {
    from: usize,
    to: usize,
    distance: usize
}


impl Edge {
    fn new(from: usize, to: usize, distance: usize) -> Self {
        Edge {from, to, distance}
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Node {
    position: Position
}

impl Node {
    fn new(x: usize, y: usize) -> Self {
        Node {position: Position::new(x, y)}
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.position.x, self.position.y)
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} ({})", self.from, self.to, self.distance)
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    importantnodes: Vec<(u8, usize)>
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for edge in self.edges.iter() {
            write!(f, "{} -- {} ({})\n", self.nodes[edge.from], self.nodes[edge.to], edge.distance)?;
        }
        for (index, node) in self.importantnodes.iter() {
            write!(f, "{}: at {}\n", index, self.nodes[*node]);
        }
        Ok(())
    }
}

use std::collections::HashSet;

impl From<&Vec<&str>> for Graph {
    fn from(value: &Vec<&str>) -> Self {
        let grid = value.iter().map(|line| line.bytes().collect()).collect::<Vec<Vec<u8>>>();
        let mut seen = HashSet::new();
        let mut zeropos = Position::new(0, 0);
        'findzero: for (y, line) in grid.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if *field == '0' as u8 {
                    zeropos = Position::new(x, y);
                    break 'findzero;
                }
            }
        }

        let mut graph = Self { nodes: Vec::new(), edges: Vec::new(), importantnodes: Vec::new()};
        graph.nodes.push(Node::new(zeropos.x, zeropos.y));
        Self::dfs(&grid, &mut seen, zeropos, zeropos, 0, 0, &mut graph);
        graph.importantnodes.sort();
        graph
    }
}

impl Graph {
    fn dfs(grid: &[Vec<u8>], seen: &mut HashSet<Position>, current: Position, from: Position, cdfdist: usize, lastnode: usize, graph: &mut Graph) {
        if !seen.insert(current) {
            // backwards or cross edge
            let tarnode = graph.nodes.iter().enumerate().filter(|(i, node)| node.position == current).next();
            if let Some((index, _)) = tarnode {
                graph.edges.push(Edge::new(lastnode, index, cdfdist));
            }
            return;
        }
        let mut neighbors = current.neighbors()
                                   .into_iter()
                                   .filter(|p| *p != from && p.x < grid[0].len() && p.y < grid.len())
                                   .filter(|p| grid[p.y][p.x] != '#' as u8)
                                   .collect::<Vec<Position>>();
        let tile = grid[current.y][current.x];
        if tile != '#' as u8 && tile != '.' as u8 {
            // special tile, emit node either way
            let newnode = graph.nodes.len();
            graph.nodes.push(Node::new(current.x, current.y));
            graph.edges.push(Edge::new(lastnode, newnode, cdfdist));
            graph.importantnodes.push((tile - 0x30, newnode));
            for neighbor in neighbors {
                Self::dfs(grid, seen, neighbor, current, 1, newnode, graph);
            }
            return;
        }
        if neighbors.len() == 0 {
            return;
        }
        if neighbors.len() == 1 {
            Self::dfs(grid, seen, neighbors[0], current, cdfdist + 1, lastnode, graph);
            return;
        }
        // more than 1 neighbor, add new node
        let newnode = graph.nodes.len();
        graph.nodes.push(Node::new(current.x, current.y));
        graph.edges.push(Edge::new(lastnode, newnode, cdfdist));
        for neighbor in neighbors {
            Self::dfs(grid, seen, neighbor, current, 1, newnode, graph);
        }
    }
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let graph = Graph::from(lines);
    // println!("{}", graph);
    None
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    //TODO: implement me
    None
}

fn main() {
    use std::fs;
    use std::env;
    use std::time::Instant;
    let args: Vec<String> =  env::args().collect();
    let infile = args.get(1).unwrap_or_else(|| {
        println!("Usage: {} <puzzle input>", args[0]);
        std::process::exit(1);
    });

    let contents = fs::read_to_string(infile)
        .expect("Could not read in file");

    let lines: Vec<&str> = contents.lines().collect();

    // execute part 1 and part 2, print their results if they exist
    // later parts may follow, so we loop over the part functions
    let parts = [part1, part2];
    for (index, part) in parts.iter().enumerate() {
        let partstart = Instant::now();
        let result = part(&lines);
        match result {
            Some(result) => println!("Part {}: {}\t({:?})", index+1, result, partstart.elapsed()),
            None => println!("Part {}: No result", index+1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TESTINPUT: &str = "CHANGEME";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(1337), part1(&lines));
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(13337), part2(&lines));
    }
}
