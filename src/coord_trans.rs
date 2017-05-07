use na::{Point2, Vector2};
use elements::{PositionDelta, Direction, Tile, Vertex};

pub type Point = Point2<i32>;
pub type Vector = Vector2<i32>;


pub struct Layout {
    world_origin: Vector,
    grid_origin: Vertex,
    grid_size: (u32, u32), // in pixels
}

impl Layout {
    pub fn new(wo: Vector, go: Vertex, w: u32, h: u32) -> Layout {
        Layout {
            world_origin: wo,
            grid_origin: go,
            grid_size: (w, h),
        }
    }

    fn normalize_world_coord(&self, p: Point) -> Point {
        p - self.world_origin
    }

    pub fn world2tile(&self, p: Point) -> Tile {
        let p = self.normalize_world_coord(p);
        let x = p.coords[0] / self.grid_size.0 as i32;
        let y = p.coords[1] / self.grid_size.1 as i32;
        Tile::new(x - self.grid_origin.pos.x, y - self.grid_origin.pos.y)
    }

    pub fn vertex2world(&self, v: Vertex) -> Point {
        let np = v.pos - PositionDelta::from_position(self.grid_origin.pos);
        Point::new(np.x * self.grid_size.0 as i32,
                   np.y * self.grid_size.1 as i32) + self.world_origin
    }

    pub fn bounding_box_of(&self, s: Tile) -> (i32, i32, u32, u32) {
        let tl = self.vertex2world(s.adjacent_vertex(Direction::NW));
        (tl.coords[0], tl.coords[1], self.grid_size.0, self.grid_size.1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world2tile() {
        let layout = Layout::new(Vector::new(13, 2), Vertex::new(2, 0), 10, 10);
        let w = Point::new(45, 33);
        let s = layout.world2tile(w);
        assert_eq!(Tile::new(1, 3), s);
    }

    #[test]
    fn test_vertex2world() {
        let layout = Layout::new(Vector::new(13, 2), Vertex::new(2, 0), 10, 10);
        let v = Vertex::new(6, 7);
        let p = layout.vertex2world(v);
        assert_eq!(Point::new(53, 72), p);
    }

    #[test]
    fn test_bounding_box() {
        let layout = Layout::new(Vector::new(13, 2), Vertex::new(2, 0), 10, 10);
        let s = Tile::new(3, 4);
        assert_eq!(layout.bounding_box_of(s), (23, 42, 10, 10));
    }
}
