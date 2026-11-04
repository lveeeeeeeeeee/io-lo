/// SAFETY: HUGE ASSUMPTION THAT IT IS USED STRICTLY TO GET STRINGS CONTAINING VALID ASCII SYMBOLS.
/// OTHERWISE IT IS UNSAFE. DO NOT USE THIS ANYWHERE.
///
/// I AM NOT TO BE HELD RESPONSIBLE FOR INCORRECT USAGE OF THIS CODE BY YOU.
///
/// you acknowledge that you have been warned
/// and that you have put this function call in an unsafe {} block at your own volition
/// repo: https://github.com/lveeeeeeeeeee/io-lo.git
pub mod scan {
    use std::io::BufRead;
    use std::str::FromStr;

    #[inline]
    fn find_symbols(buf: &[u8], sym_mask: &u128, from: usize) -> usize {
        let mut i = from;
        // assuming buf entries are ascii which we've already proclaimed
        while i < buf.len() && sym_mask & 1 << buf[i] == 0 {
            i += 1;
        }
        i
    }


    #[inline]
    fn skip_symbols(buf: &[u8], sym_mask: &u128, from: usize) -> usize {
        let mut i = from;
        // assuming buf entries are ascii which we've already proclaimed
        while i < buf.len() && sym_mask & 1 << buf[i] != 0 {
            i += 1;
        }
        i
    }

    pub struct Scanner<R> {
        // no need to save the buffer itself
        // only the BufRead implementors needed
        reader: R,
        // how much weve scanned 
        scanned_amt: usize,
        // store the intermediate result in case there wasnt a whole token present
        tip: Vec<u8>,
        bitmask: u128
    }

    impl<R: BufRead> Scanner<R> {
        pub fn new(reader: R) -> Self {
            let bitmask: u128 = 1 << b' ' ^ 1 << b'\r' ^ 1 << b'\n'; 
            Scanner {
                reader,
                scanned_amt: 0,
                tip: vec![0; 0],
                bitmask
            }
        }

        pub fn new_lut(reader: R, bitmask: u128) -> Self {
            Scanner {
                reader,
                scanned_amt: 0,
                tip: vec![0; 0],
                bitmask
            }
        }

        pub fn next_bytes(&mut self) -> Option<&[u8]> {
            let Scanner { reader, tip, .. } = self;
            tip.clear();

            let span = loop {
                let buf = reader
                    .fill_buf()
                    .expect("should be able to read uninterrupted");

                if buf.is_empty() {
                    // eof, so anything staged is the last token
                    return if self.tip.is_empty() {
                        None
                    } else {
                        Some(&self.tip)
                    };
                }

                let start = if tip.is_empty() {
                    skip_symbols(buf, &self.bitmask, self.scanned_amt)
                } else {
                    // mid token, scan the rest of the token
                    self.scanned_amt
                };

                if start == buf.len() {
                    // oops all whitespace
                    reader.consume(start);
                    self.scanned_amt = 0;
                    // continue finding non-whitespace bytes
                    continue;
                }

                let end = find_symbols(buf, &self.bitmask, start);
                // found the token within the buffer
                if end < buf.len() {
                    self.scanned_amt = end;
                    // return the token
                    break (start, end);
                }

                // eof but found only a piece of the token
                let consumed = buf.len();
                tip.extend_from_slice(&buf[start..]);
                reader.consume(consumed);
                self.scanned_amt = 0;
            };

            if tip.is_empty() {
                let buf = self.reader.fill_buf().expect("should be able to read uninterrupted");
                return Some(&buf[span.0..span.1]);
            }
            let buf = reader.fill_buf().expect("should be able to read uninterrupted");
            tip.extend_from_slice(&buf[span.0..span.1]);
            Some(&self.tip)
        }

        
        pub unsafe fn next(&mut self) -> Option<&str> {
            unsafe { Some(std::str::from_utf8_unchecked(self.next_bytes()?)) }
        }


        pub unsafe fn next_parse_opt<T: FromStr>(&mut self) -> Option<T> {
            let token = unsafe { self.next()? };
            match token.parse() {
                Ok(parsed) => Some(parsed),
                Err(_) => panic!(
                    "token {:?} should parse as {}",
                    token,
                    std::any::type_name::<T>()
                ),
            }
        }

        
        pub unsafe fn next_parsed<T: FromStr>(&mut self) -> T {
            unsafe { self.next_parse_opt::<T>().expect("token should be present") }
        }


        pub unsafe fn next_vec_n<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            let mut i: usize = 0;
            let mut res: Vec<T> = Vec::new();
            unsafe {
                while let Some(token) = self.next_parse_opt::<T>() {
                    if i >= n { break; }
                    res.push(token);
                    i += 1;
                }
            }
            res
        }


        pub unsafe fn next_vec<T: FromStr>(&mut self) -> Vec<T> {
            let mut res: Vec<T> = Vec::new();
            unsafe {
                while let Some(token) = self.next_parse_opt::<T>() {
                    res.push(token);
                }
            }
            res
        }

        
        pub unsafe fn next_string(&mut self) -> Option<String> {
            unsafe { Some(str::to_owned( self.next()? )) } 
        }
    }
}
