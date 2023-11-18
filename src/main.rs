use std::cmp::{max, min};
use std::{thread, time};

fn display_block(block: Vec<[i32; 5]>) {
    for row in block.iter() {
        for field in row.iter() {
            print!("{:?}", field);
        }
        println!();
    }
}

fn validate_block(block: Vec<[i32; 5]>, x: i8, y: i8) -> &mut Vec<[i32; 5]> {
    let mut count: i8 = 0;
    let center_point: bool = if block[x as usize][y as usize] == 1 { true } else { false };
    let x_min = max(0, x - 1);
    let x_max = min(4, x + 1);
    let y_min = max(0, y - 1);
    let y_max = min(4, y + 1);

    for i in x_min..=x_max {
        for j in y_min..=y_max {
            let field: bool = if block[i as usize][j as usize] == 1 { true } else { false };
            match field {
                true => count += 1,
                false => count += 0,
            } 
        }
    }
    // println!("{:?} {} {} block => {} {}", block, x, y, block[x as usize][y as usize], count);
    // println!("{} {} {} {}", x_min,x_max,y_min,y_max);

    match center_point {
        // match block[i][j] {
        true => {
            count -= 1;
            match count {
                // Cualquier célula viva con menos de dos vecinos vivos muere, como por subpoblación.
                0 => block[x as usize][y as usize] = 0,
                1 => block[x as usize][y as usize] = 0,
                // Cualquier célula viva con dos o tres vecinos vivos pasa a la siguiente generación.
                2 => block[x as usize][y as usize] = 1,
                3 => block[x as usize][y as usize] = 1,
                // Cualquier célula viva con más de tres vecinos vivos muere, como por sobrepoblación.
                _ => {
                    println!("AAAAA");
                    block[x as usize][y as usize] = 0;
                }
            }
        },
        false => {
            match count {
                // Cualquier célula muerta con exactamente tres vecinos vivos se convierte en una célula viva, como por reproducción.
                3 => block[x as usize][y as usize] = 1,
                _ => block[x as usize][y as usize] = 0,
            }
        },
    };
}

fn window(mut block: Vec<[i32; 5]>) {
    let mut tmp_block: Vec<[i32; 5]>;
    for i in 1..=3 {
        display_block(block.clone());
        println!("Frame: {}", i);
        thread::sleep(time::Duration::from_millis(400));
        // print!("\x1B[2J\x1B[1;1H");
        for x in 0..5 {
            for y in 0..5 {
                tmp_block = validate_block(block.clone(), x, y);
            }
        }
        block = tmp_block;
    }
}

fn main() {
    let mut block: Vec<[i32; 5]> = Vec::new();
    block.push([0, 0, 0, 0, 0]);
    block.push([0, 0, 0, 1, 0]);
    block.push([0, 0, 1, 1, 0]);
    block.push([0, 0, 0, 0, 0]);
    block.push([0, 0, 0, 0, 0]);
    window(block);
}
