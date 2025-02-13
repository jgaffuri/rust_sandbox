//https://stevedonovan.github.io/rust-gentle-intro/1-basics.html


fn main() {
    println!("Hello, World!");
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}


