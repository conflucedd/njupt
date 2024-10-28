use back::*;
use std::process::exit;
fn main() {
    let mut message: String;
    let mut checkerboard = Checkerboard::new(0);

    loop  {
        message = recv();
        println!("message: {}", &message); // for debug
        match message.as_str() {
            s if s.starts_with("~start") => {
                let size = extract_size(message);
                checkerboard = Checkerboard::new(size);
                send("~OK$");
                continue;
            }
            s if s.starts_with("~click") => {
                let (x, y) = extract_position(message);
                if checkerboard.areas[x][y].click != Status::Unclicked {
                    send(&checkerboard.to_string());
                    continue;
                }
                if checkerboard.areas[x][y].thunder == true && checkerboard.first == false {
                    send("~lost$");
                    checkerboard.areas[x][y].click = Status::Special;
                    // todo: send lose map
                    // send(&checkerboard.to_string_at_fail());
                    continue;
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
                checkerboard.first = false;
                if checkerboard.areas[x][y].property == 0 {
                    auto_expand2(&mut checkerboard, x, y);
                }
                if check_win(&checkerboard)
                {
                    send("~win$");
                    continue;
                };
                send(&checkerboard.to_string());
            }
            s if s.starts_with("~mark") => {
                let (x, y) = extract_position(message);
                if checkerboard.areas[x][y].click == Status::Unclicked {
                    checkerboard.areas[x][y].click = Status::Marked;
                    send(&checkerboard.to_string());
                    continue;
                };
                if checkerboard.areas[x][y].click == Status::Marked {
                    checkerboard.areas[x][y].click = Status::Unclicked;
                    send(&checkerboard.to_string());
                    continue;
                }
                send(&checkerboard.to_string());
            }
            s if s.starts_with("~abort") => {
                send("~OK$");
                continue;
            }
            s if s.starts_with("~stop$") => {
                exit(0);
            }
            &_ => {
                exit(1);
            }
        }
    }
}