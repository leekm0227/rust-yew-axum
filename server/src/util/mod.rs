pub mod jwt;

pub fn hash(plain_text: String) -> String {
    format!("{:x}", md5::compute(plain_text.as_bytes()))
}