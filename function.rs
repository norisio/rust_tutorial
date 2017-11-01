fn add(x: isize, y: isize) -> isize{
    x + y 
}


fn main(){
    println!("{}", add(1, 5));

    let f = add;
    println!("{}", f(1,2));
}

