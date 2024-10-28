use std::fs;

enum ClickType {
    Unclicked,
    Marked,
    Known,
}

struct Area {
    click: ClickType,
    thunder: bool
}

impl Area {
    
}

fn send(a: String) -> () {

}

fn recv() -> String {
    // message = fs::read_to_string("/tmp/a").unwrap();
}


fn checkerboard_new(size: i32) -> Vec<Vec<Area>> {
    let mut checkerboard
    for i in size {
        let mut vec = Vec::new();
        for j in size{
            let a = Area::Unclicked;
            vec.push(a);
        }
        checkerboard.push(vec);
    }
    checkerboard
}

fn extract_size(a: String) -> i32 {

}

fn extract_position(a: String) -> (i32, i32) {

}


fn boom (i: i32, j: i32) -> bool {

}

fn auto_expand(a: Vec<Vec<Area>>) -> () {

}

fn check_win(a: Vec<Vec<Area>>) -> bool {

}