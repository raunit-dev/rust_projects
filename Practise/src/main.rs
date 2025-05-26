fn main() {
    let str1 = String::from("Raunit");
    let ans;
    {
        let str2 = String::from("jaiswalistheman");
        ans = string_longest(&str1,&str2);
        println!("{}",ans);
    }
    
}

fn string_longest<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    }

    return s2;
}