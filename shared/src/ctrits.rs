use alloc::*;
use alloc::boxed::Box;
use core::ops::Drop;
use core::any::Any;
use core::mem;
use cty::*;
use trytes::*;
use trytes::constants::TRITS_PER_BYTE;

#[repr(u32)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TritEncoding {
    BYTE = 1,
    TRIT = 2,
    TRYTE = 3,
}

#[repr(C)]
pub struct CTrits {
    pub encoding: TritEncoding,
    pub length: usize,
    pub data: *mut c_void,
    pub byte_length: usize,
}

pub fn ctrits_from_bytes(length: usize, bytes: Vec<u8>) -> CTrits {
    let ptr = bytes.as_ptr() as *mut c_void;
    let blen = bytes.len();
    mem::forget(bytes);

    CTrits {
        encoding: TritEncoding::BYTE,
        length: length,
        data: ptr,
        byte_length: blen,
    }
}

pub fn ctrits_from_trits(trits: Vec<Trit>) -> CTrits {
    let ptr = trits.as_ptr() as *mut c_void;
    let len = trits.len();
    mem::forget(trits);

    CTrits {
        encoding: TritEncoding::TRIT,
        length: len,
        data: ptr,
        byte_length: len,
    }
}

pub fn ctrits_from_trytes(mut trytes: String) -> CTrits {
    trytes.shrink_to_fit();

    let mut bytes = trytes.into_bytes();
    if Some(&0) != bytes.last() {
        bytes.push(0);
    }

    let len = bytes.len();
    let ptr = bytes.as_ptr() as *mut c_void;
    mem::forget(bytes);

    CTrits {
        encoding: TritEncoding::TRYTE,
        length: len - 1,
        data: ptr,
        byte_length: len,
    }
}

pub fn ctrits_slice_bytes(ctrits: &CTrits) -> &[u8] {
    assert_eq!(ctrits.encoding, TritEncoding::BYTE);
    unsafe { slice::from_raw_parts(ctrits.data as *const u8, ctrits.byte_length) }
}

pub fn ctrits_slice_trits(ctrits: &CTrits) -> &[Trit] {
    assert_eq!(ctrits.encoding, TritEncoding::TRIT);
    unsafe { slice::from_raw_parts(ctrits.data as *const Trit, ctrits.length) }
}

pub fn ctrits_slice_str(ctrits: &CTrits) -> &str {
    assert_eq!(ctrits.encoding, TritEncoding::TRYTE);
    unsafe {
        mem::transmute(slice::from_raw_parts(
            ctrits.data as *const c_char,
            ctrits.length,
        ))
    }
}

pub fn ctrits_slice_bytes_mut(ctrits: &mut CTrits) -> &mut [u8] {
    assert_eq!(ctrits.encoding, TritEncoding::BYTE);
    unsafe { slice::from_raw_parts_mut(ctrits.data as *mut u8, ctrits.byte_length) }
}

pub fn ctrits_slice_trits_mut(ctrits: &mut CTrits) -> &mut [Trit] {
    assert_eq!(ctrits.encoding, TritEncoding::TRIT);
    unsafe { slice::from_raw_parts_mut(ctrits.data as *mut Trit, ctrits.length) }
}

pub fn ctrits_slice_str_mut(ctrits: &mut CTrits) -> &mut str {
    assert_eq!(ctrits.encoding, TritEncoding::TRYTE);
    unsafe {
        mem::transmute(slice::from_raw_parts_mut(
            ctrits.data as *mut c_char,
            ctrits.length,
        ))
    }
}

