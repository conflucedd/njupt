fn main() {
    let message: String;
    let mut size: i32;
    let mut checkerboard;

    loop  {
        message = recv();
        println!("{}", &message); // for debug
        match &message {
            "~start" => {
                size = extract_size(message);
                checkerboard = checkerboard_new(size);
                send("~OK$");
                continue;
            }
            "~click" => {
                let (i, j) = extract_position(message);
                if checkerboard[i][j].thunder == true {
                    send("~lose$");
                }
                else {
                    auto_expand(&mut checkerboard);
                }
                continue;
            }
            "~mark" => {
                let position = extract_position(message);
                checkerboard[i][j].click = Area::Marked;
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