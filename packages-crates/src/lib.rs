pub struct Credentials {
    pub username: String,
    pub password: String,
}

enum Status {
    Connected,
    Disconnected,
    Failed,
}
fn connected_to_db() -> Status {
    Status::Connected
}

fn get_user(){
    // fetch user from db
}
fn login(creds: Credentials){

    get_user()
}

pub fn authenticate(creds: Credentials) {


}

// // everything is private so use pub because w ewant to use authenticate fun in main.rs



// modules learnings
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}

        pub fn seat_at_table(){}
    }
    mod serving{
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

// new learning -> making one thing private , will need us to call pub impl for that
mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{

                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),

            }
        }
    }
}

pub fn eat_at_res(){

    // // abs
    //  crate::front_of_house::hosting::add_to_waitlist();

    // // relative
    // front_of_house::hosting::add_to_waitlist();


    // we cant construct our own breakfast here 
    // let breakfast = back_of_house::Breakfast{
    //     toast: String::from("Wheat"),
    // }

    // we need to use summer function impl from breakfast
    let breakfast = back_of_house::Breakfast::summer("Wheat");

}

