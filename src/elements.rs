use std::ops::{Add, Sub, Neg};


#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn origin() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }

    pub fn distance_to(&self, other: Position) -> PositionDelta {
        PositionDelta {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct PositionDelta {
    pub dx: i32,
    pub dy: i32,
}

impl PositionDelta {
    pub fn new(x: i32, y: i32) -> PositionDelta {
        PositionDelta { dx: x, dy: y }
    }

    pub fn from_position(p: Position) -> PositionDelta {
        PositionDelta { dx: p.x, dy: p.y }
    }
}

impl Add<PositionDelta> for Position {
    type Output = Position;
    fn add(self, other: PositionDelta) -> Position {
        Position::new(self.x + other.dx, self.y + other.dy)
    }
}

impl<'a> Add<PositionDelta> for &'a Position {
    type Output = Position;
    fn add(self, other: PositionDelta) -> Position {
        Position::new(self.x + other.dx, self.y + other.dy)
    }
}

impl<'a> Add<&'a PositionDelta> for Position {
    type Output = Position;
    fn add(self, other: &PositionDelta) -> Position {
        Position::new(self.x + other.dx, self.y + other.dy)
    }
}

impl<'a, 'b> Add<&'a PositionDelta> for &'b Position {
    type Output = Position;
    fn add(self, other: &PositionDelta) -> Position {
        Position::new(self.x + other.dx, self.y + other.dy)
    }
}

impl Sub<PositionDelta> for Position {
    type Output = Position;
    fn sub(self, other: PositionDelta) -> Position {
        Position::new(self.x - other.dx, self.y - other.dy)
    }
}

impl<'a> Sub<PositionDelta> for &'a Position {
    type Output = Position;
    fn sub(self, other: PositionDelta) -> Position {
        Position::new(self.x - other.dx, self.y - other.dy)
    }
}

impl<'a> Sub<&'a PositionDelta> for Position {
    type Output = Position;
    fn sub(self, other: &PositionDelta) -> Position {
        Position::new(self.x - other.dx, self.y - other.dy)
    }
}

impl<'a, 'b> Sub<&'a PositionDelta> for &'b Position {
    type Output = Position;
    fn sub(self, other: &PositionDelta) -> Position {
        Position::new(self.x - other.dx, self.y - other.dy)
    }
}


#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Direction {
    West,
    South,
    East,
    North,
    NE,
    NW,
    SW,
    SE,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum EdgeType {
    Horizontal,
    Vertical,
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::NW => Direction::SE,
            Direction::SW => Direction::NE,
            Direction::SE => Direction::NW,
            Direction::NE => Direction::SW,
        }
    }
}

