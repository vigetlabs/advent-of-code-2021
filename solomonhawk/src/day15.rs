#![allow(unused)]
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Point = (i8, i8);

const OFFSETS: [Point; 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

#[derive(Debug)]
pub struct Edge {
    node: usize, // index in node list
    cost: usize, // cost to move from the node that owns this edge to the node at `node`
}

#[derive(Debug)]
pub struct Node {
    position: Point,
    edges: Vec<Edge>,
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Node> {
    let points: Vec<Vec<usize>> = input
        .split("\n")
        .map(|row| row.chars().map(|n| n as usize - '0' as usize).collect())
        .collect();

    points
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let points = points.clone();

            (0..row.len()).map(move |x| {
                let position = (x as i8, y as i8);

                Node {
                    position,
                    edges: find_edges(&points, position),
                }
            })
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(graph: &Vec<Node>) -> usize {
    if let Some(distance) = shortest_path(&graph, 0, graph.len() - 1) {
        distance
    } else {
        panic!("Could not find a shortest path")
    }
}

#[aoc(day15, part2)]
pub fn part2(graph: &Vec<Node>) -> usize {
    0
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// min-heap instead of max-heap, we want the shorter paths over the longer ones
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_edges(points: &Vec<Vec<usize>>, origin: Point) -> Vec<Edge> {
    valid_neighbors(&origin, (points[0].len(), points.len()))
        .iter()
        .map(|&(x, y)| {
            Edge {
                node: x as usize + y as usize * points.len(), // dest node position in master list
                cost: points[y as usize][x as usize],
            }
        })
        .collect()
}

// A* https://en.wikipedia.org/wiki/A*_search_algorithm
// Manhattan Distance https://en.wikipedia.org/wiki/Taxicab_geometry
// Djikstra's Algorithm https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// Binary Heap https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn shortest_path(graph: &Vec<Node>, start: usize, end: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    // start at the beginning, it's free
    dist[start] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    // iterate over the lowest cost next nodes
    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        // there's a better way
        if cost > dist[position] {
            continue;
        }

        // for each node we can reach, check if that pathway is cheaper
        for edge in &graph[position].edges {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // if it is, add it to the heap and continue
            if next.cost < dist[next.position] {
                heap.push(next);

                // relaxation, we found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn valid_neighbors(point: &Point, dimensions: (usize, usize)) -> Vec<Point> {
    OFFSETS
        .iter()
        .map(|(dx, dy)| (point.0 + dx, point.1 + dy))
        .filter(|&(x, y)| valid_point(x, y, dimensions.0, dimensions.1))
        .collect()
}
fn valid_point(x: i8, y: i8, width: usize, height: usize) -> bool {
    x >= 0 && (x as usize) < width && y >= 0 && (y as usize) < height
}