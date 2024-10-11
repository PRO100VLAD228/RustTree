fn main() {
    let cols = 9;
    let rows = (cols + 1) / 2;
    let layers = 3;

    for layer in 0..layers {
        for row in 0..rows {
            for col in 0..cols {
                if col >= (cols / 2) - row && col <= (cols / 2) + row {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
