/// https://docs.rs/bitutils/latest/src/bitutils/lib.rs.html#11-14
pub fn sign_extend(data: u32, size: u32) -> i32 {
    assert!(size > 0 && size <= 32);
    ((data << (32 - size)) as i32) >> (32 - size)
}