impl Neg for EdgeType {
    type Output = EdgeType;
    fn neg(self) -> EdgeType {
        match self {
            EdgeType::Vertical => EdgeType::Horizontal,
            EdgeType::Horizontal => EdgeType::Vertical,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Tile {
    pub pos: Position,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Edge {
    pub pos: Position,
    pub t: EdgeType,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Vertex {
    pub pos: Position,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Tile {
        Tile { pos: Position { x, y } }
    }

    pub fn from_position(pos: Position) -> Tile {
        Tile { pos }
    }

    pub fn adjacent_tile(&self, d: Direction) -> Tile {
        Tile::from_position(self.pos + direction_delta(d))
    }

    /// 返回相邻的所有块。
    ///
    /// 从西北方开始顺时针旋转，即
    /// 0 1 2
    /// 7 * 3
    /// 6 5 4
    /// 注意：不进行边界检查。
    pub fn neighbors(&self) -> Vec<Tile> {
        vec![self.adjacent_tile(Direction::NW),
             self.adjacent_tile(Direction::North),
             self.adjacent_tile(Direction::NE),
             self.adjacent_tile(Direction::East),
             self.adjacent_tile(Direction::SE),
             self.adjacent_tile(Direction::South),
             self.adjacent_tile(Direction::SW),
             self.adjacent_tile(Direction::West)]
    }

    pub fn adjacent_edge(&self, d: Direction) -> Edge {
        match d {
            Direction::North => Edge::from_position(self.pos, EdgeType::Horizontal),
            Direction::West => Edge::from_position(self.pos, EdgeType::Vertical),
            Direction::South => {
                Edge::from_position(self.pos + direction_delta(d), EdgeType::Horizontal)
            }
            Direction::East => {
                Edge::from_position(self.pos + direction_delta(d), EdgeType::Vertical)
            }
            _ => unreachable!(),
        }
    }

    ///   0
    /// 3 * 1
    ///   2
    pub fn edges(&self) -> Vec<Edge> {
        vec![self.adjacent_edge(Direction::North),
             self.adjacent_edge(Direction::East),
             self.adjacent_edge(Direction::South),
             self.adjacent_edge(Direction::West)]
    }

    pub fn adjacent_vertex(&self, d: Direction) -> Vertex {
        match d {
            Direction::NW => Vertex::from_position(self.pos),
            Direction::SW => Vertex::from_position(self.pos + direction_delta(Direction::South)),
            Direction::SE => Vertex::from_position(self.pos + direction_delta(d)),
            Direction::NE => Vertex::from_position(self.pos + direction_delta(Direction::East)),
            _ => unreachable!(),
        }
    }

    /// 0   1
    ///   *
    /// 3   2
    pub fn vertices(&self) -> Vec<Vertex> {
        vec![self.adjacent_vertex(Direction::NW),
             self.adjacent_vertex(Direction::NE),
             self.adjacent_vertex(Direction::SE),
             self.adjacent_vertex(Direction::SW)]
    }
}


impl Edge {
    pub fn new(x: i32, y: i32, t: EdgeType) -> Edge {
        Edge {
            pos: Position { x, y },
            t: t,
        }
    }

    pub fn from_position(pos: Position, t: EdgeType) -> Edge {
        Edge { pos, t }
    }

    pub fn adjacent_tile(&self, d: Direction) -> Tile {
        match (self.t, d) {
            (EdgeType::Vertical, Direction::West) |
            (EdgeType::Horizontal, Direction::North) => {
                Tile::from_position(self.pos + direction_delta(d))
            }
            (EdgeType::Vertical, Direction::East) |
            (EdgeType::Horizontal, Direction::South) => Tile::from_position(self.pos),
            _ => unreachable!(),
        }
    }

    pub fn adjacent_edge(&self, d: Direction) -> Edge {
        match (self.t, d) {
            (EdgeType::Vertical, Direction::North) |
            (EdgeType::Vertical, Direction::South) |
            (EdgeType::Horizontal, Direction::West) |
            (EdgeType::Horizontal, Direction::East) => {
                Edge::from_position(self.pos + direction_delta(d), self.t)
            }
            (EdgeType::Vertical, Direction::NE) |
            (EdgeType::Horizontal, Direction::SW) => Edge::from_position(self.pos, -self.t),
            (EdgeType::Vertical, Direction::SW) |
            (EdgeType::Horizontal, Direction::NE) => {
                Edge::from_position(self.pos + direction_delta(d), -self.t)
            }
            (EdgeType::Vertical, Direction::NW) => {
                Edge::from_position(self.pos + direction_delta(Direction::West), -self.t)
            }
            (EdgeType::Vertical, Direction::SE) => {
                Edge::from_position(self.pos + direction_delta(Direction::South), -self.t)
            }
            (EdgeType::Horizontal, Direction::NW) => {
                Edge::from_position(self.pos + direction_delta(Direction::North), -self.t)
            }
            (EdgeType::Horizontal, Direction::SE) => {
                Edge::from_position(self.pos + direction_delta(Direction::East), -self.t)
            }
            _ => unreachable!(),
        }
    }

    pub fn adjacent_vertex(&self, d: Direction) -> Vertex {
        match (self.t, d) {
            (EdgeType::Vertical, Direction::South) |
            (EdgeType::Horizontal, Direction::East) => {
                Vertex::from_position(self.pos + direction_delta(d))
            }
            (EdgeType::Vertical, Direction::North) |
            (EdgeType::Horizontal, Direction::West) => Vertex::from_position(self.pos),
            _ => unreachable!(),
        }
    }
}


impl Vertex {
    pub fn new(x: i32, y: i32) -> Vertex {
        Vertex { pos: Position { x, y } }
    }

    pub fn from_position(pos: Position) -> Vertex {
        Vertex { pos }
    }

    pub fn adjacent_tile(&self, d: Direction) -> Tile {
        match d {
            Direction::NW => Tile::from_position(self.pos + direction_delta(d)),
            Direction::SW => Tile::from_position(self.pos + direction_delta(Direction::West)),
            Direction::SE => Tile::from_position(self.pos),
            Direction::NE => Tile::from_position(self.pos + direction_delta(Direction::North)),
            _ => unreachable!(),
        }
    }

    pub fn adjacent_edge(&self, d: Direction) -> Edge {
        match d {
            Direction::North => {
                Edge::from_position(self.pos + direction_delta(d), EdgeType::Vertical)
            }
            Direction::South => Edge::from_position(self.pos, EdgeType::Vertical),
            Direction::West => {
                Edge::from_position(self.pos + direction_delta(d), EdgeType::Horizontal)
            }
            Direction::East => Edge::from_position(self.pos, EdgeType::Horizontal),
            _ => unreachable!(),
        }
    }
}

fn direction_delta(d: Direction) -> PositionDelta {
    match d {
        Direction::North => PositionDelta::new(0, -1),
        Direction::West => PositionDelta::new(-1, 0),
        Direction::South => PositionDelta::new(0, 1),
        Direction::East => PositionDelta::new(1, 0),
        Direction::NW => PositionDelta::new(-1, -1),
        Direction::SW => PositionDelta::new(-1, 1),
        Direction::SE => PositionDelta::new(1, 1),
        Direction::NE => PositionDelta::new(1, -1),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to() {
        let pm = Position::new(5, 8);
        let pn = Position::new(2, 9);
        assert_eq!(pn.distance_to(pm), PositionDelta::new(-3, 1));
    }

    #[test]
    fn test_tile() {
        let p1 = Position::new(-1, -1);
        let p2 = Position::new(0, -1);
        let p3 = Position::new(1, -1);
        let p4 = Position::new(-1, 0);
        let p5 = Position::new(0, 0);
        let p6 = Position::new(1, 0);
        let p7 = Position::new(-1, 1);
        let p8 = Position::new(0, 1);
        let p9 = Position::new(1, 1);

        assert_eq!(Tile::from_position(p4),
                   Tile::from_position(p8).adjacent_tile(Direction::NW));
        assert_eq!(Tile::from_position(p5),
                   Tile::from_position(p2).adjacent_tile(Direction::South));
        assert_eq!(Vertex::from_position(p6),
                   Tile::from_position(p3).adjacent_vertex(Direction::SW));
        assert_eq!(Vertex::from_position(p5),
                   Tile::from_position(p1).adjacent_vertex(Direction::SE));
        assert_eq!(Edge::from_position(p9, EdgeType::Vertical),
                   Tile::from_position(p8).adjacent_edge(Direction::East));
        assert_eq!(Edge::from_position(p7, EdgeType::Horizontal),
                   Tile::from_position(p4).adjacent_edge(Direction::South));
    }

    #[test]
    fn test_edge() {
        let p1 = Position::new(-1, -1);
        let p2 = Position::new(0, -1);
        let p3 = Position::new(1, -1);
        let p4 = Position::new(-1, 0);
        let p5 = Position::new(0, 0);
        let p6 = Position::new(1, 0);
        let p7 = Position::new(-1, 1);
        let p8 = Position::new(0, 1);
        let p9 = Position::new(1, 1);

        assert_eq!(Tile::from_position(p3),
                   Edge::from_position(p6, EdgeType::Horizontal).adjacent_tile(Direction::North));
        assert_eq!(Tile::from_position(p4),
                   Edge::from_position(p5, EdgeType::Vertical).adjacent_tile(Direction::West));
        assert_eq!(Edge::from_position(p7, EdgeType::Horizontal),
                   Edge::from_position(p5, EdgeType::Vertical).adjacent_edge(Direction::SW));
        assert_eq!(Edge::from_position(p9, EdgeType::Horizontal),
                   Edge::from_position(p8, EdgeType::Horizontal).adjacent_edge(Direction::East));
        assert_eq!(Vertex::from_position(p1),
                   Edge::from_position(p1, EdgeType::Horizontal).adjacent_vertex(Direction::West));
        assert_eq!(Vertex::from_position(p2),
                   Edge::from_position(p1, EdgeType::Horizontal).adjacent_vertex(Direction::East));
    }

    #[test]
    fn test_vertex() {
        let p1 = Position::new(-1, -1);
        let p2 = Position::new(0, -1);
        let p3 = Position::new(1, -1);
        let p4 = Position::new(-1, 0);
        let p5 = Position::new(0, 0);
        let p6 = Position::new(1, 0);
        let p7 = Position::new(-1, 1);
        let p8 = Position::new(0, 1);
        let p9 = Position::new(1, 1);

        assert_eq!(Tile::from_position(p1),
                   Vertex::from_position(p4).adjacent_tile(Direction::NE));
        assert_eq!(Tile::from_position(p5),
                   Vertex::from_position(p6).adjacent_tile(Direction::SW));
        assert_eq!(Tile::from_position(p4),
                   Vertex::from_position(p8).adjacent_tile(Direction::NW));
        assert_eq!(Edge::from_position(p6, EdgeType::Vertical),
                   Vertex::from_position(p9).adjacent_edge(Direction::North));
        assert_eq!(Edge::from_position(p2, EdgeType::Horizontal),
                   Vertex::from_position(p3).adjacent_edge(Direction::West));
        assert_eq!(Edge::from_position(p7, EdgeType::Vertical),
                   Vertex::from_position(p7).adjacent_edge(Direction::South));
    }

}
