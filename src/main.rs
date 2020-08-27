fn main() {
    let c = skippy::command!(:SOMEthing:HEREnow[2]?, 32.5);
    println!("{:?}", c);
    println!("{}", c);
}
