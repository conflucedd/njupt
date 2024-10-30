use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let mut a: Vec<u32> = Vec::new();
    for i in buffer.trim().chars() {
        a.push(i.to_digit(10).unwrap());
    }

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let target: usize = buffer.trim().parse().unwrap();

    for _i in 0..target {
        for j in 0..(a.len() - 1) {
            if a[j] > a[j + 1] {
                a.remove(j);
                break;
            }
            if j + 1 == a.len() - 1 { // last element
                a.pop();
            }
        }
    }

    for i in a {
        print!("{i}");
    }
    
    println!("");
}