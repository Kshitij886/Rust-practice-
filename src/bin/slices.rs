// write a function that takes a string as an imput and returns a first word from it
// we use slices for this
fn main() {
    let name = String::from("KShitij khatri");
    // let ans = first_word(&name);
    // println!("First word is {}", ans);
    let ans2 = first_word_slices(&name);
    println!("{}", ans2);
}

fn first_word(name: &String) -> String {
    let mut ans = String::from("");
    for i in name.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}

// using slices
fn first_word_slices(str: &String) -> &str {
    for (index, i) in str.chars().enumerate() {
        if i == ' ' {
            return &str[0..index];
        }
    }
    return &str[..];
}
