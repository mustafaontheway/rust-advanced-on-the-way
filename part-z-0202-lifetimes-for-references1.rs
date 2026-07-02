fn main() {

    let name = "Aygün".to_string();

    {
        let ref_name_1 = &name;

        println!("Her name is {ref_name_1}")
    }

    let new_owner = name;

    //println!("Her name is {ref_name_1}") // cannot find value `ref_name_1` in this scope

    println!("Her name is {new_owner}");

    //println!("Her name is {name}") // borrow of moved value: `name`
}

