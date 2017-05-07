use na::{Point2, Vector2};
use grid::{GridElement, Position, PositionDelta, Direction, adjacent_vertex};

pub type Point = Point2<i32>;
pub type Vector = Vector2<i32>;


pub struct Layout {
    world_origin: Vector,
    grid_origin: GridElement, // a vertex
    grid_size: (u32, u32), // in pixels
}

impl Layout {
    pub fn new(wo: Vector, go: GridElement, w: u32, h: u32) -> Layout {
        Layout {
            world_origin: wo,
            grid_origin: go,
            grid_size: (w, h),
        }
    }

    fn normalize_world_coord(&self, p: Point) -> Point {
        p - self.world_origin
    }

    fn grid_origin_coords(&self) -> (i32, i32) {
        if let GridElement::Vertex(p) = self.grid_origin {
            (p.x, p.y)
        } else {
            unreachable!()
        }
    }

    pub fn world2surface(&self, p: Point) -> GridElement {
        let p = self.normalize_world_coord(p);
        let x = p.coords[0] / self.grid_size.0 as i32;
        let y = p.coords[1] / self.grid_size.1 as i32;
        let grid_origin = self.grid_origin_coords();
        GridElement::Surface(Position::new(x - grid_origin.0, y - grid_origin.1))
    }

    pub fn vertex2world(&self, v: GridElement) -> Point {
        if let GridElement::Vertex(p) = v {
            if let GridElement::Vertex(ref o) = self.grid_origin {
                let np = p - PositionDelta::from_position(o);
                Point::new(np.x * self.grid_size.0 as i32,
                           np.y * self.grid_size.1 as i32) + self.world_origin
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    pub fn bounding_box_of(&self, s: GridElement) -> Option<(i32, i32, u32, u32)> {
        if let GridElement::Surface(_) = s {
            let tl = self.vertex2world(adjacent_vertex(s, Direction::NW).unwrap());
            let br = self.vertex2world(adjacent_vertex(s, Direction::SE).unwrap());
            Some((tl.coords[0],
                  tl.coords[1],
                  (br.coords[0] - tl.coords[0]) as u32,
                  (br.coords[1] - tl.coords[1]) as u32))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world2surface() {
        let layout = Layout::new(Vector::new(13, 2),
                                 GridElement::Vertex(Position::new(2, 0)),
                                 10,
                                 10);
        let w = Point::new(45, 33);
        let s = layout.world2surface(w);
        assert_eq!(GridElement::Surface(Position::new(1, 3)), s);
    }

    #[test]
    fn test_vertex2world() {
        let layout = Layout::new(Vector::new(13, 2),
                                 GridElement::Vertex(Position::new(2, 0)),
                                 10,
                                 10);
        let v = GridElement::Vertex(Position::new(6, 7));
        let p = layout.vertex2world(v);
        assert_eq!(Point::new(53, 72), p);
    }

    #[test]
    fn test_bounding_box() {
        let layout = Layout::new(Vector::new(13, 2),
                                 GridElement::Vertex(Position::new(2, 0)),
                                 10,
                                 10);
        let s = GridElement::Surface(Position::new(3, 4));
        assert_eq!(layout.bounding_box_of(s).unwrap(), (23, 42, 10, 10));
    }
}
