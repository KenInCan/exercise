fn main() {
    let customer = "a".to_string();

    if customer == "a".to_string() {
        connect_to_server_a();
        upload_to_server_a();
    } else {
        connect_to_server_b();
        upload_to_server_b();
    }
}
