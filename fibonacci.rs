fn main(){
    let ans = fibonacci(10);
    println!("{}", ans);
}


fn fibonacci(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if n == 0 {
        return 0;
    }
    else if n == 1 { 
        return 1;
    }

    for _ in 0..n {
        let temp = first + second;
        first = second;
        second = temp;
        println!("first: {}, second: {}", first, second); // Debug output
    }
    return second;
}