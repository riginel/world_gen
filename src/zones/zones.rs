use rand::distributions::{Distribution, Uniform};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use robotics_lib::world::tile::{Content, TileType};
use crate::customization::content_distribution::CityContentDist;
use crate::pathfinding::a_star::{build_road, shortest_priority};
use crate::utils::tile::PreTileType;
use crate::utils::vector2::Vector2;

pub struct Zone{
    pub tile: PreTileType,
    pub inner: Vec<Vector2>,
    pub centroid: Vector2
}
impl Zone{
    fn bfs(world: &mut Vec<Vec<PreTileType>>, id: usize, point: Vector2) -> Self{
        let mut vec = Vec::<Vector2>::new();
        let mut stack = Vec::<Vector2>::new();
        stack.push(point);
        while let Some(p) = stack.pop(){
            let (x,y) = p.as_tuple();
            match world[x][y] {
                PreTileType::Zone(i) =>{
                    if i == id {
                        vec.push(p);
                        stack.append(&mut p.neighbours(world.len(),world[0].len()));
                        world[x][y] = PreTileType::None;
                    }
                }
                _=>{}
            }
        }
        let mut  centroid = Self::compute_centroid(&vec);
        Zone {
            tile: PreTileType::None,
            inner: vec,
            centroid
        }
    }
    fn compute_centroid(points: &Vec<Vector2>) -> Vector2 {
        if points.len()== 0{
            println!("error!!")
        }
        let mut  centroid : Vector2 = points.iter().fold(Vector2::new(0, 0), |acc, p| acc + *p);
        Vector2::new(centroid.x / points.len(), centroid.y / points.len())
    }

    pub fn fill(&mut self, world: &mut Vec<Vec<PreTileType>>, tile: PreTileType){
        self.tile = tile;
        for i in self.inner.iter(){
            world[i.x][i.y] = tile;
        }
    }



}

pub struct Zones{
    coasts: Vec<Zone>,
    cities: Vec<Zone>,
    hills:Vec<Zone>,
    mountains: Vec<Mountain>
}

impl Zones {
    pub fn get_zones(world: &mut Vec<Vec<PreTileType>>) ->Self {
        let mut zones = Zones {
            coasts: vec![],
            cities: vec![],
            hills:vec![],
            mountains: vec![]
        };
        //scrolls the world and finds zones, either coasts or mountains
        for x in 0..world.len(){
            for y in 0..world[0].len(){
                match world[x][y]{

                    PreTileType::Zone(0) => {
                        zones.coasts.push(Zone::bfs(world, 0, Vector2::new(x, y)))
                    },
                    PreTileType::Zone(1)=>{
                        zones.mountains.push(Mountain::new(world, Vector2::new(x, y)))
                    }
                    _ => {}
                }
            }
        }
        //convert some mountains in cities
        zones.mountains.shuffle(&mut rand::thread_rng());
        for i in 0..(zones.mountains.len()*2)/3{
            zones.cities.push(zones.mountains.pop().unwrap().to_city());
        }
        //convert some cities in hills
        for i in 0..zones.cities.len()/2{
            zones.hills.push(zones.cities.pop().unwrap());
        }

        /*
        for i in zones.iter_mut(){
            let mut rng = thread_rng();
            let tile = *[Tile::Sand, Tile::Grass].choose(&mut rng).unwrap();
            i.fill( world,tile);
        }
        */
        zones.fill(world);
        zones.connect_cities(world);
        zones

    }
    fn fill(&mut self, world: &mut Vec<Vec<PreTileType>>){
        let mut rng = thread_rng();
        //filling the coasts with either grass or sand
        for i in self.coasts.iter_mut(){
            let tile = *[PreTileType::Sand, PreTileType::Grass].choose(&mut rng).unwrap();
            i.fill(world,tile);
        }

        for i in self.mountains.iter_mut(){
            i.lava_pools.iter_mut().for_each(|z| z.fill(world, PreTileType::Lava));
            i.zone.fill(world, PreTileType::Mountain)
        }
        for i in self.cities.iter_mut(){
            i.fill(world, PreTileType::Grass);
        }
        for i in self.hills.iter_mut(){
            i.fill(world, PreTileType::Hill);
        }


    }
    fn connect_cities(&mut self, world: &mut Vec<Vec<PreTileType>>){
        /*
        for (index, first_zone) in self.cities.iter().enumerate(){
            for second_zone in self.cities[index.. ].iter(){
                if first_zone.centroid == second_zone.centroid{
                    continue;

                }
                let path = shortest_priority(world, first_zone.centroid, second_zone.centroid);
                if let Ok(path) = path {
                    build_road(world,path);
                }

            }
        }*/
        /*
        for (a,b) in self.cities.iter().zip(self.cities[1..].iter()){
            if a.centroid == b.centroid{
                continue;
            }
            let path = shortest_priority(world, a.centroid,b.centroid);
            if let Ok(path)= path{
                build_road(world,path);
            }
        }

         */
        for i in 0..self.cities.len()-1{
            let  (mut min_index,mut min_distance) = (i+1,self.cities[i+1].centroid.manhattan_distance(self.cities[i].centroid) );
            for j in i+1..self.cities.len(){
                let this_distance = self.cities[j].centroid.manhattan_distance(self.cities[i].centroid);
                if this_distance  <= min_distance{
                    min_distance = this_distance;
                    min_index = j;
                }
            }
            self.cities.swap(i+1, min_index);
            let path = shortest_priority(world,self.cities[i].centroid, self.cities[i+1].centroid);
            if let Ok(path ) = path{
                build_road(world,path);
            }
        }
    }
    pub fn fill_cities_with_content(&self ,world: &mut Vec<Vec<TileType>>,content_vec:&mut Vec<Vec<Content>>,city_content_dist:&CityContentDist){
        let mut rng = thread_rng();
        let mut range_generator = Uniform::new(0,100);

        for i in self.cities.iter(){
            for point in i.inner.iter(){
                if world[point.x][point.y] == TileType::Grass{
                    content_vec[point.x][point.y] = city_content_dist.get_content(range_generator.sample(&mut rng));
                }
            }
        }
    }
}
pub struct Mountain{
    zone:Zone,
    lava_pools: Vec<Zone>
}
impl Mountain{
    fn new(world: &mut Vec<Vec<PreTileType>>, point: Vector2) -> Self{
        let mut zone_vec = Vec::<Vector2>::new();
        let mut lava_pool_vec = Vec::<Zone>::new();
        let mut stack = Vec::<Vector2>::new();

        stack.push(point);
        while let Some(p) = stack.pop(){
            let (x,y) = p.as_tuple();
            match world[x][y] {
                PreTileType::Zone(1) =>{

                        zone_vec.push(p);
                        stack.append(&mut p.neighbours(world.len(),world[0].len()));
                        world[x][y] = PreTileType::None;

                }
                PreTileType::Zone(2)=>{

                    lava_pool_vec.push(Zone::bfs(world,2,p));
                }
                _=>{}
            }
        }
        let centroid = Zone::compute_centroid(&zone_vec);
        Self{
            zone: Zone{
                tile: PreTileType::None,
                inner: zone_vec,
                centroid
            },
            lava_pools: lava_pool_vec,
        }

    }
    fn to_city(mut self) -> Zone{
        let mut vec = Vec::new();
        vec.append(&mut self.zone.inner);
        for i in self.lava_pools.iter_mut(){
            vec.append(&mut i.inner);
        }

        Zone{
            tile: PreTileType::None,
            centroid: Zone::compute_centroid(&vec),
            inner: vec
        }
    }

}