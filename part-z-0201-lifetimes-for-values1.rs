fn main() {

    let x: u16 = 400;

    {
        let y: u16 = 700;

        let sum = x + y;

        println!("Sum: {sum}");

        {
            let x: f32 = 4.147;

            println!("x value: {x}");
        }
    }

    println!("x value: {x}");
}

// Sum: 1100
// x value: 4.147
// x value: 400
