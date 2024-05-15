fn main() {
    let x: i32 = 16;
    println!("{}", x);

    let y: String = String::from("Hello, Soroban");
    println!("{y}");

    let y: &str = "Hello, Stellar!";
    println!("{y}");

    event();
    
    pub fn event(){
        let name: String = String::from("Hi, I am Sai");
        println!("{name}");
    }

    let e: EventForKids = EventForKids {
        name: String::from("KidsCo"),
        date:String::from("22-3-2024"),
        no_of_participant: u32 = 1000,
        place: String::from("Vilathikulam")
    }

}

// Compiling many items in one class
struct EventForKids{
    name: String,
    date: String,
    no_of_participant: u32,
    place: String
}

//enums: compiling error
enums ErrorForEvents {
    noEvent,
    cancelledEvent,
    eventType
}
