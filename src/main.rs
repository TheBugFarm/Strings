fn main() {
    let mut r = String::from("Navneet"); //create a string
    println!("{} ", r);
    r.push_str(" Greegs"); //Insert a String
    println!("{} ", r);
    r.replace_range(7..10,"Baj"); //Delete or Replace a String portion
    println!("{} ", r);
}
