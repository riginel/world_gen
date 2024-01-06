use std::cmp::{Ordering, Reverse};
use std::collections::HashSet;
use std::hash::Hash;
use priority_queue::PriorityQueue;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::utils::tile::PreTileType;
use crate::utils::vector2::Vector2;

pub trait Walkable {
    fn cost(&self) -> u32;
}

#[derive(Debug)]
struct Node{
    //the coordinates of the point on the map
    coords: Vector2,
    //the node's parent's coordinates in the shortest path
    parent:Option<Vector2>,
    //g value: represents the shortest distance from the start point
    g: u32,
    //h value:(optimistic) estimated distance from the end point
    h:u32,
    //weight: the weight of the node
    weight:u32
}
impl Node {
    fn new(coords: Vector2, g:u32, h:u32, weight:u32) ->Self{

        Self {
            coords,
            parent:None,
            g,
            h,
            weight: weight
        }
    }
    //function used to initialize a node to a default value
    fn init(coords: Vector2, weight:u32, end: Vector2) -> Self{
        Self {
            coords,
            parent:None,
            g: 1000000,
            h: coords.manhattan_distance(end),
            weight:weight*10
        }
    }

    fn f_cost(&self) -> u32{
        self.g_cost()+self.h
    }
    //function that connects the Node to a parent,
    fn connect(&mut self, parent: Vector2, parent_cost: u32,){
        self.parent = Some(parent);
        self.g = parent_cost;
        /*
        let (c_tuple, e_tuple) = (self.coords.as_i32_tuple(),end.as_i32_tuple());
        let h = (c_tuple.0 - e_tuple.0).abs()+(c_tuple.1 - e_tuple.1).abs();
        */


    }

    fn g_cost(&self) -> u32{
        self.g +self.weight
    }

}


impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost() == other.f_cost()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.f_cost().partial_cmp(&other.f_cost())
    }
}

//function that finds shortest path using a_star
pub fn shortest_path<T>(map: &Vec<Vec<T>>, start: Vector2, end: Vector2) -> Result<Vec<Vector2>, String>
where
    T: Walkable,
{
    //check if path gets out of bounds
    if map.len() == 0 || map[0].len()==0 {
        return Err("Invalid Map".to_string())
    }
    let (width,height) = (map.len(),map[0].len());
    if !(0 <= start.x && start.x < width && 0 <=start.y && start.y < height){
        return Err("Out of bounds".to_string());
    }
    //initializes the node map with the coordinates and default values
    let mut node_map: Vec<Vec<Node>> = map.iter().enumerate().map(|(col,v)|
        v.iter().enumerate().map(|(row,w)| Node::init(Vector2::new(col, row), w.cost(), end)  ).collect()).collect();
    node_map[start.x][start.y] = Node::new(start,0,0,0);

    //the frontier of the pathfinding algorithm
    let mut open_list = HashSet::<Vector2>::new();
    //nodes already verified
    let mut closed_list = HashSet::<Vector2>::new();
    //add start to open list
    open_list.insert(start);

    loop{
        //gets point with smallest f_cost
        let current = take_min(&mut open_list,&node_map);
        closed_list.insert(current);
        if current == end{
            break;
        }

        for i in neighbours(current,width,height).iter(){
            if closed_list.contains(i) {
                continue;
            }
            if !open_list.contains(i) || node_map[i.x][i.y].g_cost() >node_map[start.x][start.y].g_cost() + node_map[i.x][i.y].weight {
                let parent_cost = node_map[start.x][start.y].g_cost();

                node_map[i.x][i.y].connect(current,parent_cost);

                open_list.insert(i.clone());


            }


        }
    }

    let mut path: Vec<Vector2> = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        if let Some(p) = node_map[current.x][current.y].parent{
            current = p;
        }else {
            return Err("Invalid Path :(".to_string());
        }
    }
    println!("cost sum : {}", path.iter().fold(0,|x,p| x+node_map[p.x][p.y].f_cost()));

    Ok(path)
}
pub fn shortest_priority<T>(map: &Vec<Vec<T>>, start: Vector2, end: Vector2) -> Result<Vec<Vector2>, String>
    where
        T: Walkable,
{
    //check if path gets out of bounds
    if map.len() == 0 || map[0].len()==0 {
        return Err("Invalid Map".to_string())
    }
    let (width,height) = (map.len(),map[0].len());
    if !(0 <= start.x && start.x < width && 0 <=start.y && start.y < height){
        return Err("Out of bounds".to_string());
    }
    //initializes the node map with the coordinates and default values
    let mut node_map: Vec<Vec<Node>> = map.iter().enumerate().map(|(col,v)|
        v.iter().enumerate().map(|(row,w)| Node::init(Vector2::new(col, row), w.cost(), end)  ).collect()).collect();
    node_map[start.x][start.y] = Node::new(start,0,0,0);

    //the frontier of the pathfinding algorithm
    let mut open_list = OpenList::new();

    //nodes already verified
    let mut closed_list = HashSet::<Vector2>::new();
    //add start to open list
    open_list.add(start,&node_map);

    loop{
        //gets point with smallest f_cost
        let current = open_list.min();
        closed_list.insert(current);
        if current == end{
            break;
        }

        for i in neighbours(current,width,height).iter(){
            if closed_list.contains(i)&& node_map[i.x][i.y].g_cost()<= node_map[start.x][start.y].g_cost()+ node_map[i.x][i.y].weight{
                continue;
            }
            if !open_list.contains(i) || node_map[i.x][i.y].g_cost() >node_map[start.x][start.y].g_cost() + node_map[i.x][i.y].weight {
                let parent_cost = node_map[start.x][start.y].g_cost();

                node_map[i.x][i.y].connect(current,parent_cost);

                open_list.add(i.clone(),&node_map);


            }


        }
    }
    let mut path: Vec<Vector2> = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        if let Some(p) = node_map[current.x][current.y].parent{
            current = p;
        }else {
            return Err("Invalid Path :(".to_string());
        }
    }
    println!("cost sum : {}", path.iter().fold(0,|x,p| x+node_map[p.x][p.y].f_cost()));

    Ok(path)

}
//refactorare un sacco, togliere sti unwrap
fn take_min(map: &mut HashSet<Vector2>, node_map: &Vec<Vec<Node>>) -> Vector2 {
    let min = map.iter().min_by(|a,b| node_map[a.x][a.y].partial_cmp(&node_map[b.x][b.y]).unwrap()).unwrap().clone();
    map.remove(&min);
    min
}
pub fn build_road(world:&mut Vec<Vec<PreTileType>>, path: Vec<Vector2>){
    for i in path {
        world[i.x][i.y] = PreTileType::Road;
    }

}

