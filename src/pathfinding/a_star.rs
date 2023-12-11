use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::{Rev, Sum};
use std::ops;
use noise::core::worley::distance_functions::euclidean;
use priority_queue::PriorityQueue;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::tile::_TileType;

pub trait Walkable {
    fn cost(&self) -> u32;
}
#[derive(Clone, Copy, Eq, PartialEq,Hash,Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn as_i32_tuple(&self) -> (i32,i32) {(self.x as i32, self.y as i32)}
    pub fn manhattan_distance(&self, other: Self) ->u32 {

        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32

    }
    fn euclidean(&self, other:Self)->u32{
        let dx = self.x.abs_diff(other.x) as f64;
        let dy = self.y.abs_diff(other.y) as f64;
        (dx * dx + dy * dy).sqrt().floor() as u32
    }
    pub fn neighbours(&self, width:usize,height:usize)->Vec<Point>{
        let mut vec: Vec<Point> = Vec::with_capacity(4);
        //⇐
        if self.x > 0{
            vec.push(Point::new(self.x-1,self.y))
        }
        //⇒
        if self.x < width-1{
            vec.push(Point::new(self.x+1,self.y))
        }
        //⇑
        if self.y > 0{
            vec.push(Point::new(self.x,self.y-1))
        }
        //⇓
        if self.y < height-1{
            vec.push(Point::new(self.x,self.y+1))
        }
        vec
    }
}
impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
#[derive(Debug)]
struct Node{
    //the coordinates of the point on the map
    coords:Point,
    //the node's parent's coordinates in the shortest path
    parent:Option<Point>,
    //g value: represents the shortest distance from the start point
    g: u32,
    //h value:(optimistic) estimated distance from the end point
    h:u32,
    //weight: the weight of the node
    weight:u32
}
impl Node {
    fn new(coords:Point,g:u32,h:u32,weight:u32)->Self{

        Self {
            coords,
            parent:None,
            g,
            h,
            weight: weight
        }
    }
    //function used to initialize a node to a default value
    fn init(coords:Point,weight:u32,end:Point) -> Self{
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
    fn connect(&mut self, parent: Point, parent_cost: u32,){
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
pub fn shortest_path<T>(map: &Vec<Vec<T>>, start: Point, end: Point) -> Result<Vec<Point>, String>
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
        v.iter().enumerate().map(|(row,w)| Node::init(Point::new(col,row), w.cost(),end)  ).collect()).collect();
    node_map[start.x][start.y] = Node::new(start,0,0,0);

    //the frontier of the pathfinding algorithm
    let mut open_list = HashSet::<Point>::new();
    //nodes already verified
    let mut closed_list = HashSet::<Point>::new();
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

    let mut path: Vec<Point> = Vec::new();
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
pub fn shortest_priority<T>(map: &Vec<Vec<T>>, start: Point, end: Point) -> Result<Vec<Point>, String>
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
        v.iter().enumerate().map(|(row,w)| Node::init(Point::new(col,row), w.cost(),end)  ).collect()).collect();
    node_map[start.x][start.y] = Node::new(start,0,0,0);

    //the frontier of the pathfinding algorithm
    let mut open_list = OpenList::new();

    //nodes already verified
    let mut closed_list = HashSet::<Point>::new();
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
    let mut path: Vec<Point> = Vec::new();
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
fn take_min(map: &mut HashSet<Point>, node_map: &Vec<Vec<Node>>) ->Point{
    let min = map.iter().min_by(|a,b| node_map[a.x][a.y].partial_cmp(&node_map[b.x][b.y]).unwrap()).unwrap().clone();
    map.remove(&min);
    min
}
pub fn build_road(world:&mut Vec<Vec<_TileType>>, path: Vec<Point>){
    for i in path {
        world[i.x][i.y] = _TileType::Road;
    }

}

fn neighbours( coords:Point,width:usize,height:usize)-> Vec<Point> {
    let mut vec: Vec<Point> = Vec::with_capacity(4);
    //⇐
    if coords.x > 0{
        vec.push(Point::new(coords.x-1,coords.y))
    }
    //⇒
    if coords.x < width-1{
        vec.push(Point::new(coords.x+1,coords.y))
    }
    //⇑
    if coords.y > 0{
        vec.push(Point::new(coords.x,coords.y-1))
    }
    //⇓
    if coords.y < height-1{
        vec.push(Point::new(coords.x,coords.y+1))
    }
    vec.shuffle(&mut thread_rng());
    vec
}
#[test]
fn test(){
    //generating a map

}
impl Walkable for u32{
    fn cost(&self) -> u32 {
        *self
    }
}
//wrapper for priority queue
pub struct OpenList {
    pq: PriorityQueue<Point,Reverse<u32>>
}
impl OpenList{
    fn new() -> Self{
        Self {
            pq: PriorityQueue::new()
        }
    }
    fn min(&mut self) -> Point {
        self.pq.pop().map(|(a,b)| a).unwrap()
    }
    fn add(&mut self, item: Point,map: &Vec<Vec<Node>>){
        if self.contains(&item){
            self.pq.change_priority(&item,Reverse(map[item.x][item.y].f_cost()));
        }else{
            self.pq.push(item, Reverse(map[item.x][item.y].f_cost()));
        }
    }
    fn contains( &self, item: &Point)-> bool{
    self.pq.get(item).map_or_else(|| false, |_| true)
    }
}