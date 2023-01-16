use simply_simconnect::simconnect::SimConnect;

fn main() {
    let s = &mut SimConnect::new();
    s.open("test").unwrap();
}
