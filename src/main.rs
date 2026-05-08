fn main() {
    let mut german  = String::from("du bist ein "); //create a string
    println!("{} ", german );
    german.push_str("Hund"); //Insert a String
    println!("{} ", german);
    german.replace_range(11..german.len(), " katze");
    println!("{}", german);
    german.replace_range(11..german.len(), "");//to replace //Delete or Replace a String portion
    println!("{} ", german);
}
