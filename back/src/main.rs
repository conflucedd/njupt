fn main() {
    let mut message: String;
    let mut checkerboard;

    loop  {
        message = recv();
        println!("{}", &message); // for debug
        match message.as_str() {
            "~start" => {
                size = extract_size(message);
                checkerboard = checkerboard::new(size);
                send("~OK$");
                continue;
            }
            "~click" => {
                let (x, y) = extract_position(message);
                if checkerboard.areas[i][j].thunder == true {
                    send("~lose$");
                }
                else {
                    auto_expand(&mut checkerboard);
                    send(&mut checkerboard)
                }
                continue;
            }
            "~mark" => {
                let (x, y) = extract_position(message);
                checkerboard.areas[x][y].click = Area::Marked;
                continue;
            }
            "~abort" => {
                send("~OK$");
                continue;
            }
            "~stop$" => {
                exit();
            }
        }

        if check_win(&mut checkerboard)
        {
            send("~win$");
        }
    }
}