#[allow(dead_code, unused_variables)]
fn example_1() {
    // Allocate space in memory
    let highest_age: i32;

    // Initialize variables
    let alice_age: i32 = 20;
    let bob_age: i32 = 21;


    // Call function
    highest_age = largest(&alice_age, &bob_age);

    // Print output
    println!("Highest age is {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

 
#[allow(dead_code, unused_variables)]
fn example_2_generics() {
    // Allocate space in memory
    let highest_age: &i32;
    let new_value: i32;

    // Initialize variables
    let alice_age: i32 = 20;

    {

        let bob_age: i32 = 21;

        // Call function
        highest_age = largest(&alice_age, &bob_age);
        new_value = *highest_age;
    }



    // Print output
    println!("New value is {}", new_value);

    fn largest<'a, 'b: 'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'b T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}


#[allow(dead_code, unused_variables)]
struct Person<'p> {
    name: &'p str,
    points: &'p f32
}

#[allow(dead_code, unused_variables)]
fn example_3_with_struct() {
    // Allocate space in memory
    let highest_age: &f32;
    let new_value: f32;

    // Initialize variables
    let alice = Person {name: "alice ", points: &50.2};

    {

        let bob = Person {name: "bob ", points: &40.5};

        // Call function
        highest_age = largest(alice.points, bob.points);
        new_value = *highest_age;
    }



    // Print output
    println!("New value is {}", new_value);

    fn largest<'a, 'b: 'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'b T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}