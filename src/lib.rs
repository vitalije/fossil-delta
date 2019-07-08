/// converts integer to String in base 64
pub fn b64str(n:u32) -> String {
  if n == 0 {
    String::from("0")
  } else {
    let mut res = String::new();
    let mut _n = n;
    while _n > 0 {
      res.insert(0, B64DIGITS[(_n & 63) as usize]);
      _n = _n >> 6;
    }
    res
  }
}

/// converts base 64 str to u32
pub fn b64int(a:&str) -> u32 {
  let mut res = 0_u32;
  for i in a.bytes() {
    let k = B64VALUES[(i & 127) as usize];
    if k == 255 { break }
    res = (res << 6) + (k as u32);
  }
  res
}
const B64DIGITS:[char;64] = [
  '0', '1', '2', '3', '4', '5', '6', '7',
  '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
  'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
  'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
  'W', 'X', 'Y', 'Z', '_', 'a', 'b', 'c',
  'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
  'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', '~'
];
const B64VALUES:[u8; 128] = [
  255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
  255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
  255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
  255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
  255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
  255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
    0u8,   1u8,   2u8,   3u8,   4u8,   5u8,   6u8,   7u8,
    8u8,   9u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
  255u8,  10u8,  11u8,  12u8,  13u8,  14u8,  15u8,  16u8,
   17u8,  18u8,  19u8,  20u8,  21u8,  22u8,  23u8,  24u8,
   25u8,  26u8,  27u8,  28u8,  29u8,  30u8,  31u8,  32u8,
   33u8,  34u8,  35u8, 255u8, 255u8, 255u8, 255u8,  36u8,
  255u8,  37u8,  38u8,  39u8,  40u8,  41u8,  42u8,  43u8,
   44u8,  45u8,  46u8,  47u8,  48u8,  49u8,  50u8,  51u8,
   52u8,  53u8,  54u8,  55u8,  56u8,  57u8,  58u8,  59u8,
   60u8,  61u8,  62u8, 255u8, 255u8, 255u8,  63u8, 255u8
];

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn b64_works() {
    for i in 0..1000 {
      let s = b64str(i);
      let s1 = b64str(i + 0x1_00_0000);
      assert_eq!(i, b64int(&s));
      assert_eq!(i, b64int(&s1) - 0x1_00_0000);
    }
  }
}
