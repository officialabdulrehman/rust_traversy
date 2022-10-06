pub fn run() {
    let name = "NizTheDev";
    let mut age = 23;

    println!("My name is {} and I am {}", name, age);

    age = 24;
    println!("My name is {} and I am {}", name, age);

    let (my_name, my_age) = ("NizTheDev", 24);
    println!("My name is {} and I am {}", my_name, my_age);

    const ID: i32 = 001;
    println!("ID is {}", ID)
}
