#[derive(Clone, Copy, PartialEq)]
pub enum Toppings {
    Onion,
    Sausage,
    Pineapple,
    Spinach,
    Cheetos,
    Oreos,
}


pub struct Pizza {
    toppings: Vec<Toppings>
}

impl Pizza {
    /// Create a new empty pizza
    pub fn new() -> Self {
        Pizza {
            toppings: Vec::new(),
        }
    }
    
    /// Add this topping to the pizza
    /// Return/throw a PizzaError if the topping is not valid according to check_topping
    pub fn add_topping(&mut self, topping: Toppings) -> Result<(), PizzaError> {
        // FIX THIS FUNCTION     
        // unwraps should only be called if we want the program to crash or 
        // are 100% certain the results is OK or some
        check_topping(&topping)?;
        self.toppings.push(topping);
        Ok(())

    }

    /// See if this pizza has a topping and how many times it was added.
    /// Return none if it was never added
    /// 

    pub fn has_topping(&self, topping: &Toppings) -> Option<usize> {
        let count = self.toppings.iter().filter(|x| *x == topping).count();
        if count > 0 {
            Some(count)
        } else {
            None
        }
    }
}

/// A simple empty struct to indicate an error. This could be an enum and could hold data.
#[derive(Debug, Clone)]
pub struct PizzaError;

/// Check if a topping is valid. Returns a Result of an Ok() or PizzaError if not
pub fn check_topping(topping: &Toppings) -> Result<(), PizzaError> {
    match topping {
        Toppings::Onion => Ok(()),
        Toppings::Pineapple => Ok(()),
        Toppings::Spinach => Ok(()),
        Toppings::Sausage => Ok(()),
        Toppings::Cheetos => Err(PizzaError{}),
        Toppings::Oreos => Err(PizzaError{}),

        /*Ok(()) => Toppings::Onion,
        Ok(()) => Toppings::Sausage,
        Ok(()) => Toppings::Pineapple,
        Err(_) => panic!("Fatal error"),*/

         // Everything else is bad news. What kind of monster would do that?
    }
}

