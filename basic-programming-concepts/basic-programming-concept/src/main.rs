fn main() {
    let x: i32 = 5;
    println!("The value of X is:{}", x);
    let x: &str = "six";
    println!("The value of X is:{}", x);

    const FOLLOWERS_COUNT: u32 = 100_000;

    //|||||Compound Types
    let game: &str = "football";
    println!("We have a game called :{}", game);
    let team_a: &str = "Barca";
    println!("Team A :{}", team_a);
    let team_b: &str = "Madrid";
    println!("Team B :{}", team_b);

    let tup: (&str, i32) = ("Barca", 3);
    let (team_name, score) = tup;
    println!("{} has scored {} goals", team_name, score);

    //chatgpt tuple challenge
    // 5 TUPLE CHALLENGES FOR PRACTICE
    // 1. Simple Tuple Access
    // Create a tuple with your name (&str), age (i32), and height (f64)

    // Print each value using dot notation

    // 2. Swap Two Numbers
    // Write a function that takes two numbers and returns them swapped in a tuple

    // Example: (2, 5) → (5, 2)

    // 3. Return Multiple Results
    // Write a function that takes two integers and returns:

    // sum

    // difference

    // product

    // Use a tuple to return all 3 results

    // 4. Destructure a Tuple
    // Make a tuple for (Nigeria, 60, true)

    // Destructure it into country, age, is_independent

    // Print each one

    // 5. Access Tuple in an Array
    // Create an array of 3 tuples:
    // let students = [("Mide", 80), ("Fola", 95), ("Seyi", 70)];
    // Print each student’s name and score using a loop and tuple dot notation

    //Challenge One
    let challenge: &str = "Tuple Challenge👇";
    println!("Challenge: {}", challenge);

    let challenge_one: &str = "1. Simple Tuple Access";
    let my_info: (&str, i32, f64) = ("Haleem", 23, 172.5);
    println!(
        "{}:name:{},age:{},height:{}",
        challenge_one, my_info.0, my_info.1, my_info.2
    );

    //Challenge Two
    // 2. Swap Two Numbers
    // Write a function that takes two numbers and returns them swapped in a tuple

    let challenge_two: &str = "2. Swap Two Numbers";
    fn swap_numbers(a: i32, b: i32) -> (i32, i32) {
        (b, a)
    }
    let a = 2;
    let b = 5;
    let result = swap_numbers(a, b);
    println!(
        "{}:Swapped numbers: ({}, {})",
        challenge_two, result.0, result.1
    );

    //Challenge Three
    //3. Return Multiple Results
    // Write a function that takes two integers and returns sum, difference, product
    // Use a tuple to return all 3 results
    // I WILL USE (10 AND 5)AS MY INPUTS
    let challenge_three: &str = "3. Return Mutiple Results";
    fn calculate(a: i32, b: i32) -> (i32, i32, i32) {
        (a + b, a - b, a * b)
    }
    let a = 10;
    let b = 5;
    let result = calculate(a, b);
    println!(
        "{}:Sum={},Difference={},Product={}",
        challenge_three, result.0, result.1, result.2
    );

    //Challenge 4
    // 4. Destructure a Tuple
    // Make a tuple for (Nigeria, 60, true)
    // Destructure it into country, age, is_independent
    // Print each one
    let challene_four: &str = "4. Destructure a Tuple";
    let country_info: (&str, i32, &str) = ("Nigeria", 60, "true");
    println!(
        "{}:country:{},age:{},independence:{}",
        challene_four, country_info.0, country_info.1, country_info.2
    );
    //Challenge 5
    // 5. Access Tuple in an Array
    // Create an array of 3 tuples:
    let challenge_five:&str="5. Access Tuple in an Array";
    let students:[(&str,i32);3]=[("Mide",80),("Fola",95),("Seyi",70)];
    println!("{}:Students and their scores:\n1. {:?}\n2. {:?}\n3. {:?}", challenge_five, students[0], students[1], students[2]);

    loop{
        println!("Haleem is the best Rust programmer in the world");
        break;
    }

    //counter from 1- 100
let mut counter=0;
loop{
    counter+=1;
    if counter==100{
        println!("Counter has reached 100, exiting loop");
        break counter;
    }
}
}
