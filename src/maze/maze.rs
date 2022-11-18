use crate::maze::graph::Graph;

type Coords = (isize, isize);

pub struct Maze {
    graph: Graph<Coords>,
}
