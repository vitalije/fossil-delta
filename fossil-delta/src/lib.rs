/*! Implementation of the delta algorithm used by fossil version control system.

```rust
use fossil_delta::{delta, deltainv};
let a = b"line 1
  yet another (a bit longer) line 2
  yet another (a bit longer) line 3
  yet another (a bit longer) line 4
  yet another (a bit longer) line 5
  yet another (a bit longer) line 6
  yet another (a bit longer) line 7
  yet another (a bit longer) line 8
  yet another (a bit longer) line 9
  yet another (a bit longer) line 10";
let b = b"line 1
  yet another (a bit longer) line 2
  yet another (a bit longer) line 3
  yet another (a bit longer) line 4
  yet another (a bit longer) line 5
  yet another (a bit longer) line 6
  yet another (a bit longer) line 6 1/2
  yet another (a bit longer) line 7
  yet another (a bit longer) line 8
  yet another (a bit longer) line 9
  and finally last line 10";
let d = delta(a, b);
let s = deltainv(b, &d);
assert_eq!(&s, a);
assert_eq!(d.len(), 43);
```
*/

use std::convert::TryInto;
const NHASH: usize = 16;
/// converts integer to String in base 64
pub fn b64str(n: u32) -> String {
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
pub fn b64int<T: AsRef<[u8]> + ?Sized>(a: &T) -> u32 {
    b64int_read(a.as_ref()).0 as u32
}

pub fn b64int_read<T: AsRef<[u8]> + ?Sized>(a: &T) -> (usize, &[u8]) {
    let mut res = 0_usize;
    for (j, i) in a.as_ref().iter().enumerate() {
        let k = B64VALUES[(i & 127) as usize];
        if k == 255 {
             let a = a.as_ref();
            return (res, &a[j..]);
        }
        res = (res << 6) + (k as usize);
    }
    (res, b"")
}

const B64DIGITS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_', 'a',
    'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z', '~',
];
const B64VALUES: [u8; 128] = [
    255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
    255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
    255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
    255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8,
    6u8, 7u8, 8u8, 9u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 10u8, 11u8, 12u8, 13u8,
    14u8, 15u8, 16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8,
    30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 255u8, 255u8, 255u8, 255u8, 36u8, 255u8, 37u8, 38u8, 39u8,
    40u8, 41u8, 42u8, 43u8, 44u8, 45u8, 46u8, 47u8, 48u8, 49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8,
    56u8, 57u8, 58u8, 59u8, 60u8, 61u8, 62u8, 255u8, 255u8, 255u8, 63u8, 255u8,
];

/// Return the number digits in the base-64 representation of a positive integer
pub fn digit_count(v: usize) -> usize {
    let mut x = 64;
    for i in 1..10 {
        if x > v {
            return i;
        };
        x = x << 6;
    }
    11
}

