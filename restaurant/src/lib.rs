// The way privacy works in Rust is that all items(functions, methods, structs, enums, modules and constants) are private by default. Items in a parent module can't use the private items inside the child modules, but items in child modules can use the items in their ancestor modules.

// a path can take 2 forms:
// - an absolute path starts from a crate root by using a crate name or literal crate
// - a relative path starts from the current module and uses 'selsf', 'super', or an indentifier in the current module
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    // inside modules we can have other modules. Modules can also hold definitions for other items, such as structs, enums, constants, traits or functions

    // the 'pub' keyword on a module only lets code in its ancestor modules refer to it
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        // fn serve_order() {}
        fn take_payment() {}
    }
}
// _______________________________________

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        // the fix_incorrect_order() in in the back_of_house mod, so we can use 'super' to go to parent mod of the back_of_house, which in this case is crate, the root.
        super::serve_order();
    }
}

// bringing path into scope with absolute path, so that we use hosting::...
// use crate::front_of_house::hosting;

// bringing path into scope with relative path, so that we use hosting::...
// use front_of_house::hosting;

// eat_at_restaurant is part of our library crate's public API, so we mark it with the 'pub' keyword
pub fn eat_at_restaurant() {
    // absolute path
    // add_to_waitlist is defined in the same crate as eat_at_restaurant, which means we can use 'crate' to start an absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

// bringing two types with the same name in the same scope requires using their prent modules
