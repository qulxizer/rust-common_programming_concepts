fn main() {
    variables_and_mutability()
}

fn variables_and_mutability() {
    // mut keyword used to make the variable a mutable(editable)
    
    // {
    //     let mut x = 5;
    //     println!("The value of x is: {x}");
    // 5
    //     x = 6;
    //     println!("The value of x is: {x}");
    // 6
    // }

    // Define a constant, constant not allowed to change or convert to mutable
    // {
    //     const SPEED_OF_LIGHT: u32 = 299_792_458;
    //     println!("{SPEED_OF_LIGHT}")
    // 299_792_458
    // }

    /* Shadowing, is creating a new var with the same name as the old one,
    but there is a catch if you declare var in another scope when the scope ends
    the var takes the old value.
    */

    //     {
    //         let x:i32 = 5;
    //         let x = x + 1;

    //         {
    //             let x = x * 2;
    //             println!("{x}")
    //             // 12
    //         }
    //         println!("{x}");
    //         // 6

    //     }

    // use case for shadowing, you dont have to make temp var as shown
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

}
