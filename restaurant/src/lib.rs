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
    mod hosting {
        fn add_to_waitlist() {}
        fn seata_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