/// Compute a 32-bit big-endian checksum on the N-byte buffer.  If the
/// buffer is not a multiple of 4 bytes length, compute the sum that would
/// have occurred if the buffer was padded with zeros to the next multiple
/// of four bytes.
fn checksum<T: AsRef<[u8]>>(z_in: T) -> u32 {
    let it = z_in.as_ref().chunks_exact(4);
    let b = it.remainder();
    let a_b: [u8; 4] = match b.len() {
        0 => [0, 0, 0, 0],
        1 => [b[0], 0, 0, 0],
        2 => [b[0], b[1], 0, 0],
        _ => [b[0], b[1], b[2], 0],
    };
    let mut s: u32 = u32::from_be_bytes(a_b);
    for b in it {
        let a_b: &[u8; 4] = unsafe { &*(b.as_ptr() as *const [u8; 4]) };
        let a = u32::from_be_bytes(*a_b);
        s = s.overflowing_add(a).0;
    }
    s
}
/// Generate new delta in given mutable string reference
///
/// ## Output Format:
///
/// The delta begins with a base64 number followed by a newline.  This
/// number is the number of bytes in the TARGET file.  Thus, given a
/// delta file z, a program can compute the size of the output file
/// simply by reading the first line and decoding the base-64 number
/// found there. The delta_output_size() routine does exactly this.
///
/// After the initial size number, the delta consists of a series of
/// literal text segments and commands to copy from the SOURCE file.
/// A copy command looks like this:
///
/// <pre>NNN@MMM,</pre>
///
/// where `NNN` is the number of bytes to be copied and `MMM` is the offset
/// into the source file of the first byte (both base-64).   If `NNN` is 0
/// it means copy the rest of the input file.  Literal text is like this:
///
/// <pre>NNN:TTTTT</pre>
///
/// where `NNN` is the number of bytes of text (base-64) and `TTTTT` is
/// the text.
/// The last term is of the form
///
/// <pre>NNN;</pre>
///
/// In this case, `NNN` is a 32-bit bigendian checksum of the output file
/// that can be used to verify that the delta applied correctly.  All
/// numbers are in base-64.
///
/// Pure text files generate a pure text delta.  Binary files generate a
/// delta that may contain some binary data.
///
/// ## Algorithm:
///
/// The encoder first builds a hash table to help it find matching
/// patterns in the source file.  16-byte chunks of the source file
/// sampled at evenly spaced intervals are used to populate the hash
/// table.
///
/// Next we begin scanning the target file using a sliding 16-byte
/// window.  The hash of the 16-byte window in the target is used to
/// search for a matching section in the source file.  When a match
/// is found, a copy command is added to the delta.  An effort is
/// made to extend the matching section to regions that come before
/// and after the 16-byte hash window.  A copy command is only issued
/// if the result would use less space that just quoting the text
/// literally. Literal text is added to the delta for sections that
/// do not match or which can not be encoded efficiently using copy
/// commands.
///
pub fn generate_delta<T: AsRef<[u8]>, V: AsRef<[u8]>>(
    z_out_t: T,         /* The target text */
    z_src_t: V,         /* The source text */
    z_delta: &mut Vec<u8>, /* A string to hold the resulting delta */
) {
    z_delta.clear();
    let z_src = z_src_t.as_ref();
    let z_out = z_out_t.as_ref();
    // match block backward
    let mb_backward = |i, j, n| {
        if i == 0 || j <= n {
            return 0;
        }
        let mut k = i - 1;
        let mut m = j - 1;
        let n_1 = if n == 0 { 0 } else { n - 1 };
        while k > 0 && m > n_1 {
            if z_src[k] != z_out[m] {
                return i - k - 1;
            }
            k -= 1;
            m -= 1;
        }
        i - k - 1
    };
    // match block forward
    let mb_forward = |i, j| {
        let mut k = i + 1;
        let mut m = j + 1;
        while k < z_src.len() && m < z_out.len() {
            if z_src[k] != z_out[m] {
                return k - i - 1;
            }
            k += 1;
            m += 1;
        }
        if z_src.len() - i < z_out.len() - j {
            z_src.len() - i - 1
        } else {
            z_out.len() - j - 1
        }
    };
    z_delta.extend_from_slice(b64str(z_out.len() as u32).as_bytes());
    z_delta.push(b'\n');

    /* If the source file is very small, it means that we have no
     ** chance of ever doing a copy command.  Just output a single
     ** literal segment for the entire target and exit.
     */
    if z_src.len() <= NHASH {
        z_delta.extend_from_slice(b64str(z_out.len() as u32).as_bytes());
        z_delta.push(b':');
        z_delta.extend_from_slice(z_out);
        z_delta.extend_from_slice(b64str(checksum(&z_out)).as_bytes());
        z_delta.push(b';');
        return;
    }
    /* Compute the hash table used to locate matching sections in the
     ** source file.
     */
    let n_hash = z_src.len() / NHASH;
    let mut collide = vec![0xffff_ffff_u32; 2 * n_hash];
    let mut h = Hash::new();
    for i in 0..n_hash {
        h.init(&z_src[(NHASH * i)..]);
        let hv = h.as_usize() % n_hash + n_hash;
        collide[i] = collide[hv];
        collide[hv] = i as u32;
    }
    let mut base = 0usize;
    while base + NHASH < z_out.len() {
        let mut i = 0;
        h.init(&z_out[base..]);
        let mut best_count = 0;
        let mut best_offset = 0;
        let mut best_lit_size = 0;
        loop {
            let hv = h.as_usize() % n_hash;
            let mut i_block = collide[n_hash + hv];
            let mut limit = 250;
            while i_block != 0xffff_ffff && limit > 0 {
                limit -= 1;
                let i_src = (i_block as usize) * NHASH;
                if z_src[i_src] == z_out[base + i] {
                    let j = mb_forward(i_src, base + i);
                    let k = mb_backward(i_src, base + i, base);
                    let ofst = i_src - k;
                    let cnt = j + k + 1;
                    let litsz = i - k;
                    let sz = digit_count(litsz) + digit_count(cnt) + digit_count(ofst) + 3;
                    if cnt > sz && cnt > best_count {
                        best_count = cnt;
                        best_offset = ofst;
                        best_lit_size = litsz;
                    }
                }
                i_block = collide[i_block as usize];
            }
            if best_count > 0 {
                if best_lit_size > 0 {
                    z_delta.extend_from_slice(b64str(best_lit_size as u32).as_bytes());
                    z_delta.push(b':');
                    z_delta.extend_from_slice(&z_out[base..(base + best_lit_size)]);
                    base += best_lit_size;
                }
                base += best_count;
                z_delta.extend_from_slice(b64str(best_count as u32).as_bytes());
                z_delta.push(b'@');
                z_delta.extend_from_slice(b64str(best_offset as u32).as_bytes());
                z_delta.push(b',');
                break;
            } else if base + i + NHASH >= z_out.len() {
                z_delta.extend_from_slice(b64str((z_out.len() - base) as u32).as_bytes());
                z_delta.push(b':');
                z_delta.extend_from_slice(&z_out[base..]);
                base = z_out.len();
                break;
            } else {
                h.update(z_out[base + NHASH + i]);
                i += 1;
            }
        }
    }
    if base < z_out.len() {
        z_delta.extend_from_slice(b64str((z_out.len() - base) as u32).as_bytes());
        z_delta.push(b':');
        z_delta.extend_from_slice(&z_out[base..]);
    }
    z_delta.extend_from_slice(b64str(checksum(z_out)).as_bytes());
    z_delta.push(b';');
}

