fn main() {

    let profit_or_loss = |sales: u128, cost: u128| -> i128 {
        
        return sales as i128 - cost as i128
    };

    println!("Sales result1: {}", profit_or_loss(500_000, 487_000));

    println!("Sales result2: {}", profit_or_loss(500_000, 687_000));

}

// Sales result1: 13000
// Sales result2: -187000
