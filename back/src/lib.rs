use std::fs;
use rand::prelude::random;
use std::iter::Peekable;
#[derive(PartialEq)]
enum Status {
    Unclicked,
    Marked,
    Known,
}

struct Area {
    click: Status,
    thunder: bool,
    property: i32 // hint number
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

struct Checkerboard {
    areas: Vec<Vec<Area>>,
    size: usize
}

impl Checkerboard {
    fn new(size: usize) -> Self {
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
}

fn send(a: String) -> () {
    // message = fs::read_to_string("/tmp/a").unwrap();
}

fn recv() -> String {
    // message = fs::read_to_string("/tmp/a").unwrap();
}


fn extract_size(a: String) -> i32 {
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

fn extract_position(mut a: String) -> (i32, i32) {
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

fn auto_expand(mut checkreboard: Checkerboard, x: i32, y: i32) -> () {

}

fn check_win(mut checkreboard: Checkerboard) -> bool {
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