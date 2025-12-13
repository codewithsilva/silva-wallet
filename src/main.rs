#[derive(Debug)]
struct CdiEvent {
  description:String,
  amount:i64,
}

#[derive(Debug)]
struct Wallet {
  owner:String,
  balance:i64,
  cdi_events: Vec<CdiEvent>
}

fn main() {
  let wallet = Wallet {
    owner: "Silva".to_string(),
    balance:0, 
  };

  println!("Balance: {:?}, Owner: {:?}", wallet.balance, wallet.owner);
}