/// Creates delta and returns it as a String
/// see [`generate_delta`]
pub fn delta<T: AsRef<[u8]>, V: AsRef<[u8]>>(a: T, b: V) -> Vec<u8> {
    let mut d = Vec::with_capacity(b.as_ref().len() + 60);
    generate_delta(a, b, &mut d);
    d
}

pub fn apply<T: AsRef<[u8]>, V: AsRef<[u8]>>(source: T, delta: V) -> Vec<u8> {
    let source = source.as_ref();

    let (total_length, mut delta) = b64int_read(&delta);
    let mut output = Vec::with_capacity(total_length);

    delta = &delta[1..];
    while delta.len() > 0 {
        let (cnt, delta_read) = b64int_read(delta);
        match delta_read[0] {
            b'@' => {
                let (offset, delta_read) = b64int_read(&delta_read[1..]);
                output.extend_from_slice(&source[offset..(offset + cnt)]);
                delta = &delta_read[1..];
            }
            b':'=> {
                let i = delta.len() - delta_read.len() + 1;
                output.extend_from_slice(&delta[i..(cnt + i)]);
                delta = &delta_read[(1 + cnt)..];
            }
            b';' => {
                if cnt != checksum(&output).try_into().unwrap() {
                    panic!("{}", "bad checksum");
                }
                return output;
            }
            c => {
                panic!("unexpected character {}", c)
            }
        }
    }
    
    output
}

/// Return the size (in bytes) of the output from applying
/// a delta.
///
/// This routine is provided so that an procedure that is able
/// to call delta_apply() can learn how much space is required
/// for the output and hence allocate nor more space that is really
/// needed.
///
pub fn delta_output_size<T: AsRef<[u8]>>(z_delta: T) -> usize {
    b64int(&z_delta) as usize
}

/// Given the current version of text value `b_txt` and delta value as `d_txt`
/// this function returns the previous version of text b_txt.
pub fn deltainv<T: AsRef<[u8]>, V: AsRef<[u8]>>(b_txt: T, d_txt: V) -> Vec<u8> {
    let (total_length, mut d_src) = b64int_read(&d_txt);

    let mut a_res = Vec::with_capacity(total_length);
    let b_txt = b_txt.as_ref();
    let b_bytes = b_txt;
    let d_txt = d_txt.as_ref();
    d_src = &d_src[1..];
    while d_src.len() > 0 {
        let (cnt, d1_src) = b64int_read(d_src);
        match d1_src[0] {
            b'@' => {
                let (ofst, d1_src) = b64int_read(&d1_src[1..]);
                a_res.extend_from_slice(&b_bytes[ofst..(ofst + cnt)]);
                d_src = &d1_src[1..];
            }
            b':' => {
                let i = d_txt.len() - d1_src.len() + 1;
                a_res.extend_from_slice(&d_txt[i..(cnt + i)]);
                d_src = &d1_src[(1 + cnt)..];
            }
            b';' => return a_res,
            _ => {
                let msg = format!(
                    r#"Error in applying delta
        txt: {:?}
        -----------------------------
        delta: {:?}
        =============================
        index: {}
        "#,
                    b_txt,
                    d_txt,
                    d_txt.len() - d1_src.len()
                );
                panic!("{}", msg)
            }
        }
    }
    a_res
}

