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
    pub size: usize,
    pub first: bool
}

impl Checkerboard {
    pub fn new(size: usize) -> Self {
        let mut checkerboard = Checkerboard {
            areas: Vec::new(),
            size,
            first: true
        };
        for _x in 0..size {
            let mut vec = Vec::new();
            for _y in 0..size {
                let a = Area::new(Status::Unclicked, Self::thunder_random(5));
                vec.push(a);
            }
            checkerboard.areas.push(vec);
        };
        for x in 0..size {
            for y in 0..size {
                if checkerboard.areas[x][y].thunder == true {
                    checkerboard.areas[x][y].property = -1;
                }
                else  {
                    if x >= 1 && y >= 1 && checkerboard.areas[x - 1][y - 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x >= 1 && checkerboard.areas[x - 1][y].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x >= 1  && y + 1 < size && checkerboard.areas[x - 1][y + 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if y >= 1 && checkerboard.areas[x][y - 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if y + 1 < size && checkerboard.areas[x][y + 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x + 1 < size && y >= 1 && checkerboard.areas[x + 1][y - 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x + 1 < size && checkerboard.areas[x + 1][y].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                    if x + 1 < size && y + 1 < size && checkerboard.areas[x + 1][y + 1].thunder == true
                        { checkerboard.areas[x][y].property += 1 }
                }
            }
        };
        checkerboard
    }

    fn thunder_random(a: usize) -> bool {
        if random::<usize>() % 100 <= a {
            return true;
        };
        false
    }

    pub fn to_string(&self) -> String {
        let mut a = String::new();
        a.push('~');
        for i in 0..self.size {
            for j in 0..self.size {
                if self.areas[i][j].click == Status::Marked {
                    a.push_str("@");
                }
                if self.areas[i][j].click == Status::Unclicked {
                    a.push_str("9");
                }
                if self.areas[i][j].click == Status::Known {
                    a.push_str(&self.areas[i][j].property.to_string());
                }
            }
        }
        a.push('$');
        a
    }

    pub fn to_string_at_fail(&self) -> String {
        let mut a = String::new();
        a.push('~');
        for i in 0..self.size {
            for j in 0..self.size {
                if self.areas[i][j].click == Status::Special {
                    a.push_str("s");
                    continue;
                }
                if self.areas[i][j].thunder == true {
                    a.push_str("t");
                }
                if self.areas[i][j].click == Status::Marked {
                    a.push_str("@");
                }
                if self.areas[i][j].click == Status::Unclicked {
                    a.push_str("9");
                }
                if self.areas[i][j].click == Status::Known {
                    a.push_str(&self.areas[i][j].property.to_string());
                }
            }
        }
        a.push('$');
        a
    }
}

pub fn send(a: &str) -> () {
    loop {
        if fs::exists("/tmp/recv").unwrap() == true {
            thread::sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }
    fs::write("/tmp/recv", a).unwrap();
    // for debug
    println!("{}", a);
}

pub fn recv() -> String {
    loop {
        if fs::exists("/tmp/send").unwrap() == false {
            thread::sleep(Duration::from_millis(100));
        } else {
            break;
        }
    }

    let message = fs::read_to_string("/tmp/send").unwrap();
    fs::remove_file("/tmp/send").unwrap();
    // for debug
    println!("{}", &message);
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

pub fn auto_expand(checkerboard: &mut Checkerboard, x: usize, y: usize) -> () {
    if x >= 1 && checkerboard.areas[x - 1][y].thunder == false {
        checkerboard.areas[x - 1][y].click = Status::Known;
        
        if checkerboard.areas[x - 1][y].property == 0 {
            auto_expand(checkerboard, x - 1, y)
        }
    }
    if y >= 1 && checkerboard.areas[x][y - 1].thunder == false {
        checkerboard.areas[x][y - 1].click = Status::Known;
        
        if checkerboard.areas[x][y - 1].property == 0 {
            auto_expand(checkerboard, x, y - 1)
        }
    }
    if y + 1 < checkerboard.size && checkerboard.areas[x][y + 1].thunder == false {
        checkerboard.areas[x][y + 1].click = Status::Known;
        
        if checkerboard.areas[x][y + 1].property == 0 {
            auto_expand(checkerboard, x, y + 1)
        }
    }
    if x + 1 < checkerboard.size && checkerboard.areas[x + 1][y].thunder == false {
        checkerboard.areas[x + 1][y].click = Status::Known;
        
        if checkerboard.areas[x + 1][y].property == 0 {
            auto_expand(checkerboard, x + 1, y)
        }
    }
}

pub fn expand(checkerboard: &mut Checkerboard, a: &mut VecDeque<(usize, usize)>) -> () {
    let (x, y) = a.pop_back().unwrap();
    if x >= 1 && checkerboard.areas[x - 1][y].thunder == false {
        checkerboard.areas[x - 1][y].click = Status::Known;
        
        if checkerboard.areas[x - 1][y].property == 0 {
            a.push_back((x - 1, y));
        }
    }
    if y >= 1 && checkerboard.areas[x][y - 1].thunder == false {
        checkerboard.areas[x][y - 1].click = Status::Known;
        
        if checkerboard.areas[x][y - 1].property == 0 {
            a.push_back((x, y - 1));
        }
    }
    if y + 1 < checkerboard.size && checkerboard.areas[x][y + 1].thunder == false {
        checkerboard.areas[x][y + 1].click = Status::Known;
        
        if checkerboard.areas[x][y + 1].property == 0 {
            a.push_back((x, y + 1));
        }
    }
    if x + 1 < checkerboard.size && checkerboard.areas[x + 1][y].thunder == false {
        checkerboard.areas[x + 1][y].click = Status::Known;
        
        if checkerboard.areas[x + 1][y].property == 0 {
            a.push_back((x + 1, y));
        }
    }
    a.pop_front().unwrap();
}

pub fn auto_expand2(checkerboard: &mut Checkerboard, x: usize, y: usize) -> () {
    let mut a: VecDeque<(usize, usize)> = VecDeque::new();
    a.push_back((x, y));
    
    while a.is_empty() == false {
        expand(checkerboard, &mut a); 
    }
}

pub fn check_win(checkreboard: &Checkerboard) -> bool {
    for x in 0..checkreboard.size {
        for y in 0..checkreboard.size {
            if checkreboard.areas[x][y].thunder == false {
                if checkreboard.areas[x][y].click != Status::Known {
                    return false;
                }
            }
        }
    }
    true
}