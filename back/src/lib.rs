use std::fs;
use rand::prelude::random;
use std::{thread, time::Duration};
use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum Status {
    Unclicked,
    Marked,
    Known,
    Special
}

pub struct Area {
    pub click: Status,
    pub thunder: bool,
    pub property: i32 // hint number
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
    pub first: bool
}

impl Checkerboard {
    pub fn new(length: usize, width: usize) -> Self {
        let mut checkerboard = Checkerboard {
            areas: Vec::new(),
            length,
            width,
            first: true
        };
        for _x in 0..length {
            let mut vec = Vec::new();
            for _y in 0..width {
                let a = Area::new();
                vec.push(a);
            }
            checkerboard.areas.push(vec);
        };

        let a = Self::thunder_random(2, length, width); // target is fix for debug
        for (x, y) in a {
            checkerboard.areas[x][y].thunder = true;
        }

        for x in 0..length {
            for y in 0..width {
                if checkerboard.areas[x][y].thunder == true {
                    checkerboard.areas[x][y].property = -1;
                }
                else  {
                    if x >= 1 && y >= 1 && checkerboard.areas[x - 1][y - 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x >= 1 && checkerboard.areas[x - 1][y].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x >= 1  && y + 1 < width && checkerboard.areas[x - 1][y + 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if y >= 1 && checkerboard.areas[x][y - 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if y + 1 < width && checkerboard.areas[x][y + 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x + 1 < length && y >= 1 && checkerboard.areas[x + 1][y - 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x + 1 < length && checkerboard.areas[x + 1][y].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x + 1 < length && y + 1 < width && checkerboard.areas[x + 1][y + 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                }
            }
        };
        checkerboard
    }

    
    fn thunder_random(target: usize, length: usize, width: usize) -> Vec<(usize, usize)> {
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
            res.push((i / length, i % length - 1));
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

    pub fn to_string_at_fail(&self) -> String {
        let mut a = String::new();
        a.push('~');
        for x in 0..self.length {
            for y in 0..self.width {
                if self.areas[x][y].click == Status::Special {
                    a.push_str("s");
                    continue;
                }
                if self.areas[x][y].thunder == true {
                    a.push_str("t");
                }
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
    fs::remove_file("/tmp/send").unwrap();
    message
}

pub fn extract_length_and_width(a: String) -> (usize, usize) {
    extract_position(a)
}

pub fn extract_position(a: String) -> (usize, usize) {
    let str_itr = a.chars();
    let mut x: String = String::new();
    let mut y: String = String::new();
    
    let mut for_y = false;

    for c in str_itr {
        if c.is_numeric() == true && for_y == false {
            x.push(c)
        }
        if c == ',' {
            for_y = true;
            continue;
        }
        if c.is_numeric() == true && for_y == true {
            y.push(c)
        }
    }

    (x.parse().unwrap(), y.parse().unwrap())
}

pub fn expand(checkerboard: &mut Checkerboard, a: &mut VecDeque<(usize, usize)>) -> () {
    let (x, y) = a.pop_front().unwrap();
    if x >= 1 && checkerboard.areas[x - 1][y].click == Status::Unclicked {
        checkerboard.areas[x - 1][y].click = Status::Known;
        
        if checkerboard.areas[x - 1][y].property == 0 {
            a.push_back((x - 1, y));
        }
    }
    if y >= 1 && checkerboard.areas[x][y - 1].click == Status::Unclicked {
        checkerboard.areas[x][y - 1].click = Status::Known;
        
        if checkerboard.areas[x][y - 1].property == 0 {
            a.push_back((x, y - 1));
        }
    }
    if y + 1 < checkerboard.width && checkerboard.areas[x][y + 1].click == Status::Unclicked {
        checkerboard.areas[x][y + 1].click = Status::Known;
        
        if checkerboard.areas[x][y + 1].property == 0 {
            a.push_back((x, y + 1));
        }
    }
    if x + 1 < checkerboard.length && checkerboard.areas[x + 1][y].click == Status::Unclicked {
        checkerboard.areas[x + 1][y].click = Status::Known;
        
        if checkerboard.areas[x + 1][y].property == 0 {
            a.push_back((x + 1, y));
        }
    }
}

pub fn auto_expand(checkerboard: &mut Checkerboard, x: usize, y: usize) -> () {
    let mut a: VecDeque<(usize, usize)> = VecDeque::new();
    a.push_back((x, y));
    
    while a.is_empty() == false {
        expand(checkerboard, &mut a); 
    }
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