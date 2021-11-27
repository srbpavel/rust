#![allow(unused_variables)]
fn main() {
   #[derive(Debug)]
   enum UsState {
   	Alabama,
	Alaska,
	}

   enum Coin {
   	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
	}

fn value_in_cents(coin: Coin) -> u32 {
   match coin {
   	 Coin::Penny => {
	 	     println!("\nLucky penny!");
		     1
	 }
         Coin::Nickel => 5,
	 Coin::Dime => 10,
	 Coin::Quarter(state) => {
 	             println!("\nState quarter from {:?}", state);
		     25
	 }
	}
   }


   let ccc = value_in_cents(Coin::Quarter(UsState::Alaska));
   println!("ccc: {}", ccc);

   let ppp = value_in_cents(Coin::Penny);
   println!("ppp: {}", ppp);
}
