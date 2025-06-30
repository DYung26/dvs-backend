use rand::Rng;

pub fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100_000..1_000_000); // Ensures a 6-digit number
    otp.to_string()
}
