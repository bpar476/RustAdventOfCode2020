use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(e) => panic!(e),
    };
    let reader = BufReader::new(file);

    let mut max_seat_id = 0;
    let mut occupied_seats = vec![false; 127 * 8 + 7];
    for line_raw in reader.lines() {
        let line = match line_raw {
            Ok(line) => line,
            Err(e) => panic!(e),
        };

        let row_definition = &line[0..7];
        let col_definition = &line[7..10];

        let row = binary_partition(row_definition, 'F', (0, 127));
        let col = binary_partition(col_definition, 'L', (0, 7));

        let seat_id = row * 8 + col;

        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }

        occupied_seats[(seat_id - 1) as usize] = true;
    }

    println!("The highest seat ID was {}", max_seat_id);

    let mut my_seat: i32 = -1;
    for i in 1..occupied_seats.len() - 1 {
        if occupied_seats[i - 1] && occupied_seats[i + 1] && !occupied_seats[i] {
            my_seat = (i + 1) as i32
        }
    }

    println!("My seat is {}", my_seat);
}

fn binary_partition(def: &str, lower_half_indicator: char, range: (i32, i32)) -> i32 {
    def.chars()
        .fold(range, |acc, half| -> (i32, i32) {
            let half_len = (acc.1 + 1 - acc.0) / 2;
            if half == lower_half_indicator {
                (acc.0, acc.1 - half_len)
            } else {
                (acc.0 + half_len, acc.1)
            }
        })
        .0
}
