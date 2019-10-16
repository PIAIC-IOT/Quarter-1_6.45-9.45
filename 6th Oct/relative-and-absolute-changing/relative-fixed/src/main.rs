pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


pub mod dining {

     pub fn eat_at_restaurant() {
         // Absolute path ( after )
         crate::front_of_house::hosting::add_to_waitlist();

         // Relative path ( changes)
         front_of_house::hosting::add_to_waitlist();
     }
     
}