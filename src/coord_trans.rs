use na::{Point2, Vector2};
use location::{Location, Position, PositionDelta};

type Point2f = Point2<f32>;
type Vector2f = Vector2<f32>;


pub struct Layout {
    world_origin: Vector2f,
    grid_origin: Location,  // a vertex
    grid_size: (f32, f32),
}

impl Layout {
    pub fn new(so: Vector2f, go: Location, w: f32, h: f32) -> Layout {
        Layout {
            world_origin: so,
            grid_origin: go,
            grid_size: (w, h),
        }
    }

    fn normalize_world_coord(&self, p: Point2f) -> Point2f {
        p - self.world_origin
    }

    pub fn world2grid(&self, p: Point2f) -> Location {
        let p = self.normalize_world_coord(p);
        let x = (p.coords[0] / self.grid_size.0) as i32;
        let y = (p.coords[1] / self.grid_size.1) as i32;
        Location::Surface(Position::new(x, y))
    }

    pub fn vertex2world(&self, v: Location) -> Point2f {
        if let Location::Vertex(p) = v {
            if let Location::Vertex(ref o) = self.grid_origin {
                let np = p - PositionDelta::from_position(o);
                Point2f::new(np.x as f32 * self.grid_size.0, np.y as f32 * self.grid_size.1) + self.world_origin
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}