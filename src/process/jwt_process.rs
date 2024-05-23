use anyhow::Result;
use jwt_simple::{
    algorithms::{HS256Key, MACLike},
    claims::{Claims, NoCustomClaims},
    common::VerificationOptions,
};
use std::{fs, time::Duration};
pub fn process_jwt_sign(sub: impl ToString, aud: impl ToString, exp: &str) -> Result<String> {
    let key = HS256Key::generate();
    let key_vec = key.to_bytes().to_vec();
    fs::write("./fixtures/jwt.key", key_vec)?;
    let valid_for = parse_duration(exp)?;
    let claims = Claims::create(valid_for.into())
        .with_subject(sub)
        .with_audience(aud);
    let token = key.authenticate(claims)?;
    Ok(token)
}

pub fn process_jwt_verify(token: &str) -> Result<bool> {
    let mut options = VerificationOptions::default();
    // 不接受将来的时间
    options.accept_future = false;
    // 时间容忍度为0
    options.time_tolerance = Some(Duration::from_secs(0).into());
    let key = fs::read("./fixtures/jwt.key")?;
    let key = HS256Key::from_bytes(key.as_slice());
    let ret = match key.verify_token::<NoCustomClaims>(&token, Some(options)) {
        Ok(_) => true,
        Err(_) => false,
    };
    Ok(ret)
}

fn parse_duration(exp: &str) -> Result<Duration> {
    let mut seconds = 0u64;
    let mut num_str = String::new();
    for c in exp.chars() {
        if c.is_digit(10) {
            num_str.push(c);
        } else {
            let num: u64 = num_str.parse().expect("Invalid number");
            num_str.clear();
            seconds += match c {
                's' => num,
                'm' => num * 60,
                'h' => num * 60 * 60,
                'd' => num * 60 * 60 * 24,
                _ => return Err(anyhow::anyhow!("Invalid duration unit")),
            };
        }
    }
    Ok(Duration::from_secs(seconds))
}