fn neighbours(coords: Vector2, width:usize, height:usize) -> Vec<Vector2> {
    let mut vec: Vec<Vector2> = Vec::with_capacity(4);
    //⇐
    if coords.x > 0{
        vec.push(Vector2::new(coords.x-1, coords.y))
    }
    //⇒
    if coords.x < width-1{
        vec.push(Vector2::new(coords.x+1, coords.y))
    }
    //⇑
    if coords.y > 0{
        vec.push(Vector2::new(coords.x, coords.y-1))
    }
    //⇓
    if coords.y < height-1{
        vec.push(Vector2::new(coords.x, coords.y+1))
    }
    vec.shuffle(&mut thread_rng());
    vec
}


//wrapper for priority queue
pub struct OpenList {
    pq: PriorityQueue<Vector2,Reverse<u32>>
}
impl OpenList{
    fn new() -> Self{
        Self {
            pq: PriorityQueue::new()
        }
    }
    fn min(&mut self) -> Vector2 {
        self.pq.pop().map(|(a,b)| a).unwrap()
    }
    fn add(&mut self, item: Vector2, map: &Vec<Vec<Node>>){
        if self.contains(&item){
            self.pq.change_priority(&item,Reverse(map[item.x][item.y].f_cost()));
        }else{
            self.pq.push(item, Reverse(map[item.x][item.y].f_cost()));
        }
    }
    fn contains(&self, item: &Vector2) -> bool{
    self.pq.get(item).map_or_else(|| false, |_| true)
    }
}


impl Walkable for PreTileType {
    fn cost(&self) -> u32 {
        match self{
            PreTileType::Grass => {1}
            PreTileType::Water => {3}
            PreTileType::DeepWater => {10}
            PreTileType::Sand => {1}
            PreTileType::Mountain => {500}
            PreTileType::Lava => {100000}
            PreTileType::Road => {0}
            _ => {1}
        }
    }
}
