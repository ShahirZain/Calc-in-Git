use std::io;

fn main() {
    let mut x1 = String::new();
    let mut x2 = String::new();
    let mut flag = String::new();
    while true {
        println!("Enter Your Choice from 1 to 5");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Quit");

        let mut mynum = String::new();
        io::stdin()
            .read_line(&mut mynum)
            .expect("Failed to read your input");
        let  mynum: i32 = match mynum.trim().parse() {
            Ok(num) => num,
            Err(_) => 4,
        };

        match mynum {
            1 => {
                x1 = "0".to_string();
                x2 = "0".to_string();
                println!("enter first value");
                io::stdin()
                    .read_line(&mut x1)
                    .expect("Failed to read your input");
                let  x1: f32 = match x1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 4.0,
                };
                println!("enter second value");
                io::stdin()
                    .read_line(&mut x2)
                    .expect("Failed to read your input");
                let  x2: f32 = match x2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 4.0,
                };
                println!("Sum of two number is {}", x1 + x2);

               
            }
            2 => {
                x1 = "0".to_string();
                x2 = "0".to_string();
                println!("enter first value");
                io::stdin()
                    .read_line(&mut x1)
                    .expect("Failed to read your input");
                let  x1: f32 = match x1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 4.0,
                };
                println!("enter second value");
                io::stdin()
                    .read_line(&mut x2)
                    .expect("Failed to read your input");
                let  x2: f32 = match x2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 4.0,
                };
                println!("Subtraction of two number is {}", x1 - x2);

                
            }
            3 => {
                x1 = "0".to_string();
                x2 = "0".to_string();
                println!("enter first value");
                io::stdin()
                    .read_line(&mut x1)
                    .expect("Failed to read your input");
                let  x1: f32 = match x1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 4.0,
                };
                println!("enter second value");
                io::stdin()
                    .read_line(&mut x2)
                    .expect("Failed to read your input");
                let  x2: f32 = match x2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 4.0,
                };
                println!("Multiplication of two number is {}", x1 * x2);

                
            },
               4 => {
               x1 = "0".to_string();
               x2 = "0".to_string();
               println!("enter first value");      
               io::stdin().read_line(&mut x1)
               .expect("Failed to read your input");
                let  x1: f32 = match x1.trim().parse() {
                Ok(num) => num,
                 Err(_) => 4.0
                };
                println!("enter second value"); 
                io::stdin().read_line(&mut x2)
               .expect("Failed to read your input");
                let  x2: f32 = match x2.trim().parse() {
                Ok(num) => num,
                 Err(_) => 4.0
                };   
                println! ("Division of two number is {}",x1 / x2);

                
               } ,
            5 => break,
            _ => println!("something went wrong"),
        };

    println!("do you want to continue?");
    io::stdin().read_line(&mut flag)
               .expect("Failed to read your input");
       print!("{}", flag);
    if flag.to_string() == String::from("no") {
        print!("in {}", flag);
    }
    
    }
}
