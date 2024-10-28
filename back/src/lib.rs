use std::fs;
use rand::prelude::random;
use std::{thread, time::Duration};

#[derive(PartialEq)]
pub enum Status {
    Unclicked,
    Marked,
    Known,
}

pub struct Area {
    pub click: Status,
    pub thunder: bool,
    pub property: i32 // hint number
}

impl Area {
    fn new(click: Status, thunder: bool) -> Self {
        Area {
            click,
            thunder,
            property: 0
        }
    }
}

pub struct Checkerboard {
    pub areas: Vec<Vec<Area>>,
    pub size: usize
}

impl Checkerboard {
    pub fn new(size: usize) -> Self {
        let mut checkerboard = Checkerboard {
            areas: Vec::new(),
            size
        };
        for _x in 0..size + 2 {
            let mut vec = Vec::new();
            for _y in 0..size + 2 {
                let a = Area::new(Status::Unclicked, false);
                vec.push(a);
            }
            checkerboard.areas.push(vec);
        };
        for _x in 1..size + 1{
            let mut vec = Vec::new();
            for _y in 1..size + 1 {
                let a = Area::new(Status::Unclicked, random::<bool>());
                vec.push(a);
            }
            checkerboard.areas.push(vec);
        };
        for x in 1..size {
            for y in 1..size {
                if checkerboard.areas[x][y].thunder == true {
                    checkerboard.areas[x][y].property = -1;
                }
                else  {
                    if checkerboard.areas[x - 1][y - 1].thunder == true { checkerboard.areas[x][y].property += 1 }
                    if checkerboard.areas[x - 1][y].thunder == true { checkerboard.areas[x][y].property += 1 }
                    if checkerboard.areas[x - 1][y + 1].thunder == true { checkerboard.areas[x][y].property += 1 }
                    if checkerboard.areas[x][y - 1].thunder == true { checkerboard.areas[x][y].property += 1 }
                    if checkerboard.areas[x][y + 1].thunder == true { checkerboard.areas[x][y].property += 1 }
                    if checkerboard.areas[x + 1][y - 1].thunder == true { checkerboard.areas[x][y].property += 1 }
                    if checkerboard.areas[x + 1][y].thunder == true { checkerboard.areas[x][y].property += 1 }
                    if checkerboard.areas[x + 1][y + 1].thunder == true { checkerboard.areas[x][y].property += 1 }
                }
            }
        };
        checkerboard
    }

    pub fn to_string(&self) -> String {
        let mut a = String::new();
        a.push('~');
        for i in 1..self.size + 1 {
            for j in 1..self.size +1 {
                a.push_str(&self.areas[i][j].property.to_string());
                a.push(',');
            }
        }
        a.push('$');
        a
    }
}

pub fn send(a: &str) -> () {
    loop {
        if fs::exists("/tmp/send").unwrap() == true {
            thread::sleep(Duration::from_millis(500));
        } else {
            break;
        }
    }
    fs::write("/tmp/recv", &a).unwrap();
}

pub fn recv() -> String {
    loop {
        if fs::exists("/tmp/send").unwrap() == false {
            thread::sleep(Duration::from_millis(500));
        } else {
            break;
        }
    }

    let message = fs::read_to_string("/tmp/send").unwrap();
    fs::remove_file("/tmp/send").unwrap();
    message
}

pub fn extract_size(a: String) -> usize {
    let str_itr = a.chars();
    let mut num: String = String::new();
    for c in str_itr {
        if c == '~' || c == ',' {
            continue
        }
        if c.is_numeric() == true {
            num.push(c)
        }
    }
    num.parse().unwrap()
}

pub fn extract_position(a: String) -> (usize, usize) {
    let str_itr = a.chars();
    let mut x: String = String::new();
    let mut y: String = String::new();
    
    let mut for_y = false;

    for c in str_itr {
        if c == '~' {
            continue;
        }
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

pub fn auto_expand(checkreboard: &mut Checkerboard, x: usize, y: usize) -> () {











    
}

pub fn check_win(checkreboard: &Checkerboard) -> bool {
    for x in 1..checkreboard.size + 1 {
        for y in 1..checkreboard.size + 1 {
            if checkreboard.areas[x][y].thunder == true && checkreboard.areas[x][y].click != Status::Marked {
                return false;
            }
            if checkreboard.areas[x][y].thunder == false && checkreboard.areas[x][y].click == Status::Marked {
                return false;
            }
        }
    }
    true
}