pub mod customer_experience {
   pub mod front_of_house {
      pub mod hosting {
            pub fn add_to_waitlist() {}
         }
     }

     pub fn eat_at_restaurant() {
         //absolute
         crate::customer_experience::front_of_house::hosting::add_to_waitlist();

         // Relative path ( after changing)
         front_of_house::hosting::add_to_waitlist();
     }
}