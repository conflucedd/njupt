use std::fs;
use rand::prelude::random;
use std::{thread, time::Duration};
use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum Status {
    Unclicked,
    Marked,
    Known,
    Boom
}

pub struct Area {
    pub click: Status,
    pub thunder: bool,
    pub property: i32 // hint number
}

pub fn around(x: usize, y: usize, length: usize, width: usize) -> Vec<(usize, usize)> {
    let mut a = Vec::new();
    
    if x >= 1 && y >= 1
        { a.push((x - 1, y - 1)); }
    if x >= 1 
        { a.push((x - 1, y)); }
    if x >= 1  && y + 1 < width 
        { a.push((x - 1, y + 1)); }
    if y >= 1
        { a.push((x, y - 1)); }
    if y + 1 < width
        { a.push((x, y + 1)); }
    if x + 1 < length && y >= 1
        { a.push((x + 1, y - 1)); }
    if x + 1 < length
        { a.push((x + 1, y)); }
    if x + 1 < length && y + 1 < width
        { a.push((x + 1, y + 1)); }
    
    a
}

impl Area {
    fn new() -> Self {
        Area {
            click: Status::Unclicked,
            thunder: false,
            property: 0
        }
    }
}

pub struct Checkerboard {
    pub areas: Vec<Vec<Area>>,
    pub length: usize,
    pub width: usize,
    pub first: bool,
    pub target: usize,
    pub left_target: i32
}

impl Checkerboard {
    pub fn new(length: usize, width: usize, target: usize) -> Self {
        let mut checkerboard = Checkerboard {
            areas: Vec::new(),
            length,
            width,
            target,
            first: true,
            left_target: i32::try_from(target).unwrap()
        };
        for _x in 0..length {
            let mut vec = Vec::new();
            for _y in 0..width {
                let a = Area::new();
                vec.push(a);
            }
            checkerboard.areas.push(vec);
        };

        let a = Self::thunder_random(target, length, width); // target is fix for debug
        for (x, y) in a {
            checkerboard.areas[x][y].thunder = true;
        }

        for x in 0..length {
            for y in 0..width {
                if checkerboard.areas[x][y].thunder == true {
                    checkerboard.areas[x][y].property = -1;
                }
                else  {
                    let mut a = 0;
                    for (x, y) in around(x, y, checkerboard.length, checkerboard.width) {
                        if checkerboard.areas[x][y].thunder == true
                        { a += 1 }
                    }
                    checkerboard.areas[x][y].property = a;
                }
            }
        };
        checkerboard
    }

    
    fn thunder_random(target: usize, length: usize, width: usize) -> Vec<(usize, usize)> {
        println!("Starting random: length: {}, width: {}", length, width);
        let mut a: Vec<bool> = Vec::new();
        let mut b: Vec<usize> = Vec::new();
        let size = length * width;
        for _i in 0..size {
            a.push(true);
        }
        while Vec::len(&b) != target {
            let c = random::<usize>() % size;
            if a[c] == true {
                b.push(c);
                a[c] = false;
            }
        }

        let mut res: Vec<(usize, usize)> = Vec::new();

        for i in b {
            println!("{:?}", (i / width, i % width));
            res.push((i / width, i % width));
        }
        res
    }

    pub fn to_string(&self) -> String {
        let mut a = String::new();
        a.push('~');
        for x in 0..self.length {
            for y in 0..self.width {
                if self.areas[x][y].click == Status::Marked {
                    a.push_str("@");
                }
                if self.areas[x][y].click == Status::Unclicked {
                    a.push_str("9");
                }
                if self.areas[x][y].click == Status::Known {
                    a.push_str(&self.areas[x][y].property.to_string());
                }
            }
        }
        a.push('$');
        a
    }

