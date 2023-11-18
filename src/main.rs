use std::cmp::{max, min};
use std::{thread, time};
const TIME: u64 = 180;
const FRAMES: i32 = 120;

fn display_block(block: Vec<Vec<u32>>) {
    for row in block.iter() {
        for field in row.iter() {
            print!("{:?}", field);
        }
        println!();
    }
}

fn validate_block(mut block: Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count: i8 = 0;
    let center_point: bool = if block[x][y] == 1 { true } else { false };
    let x_min = max(0, x as i8 - 1);
    let x_max = min(block.len() as i8 - 1, x as i8 + 1);
    let y_min = max(0, y as i8 - 1);
    let y_max = min(block[0].len() as i8 - 1, y as i8 + 1);

    for i in x_min..=x_max {
        for j in y_min..=y_max {
            let field: bool = if block[i as usize][j as usize] == 1 {
                true
            } else {
                false
            };
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
                _ => block[x as usize][y as usize] = 0,
            }
        }
        false => {
            match count {
                // Cualquier célula muerta con exactamente tres vecinos vivos se convierte en una célula viva, como por reproducción.
                3 => block[x as usize][y as usize] = 1,
                _ => block[x as usize][y as usize] = 0,
            }
        }
    };
    // println!(">{:?}", block);
    block[x as usize][y as usize]
}

fn window(mut block: Vec<Vec<u32>>) {
    let mut tmp_block: Vec<Vec<u32>> = block.clone();
    for i in 1..=FRAMES {
        thread::sleep(time::Duration::from_millis(TIME));
        print!("\x1B[2J\x1B[1;1H");
        display_block(block.clone());
        println!("Frame: {} | milliseconds: {}", i, TIME);
        for x in 0..block.len() {
            for y in 0..block[0].len() {
                tmp_block[x][y] = validate_block(block.clone(), x, y);
            }
        }
        block = tmp_block.clone();
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_block() -> Vec<Vec<u32>> {
    let path = Path::new("mapping.txt");
    let file = File::open(&path);
    
    let file = match file {
        Ok(file) => file,
        Err(_) => panic!()
    };

    // Vector 2D para almacenar números
    let mut mapping: Vec<Vec<u32>> = Vec::new();

    // Iterar sobre cada línea del archivo
    for line in io::BufReader::new(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => panic!()
        };

        // Validar si la línea contiene solo números
        if line.chars().all(char::is_numeric) {
            // Convertir la línea a un vector de números
            let nums: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            // Agregar el vector de números al vector 2D
            mapping.push(nums);
        } else {
            // En caso de que la línea no contenga solo números, mostrar un error
            eprintln!("Error: La línea no contiene solo números - {}", line);
        }
    }

    // Verificar que todos los vectores tengan la misma longitud
    let consistent_size = mapping.iter().all(|v| v.len() == mapping[0].len());

    if !consistent_size {
        eprintln!("Error: Los vectores no tienen la misma longitud");
    } else {
        // Imprimir el vector 2D resultante
        println!("{:?}", mapping);
    }
    mapping
}



fn main() {
    let block: Vec<Vec<u32>>;
    block = get_block();
    window(block);
}
