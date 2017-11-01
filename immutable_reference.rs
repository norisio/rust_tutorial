fn ref_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = "resource".to_string();
    let t = &s;

    ref_string(t);
    ref_string(&s);
}
