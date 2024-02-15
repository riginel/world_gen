use super::{tile::{PreTile, PreTileType}, vector2::Vector2};

pub (crate) struct Zone {
    pub inner: Vec<Vector2>,
    pub centroid: Vector2,
}

impl Zone {
    pub fn dfs(world: &mut Vec<Vec<PreTile>>, id: usize, point: Vector2) -> Self {
        let mut vec = Vec::<Vector2>::new();
        let mut stack = Vec::<Vector2>::new();
        stack.push(point);
        while let Some(p) = stack.pop() {
            let (x, y) = p.as_tuple();
            match world[x][y].pre_tiletype {
                PreTileType::Zone(i) => {
                    if i == id {
                        vec.push(p);
                        stack.append(&mut p.neighbours(world.len(), world[0].len()));
                        world[x][y].pre_tiletype = PreTileType::Void;
                    }
                }
                _ => {}
            }
        }
        let centroid = Self::compute_centroid(&vec);
        Zone {
            inner: vec,
            centroid,
        }
    }

    fn compute_centroid(points: &Vec<Vector2>) -> Vector2 {
        assert!(!points.is_empty());
        let centroid: Vector2 = points.iter().fold(Vector2::new(0, 0), |acc, p| acc + *p);
        Vector2::new(centroid.x / points.len(), centroid.y / points.len())
    }
}