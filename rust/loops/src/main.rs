fn main() {
    let a = [5; 10];

    let mut sum = 0;

    for x in a {
        println!("{x}");
        sum += x;
    }

    println!("{sum}");
}
