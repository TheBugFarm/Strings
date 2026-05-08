use std::io;
fn main() {
    let mut s = String::new();
    println!("Enter String ");
    io::stdin().read_line(&mut s).expect("Failed to take input");

    let ans = first_word(s);
    println!("{}", ans);
}
fn first_word(s: String) -> String {
    let mut ans = String::from("");
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        ans.push(i);
    }
    return ans;
}

/*fn main() {
    let mut german  = String::from("du bist ein "); //create a string
    println!("{} ", german );
    german.push_str("Hund"); //Insert a String
    println!("{} ", german);
    german.replace_range(11..german.len(), " katze");
    println!("{}", german);
    german.replace_range(11..german.len(), "");//to replace //Delete or Replace a String portion
    println!("{} ", german);
}*/
