pub enum IOEnum {
    Greeting {name: String }, 
    NewUser ,
    Help,
}

impl IOEnum {

    pub fn output(&self) -> String{
        match &*self {
            IOEnum::Greeting {name} 
                => format!("Welcome to your address book, {}!\n 
                             Enter `help` to find out available commands :)", name),
            IOEnum::NewUser
            => format!("Aha, a new user!\nMight I have a name to call you? -> "),
            IOEnum::Help => todo!(),
        }
    }
}
