fn task1() {
    /*
    TASK 1
    *******
    Clippy is a tool that allows you to catch mistakes and improve your code. For this
    exercise, run ``` cargo clippy ``` to see the suggestions and correct the errors.
    */
    let x = 1.2331f64;
    let y = 1.2332f64;
    if y != x {
        // clippy will suggest adding in a margin of error
        println!("Success!");
    }

    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        // clippy will suggest changing this to 'if let Some(x) = option'
        res += x;
    }
    println!("{}", res);
}

/*
    TASK 2
    *******
    Go back to the exercises in Module 1 and Module 2 and run cargo fmt
    Note if this changes your formatting.

    // no particular change in formatting with Module 1...
    // changes to Module 2 included: getting rid of ; for bool responses
    // and adding a comma to the last attribute in the struct I created for the last task
*/

/*
    TASK 3
    *******
    Generics Task:
    Create a generic struct for an Animal, containing name, animal type, and age. 
    Then, intitialize 3 different animals, and run cargo build to make sure your animals were initialized correctly.
*/
struct Animal<T, U> {
    name: T,
    animal_type: T,
    age: U,
}

fn main(){

    task1();

    let _snake = Animal {
        name: "Barry",
        animal_type: "Boa",
        age: 12,
    };

    let _sloth= Animal {
        name: "Garry",
        animal_type: "Pygmy",
        age: 4,
    };

    let _penguin= Animal {
        name: "Larry",
        animal_type: "Reptile",
        age: 81,
    };

   
}