const NHASH_1: usize = NHASH - 1;
const NHASHI32: i32 = NHASH as i32;
struct Hash {
    a: u16,
    b: u16,
    i: usize,
    z: [u8; NHASH],
}
impl Hash {
    fn new() -> Self {
        Hash {
            a: 0,
            b: 0,
            i: 0,
            z: [0; NHASH],
        }
    }
    /// Initialize the rolling hash using the first NHASH characters of z[]
    fn init(&mut self, z: &[u8]) {
        let mut a = z[0] as u32;
        let mut b = z[0] as u32;
        self.z[0] = z[0];
        for i in 1..NHASH {
            a = (a + (z[i] as u32)) & 0xffff;
            b = (b + a) & 0xffff;
            self.z[i] = z[i];
        }
        self.a = a as u16;
        self.b = b as u16;
        self.i = 0;
    }
    /// Advance the rolling hash by a single character c
    fn update(&mut self, c: u8) {
        let old = self.z[self.i];
        self.z[self.i] = c;
        self.i = (self.i + 1) & NHASH_1;
        let a = (self.a as i32) + (c as i32) - (old as i32);
        let b = (self.b as i32) - NHASHI32 * (old as i32) + (a & 0xffff);
        self.a = (a & 0xffff) as u16;
        self.b = (b & 0xffff) as u16;
    }
    /// Return a usize hash value
    fn as_usize(&self) -> usize {
        (self.a as usize) | ((self.b as usize) << 16)
    }
}
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
    #[test]
    fn test_hash_update() {
        let mut h = Hash::new();
        h.init(b"0123456789ABCDEFFEDCBA9876543210");
        assert_eq!(h.as_usize(), 0x1cbb03a2);
        let mut h2 = Hash::new();
        h2.init(b"123456789ABCDEFFEDCBA9876543210");
        h.update(b'F');
        assert_eq!(h.as_usize(), h2.as_usize())
    }
    #[test]
    fn delta_gen() {
        let old = include_str!("test-data/file-a.txt");
        let cur = include_str!("test-data/file-b.txt");
        let d1: &[u8] = include_bytes!("test-data/file-delta.txt");
        let mut d = Vec::new();
        generate_delta(&cur, &old, &mut d);
        assert_eq!(d.as_slice(), d1);
    }
    #[test]
    fn round_trip_test() {
        let a = b"line 1
            yet another (a bit longer) line 2
            yet another (a bit longer) line 3
            yet another (a bit longer) line 4
            yet another (a bit longer) line 5
            yet another (a bit longer) line 6
            yet another (a bit longer) line 7
            yet another (a bit longer) line 8
            yet another (a bit longer) line 9
            yet another (a bit longer) line 10";
        let b = b"line 1
            yet another (a bit longer) line 2
            yet another (a bit longer) line 3
            yet another (a bit longer) line 4
            yet another (a bit longer) line 5
            yet another (a bit longer) line 6
            yet another (a bit longer) line 6 1/2
            yet another (a bit longer) line 7
            yet another (a bit longer) line 8
            yet another (a bit longer) line 9
            and finally last line 10";
        let d = delta(a, b);
        println!("delta:{:?}", &d);
        let s = deltainv(b, &d);
        assert_eq!(&s, a);
        assert_eq!(d.len(), 43);
    }
    #[test]
    fn round_trip_test2() {
        let a = r#"def do_Expression(self, node):\n    '''An inner expression'''\n    self.visit(node.body)\n"#.as_bytes();
        let b = r#"sion(self, node):\n    '''An inner expression'''\n    self.visit(node.body)\n"#.as_bytes();
        println!(
            "a.len={}, b.len={}, b64={}",
            a.len(),
            b.len(),
            &b64str(a.len() as u32)
        );
        let d = delta(b, a);
        println!("delta:{:?}", &d);
        let s = deltainv(a, &d);
        assert_eq!(&s, b);
    }
    #[test]
    fn empty_txt() {
        let a = "".as_bytes();
        let b = r#"line 1
      yet another (a bit longer) line 2
      yet another (a bit longer) line 3
      yet another (a bit longer) line 4
      yet another (a bit longer) line 5
      yet another (a bit longer) line 6
      yet another (a bit longer) line 6 1/2
      yet another (a bit longer) line 7
      yet another (a bit longer) line 8
      yet another (a bit longer) line 9
      and finally last line 10"#.as_bytes();
        let d = delta(b, a);
        println!("empty delta:{:?}", &d);
        let s = deltainv(a, &d);
        assert_eq!(b, &s);
    }
    #[test]
    fn test_deltainv() {
        let old = include_bytes!("test-data/file-a.txt");
        let cur = include_bytes!("test-data/file-b.txt");
        let d1: &[u8] = include_bytes!("test-data/file-delta.txt");
        let res = deltainv(cur, d1);
        assert_eq!(&res[..30], &old[..30]);
    }
    #[test] 
    fn apply_test() {
        let old = include_str!("test-data/file-a.txt");
        let cur = include_str!("test-data/file-b.txt");
        let d1: &[u8] = include_bytes!("test-data/file-delta.txt");
        let out = apply(&old, &d1);
        assert_eq!(cur, String::from_utf8_lossy(&out));
    }
    #[test]
    fn test_bug_001() {
        let a=b"send-snap\nimport zmq\n#c.user_dict.pop('sendsnap', None)\n@others\nmsg = \"snapshot %s\"% snap()\nsend(msg)\n#msg = \"getat 2019-07-11 10:06:21\"\n#res = send(msg, True)\n#with open('/tmp/proba', 'w') as out:\n#    out.write(res)\ng.es('ok')\n";
        let b=b"send-snap\nimport zmq\n#c.user_dict.pop('sendsnap', None)\n@others\nmsg = \"snapshot %s\"% snap()\n\nsend(msg)\n#msg = \"getat 2019-07-11 10:06:21\"\n#res = send(msg, True)\n#with open('/tmp/proba', 'w') as out:\n#    out.write(res)\ng.es('ok')\n";
        let d = b"3a\n1S@0,29@1T,31_Pqh;";
        let d1 = delta(&a, &b);
        assert_eq!(&d1, &d);
        let s = deltainv(b, &d1);
        assert_eq!(&s, a);
    }
    #[test]
    fn test_bug_002() {
        let a="from student import moja_tajna_funkcija\n\ndef check(a, b):\n    assert moja_tajna_funkcija(a, b) == a + b, \"Функција не даје добар резултат за аргументе: %r и %r\"%(a, b)\n\nif __name__ == '__main__':\n    for x in range(-100, 101):\n        for y in range(-100, 101):\n            check(x, y)\n    print(\"Функција ради коректно\")\n";
        let b="from student import moja_tajna_funkcija\n\ndef check(a, b):\n    assert moja_tajna_funkcija(a, b) == a + b, \"Функција не даје добар резултат за аргументе: %r и %r\"%(a, b)\n\nif __name__ == '__main__':\n    for x in range(-100, 101):\n        for y in range(-100, 101):\n            check(x, y)\n    print(\"Није пронађена грешка у твом програму.\")\n";
        let d = delta(&b, &a);
        let mut d1 = Vec::new();
        d1.extend_from_slice(b"6P\n5H@0,18:\x9d\xd0\xb8\xd1\x98\xd0\xb5 \xd0\xbf\xd1\x80\xd0\xbe\xd0\xbd\xd0\xb0\xd1\x92\xd0\xb5\xd0\xbd\xd0\xb0 \xd0\xb3\xd1\x80\xd0\xb5\xd1\x88\xd0\xba\xd0\xb0 \xd1\x83 \xd1\x82\xd0\xb2\xd0\xbe\xd0\xbc \xd0\xbf\xd1\x80\xd0\xbe\xd0\xb3\xd1\x80\xd0\xb0\xd0\xbc\xd1\x83.\")\n2mdlCq;");
        assert_eq!(d, d1);
    }
    #[allow(dead_code)]
    static BUG002_A:&str = r#"from student import moja_tajna_funkcija

def check(a, b):
    assert moja_tajna_funkcija(a, b) == a + b, "Функција не даје добар резултат за аргументе: %r и %r"%(a, b)

if __name__ == '__main__':
    for x in range(-100, 101):
        for y in range(-100, 101):
            check(x, y)
    print("Функција ради коректно")"#;

    #[allow(dead_code)]
    static BUG002_B:&str = r#"from student import moja_tajna_funkcija

def check(a, b):
    assert moja_tajna_funkcija(a, b) == a + b, "Функција не даје добар резултат за аргументе: %r и %r"%(a, b)

if __name__ == '__main__':
    for x in range(-100, 101):
        for y in range(-100, 101):
            check(x, y)
    print("Није пронађена грешка у твом програму.")"#;
}
