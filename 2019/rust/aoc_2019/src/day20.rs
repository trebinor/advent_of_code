use petgraph::algo::dijkstra;
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Graph, Undirected};
use std::collections::HashMap;

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Maze {
    let v = input.lines().map(|l| l).collect::<Vec<&str>>();
    let mut maze: Maze = vec![vec![' '; v[0].len()]; v.len()];

    for (y, l) in v.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            maze[y][x] = c;
        }
    }
    maze
}

#[aoc(day20, part1)]
pub fn shortest_portal_path(input: &Maze) -> usize {
    let mut portals: HashMap<String, Vec<Point>> = HashMap::new();
    let mut spaces: HashMap<Point, NodeIndex<DefaultIx>> = HashMap::new();
    let mut graph = Graph::<Point, usize, Undirected>::new_undirected();
    for (j, y) in input.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            if input[j][i] == '.' {
                let mut name: String = "".to_string();
                if input[j - 1][i].is_ascii_uppercase() {
                    name.push(input[j - 2][i]);
                    name.push(input[j - 1][i]);
                } else if input[j + 1][i].is_ascii_uppercase() {
                    name.push(input[j + 1][i]);
                    name.push(input[j + 2][i]);
                } else if input[j][i - 1].is_ascii_uppercase() {
                    name.push(input[j][i - 2]);
                    name.push(input[j][i - 1]);
                } else if input[j][i + 1].is_ascii_uppercase() {
                    name.push(input[j][i + 1]);
                    name.push(input[j][i + 2]);
                }
                let p = Point { x: i, y: j };
                let node_idx = graph.add_node(p);
                spaces.insert(p, node_idx);
                if input[j - 1][i] == '.' {
                    let other_node = spaces.get(&Point { x: i, y: j - 1 });
                    graph.update_edge(node_idx, *other_node.unwrap(), 1);
                }
                if input[j][i - 1] == '.' {
                    let other_node = spaces.get(&Point { x: i - 1, y: j });
                    graph.update_edge(node_idx, *other_node.unwrap(), 1);
                }

                if !name.is_empty() {
                    portals.entry(name).and_modify(|e| e.push(p)).or_insert({
                        let mut v = Vec::new();
                        v.push(p);
                        v
                    });
                }
            }
        }
    }

    for (k, v) in portals.iter() {
        if k != "AA" && k != "ZZ" {
            graph.update_edge(*spaces.get(&v[0]).unwrap(), *spaces.get(&v[1]).unwrap(), 1);
        }
    }

    let start = *spaces.get(&portals.get("AA").unwrap()[0]).unwrap();
    let goal = *spaces.get(&portals.get("ZZ").unwrap()[0]).unwrap();
    let res = dijkstra(&graph, start, Some(goal), |_| 1);

    *res.get(&goal).unwrap()
}

#[aoc(day20, part2)]
pub fn shortest_layered_portal_path(input: &Maze) -> usize {
    let mut portals: HashMap<String, Vec<LayeredPoint>> = HashMap::new();
    let mut spaces: HashMap<LayeredPoint, NodeIndex<DefaultIx>> = HashMap::new();
    let mut graph = Graph::<LayeredPoint, usize, Undirected>::new_undirected();
    const LAYERS: usize = 100;
    let h = input.len();
    let w = input[0].len();
    for (j, y) in input.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            if input[j][i] == '.' {
                let mut name = String::from("");
                if input[j - 1][i].is_ascii_uppercase() {
                    name.push(input[j - 2][i]);
                    name.push(input[j - 1][i]);
                } else if input[j + 1][i].is_ascii_uppercase() {
                    name.push(input[j + 1][i]);
                    name.push(input[j + 2][i]);
                } else if input[j][i - 1].is_ascii_uppercase() {
                    name.push(input[j][i - 2]);
                    name.push(input[j][i - 1]);
                } else if input[j][i + 1].is_ascii_uppercase() {
                    name.push(input[j][i + 1]);
                    name.push(input[j][i + 2]);
                }
                for layer in 0..LAYERS {
                    let p = LayeredPoint {
                        x: i,
                        y: j,
                        l: layer,
                    };
                    let node_idx = graph.add_node(p);
                    spaces.insert(p, node_idx);
                    if input[j - 1][i] == '.' {
                        let other_node = spaces.get(&LayeredPoint {
                            x: i,
                            y: j - 1,
                            l: layer,
                        });
                        graph.update_edge(node_idx, *other_node.unwrap(), 1);
                    }
                    if input[j][i - 1] == '.' {
                        let other_node = spaces.get(&LayeredPoint {
                            x: i - 1,
                            y: j,
                            l: layer,
                        });
                        graph.update_edge(node_idx, *other_node.unwrap(), 1);
                    }
                    if !name.is_empty() {
                        if (name == "AA" || name == "ZZ") && layer != 0 {
                            // start and goal portal only appear on the top layer
                            continue;
                        }
                        portals
                            .entry(name.clone())
                            .and_modify(|e| e.push(p))
                            .or_insert({
                                let mut v = Vec::new();
                                v.push(p);
                                v
                            });
                    }
                }
            }
        }
    }

    // Portals handled in the n-1 to n layer pair. Inner portals in the last layer will be unconnected, it and several of its neighbors will not have nodes on the shortest path.
    for layer in 1..LAYERS {
        for (k, v) in portals.iter() {
            if k != "AA" && k != "ZZ" {
                let inner = v
                    .iter()
                    .find(|&&p| p.l == layer - 1 && p.inner(w, h))
                    .unwrap();
                let outer = v.iter().find(|&&p| p.l == layer && p.outer(w, h)).unwrap();
                graph.update_edge(
                    *spaces.get(&inner).unwrap(),
                    *spaces.get(&outer).unwrap(),
                    1,
                );
            }
        }
    }

    assert_eq!(1, portals.get("AA").unwrap().len());
    assert_eq!(1, portals.get("ZZ").unwrap().len());
    let start = *spaces.get(&portals.get("AA").unwrap()[0]).unwrap();
    let goal = *spaces.get(&portals.get("ZZ").unwrap()[0]).unwrap();
    let res = dijkstra(&graph, start, Some(goal), |_| 1);

    *res.get(&goal).unwrap()
}

type Maze = Vec<Vec<char>>;

#[allow(dead_code)]
fn display(maze: Maze) {
    for (j, y) in maze.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            print!("{}", maze[j][i]);
        }
        println!();
    }
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct LayeredPoint {
    x: usize,
    y: usize,
    l: usize,
}

impl LayeredPoint {
    fn inner(&self, w: usize, h: usize) -> bool {
        !self.outer(w, h)
    }

    fn outer(&self, w: usize, h: usize) -> bool {
        self.x == 2 || self.x == w - 3 || self.y == 2 || self.y == h - 3
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for LayeredPoint {
    fn eq(&self, other: &LayeredPoint) -> bool {
        self.x == other.x && self.y == other.y && self.l == other.l
    }
}
