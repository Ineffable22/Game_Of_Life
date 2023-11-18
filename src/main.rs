use std::{thread, time};


fn display_block(block: &[[i8;5]]) {
    for row in block.iter(){
        for field in row.iter() {
            print!("{}", field);
        }
        println!();
    }
}

fn validate_block(block: &[[i8;5]], x: i8, y: i8) {
    // Cualquier célula viva con menos de dos vecinos vivos muere, como por subpoblación.
    // block
    // Cualquier célula viva con dos o tres vecinos vivos pasa a la siguiente generación.
    // Cualquier célula viva con más de tres vecinos vivos muere, como por sobrepoblación.
    // Cualquier célula muerta con exactamente tres vecinos vivos se convierte en una célula viva, como por reproducción.
}

fn window(block: &[[i8;5]]){
    for i in 1..=2 {
        display_block(&block);
        println!("Frame: {}", i);
        thread::sleep(time::Duration::from_millis(400));
        // print!("\x1B[2J\x1B[1;1H");
        for x in 0..5 {
            for y in 0..5 {
                validate_block(block, x, y);
            }
        }
    }
}

fn main() {
    let block = [
        [0,0,0,0,0],
        [0,0,1,1,0],
        [0,0,1,1,0],
        [0,0,0,0,0],
        [0,0,0,0,0]
    ];
    window(&block);

    
    // print!("{}[2J", 27 as char);
}