    pub fn to_answer(&self) -> String {
        let mut a = String::new();
        a.push('~');
        for x in 0..self.length {
            for y in 0..self.width {
                match (x, y) {
                    (x, y) if self.areas[x][y].click == Status::Boom => {
                        a.push_str("b");
                    }
                    (x, y) if self.areas[x][y].click == Status::Marked && self.areas[x][y].thunder => {
                        a.push_str("r");
                    }
                    (x, y) if self.areas[x][y].thunder => {
                        a.push_str("t");
                    }
                    (x, y) if self.areas[x][y].click == Status::Marked => {
                        a.push_str("@");
                    }
                    (x, y) if self.areas[x][y].click == Status::Unclicked => {
                        a.push_str("9");
                    }
                    (x, y) if self.areas[x][y].click == Status::Known => {
                        a.push_str(&self.areas[x][y].property.to_string());
                    }
                    _ => {}
                }
            }
        }
        a.push('$');
        a
    }
}

pub fn prepare() {
    if fs::exists("/tmp/send").unwrap() {
        fs::remove_file("/tmp/send").unwrap();
    }
    if fs::exists("/tmp/recv").unwrap() {
        fs::remove_file("/tmp/recv").unwrap();
    }
}

pub fn send(a: &str) -> () {
    println!("sending: {}", a);
    loop {
        if fs::exists("/tmp/recv").unwrap() == true {
            thread::sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }
    fs::write("/tmp/recv", a).unwrap();
    // for debug
    println!("send complete");
}

pub fn recv() -> String {
    println!("reciving");
    loop {
        if fs::exists("/tmp/send").unwrap() == false {
            thread::sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }

    let message = fs::read_to_string("/tmp/send").unwrap();
    println!("message: {}", &message); // for debug
    fs::remove_file("/tmp/send").unwrap();
    message
}

fn extract_number<I>(a: &mut I) -> usize
where
    I: Iterator<Item = char>,
{
    let mut x = String::new();
    while let Some(c) = a.next() {
        if c == ',' {
            break;
        }
        if c.is_numeric() {
            x.push(c);
        }
    }
    x.parse().unwrap()
}

pub fn extract_position(a: String) -> (usize, usize) {
    let mut str_itr = a.chars();
    (extract_number(&mut str_itr), extract_number(&mut str_itr))
}

pub fn extract_start(a: String) -> (usize, usize, usize) {
    let mut str_itr = a.chars();
    (extract_number(&mut str_itr), extract_number(&mut str_itr), extract_number(&mut str_itr))
}

pub fn auto_expand(checkerboard: &mut Checkerboard, x: usize, y: usize) -> () {
    let mut a: VecDeque<(usize, usize)> = VecDeque::new();
    a.push_back((x, y));
    
    while a.is_empty() == false {
        let (x, y) = a.pop_front().unwrap();

        for (x, y) in around(x, y, checkerboard.length, checkerboard.width) {
            if checkerboard.areas[x][y].property == 0 && checkerboard.areas[x][y].click == Status::Unclicked {
                a.push_back((x, y));
            }
            checkerboard.areas[x][y].click = Status::Known;
        }
    }
}

pub fn auto_click(checkerboard: &mut Checkerboard, x: usize, y: usize) -> bool {
    let mut a = 0;

    for (x, y) in around(x, y, checkerboard.length, checkerboard.width) {
        if checkerboard.areas[x][y].click == Status::Marked
            { a += 1; }
    }

    if a >= checkerboard.areas[x][y].property {
        for (x, y) in around(x, y, checkerboard.length, checkerboard.width) {
            if checkerboard.areas[x][y].click == Status::Unclicked {
                if checkerboard.areas[x][y].thunder {
                    checkerboard.areas[x][y].click = Status::Boom;
                    return false; // means lost
                }
                checkerboard.areas[x][y].click = Status::Known;
            }
        }
    }
    true // means ok, not win
}

pub fn check_win(checkreboard: &Checkerboard) -> bool {
    for x in 0..checkreboard.length {
        for y in 0..checkreboard.width {
            if checkreboard.areas[x][y].thunder == false {
                if checkreboard.areas[x][y].click != Status::Known {
                    return false;
                }
            }
        }
    }
    true
}