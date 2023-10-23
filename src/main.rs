fn main() {
    println!("Hello, world!");
    println!("This is my first line in Rust");

    // integer variable
    let int1 = 4;
    println!("int1 = {int1}");

    // mutable integer variable
    let mut int2 = 16;
    println!("int2 = {int2}");

    // change the value of int2
    int2 = int1 * int2;
    println!("int2 = {int2}");



    // float variable
    let float1 = 4.5;
    println!("float1 = {float1}");

    // mutable float variable
    let mut float2: f32 = 16.5;
    println!("float2 = {float2}");

    // change the value of float2
    float2 = float1 * float2;
    println!("float2 = {float2}");



    // Strings
    let mut string1 = String::new();
    string1.push_str("This is string 1");
    println!("string1 = {}", string1);

    let string2 = String::from("This is string 2");
    println!("string2 = {string2}");

    let mut string3 = String::new();
    string3 = string3 + "This is string 3";
    println!("string3 = {string3}");

    let string4 = "This is string 4";
    println!("string4 = {string4}");

    // print all strings
    println!("strings: {}, {}, {}, {}", string1, string2, string3, string4);
    println!("strings: {string1}, {string2}, {string3}, {string4}");



    let int3 = 3;

    // function
    fn function1(a: i32, b: i32) {
        println!("This is function 1 with parameters {a} and {b}");
    }

    let int4 = 4;
    function1(int3, int4);

    // function with return value
    fn function2() -> &'static str {
        println!("This is function 2");
        return "This is the return value of function2";
    }

    let _string5 = String::from(function2());
    println!("string5 = {_string5}");

    // struct
    struct Person {
        name: String,
        age: u32,
        favorite_color: String,
        likes_oranges: bool,
    }

    let person1 = Person {
        name: String::from("Sam"),
        age: 22,
        favorite_color: String::from("red"),
        likes_oranges: true,
    };

    let person2 = Person {
        name: String::from("Alex"),
        age: 25,
        favorite_color: String::from("blue"),
        likes_oranges: false,
    };

    // call before method is defined
    person1.data();

    // method
    impl Person {
        fn data(&self) {
            if self.likes_oranges == true {
                println!("{} is {} years old, likes the color {} and likes oranges",
                         self.name, self.age, self.favorite_color);
            } else {
                println!("{} is {} years old, likes the color {} and does not like oranges",
                         self.name, self.age, self.favorite_color);
            }
        }
    }

    // call after method is defined
    person2.data();
}




