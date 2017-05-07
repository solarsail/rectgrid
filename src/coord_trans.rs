use na::{Point2, Vector2};
use grid::{GridElement, Position, PositionDelta};

pub type Point2f = Point2<f32>;
pub type Vector2f = Vector2<f32>;


pub struct Layout {
    world_origin: Vector2f,
    grid_origin: GridElement,  // a vertex
    grid_size: (f32, f32),
}

impl Layout {
    pub fn new(wo: Vector2f, go: GridElement, w: f32, h: f32) -> Layout {
        Layout {
            world_origin: wo,
            grid_origin: go,
            grid_size: (w, h),
        }
    }

    fn normalize_world_coord(&self, p: Point2f) -> Point2f {
        p - self.world_origin
    }

    fn grid_origin_coords(&self) -> (i32, i32) {
        if let GridElement::Vertex(p) = self.grid_origin {
            (p.x, p.y)
        } else {
            unreachable!()
        }
    }

    pub fn world2surface(&self, p: Point2f) -> GridElement {
        let p = self.normalize_world_coord(p);
        let x = (p.coords[0] / self.grid_size.0) as i32;
        let y = (p.coords[1] / self.grid_size.1) as i32;
        let grid_origin = self.grid_origin_coords();
        GridElement::Surface(Position::new(x - grid_origin.0, y - grid_origin.1))
    }

    pub fn vertex2world(&self, v: GridElement) -> Point2f {
        if let GridElement::Vertex(p) = v {
            if let GridElement::Vertex(ref o) = self.grid_origin {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world2surface() {
        let layout = Layout::new(Vector2f::new(13.5, 2.0), GridElement::Vertex(Position::new(2, 0)), 10.0, 10.0);
        let w = Point2f::new(45.6, 33.4);
        let s = layout.world2surface(w);
        assert_eq!(GridElement::Surface(Position::new(1, 3)), s);
    }

    #[test]
    fn test_vertex2world() {
        let layout = Layout::new(Vector2f::new(13.5, 2.0), GridElement::Vertex(Position::new(2, 0)), 10.0, 10.0);
        let v = GridElement::Vertex(Position::new(6, 7));
        let p = layout.vertex2world(v);
        assert_eq!(Point2f::new(53.5, 72.0), p);
    }
}