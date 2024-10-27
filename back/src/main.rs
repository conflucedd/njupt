
fn main() {
    let message: String;
    let mut size: i32;
    let mut checkerboard: Vec<Vec<Area>> = Vec::new();

    loop  {
        message = recv();
        println!("{}", message); // for debug
        match &message {
            "~start" => {
                size = extract_size(message);
                for i in size {
                    let mut vec = Vec::new();
                    for j in size{
                        let a = Area::Unclicked;
                        vec.push(a);
                    }
                    checkerboard.push(vec);
                }
                send("~OK$");
                continue;
            }
            "~click" => {
                let position = extract_position(message);
                if boom(position) {
                    send
                }
                else {
                    auto_expand(checkerboard);
                }
                continue;
            }
            "~mark" => {
                let position = extract_position(message);
                checkerboard[i][j] = Area::Marked;
                continue;
            }
            "~abort" => {
                send("~OK$");
                continue;
            }
        }

        if check_win(checkerboard)
        {
            send("~win$");
        }
        else
        {
            send("~lose$");
        }
    }
}