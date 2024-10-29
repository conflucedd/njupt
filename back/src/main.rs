use back::*;
use std::process::exit;
fn main() {
    let mut message: String;
    let mut checkerboard = Checkerboard::new(0);

    loop  {
        message = recv();
        println!("{}", &message); // for debug
        match message.as_str() {
            "~start" => {
                let size = extract_size(message);
                checkerboard = Checkerboard::new(size);
                send("~OK$");
                continue;
            }
            "~click" => {
                let (x, y) = extract_position(message);
                
                if checkerboard.areas[x][y].thunder == true && checkerboard.first == false {
                    send("~lose$");
                }
                if checkerboard.areas[x][y].thunder == true && checkerboard.first == true {
                    loop {
                        checkerboard = Checkerboard::new(checkerboard.size);
                        if checkerboard.areas[x][y].thunder == false {
                            checkerboard.first = false;
                            break;
                        }
                    }
                }
                
                checkerboard.areas[x][y].click = Status::Known;
                if checkerboard.areas[x][y].property == 0 {
                    auto_expand(&mut checkerboard, x, y);
                }
                send(&checkerboard.to_string());
            }
            "~mark" => {
                let (x, y) = extract_position(message);
                checkerboard.areas[x][y].click = Status::Marked;
            }
            "~abort" => {
                send("~OK$");
                continue;
            }
            "~stop$" => {
                exit(0);
            }
            &_ => {
                exit(0);
            }
        }

        if check_win(&checkerboard)
        {
            send("~win$");
        }
    }
}