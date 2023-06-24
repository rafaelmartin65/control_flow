
fn main() {

    //LOOP CONTROL FLOW
    println!("*** LOOP CONTROL FLOW ***");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{counter}");

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);
    

    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count: {count}");


    // WHILE CONTROL FLOW
    println!("*** WHILE CONTROL FLOW ***");
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF");


    //FOR CONTROL FLOW
    println!("*** FOR CONTROL FLOW ***");
    let a = [10,20,30,40];

    for element in a {
        println!("The value is: {}", element);

    }
    
}
