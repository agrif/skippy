fn main() {
    let c = skippy::command!(:SOMEthing:HEREnow[2 + 5]?, 5 * 2, "they said \"hi\"");
    println!("{:?}", c);
    println!("{}", c);
}
