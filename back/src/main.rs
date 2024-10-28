fn main() {
    let message: String;
    let mut size: i32;
    let mut checkerboard;

    loop  {
        message = recv();
        println!("{}", message); // for debug
        match &message {
            "~start" => {
                size = extract_size(message);
                checkerboard = checkerboard_new(size);
                send("~OK$");
                continue;
            }
            "~click" => {
                let (i, j) = extract_position(message);
                if checkerboard[i][j] == {
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