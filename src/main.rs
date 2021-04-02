use std::io;

fn main() {
    loop {
        println!("Enter a sequence length greater than 1");

        let mut sequence_length = String::new();

        io::stdin()
            .read_line(&mut sequence_length)
            .expect("Failed to read line");

        let sequence_length: u32 = match sequence_length.trim().parse() {
            Ok(num) => match num {
                0 | 1 => continue,
                _ => num,
            },
            Err(_) => continue,
        };

        let mut f_n1 = 1;
        let mut f_n2 = 0;

        for x in 0..sequence_length {
            let mut num_str = String::new();
            if x < 2 {
                num_str = x.to_string();
            } else {
                let f_n = f_n1 + f_n2;
                num_str = f_n.to_string();
                f_n2 = f_n1;
                f_n1 = f_n;
            }
            if x < sequence_length - 1 {
                print!("{}, ", num_str);
            } else {
                println!("{}", num_str);
            }
        }
    }
}
