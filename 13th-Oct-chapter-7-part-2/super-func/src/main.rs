mod first{
    pub fn serve_order() {
         println!("here is your food");
    }
     pub mod hello {
         pub mod back_of_house {
             pub fn fix_incorrect_order() {
                 crate::first::serve_order();
             }
         }
     }  
}

fn main() {
    first::hello::back_of_house::fix_incorrect_order()
}