pub fn ctrits_to_bytes(ctrits: &CTrits) -> (usize, Vec<u8>) {
    match ctrits.encoding {
        TritEncoding::BYTE => {
            let bytes = ctrits_slice_bytes(ctrits);
            (ctrits.length, bytes.to_vec())
        }
        TritEncoding::TRIT => {
            let byte_length = (ctrits.length / TRITS_PER_BYTE) +
                if ctrits.length % TRITS_PER_BYTE == 0 {
                    0
                } else {
                    1
                };
            let trits = ctrits_slice_trits(ctrits);
            let mut bytes: Vec<u8> = Vec::with_capacity(byte_length);

            for chunk in trits.chunks(TRITS_PER_BYTE) {
                bytes.push(trits_to_byte(chunk))
            }

            (ctrits.length, bytes)
        }
        TritEncoding::TRYTE => {
            let chars = ctrits_slice_str(ctrits);
            let mut trits: Vec<Trit> = Vec::new();
            let mut bytes: Vec<u8> = Vec::new();

            for c in chars.chars() {
                trits.extend_from_slice(char_to_trits(c));

                if trits.len() >= TRITS_PER_BYTE {
                    bytes.push(trits_to_byte(&trits[0..TRITS_PER_BYTE]));
                    trits = trits.split_off(TRITS_PER_BYTE);
                }
            }

            (ctrits.length * TRITS_PER_TRYTE, bytes)
        }
    }
}
pub fn ctrits_to_trits(ctrits: &CTrits) -> Vec<Trit> {
    match ctrits.encoding {
        TritEncoding::TRIT => {
            let trits = ctrits_slice_trits(ctrits);
            trits.to_vec()
        }
        TritEncoding::BYTE => {
            let bytes = ctrits_slice_bytes(ctrits);

            let mut rem = ctrits.length;
            let mut idx = 0;

            let mut trits: Vec<Trit> = Vec::with_capacity(rem);
            while idx < ctrits.byte_length {
                let ts = byte_to_trits(bytes[idx]);

                if rem > TRITS_PER_BYTE {
                    trits.extend_from_slice(ts);
                    rem -= TRITS_PER_BYTE;
                } else {
                    trits.extend_from_slice(&ts[..rem]);
                    rem -= rem;
                }

                idx += 1;
            }

            if rem > 0 {
                panic!("Invalid trit length.");
            }

            trits
        }
        TritEncoding::TRYTE => {
            let chars = ctrits_slice_str(ctrits);
            let mut trits: Vec<Trit> = Vec::with_capacity(ctrits.length * TRITS_PER_TRYTE);

            for c in chars.chars() {
                trits.extend_from_slice(char_to_trits(c));
            }

            trits
        }
    }
}
pub fn ctrits_to_trytes(ctrits: &CTrits) -> String {
    match ctrits.encoding {
        TritEncoding::TRYTE => {
            // already encoded as a string.
            let chars = unsafe { slice::from_raw_parts(ctrits.data as *mut u8, ctrits.length) };
            for c in chars.iter() {
                let chr = *c as char;
                if !TRYTE_ALPHABET.binary_search(&chr).is_ok() {
                    panic!("Invalid character found.")
                }
            }

            String::from_utf8(chars.to_vec()).unwrap()
        }
        TritEncoding::TRIT => {
            let trits = unsafe { slice::from_raw_parts(ctrits.data as *mut Trit, ctrits.length) };
            let mut string = String::new();
            for chunk in trits.chunks(TRITS_PER_TRYTE) {
                string.push(trits_to_char(chunk));
            }

            string
        }
        TritEncoding::BYTE => {
            let mut trits = ctrits_to_trits(ctrits);
            let slen = trits.len() / TRITS_PER_TRYTE;

            for i in 0..slen {
                let c = trits_to_char(&trits[i * TRITS_PER_TRYTE..][..TRITS_PER_TRYTE]);
                trits[i] = c as i8;
            }

            let sptr = trits.as_ptr() as *mut u8;
            let scap = trits.capacity();
            mem::forget(trits);

            unsafe { String::from_raw_parts(sptr, slen, scap) }
        }
    }
}

pub unsafe fn ctrits_trits_to_trytes_inplace(ctrits: &mut CTrits) {
    assert_eq!(ctrits.encoding, TritEncoding::TRIT);
    assert_eq!(ctrits.length % TRITS_PER_TRYTE, 0);

    let trytes: &mut [u8] = mem::transmute(slice::from_raw_parts_mut(
        ctrits.data as *mut c_char,
        ctrits.length,
    ));

    {
        let trits = ctrits_slice_trits(ctrits);
        for i in 0..(ctrits.length / TRITS_PER_TRYTE) {
            let slice = &trits[i * TRITS_PER_TRYTE..(i + 1) * TRITS_PER_TRYTE];
            trytes[i] = trits_to_char(slice) as u8;
        }
    }

    ctrits.encoding = TritEncoding::TRYTE;
    ctrits.length = ctrits.length / TRITS_PER_TRYTE;
    trytes[ctrits.length] = '\0' as u8;
    ctrits.byte_length = ctrits.length + 1;
}

