fn connect_to_s3() {}

fn connect_to_webdev() {}

fn upload_to_s3() {}

fn upload_to_webdev() {}

fn upload_image(image: Image, company: Company) {
    match company.storage_preference {
        Storage::S3 => upload_to_s3(),
        Storage::Webdev => upload_to_webdev(),
    }
}

struct Image {}

enum Storage {
    S3,
    Webdev,
}

struct Company {
    storage_preference: Storage,
}

fn main() {
}
