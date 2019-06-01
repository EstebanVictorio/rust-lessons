fn serve_order() { }

mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist() { }

        fn seat_at_table() { }
    }

    pub mod serving {
        fn take_order() { }
        
        fn serve_order() { }

        fn take_payment() { }
    }
}

pub use front_of_house::hosting as Hosting;

pub fn eat_at_restaurant() {
    Hosting::add_to_waitlist();

    Hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }


    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")

            }
        }
    }


    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() { }
    
}