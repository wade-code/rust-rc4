use std::str;

fn rc4(key: &[u8], enc: &[u8], dec: &mut [u8]) {
  // KSA
  let key_sz = key.len();
  let mut s:[usize; 256] = [0; 256];
  let mut k:[usize; 256] = [0; 256];

  for i in 0..256 {
    s[i] = i;
    k[i] = usize::try_from(key[i % key_sz]).unwrap();
  }

  let mut j: usize = 0;
  for i in 0..256 {
    j = (j + s[i] + k[i]) & 0xff;
    s.swap(i, j);
  }

  // PRGA
  j = 0;
  let mut i: usize = 0;
  let enc_sz = enc.len();
  let mut key_stream: Vec<u8> = Vec::new();
  for _ in 0..enc_sz {
    i = (i + 1) & 0xff;
    j = (j + s[i]) & 0xff;
    s.swap(i, j);
    let t = (s[i] + s[j]) & 0xff;
    key_stream.push(s[t].try_into().unwrap());
  }

  // XOR
  dec.clone_from_slice(&enc[..enc_sz]);
  dec.iter_mut()
    .zip(key_stream.iter())
    .for_each(|(x0, x1)| *x0 ^= *x1);
}
