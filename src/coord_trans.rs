use cgmath::{Point2, Vector2};
use elements::{PositionDelta, Direction, Tile, Vertex};

pub type Point = Point2<f32>;
pub type Vector = Vector2<f32>;


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
        p + -self.world_origin
    }

    pub fn world2tile(&self, p: Point) -> Tile {
        let p = self.normalize_world_coord(p);
        let x = (p.x / self.grid_size.0 as f32) as i32;
        let y = (p.y / self.grid_size.1 as f32) as i32;
        Tile::new(x - self.grid_origin.pos.x, y - self.grid_origin.pos.y)
    }

    pub fn vertex2world(&self, v: Vertex) -> Point {
        let np = v.pos - PositionDelta::from_position(self.grid_origin.pos);
        Point::new(np.x as f32 * self.grid_size.0 as f32,
                   np.y as f32 * self.grid_size.1 as f32) + self.world_origin
    }

    pub fn bounding_box_of(&self, s: Tile) -> (f32, f32, u32, u32) {
        let tl = self.vertex2world(s.adjacent_vertex(Direction::NW));
        (tl.x, tl.y, self.grid_size.0, self.grid_size.1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world2tile() {
        let layout = Layout::new(Vector::new(13.0, 2.0), Vertex::new(2, 0), 10, 10);
        let w = Point::new(45.0, 33.0);
        let s = layout.world2tile(w);
        assert_eq!(Tile::new(1, 3), s);
    }

    #[test]
    fn test_vertex2world() {
        let layout = Layout::new(Vector::new(13.0, 2.0), Vertex::new(2, 0), 10, 10);
        let v = Vertex::new(6, 7);
        let p = layout.vertex2world(v);
        assert_eq!(Point::new(53.0, 72.0), p);
    }

    #[test]
    fn test_bounding_box() {
        let layout = Layout::new(Vector::new(13.0, 2.0), Vertex::new(2, 0), 10, 10);
        let s = Tile::new(3, 4);
        assert_eq!(layout.bounding_box_of(s), (23.0, 42.0, 10, 10));
    }
}
