pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
}

pub enum Direction {
    West, South, East, North,
}

pub enum Location {
    Vertex(Position),
    Surface(Position),
    Edge(Position, Direction),
}


pub fn neighbor(l: Location, d: Direction) -> Location {
    let delta = match d {
        Direction::North => Position::new( 0, -1),
        Direction::West  => Position::new(-1,  0),
        Direction::South => Position::new( 0,  1),
        Direction::East  => Position::new( 1,  0),
    }

    let np = p + delta;

    match l {
        Location::Vertex(p) => Location::Vertex(np),
        Location::Surface(p) => Location::Surface(np),
        Location::Edge(p, ed) => Location::Edge(np, ed),
    }
}