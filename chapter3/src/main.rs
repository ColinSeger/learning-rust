fn main() {
    let mut changing_variable: i16 = 10;
    println!("Hello, world! The variable is {changing_variable}");
    changing_variable += 5;

    println!("The variable is now {changing_variable}");

    {
        let changing_variable: i16 = 1337;
        println!("But here it is {changing_variable}");
    }

    println!("Yet it is still this {changing_variable}");

    function_call(changing_variable);
}

fn function_call(variable : i16){
    println!("function call with variable {variable}");
}