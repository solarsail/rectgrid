use std::ops::{Add, Sub, Neg};


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn origin() -> Position {
        Position {
            x: 0,
            y: 0,
        }
    }

    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
        }
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
        PositionDelta {
            dx: x,
            dy: y,
        }
    }

    pub fn from_position(p: &Position) -> PositionDelta {
        PositionDelta {
            dx: p.x,
            dy: p.y,
        }
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


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    West, South, East, North,
    NE, NW, SW, SE,
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

#[derive(PartialEq, Eq, Debug)]
pub enum GridElement {
    Vertex(Position),
    Surface(Position),
    Edge(Position, Direction),
}


fn direction_delta(d: Direction) -> PositionDelta {
    match d {
        Direction::North => PositionDelta::new( 0, -1),
        Direction::West  => PositionDelta::new(-1,  0),
        Direction::South => PositionDelta::new( 0,  1),
        Direction::East  => PositionDelta::new( 1,  0),
        Direction::NW => PositionDelta::new(-1, -1),
        Direction::SW => PositionDelta::new(-1,  1),
        Direction::SE => PositionDelta::new( 1,  1),
        Direction::NE => PositionDelta::new( 1, -1),
    }
}

pub fn neighbor(l: GridElement, d: Direction) -> GridElement {
    match l {
        GridElement::Vertex(p) => GridElement::Vertex(p + direction_delta(d)),
        GridElement::Surface(p) => GridElement::Surface(p + direction_delta(d)),
        GridElement::Edge(p, ed) => match d {
            Direction::North | Direction::West | Direction::South | Direction::East => GridElement::Edge(p + direction_delta(d), ed),
            _ => match ed {
                Direction::West => {
                    let v = GridElement::Vertex(p);
                    let s = adjacent_surface(v, d).unwrap();
                    adjacent_edge(s, Direction::South).unwrap()
                }
                Direction::North => {
                    let v = adjacent_vertex(l, Direction::East).unwrap();
                    let s = adjacent_surface(v, d).unwrap();
                    adjacent_edge(s, Direction::West).unwrap()
                }
                _ => unreachable!()
            }
        }
    }
}


pub fn adjacent_surface(l: GridElement, d: Direction) -> Option<GridElement> {
    match l {
        GridElement::Surface(_) => Some(neighbor(l, d)),
        GridElement::Vertex(p) => match d {
            Direction::NW => Some(GridElement::Surface(p+direction_delta(d))),
            Direction::SW => Some(GridElement::Surface(p+direction_delta(Direction::West))),
            Direction::SE => Some(GridElement::Surface(p)),
            Direction::NE => Some(GridElement::Surface(p+direction_delta(Direction::North))),
            _ => None
        },
        GridElement::Edge(p, ed) => match ed {
            Direction::West => match d {
                Direction::West => Some(GridElement::Surface(p+direction_delta(d))),
                Direction::East => Some(GridElement::Surface(p)),
                _ => None
            },
            Direction::North => match d {
                Direction::North => Some(GridElement::Surface(p+direction_delta(d))),
                Direction::South => Some(GridElement::Surface(p)),
                _ => None
            },
            _ => None
        }
    }
}

pub fn adjacent_edge(l: GridElement, d: Direction) -> Option<GridElement> {
    match l {
        GridElement::Surface(p) => {
            match d {
                Direction::North | Direction::West => Some(GridElement::Edge(p, d)),
                Direction::South | Direction::East => Some(GridElement::Edge(p+direction_delta(d), -d)),
                _ => None
            }
        }
        GridElement::Edge(p, ed) => {
            match ed {
                Direction::West => {
                    match d {
                        Direction::North | Direction::South => Some(GridElement::Edge(p+direction_delta(d), ed)),
                        _ => None
                    }
                }
                Direction::North => {
                    match d {
                        Direction::West | Direction::East => Some(GridElement::Edge(p+direction_delta(d), ed)),
                        _ => None
                    }
                }
                _ => None
            }
        }
        GridElement::Vertex(p) => {
            match d {
                Direction::North => Some(GridElement::Edge(p+direction_delta(d), Direction::West)),
                Direction::South => Some(GridElement::Edge(p, Direction::West)),
                Direction::West => Some(GridElement::Edge(p+direction_delta(d), Direction::North)),
                Direction::East => Some(GridElement::Edge(p, Direction::North)),
                _ => None
            }
        }
    }
}

pub fn adjacent_vertex(l: GridElement,  d: Direction) -> Option<GridElement> {
    match l {
        GridElement::Surface(p) => match d {
            Direction::NW => Some(GridElement::Vertex(p)),
            Direction::SW => Some(GridElement::Vertex(p+direction_delta(Direction::South))),
            Direction::SE => Some(GridElement::Vertex(p+direction_delta(d))),
            Direction::NE => Some(GridElement::Vertex(p+direction_delta(Direction::East))),
            _ => None
        },
        GridElement::Edge(p, ed) => match ed {
            Direction::West => match d {
                Direction::North => Some(GridElement::Vertex(p)),
                Direction::South => Some(GridElement::Vertex(p+direction_delta(d))),
                _ => None
            },
            Direction::North => match d {
                Direction::West => Some(GridElement::Vertex(p)),
                Direction::East => Some(GridElement::Vertex(p+direction_delta(d))),
                _ => None
            },
            _ => None
        },
        _ => None
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
    fn test_neighbor() {
        let p1 = Position::new(-1, -1);
        let p2 = Position::new( 0, -1);
        let p3 = Position::new( 1, -1);
        let p4 = Position::new(-1,  0);
        let p5 = Position::new( 0,  0);
        let p6 = Position::new( 1,  0);
        let p7 = Position::new(-1,  1);
        let p8 = Position::new( 0,  1);
        let p9 = Position::new( 1,  1);

        assert_eq!(GridElement::Surface(p4), neighbor(GridElement::Surface(p8), Direction::NW));
        assert_eq!(GridElement::Surface(p5), neighbor(GridElement::Surface(p2), Direction::South));
        assert_eq!(GridElement::Vertex(p7), neighbor(GridElement::Vertex(p8), Direction::West));
        assert_eq!(GridElement::Vertex(p3), neighbor(GridElement::Vertex(p5), Direction::NE));
        assert_eq!(GridElement::Edge(p8, Direction::North), neighbor(GridElement::Edge(p6, Direction::West), Direction::SW));
        assert_eq!(GridElement::Edge(p2, Direction::West), neighbor(GridElement::Edge(p1, Direction::North), Direction::SE));
        assert_eq!(GridElement::Edge(p9, Direction::North), neighbor(GridElement::Edge(p6, Direction::North), Direction::South));
    }

    #[test]
    fn test_adjcent_surface() {
        let p1 = Position::new(-1, -1);
        let p2 = Position::new( 0, -1);
        let p3 = Position::new( 1, -1);
        let p4 = Position::new(-1,  0);
        let p5 = Position::new( 0,  0);
        let p6 = Position::new( 1,  0);
        let p7 = Position::new(-1,  1);
        let p8 = Position::new( 0,  1);
        let p9 = Position::new( 1,  1);

        assert_eq!(GridElement::Surface(p3), adjacent_surface(GridElement::Surface(p2), Direction::East).unwrap());
        assert_eq!(GridElement::Surface(p4), adjacent_surface(GridElement::Surface(p2), Direction::SW).unwrap());
        assert_eq!(GridElement::Surface(p1), adjacent_surface(GridElement::Edge(p4, Direction::North), Direction::North).unwrap());
        assert_eq!(GridElement::Surface(p5), adjacent_surface(GridElement::Vertex(p9), Direction::NW).unwrap());
        assert_eq!(GridElement::Surface(p7), adjacent_surface(GridElement::Vertex(p8), Direction::SW).unwrap());
        assert_eq!(GridElement::Surface(p6), adjacent_surface(GridElement::Edge(p6, Direction::West), Direction::East).unwrap());
    }

    #[test]
    fn test_adjcent_edge() {
        let p1 = Position::new(-1, -1);
        let p2 = Position::new( 0, -1);
        let p3 = Position::new( 1, -1);
        let p4 = Position::new(-1,  0);
        let p5 = Position::new( 0,  0);
        let p6 = Position::new( 1,  0);
        let p7 = Position::new(-1,  1);
        let p8 = Position::new( 0,  1);
        let p9 = Position::new( 1,  1);

        assert_eq!(GridElement::Edge(p1, Direction::West), adjacent_edge(GridElement::Vertex(p4), Direction::North).unwrap());
        assert_eq!(GridElement::Edge(p2, Direction::North), adjacent_edge(GridElement::Edge(p3, Direction::North), Direction::West).unwrap());
        assert_eq!(GridElement::Edge(p5, Direction::North), adjacent_edge(GridElement::Vertex(p6), Direction::West).unwrap());
        assert_eq!(GridElement::Edge(p8, Direction::West), adjacent_edge(GridElement::Surface(p7), Direction::East).unwrap());
        assert_eq!(GridElement::Edge(p9, Direction::North), adjacent_edge(GridElement::Surface(p6), Direction::South).unwrap());
    }

    #[test]
    fn test_adjcent_vertex() {
        let p1 = Position::new(-1, -1);
        let p2 = Position::new( 0, -1);
        let p3 = Position::new( 1, -1);
        let p4 = Position::new(-1,  0);
        let p5 = Position::new( 0,  0);
        let p6 = Position::new( 1,  0);
        let p7 = Position::new(-1,  1);
        let p8 = Position::new( 0,  1);
        let p9 = Position::new( 1,  1);

        assert_eq!(GridElement::Vertex(p4), adjacent_vertex(GridElement::Edge(p1, Direction::West), Direction::South).unwrap());
        assert_eq!(GridElement::Vertex(p3), adjacent_vertex(GridElement::Surface(p2), Direction::NE).unwrap());
        assert_eq!(GridElement::Vertex(p9), adjacent_vertex(GridElement::Surface(p6), Direction::SW).unwrap());
        assert_eq!(GridElement::Vertex(p8), adjacent_vertex(GridElement::Edge(p7, Direction::North), Direction::East).unwrap());
        assert_eq!(GridElement::Vertex(p5), adjacent_vertex(GridElement::Surface(p5), Direction::NW).unwrap());
    }
}
