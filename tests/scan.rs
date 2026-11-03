#[cfg(test)]
mod test {
    use io_lo::scan::Scanner;
    use std::io::BufReader;

    fn guh(input: &[u8], reader_capacity: usize) -> std::io::Result<()> {
        let input_str = std::str::from_utf8(input).expect("ascii");
        let mut scan = Scanner::new(BufReader::with_capacity(reader_capacity, input));
        unsafe {
            for elem in input_str.split_ascii_whitespace() {
                let got = scan.next().expect("token should be present");
                assert_eq!(got, elem);
            }
            assert!(scan.next().is_none(), "no trailing tokens");
        }
        Ok(())
    }

    #[test]
    fn single_token_capable_reader() {
        guh(b"hello", 5).expect("should be one token");
    }

    #[test]
    fn multiple_tokens_capable_reader() {
        guh(b"hello mrrow boing horse boops", 6).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_capable_reader_cr() {
        guh(b"hello\rmrrow\rboing\rhorse\rboops", 6).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_capable_reader_lf() {
        guh(b"hello\nmrrow\nboing\nhorse\nboops", 6).expect("should be ok");
    }

    #[test]
    fn no_tokens() {
        guh(b"", 1).expect("should be ok");
    }

    #[test]
    fn straight_whitespace() {
        // should only check for trailing tokens and if there are any then that would be wrong
        guh(b"\n\r\r\r\r\n\n  \n \n \n\r  \r\n", 2).expect("should be no tokens");
    }

    #[test]
    fn single_token_sticky() {
        guh(b"hello", 3).expect("should be ok");
    }

    #[test]
    fn single_token_sticky_several_reads() {
        guh(b"hellooooo", 3).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_sticky() {
        guh(b"hello mom", 2).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_sticky_several_reads() {
        guh(b"hello    \n  mom   i   \r\rgot   home", 2).expect("should be ok");
    }

    #[test]
    fn single_token_whitespace_led() {
        guh(b" hello", 6).expect("should be ok");
    }

    #[test]
    fn single_token_whitespace_ended() {
        guh(b"hello ", 6).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_whitespace_led_sticky() {
        guh(b"\n\n\n\nhello    \n  mom   i   \r\rgot   home", 2).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_whitespace_ended_sticky() {
        guh(b"hello    \n  mom   i   \r\rgot   home\n\n\n\n", 2).expect("should be ok");
    }

    #[test]
    fn single_token_incapable_reader() {
        guh(b"hello", 1).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_incapable_reader() {
        guh(b"hello\nmrrow\nboing\nhorse\nboops", 1).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_first_snug_fit() {
        guh(b"hello\nmrrow\nboing\nhorse\nboops", 5).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_combo() {
        guh(b"hello\nmeow bowl\rcrow\nboops arf\r3\n45 789\r", 10).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_combo_sloppy() {
        guh(b"    hello\n211212ev mwe ow b o w l \\ r c \r  lqwi ow\nboop s   a rf\r3\n45 78  9 \r   d \n", 10).expect("should be ok");
    }

    #[test]
    fn single_token_vast_space() {
        guh(b"                                                                                       d", 10).expect("should be ok");
    }

    #[test]
    fn multiple_tokens_vast_space() {
        guh(b" sd                                                                                    d", 10).expect("should be ok");
    }
}
