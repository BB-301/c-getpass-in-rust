use c_getpass::getpass;

fn main() {
    // Using a prompt
    let max_len: usize = 5;
    let prompt = Some("Enter a secret value: ");
    let secret_value = getpass(max_len, prompt).expect("a multi-byte character was probably split");
    assert!(secret_value.len() <= max_len);
    println!("secret_value[{}] = {}", secret_value.len(), secret_value);

    // Without a prompt
    let max_len: usize = 50;
    let secret_value = getpass(max_len, <Option<String>>::None)
        .expect("a multi-byte character was probably split");
    assert!(secret_value.len() <= max_len);
    println!("secret_value[{}] = {}", secret_value.len(), secret_value);
}
