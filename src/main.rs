fn main() {
    let data: Vec<String> = std::env::args().collect();
    let decoded = base58::FromBase58::from_base58(data[1].as_str()).expect("Invalid base58");
    println!("{}", hex::encode(decoded));
}
