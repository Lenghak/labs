pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let rev = reverse("something");
    println!("{rev}");
}
