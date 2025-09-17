// Hamming weight functions

pub fn weight_u8(byte:u8) -> u64 {
  let mut count = 0;
  let mut b = byte;
  while b !=0 {
    count += (b & 1) as u64;
    b >>=1;
  }
  count
}


pub fn weight_u64(word:u64) -> u64 {
  let mut count = 0;
  let mut w = word;
  while w !=0 {
    count += w & 1;
    w >>=1;
  }
  count
}


pub fn weight_bytes(bytes:Vec<u8>)-> u64 {
  bytes.iter().map(|&b| weight_u8(b)).sum()
}


pub fn weight_words(words: Vec<u64>) -> u64 {
  words.iter().map(|&w| weight_u64(w)).sum()
}


// Hamming distance functions

pub fn distance_u8(b: u8, c: u8) -> u64 {
  weight_u8(b^c)
}

pub fn distance_u64(w: u64, x: u64) -> u64 {
  weight_u64(w^x)
}

pub fn distance_bytes(bs: Vec<u8>, cs: Vec<u8>) ->u64 {
  assert_eq!(bs.len(), cs.len(), "Vectors must be the same length");
  bs.iter().zip(cs.iter()).map(|(&b, &c)| distance_u8(b, c)).sum()
}

pub fn distance_words(ws:Vec<u64>, xs:Vec<u64>) -> u64 {
  assert_eq!(ws.len(), xs.len(), "Vectors must be the same length");
  ws.iter().zip(xs.iter()).map(|(&w, &x)| distance_u64(w, x)).sum()
}
  
  
