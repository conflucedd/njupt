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

    loop {
        message = recv();
        match message.as_str() {
            s if s.starts_with("~start") => {
                let (length, width) = extract_length_and_width(message);
                checkerboard = Checkerboard::new(length, width);
                send("~OK$");
            }
            s if s.starts_with("~click") => {
                let (x, y) = extract_position(message);
                match (x, y) {
                    (x, y) if checkerboard.areas[x][y].click == Status::Marked => {
                        checkerboard.areas[x][y].click = Status::Unclicked;
                    }
                    (x, y) if checkerboard.areas[x][y].click == Status::Unclicked => {
                        // first click protect
                        if checkerboard.areas[x][y].thunder == true && checkerboard.first == true {
                            loop {
                                checkerboard = Checkerboard::new(checkerboard.length, checkerboard.width);
                                if checkerboard.areas[x][y].thunder == false {
                                    break;
                                }
                            }
                        }
                        checkerboard.first = false;

                        if checkerboard.areas[x][y].thunder == true {
                            checkerboard.areas[x][y].click = Status::Boom;
                            send("~lost$");
                            continue;
                        }
                        checkerboard.areas[x][y].click = Status::Known;
                        if checkerboard.areas[x][y].property == 0 {
                            auto_expand(&mut checkerboard, x, y);
                        }
                        if check_win(&checkerboard)
                        {
                            send("~win$");
                            continue;
                        };

                    }
                    (x, y) if checkerboard.areas[x][y].click == Status::Known => {
                        if auto_click(&mut checkerboard, x, y) {
                            if check_win(&checkerboard) {
                                send("~win$");
                                continue;
                            };
                        } else {
                            send("~lost$");
                            continue;
                        }
                    }
                    _ => {}
                }

                send(&checkerboard.to_string());
            }
            s if s.starts_with("~mark") => {
                let (x, y) = extract_position(message);
                match (x, y) {
                    (x, y) if checkerboard.areas[x][y].click == Status::Unclicked => {
                        checkerboard.areas[x][y].click = Status::Marked;
                    }
                    (x, y) if checkerboard.areas[x][y].click == Status::Marked => {
                        checkerboard.areas[x][y].click = Status::Unclicked;
                    }
                    _ => {}
                }
                send(&checkerboard.to_string());
            }
            s if s.starts_with("~answer") => {
                send(&checkerboard.to_answer());
            }
            s if s.starts_with("~abort") => {
                send("~OK$");
            }
            s if s.starts_with("~stop$") => {
                exit(0);
            }
            _ => {}
        }
    }
}