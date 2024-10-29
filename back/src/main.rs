use back::*;
use std::process::exit;

fn main() {
    prepare();
    let mut message: String;
    let mut checkerboard;

    message = recv();
    if message.starts_with("~start") {
        let (length, width) = extract_length_and_width(message);
        checkerboard = Checkerboard::new(length, width);
        send("~OK$");
    } else {
        exit(1);
    }

    loop  {
        message = recv();
        println!("message: {}", &message); // for debug
        match message.as_str() {
            s if s.starts_with("~start") => {
                let (length, width) = extract_length_and_width(message);
                checkerboard = Checkerboard::new(length, width);
                send("~OK$");
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
                        checkerboard = Checkerboard::new(checkerboard.length, checkerboard.width);
                        if checkerboard.areas[x][y].thunder == false {
                            checkerboard.first = false;
                            break;
                        }
                    }
                }
                
                checkerboard.areas[x][y].click = Status::Known;
                checkerboard.first = false;
                if checkerboard.areas[x][y].property == 0 {
                    auto_expand(&mut checkerboard, x, y);
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
                send(&checkerboard.to_string()); // random click
            }
            s if s.starts_with("~complete") => {
                let (x, y) = extract_position(message);
                auto_click(&mut checkerboard, x, y);
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
                exit(2);
            }
        }
    }
}