pub fn ctrits_convert(ctrits: &CTrits, to: TritEncoding) -> CTrits {
    match to {
        TritEncoding::BYTE => {
            let (l, b) = ctrits_to_bytes(ctrits);
            let blen = b.len();
            let bptr = b.as_ptr() as *mut c_void;
            mem::forget(b);

            CTrits {
                encoding: TritEncoding::BYTE,
                length: l,
                data: bptr,
                byte_length: blen,
            }
        }
        TritEncoding::TRIT => {
            let trits = ctrits_to_trits(ctrits);
            let tlen = trits.len();
            let tptr = trits.as_ptr() as *mut c_void;
            mem::forget(trits);

            CTrits {
                encoding: TritEncoding::TRIT,
                length: tlen,
                data: tptr,
                byte_length: tlen,
            }
        }
        TritEncoding::TRYTE => {
            let mut trytes = ctrits_to_trytes(ctrits);
            trytes.shrink_to_fit();
            let mut bytes = trytes.into_bytes();
            if Some(&0) != bytes.last() {
                bytes.push(0);
            }

            let slen = bytes.len();
            let sptr = bytes.as_ptr() as *mut c_void;
            mem::forget(bytes);

            CTrits {
                encoding: TritEncoding::TRYTE,
                length: slen - 1,
                data: sptr,
                byte_length: slen,
            }
        }
    }
}

// Rust owns this CTrits instance, so we need to make sure to clean up after ourselves.
impl Drop for CTrits {
    fn drop(&mut self) {
        let raw = self.data as *mut Box<Any>;
        if !raw.is_null() {
            mem::drop(raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::ToOwned;

    static TRYTES: &'static str = "ABABZZDD99";
    static TRITS: [Trit; 30] = [
        1,
        0,
        0,
        -1,
        1,
        0,
        1,
        0,
        0,
        -1,
        1,
        0,
        -1,
        0,
        0,
        -1,
        0,
        0,
        1,
        1,
        0,
        1,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    static BYTES: (usize, [u8; 6]) = (30, [55, 178, 248, 107, 12, 0]);

    #[test]
    fn ctrits_trytes() {
        let string = TRYTES.to_owned();
        let ctrits = ctrits_from_trytes(string);
        assert_eq!(ctrits_slice_str(&ctrits), TRYTES);
        assert_eq!(ctrits_to_trytes(&ctrits), TRYTES.to_owned());
        assert_eq!(ctrits_to_trits(&ctrits), TRITS.to_vec());
        assert_eq!(ctrits_to_bytes(&ctrits), (BYTES.0, BYTES.1.to_vec()));
    }

    #[test]
    #[should_panic]
    fn ctrits_trytes_slice_bytes() {
        let string = TRYTES.to_owned();
        let ctrits = ctrits_from_trytes(string);
        ctrits_slice_bytes(&ctrits);
    }

    #[test]
    #[should_panic]
    fn ctrits_trytes_slice_trits() {
        let string = TRYTES.to_owned();
        let ctrits = ctrits_from_trytes(string);
        ctrits_slice_trits(&ctrits);
    }

    #[test]
    fn ctrits_trits() {
        let ctrits = ctrits_from_trits(TRITS.to_vec());
        assert_eq!(ctrits_slice_trits(&ctrits), TRITS);
        assert_eq!(ctrits_to_trytes(&ctrits), TRYTES.to_owned());
        assert_eq!(ctrits_to_trits(&ctrits), TRITS.to_vec());
        assert_eq!(ctrits_to_bytes(&ctrits), (BYTES.0, BYTES.1.to_vec()));
    }

    #[test]
    fn ctrits_bytes() {
        let ctrits = ctrits_from_bytes(BYTES.0, BYTES.1.to_vec());
        assert_eq!(ctrits_slice_bytes(&ctrits), BYTES.1);
        assert_eq!(ctrits_to_trytes(&ctrits), TRYTES.to_owned());
        assert_eq!(ctrits_to_trits(&ctrits), TRITS.to_vec());
        assert_eq!(ctrits_to_bytes(&ctrits), (BYTES.0, BYTES.1.to_vec()));
    }
}
