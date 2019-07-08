<?xml version="1.0" encoding="utf-8"?>
<!-- Created by Leo: http://leoeditor.com/leo_toc.html -->
<leo_file xmlns:leo="http://leoeditor.com/namespaces/leo-python-editor/1.1" >
<leo_header file_format="2"/>
<globals/>
<preferences/>
<find_panel_settings/>
<vnodes>
<v t="vitalije.20190708073913.1"><vh>@clean README.md</vh></v>
<v t="vitalije.20190708073255.1"><vh>project settings</vh>
<v t="vitalije.20190708073246.1"><vh>@button n-save @key=Ctrl-s</vh></v>
<v t="vitalije.20190708073146.1"><vh>@clean .gitignore</vh></v>
<v t="vitalije.20190708073324.1"><vh>@clean Cargo.toml</vh></v>
</v>
<v t="vitalije.20190708073438.1"><vh>@clean src/lib.rs</vh>
<v t="vitalije.20190708080758.1"><vh>b64 ints</vh>
<v t="vitalije.20190708080758.2"><vh>constants ...</vh></v>
<v t="vitalije.20190708090600.1"><vh>digit_count</vh></v>
<v t="vitalije.20190708091311.1"><vh>checksum</vh></v>
<v t="vitalije.20190708102546.1"><vh>delta_create</vh>
<v t="vitalije.20190708103528.1"><vh>delta_create - implementation</vh>
<v t="vitalije.20190708130411.1"><vh>0 - some utilities</vh>
<v t="vitalije.20190708131459.1"><vh>mb_backward</vh></v>
<v t="vitalije.20190708131503.1"><vh>mb_forward</vh></v>
</v>
<v t="vitalije.20190708105710.1"><vh>1 - write target length</vh></v>
<v t="vitalije.20190708105810.1"><vh>2 - check for small targets</vh></v>
<v t="vitalije.20190708110328.1"><vh>3 - compute the hash table</vh></v>
<v t="vitalije.20190708123712.1"><vh>4 - generate delta</vh></v>
<v t="vitalije.20190708134856.1"><vh>5 - tail</vh></v>
<v t="vitalije.20190708135156.1"><vh>6 - checksum record</vh></v>
</v>
</v>
<v t="vitalije.20190708102914.1"><vh>delta_output_size</vh></v>
</v>
<v t="vitalije.20190708083136.1"><vh>Hash</vh></v>
<v t="vitalije.20190708081957.1"><vh>LIBRARY TESTS</vh>
<v t="vitalije.20190708082017.1"><vh>b64_works</vh></v>
</v>
</v>
</vnodes>
<tnodes>
<t tx="vitalije.20190708073146.1" icons="5d71007d7101285804000000747970657102580400000066696c657103680358620000002f6d656469612f766974616c696a652f686f6d6531312f766974616c696a652f70726f6772616d692f6c656f2d656469746f722f7472756e6b2d6769742f6c656f2f49636f6e732f66696c655f69636f6e732f66696c655f636c65616e322e706e677104580700000072656c506174687105581a00000066696c655f69636f6e732f66696c655f636c65616e322e706e677106580500000077686572657107580e0000006265666f7265486561646c696e6571085807000000796f666673657471094b005807000000786f6666736574710a4b02580400000078706164710b4b0158020000006f6e710c5805000000766e6f6465710d580a00000076697375616c49636f6e710e580100000031710f75612e">@nocolor
/target
**/*.rs.bk
Cargo.lock
fossil-delta.leo</t>
<t tx="vitalije.20190708073246.1">c.save()
c.fileCommands.save_ref()</t>
<t tx="vitalije.20190708073255.1"></t>
<t tx="vitalije.20190708073324.1" icons="5d71007d7101285804000000747970657102580400000066696c657103680358620000002f6d656469612f766974616c696a652f686f6d6531312f766974616c696a652f70726f6772616d692f6c656f2d656469746f722f7472756e6b2d6769742f6c656f2f49636f6e732f66696c655f69636f6e732f66696c655f636c65616e322e706e677104580700000072656c506174687105581a00000066696c655f69636f6e732f66696c655f636c65616e322e706e677106580500000077686572657107580e0000006265666f7265486561646c696e6571085807000000796f666673657471094b005807000000786f6666736574710a4b02580400000078706164710b4b0158020000006f6e710c5805000000766e6f6465710d580a00000076697375616c49636f6e710e580100000031710f75612e">@nocolor
[package]
name = "fossil-delta"
version = "0.1.0"
authors = ["vitalije &lt;vitalije@kviziracija.net&gt;"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
</t>
<t tx="vitalije.20190708073438.1" icons="5d71007d7101285804000000747970657102580400000066696c657103680358620000002f6d656469612f766974616c696a652f686f6d6531312f766974616c696a652f70726f6772616d692f6c656f2d656469746f722f7472756e6b2d6769742f6c656f2f49636f6e732f66696c655f69636f6e732f66696c655f636c65616e322e706e677104580700000072656c506174687105581a00000066696c655f69636f6e732f66696c655f636c65616e322e706e677106580500000077686572657107580e0000006265666f7265486561646c696e6571085807000000796f666673657471094b005807000000786f6666736574710a4b02580400000078706164710b4b0158020000006f6e710c5805000000766e6f6465710d580a00000076697375616c49636f6e710e580100000031710f75612e">@language rust
@tabwidth -2
const NHASH:usize = 16;
@others
</t>
<t tx="vitalije.20190708073913.1" icons="5d71007d7101285804000000747970657102580400000066696c657103680358620000002f6d656469612f766974616c696a652f686f6d6531312f766974616c696a652f70726f6772616d692f6c656f2d656469746f722f7472756e6b2d6769742f6c656f2f49636f6e732f66696c655f69636f6e732f66696c655f636c65616e322e706e677104580700000072656c506174687105581a00000066696c655f69636f6e732f66696c655f636c65616e322e706e677106580500000077686572657107580e0000006265666f7265486561646c696e6571085807000000796f666673657471094b005807000000786f6666736574710a4b02580400000078706164710b4b0158020000006f6e710c5805000000766e6f6465710d580a00000076697375616c49636f6e710e580100000031710f75612e"># A Delta algorithm ported to Rust

### Work In Progress !!!

This algorithm is used in [fossil](https://fossil-scm.org) and there it has been implemented in C-99.
I needed it in my Rust projects and here is reimplementation in Rust.

Once it is ported it will expose two public functions:
    - delta:  fn(a:&amp;str, b:&amp;str) -&gt; String
    - deltainv:  fn(a:&amp;str, d:&amp;str) -&gt; String

```
// creating delta between a and its next version b
let d = delta(a, b);

// applying delta to b to get previous version a
let s = deltainv(b, d);

assert_eq!(s, a);
```

Therefore if you have some text value `a`, and user changes that text to value `b`, using
`delta` function you can get a compressed delta value `d` that you can store and keep it
along with the new text value `b`. If later user wants to see the previous version of text,
you can use `deltainv(b, d)` to get the previous value. If you keep all consequtive deltas
you can use `deltainv` multiple times to get any of the earlier text versions.

The Delta value calculated between two utf-8 encoded strings is itself utf-8 encoded string.

The algorithm is described very well
[here](https://fossil-scm.org/home/doc/trunk/www/delta_encoder_algorithm.wiki) in the fossil wiki.
</t>
<t tx="vitalije.20190708080758.1">/// converts integer to String in base 64
pub fn b64str(n:u32) -&gt; String {
  if n == 0 {
    String::from("0")
  } else {
    let mut res = String::new();
    let mut _n = n;
    while _n &gt; 0 {
      res.insert(0, B64DIGITS[(_n &amp; 63) as usize]);
      _n = _n &gt;&gt; 6;
    }
    res
  }
}

/// converts base 64 str to u32
pub fn b64int(a:&amp;str) -&gt; u32 { b64int_read(a).0 }

pub fn b64int_read(a:&amp;str) -&gt; (u32, &amp;str) {
  let mut res = 0_u32;
  for (j, i) in a.bytes().enumerate() {
    let k = B64VALUES[(i &amp; 127) as usize];
    if k == 255 { return (res, &amp;a[j..]); }
    res = (res &lt;&lt; 6) + (k as u32);
  }
  (res, "")
}
</t>
<t tx="vitalije.20190708080758.2">const B64DIGITS:[char;64] = [
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
];</t>
<t tx="vitalije.20190708081957.1">#[cfg(test)]
mod tests {
  use super::*;
  @others
  #[test]
  fn test_overflow(){
    let a:u16 = 65535;
    let b:u8 = 255;
    let c:u16 = a.overflowing_add(b as u16).0;
    assert_eq!(c, 254);
  }
  #[test]
  fn test_hash_update(){
    let mut h = Hash::new();
    h.init(b"0123456789ABCDEFFEDCBA9876543210");
    assert_eq!(h.as_u32(), 0x1cbb03a2);
    let mut h2 = Hash::new();
    h2.init(b"123456789ABCDEFFEDCBA9876543210");
    h.update(b'F');
    assert_eq!(h.as_u32(), h2.as_u32())
  }
  #[test]
  fn delta_gen() {
    let old = include_str!("test-data/file-a.txt");
    let cur = include_str!("test-data/file-b.txt");
    let d1 = include_str!("test-data/file-delta.txt");
    let mut d = String::new();
    delta_create(old, cur, &amp;mut d);
    assert_eq!(d, d1);
  }
}</t>
<t tx="vitalije.20190708082017.1">#[test]
fn b64_works() {
  for i in 0..1000 {
    let s = b64str(i);
    let s1 = b64str(i + 0x1_00_0000);
    assert_eq!(i, b64int(&amp;s));
    assert_eq!(i, b64int(&amp;s1) - 0x1_00_0000);
  }
}</t>
<t tx="vitalije.20190708083136.1">const NHASH_1:usize = NHASH - 1;
const NHASHU8:u8 = NHASH as u8;
const NHASHU16:u16 = NHASH as u16;
const NHASHI32:i32 = NHASH as i32;
struct Hash {
  a: u16,
  b: u16,
  i: usize,
  z: [u8; NHASH]
}
impl Hash {
  fn new() -&gt; Self { Hash {a:0, b:0, i:0, z:[0;NHASH]} }
  /// Initialize the rolling hash using the first NHASH characters of z[]
  fn init(&amp;mut self, z:&amp;[u8]) {
    let mut a = z[0] as u32;
    let mut b = z[0] as u32;
    self.z[0] = z[0];
    for i in 1..NHASH{
      a = (a + (z[i] as u32)) &amp; 0xffff;
      b = (b + a) &amp; 0xffff;
      self.z[i] = z[i];
    }
    self.a = a as u16;
    self.b = b as u16;
    self.i = 0;
  }
  /// Advance the rolling hash by a single character c
  fn update(&amp;mut self, c:u8) {
    let old = self.z[self.i];
    self.z[self.i] = c;
    self.i = (self.i + 1) &amp; NHASH_1;
    let a = (self.a as i32) + (c as i32) - (old as i32);
    let b = (self.b as i32) - NHASHI32 * (old as i32) + (a &amp; 0xffff);
    self.a = (a &amp; 0xffff) as u16;
    self.b = (b &amp; 0xffff) as u16;
  }
  /// Return a 32-bit hash value
  fn as_u32(&amp;self) -&gt; u32 { (self.a as u32) | ((self.b as u32) &lt;&lt; 16) }
  /// Return a usize hash value
  fn as_usize(&amp;self) -&gt; usize { (self.a as usize) | ((self.b as usize) &lt;&lt; 16)}
}
</t>
<t tx="vitalije.20190708090600.1">/// Return the number digits in the base-64 representation of a positive integer
pub fn digit_count(v:usize) -&gt; usize {
  let mut x = 64;
  for i in 1..10 {
    if x &gt; v { return i };
    x = x &lt;&lt; 6;
  }
  11
}</t>
<t tx="vitalije.20190708091311.1">
/// Compute a 32-bit big-endian checksum on the N-byte buffer.  If the
/// buffer is not a multiple of 4 bytes length, compute the sum that would
/// have occurred if the buffer was padded with zeros to the next multiple
/// of four bytes.

fn checksum(z_in:&amp;[u8]) -&gt; u32 {
  let it = z_in.chunks_exact(4);
  let b = it.remainder();
  let a_b:[u8;4] = match b.len() {
      0 =&gt; [0, 0, 0, 0],
      1 =&gt; [b[0], 0, 0, 0],
      2 =&gt; [b[0], b[1], 0, 0],
      _ =&gt; [b[0], b[1], b[2], 0]
  };
  let mut s:u32 = u32::from_be_bytes(a_b);
  for b in it {
    let a_b:&amp;[u8;4] = unsafe {&amp;*(b.as_ptr() as *const [u8; 4])};
    let a = u32::from_be_bytes(*a_b);
    s = s.overflowing_add(a).0;
  }
  s
}</t>
<t tx="vitalije.20190708102546.1">/// Create a new delta.
///
/// This function writes the delta in provided mutable string reference
///
/// Output Format:
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
/// &lt;pre&gt;NNN@MMM,&lt;/pre&gt;
///
/// where `NNN` is the number of bytes to be copied and `MMM` is the offset
/// into the source file of the first byte (both base-64).   If `NNN` is 0
/// it means copy the rest of the input file.  Literal text is like this:
///
/// &lt;pre&gt;NNN:TTTTT&lt;/pre&gt;
///
/// where `NNN` is the number of bytes of text (base-64) and `TTTTT` is
/// the text.
/// The last term is of the form
///
/// &lt;pre&gt;NNN;&lt;/pre&gt;
///
/// In this case, `NNN` is a 32-bit bigendian checksum of the output file
/// that can be used to verify that the delta applied correctly.  All
/// numbers are in base-64.
///
/// Pure text files generate a pure text delta.  Binary files generate a
/// delta that may contain some binary data.
///
/// Algorithm:
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
</t>
<t tx="vitalije.20190708102914.1">/// Return the size (in bytes) of the output from applying
/// a delta.
///
/// This routine is provided so that an procedure that is able
/// to call delta_apply() can learn how much space is required
/// for the output and hence allocate nor more space that is really
/// needed.
///
fn delta_output_size(z_delta:&amp;str) -&gt; usize { b64int(z_delta) as usize }</t>
<t tx="vitalije.20190708103528.1">fn delta_create(
  z_src_t:&amp;str /* The source text */,
  z_out_t:&amp;str /* The target text */,
  z_delta:&amp;mut String /* A string to hold the resulting delta */) {
  z_delta.clear();
  let z_src = z_src_t.as_bytes();
  let z_out = z_out_t.as_bytes();
  @others
}</t>
<t tx="vitalije.20190708105710.1">z_delta.push_str(&amp;b64str(z_out.len() as u32));
z_delta.push('\n');</t>
<t tx="vitalije.20190708105810.1">/* If the source file is very small, it means that we have no
** chance of ever doing a copy command.  Just output a single
** literal segment for the entire target and exit.
*/
if  z_src.len() &lt;= NHASH {
  z_delta.push_str(&amp;b64str(z_out.len() as u32));
  z_delta.push(':');
  z_delta.push_str(&amp;z_out_t);
  z_delta.push_str(&amp;b64str(checksum(&amp;z_out)));
  z_delta.push(';');
  return
}
</t>
<t tx="vitalije.20190708110328.1">/* Compute the hash table used to locate matching sections in the
** source file.
*/
let n_hash = z_src.len() / NHASH;
let mut collide = vec![0xffff_ffff_u32; 2 * n_hash];
let mut h = Hash::new();
for i in 0..n_hash {
  h.init(&amp;z_src[(NHASH * i)..]);
  let hv = h.as_usize() % n_hash;
  collide[i] = collide[n_hash + hv];
  collide[n_hash + hv] = i as u32;
}</t>
<t tx="vitalije.20190708123712.1">let mut base = 0usize;
while base + NHASH &lt; z_out.len() {
  let mut i = 0;
  h.init(&amp;z_out[base..]);
  let mut best_count = 0;
  let mut best_offset  = 0;
  let mut best_lit_size = 0;
  loop {
    let mut hv = h.as_usize() % n_hash;
    let mut i_block = collide[n_hash + hv];
    let mut limit = 250;
    while i_block != 0xffff_ffff &amp;&amp; limit &gt; 0 {
      limit -= 1;
      let i_src = (i_block as usize) * NHASH;
      if z_src[i_src] == z_out[base + i] {
        let j = mb_forward(i_src, base + i);
        let k = mb_backward(i_src, base + i, base);
        let ofst = i_src - k;
        let cnt = j + k + 1;
        let litsz = i - k;
        let sz = digit_count(litsz) + digit_count(cnt) + digit_count(ofst) + 3;
        if cnt &gt; sz &amp;&amp; cnt &gt; best_count {
          best_count = cnt;
          best_offset = ofst;
          best_lit_size = litsz;
        }
      }
      i_block = collide[i_block as usize];
    }
    if best_count &gt; 0 {
      if best_lit_size &gt; 0 {
        z_delta.push_str(&amp;b64str(best_lit_size as u32));
        z_delta.push(':');
        z_delta.push_str(&amp;z_out_t[base..(base + best_lit_size)]);
        base += best_lit_size;
      }
      base += best_count;
      z_delta.push_str(&amp;b64str(best_count as u32));
      z_delta.push('@');
      z_delta.push_str(&amp;b64str(best_offset as u32));
      z_delta.push(',');
      best_count = 0;
      break;
    } else if base + i + NHASH &gt;= z_out.len() {
      z_delta.push_str(&amp;b64str((z_out.len() - base) as u32));
      z_delta.push(':');
      z_delta.push_str(&amp;z_out_t[base..]);
      base = z_out.len();
      break;
    } else {
      h.update(z_out[base + NHASH + i]);
      i += 1;
    }
  }
}</t>
<t tx="vitalije.20190708130411.1"></t>
<t tx="vitalije.20190708131459.1">// match block backward
let mb_backward = |i, j, n| {
  if i == 0 || j &lt;= n {return 0}
  let mut k = i - 1;
  let mut m = j - 1;
  while k &gt; 0 &amp;&amp; m &gt;= n {
    if z_src[k] != z_out[m] { return i - k - 1; }
    k -= 1;
    m -= 1;
  }
  i - k - 1
};</t>
<t tx="vitalije.20190708131503.1">// match block forward
let mb_forward = |i, j| {
  let mut k = i + 1;
  let mut m = j + 1;
  while k &lt; z_src.len() &amp;&amp; m &lt; z_out.len() {
    if z_src[k] != z_out[m] { return k - i - 1; }
    k += 1;
    m += 1;
  }
  if z_src.len() - i &lt; z_out.len() - j {
    z_src.len() - i
  } else {
    z_out.len() - j
  }
};</t>
<t tx="vitalije.20190708134856.1">if  base &lt; z_out.len() {
    z_delta.push_str(&amp;b64str((z_out.len() - base) as u32));
    z_delta.push(':');
    z_delta.push_str(&amp;z_out_t[base..]);
}
</t>
<t tx="vitalije.20190708135156.1">z_delta.push_str(&amp;b64str(checksum(z_out)));
z_delta.push(';');</t>
</tnodes>
</leo_file>
