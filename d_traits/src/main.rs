
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cake {
    Chocolate,
    MapleBacon,
    Spice,
}

#[derive(Debug)]
pub struct Party {
    pub at_restaurant: bool,
    pub num_people: u8,
    pub cake: Cake,
}

impl Default for Party {
    fn default() -> Self {
        Self {
            at_restaurant: true,
            num_people: 8,
            cake: Cake::Chocolate,
        }
    }
}

impl PartialEq for Party {
    fn eq(&self, other: &Self) -> bool {
        self.cake == other.cake
    }
}


fn main() {
    
    let cake = Cake::Spice;
    admire_cake(cake); 
    
    match cake {
         Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate."),
         Cake::MapleBacon => println!("Dreams do come true!"),
         Cake::Spice => println!("Great, let's spice it up!"),
     }

     println!("The default Party is\n{:#?}", Party::default());


     let party = Party {
         cake: Cake::MapleBacon,
         ..Default::default()
    };
    println!("Yes! My party has my favorite {:?} cake!", party.cake);


    let other_party = Party {
        at_restaurant: false,
        num_people: 235,
        cake: Cake::MapleBacon,
    };
    if party == other_party {
        println!("Your party is just like mine!");
    }
}


pub fn admire_cake(cake: Cake) {
    println!("What a nice {:?} cake! 🎂", cake);
}