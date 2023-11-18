fn show_block(block: &[i8]) {
    for i in block.iter(){
        println!("{}", i);
    }
}

fn main() {
    let block : [i8; 4] = [1,2,3,4];

    println!("Hello, world!");
    show_block(&block);
}
