mod caesar;

fn main() {
    let message = "HELLO WORLD";
    let encrypted = caesar::encrypt(message);
    let decrypted = caesar::decrypt(&encrypted);

    println!("{} => {}", encrypted, decrypted);
}
