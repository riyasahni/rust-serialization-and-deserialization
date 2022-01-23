use solution::*;

#[test]
#[should_panic]
fn unwrap_bad_topping() {
    check_topping(&Toppings::Cheetos).unwrap();
}

#[test]
fn test_check_topping() {
    let sausage_ok = check_topping(&Toppings::Sausage).is_ok();
    let spinach_ok = check_topping(&Toppings::Spinach).is_ok();
    let cheetos_err = check_topping(&Toppings::Cheetos).is_err();
    assert!(sausage_ok);
    assert!(spinach_ok);
    assert!(cheetos_err);
}

#[test]
fn test_add_good_toppings() {
    let mut za = Pizza::new();
    let sausage_ok = za.add_topping(Toppings::Sausage).is_ok();
    let spinach_ok = za.add_topping(Toppings::Spinach).is_ok();
    let cheetos_err = za.add_topping(Toppings::Cheetos).is_err();
    assert!(sausage_ok);
    assert!(spinach_ok);
    assert!(cheetos_err);
}

#[test]
fn test_has_topping()  {
    let mut za = Pizza::new();
    assert!(za.add_topping(Toppings::Sausage).is_ok());
    assert!(za.add_topping(Toppings::Sausage).is_ok());
    //This passes but is bad form... Can you fix it?
    assert_eq!(Some(2), za.has_topping(&Toppings::Sausage));
    //FIXME, this does not use the interface correctly
    assert_eq!(None, za.has_topping(&Toppings::Spinach));

}