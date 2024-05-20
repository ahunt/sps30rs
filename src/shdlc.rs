/// stuff_data stuffs data following SHDLC conventions.
///
/// Or, to be more precise, data is stuffed following the convention documented
/// in [the SPS30 datasheet][sps30_datasheet] as that's what I had available.
///
/// Note that SHDLC "detailed protocol document is not publicly available (yet)"
/// [according to Sensirion][shdlc_python_driver_docs], although they do publish
/// a [Python Driver][python-shdlc-driver].
///
/// [sps30_datasheet]: <https://sensirion.github.io/python-shdlc-driver/shdlc.html>
/// [shdlc_python_driver_docs]: <https://sensirion.github.io/python-shdlc-driver/shdlc.html>
/// [python-shdlc-driver]: <https://github.com/Sensirion/python-shdlc-driver?tab=readme-ov-file>
fn stuff_data(data: &[u8], out: &mut Vec<u8>) {
    for byte in data {
        let mapped = match byte {
            0x7E => Some(0x5E),
            0x7D => Some(0x5D),
            0x11 => Some(0x31),
            0x13 => Some(0x33),
            _ => None,
        };
        if let Some(mapped_byte) = mapped {
            out.push(0x7D);
            out.push(mapped_byte);
        } else {
            out.push(*byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stuffing() {
        struct TestCase<'a> {
            input: &'a [u8],
            expected_output: &'a [u8],
        }
        let tests = [
            TestCase {
                input: &[],
                expected_output: &[],
            },
            TestCase {
                input: &[0x00],
                expected_output: &[0x00],
            },
            TestCase {
                input: &[0xFF],
                expected_output: &[0xFF],
            },
            TestCase {
                input: &[0x00, 0xFF, 0x10],
                expected_output: &[0x00, 0xFF, 0x10],
            },
            TestCase {
                input: &[0x7E],
                expected_output: &[0x7D, 0x5E],
            },
            TestCase {
                input: &[0x7D],
                expected_output: &[0x7D, 0x5D],
            },
            TestCase {
                input: &[0x11],
                expected_output: &[0x7D, 0x31],
            },
            TestCase {
                input: &[0x13],
                expected_output: &[0x7D, 0x33],
            },
            TestCase {
                input: &[0, 0, 0, 0x7E, 1, 1, 1],
                expected_output: &[0, 0, 0, 0x7D, 0x5E, 1, 1, 1],
            },
        ];
        for case in tests {
            let mut out = Vec::new();
            stuff_data(case.input, &mut out);
            assert_eq!(case.expected_output, out);
        }
    }
}
