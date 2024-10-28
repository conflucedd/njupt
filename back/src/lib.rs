use std::fs;
use rand::prelude::random;
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

fn send(a: String) -> () {

}

fn recv() -> String {
    // message = fs::read_to_string("/tmp/a").unwrap();
}


fn extract_size(a: String) -> i32 {

}

fn checkerboard_new(size: i32) -> Vec<Vec<Area>> {
    let mut checkerboard: Vec<Vec<Area>> = Vec::new();
    for i in 0..size {
        let mut vec = Vec::new();
        for j in 0..size{
            let a = Area::new(Status::Unclicked, random::<bool>());
            vec.push(a);
        }
        checkerboard.push(vec);
    }
    checkerboard
}

fn extract_position(a: String) -> (i32, i32) {

}


fn boom (i: i32, j: i32) -> bool {

}

fn auto_expand(a: Vec<Vec<Area>>) -> () {

}

fn check_win(a: Vec<Vec<Area>>) -> bool {

}