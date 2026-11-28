#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Digits<const N: usize>([u8; N], u8);
impl<const N: usize> Digits<N> {
    #[inline]
    pub fn ascii(&self) -> &[u8] {
        &self.0[N - self.1 as usize..]
    }

    #[inline]
    pub const fn count(&self) -> usize {
        self.1 as usize
    }
}

impl Digits<3> {
    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        let count = 1 + (value >= 10) as u8 + (value >= 100) as u8;
        let ascii = [
            b'0' + (value / 100 % 10),
            b'0' + (value / 10 % 10),
            b'0' + (value % 10),
        ];

        Self(ascii, count)
    }

    #[inline]
    pub fn lookup_u8(value: u8) -> Self {
        static TABLE: [Digits<3>; 256] = {
            let mut table = [Digits::from_u8(0); 256];
            let mut i = 0;
            while i < 256 {
                table[i] = Digits::from_u8(i as u8);
                i += 1;
            }
            table
        };

        TABLE[value as usize]
    }
}

impl Digits<5> {
    #[inline]
    pub const fn from_u16(value: u16) -> Self {
        let count = 1
            + (value >= 10) as u8
            + (value >= 100) as u8
            + (value >= 1000) as u8
            + (value >= 10_000) as u8;
        let ascii = [
            b'0' + (value / 10000) as u8,
            b'0' + (value / 1000 % 10) as u8,
            b'0' + (value / 100 % 10) as u8,
            b'0' + (value / 10 % 10) as u8,
            b'0' + (value % 10) as u8,
        ];

        Self(ascii, count)
    }
}

/*pub trait Digits<const N: usize>: Copy {
    /// Returns the count of digits of a number.
    fn count(self) -> usize;
    /// Returns zero-padded ASCII representation of a number.
    fn ascii(self) -> [u8; N];
}

impl Digits<3> for u8 {
    #[inline]
    fn count(self) -> usize {
        1 + (self >= 10) as usize + (self >= 100) as usize
    }

    #[inline]
    fn ascii(self) -> [u8; 3] {
        [b'0' + self / 100, b'0' + self / 10 % 10, b'0' + self % 10]
    }
}

impl Digits<5> for u16 {
    #[inline]
    fn ascii(self) -> [u8; 5] {
        [
            b'0' + (self / 10000) as u8,
            b'0' + (self / 1000 % 10) as u8,
            b'0' + (self / 100 % 10) as u8,
            b'0' + (self / 10 % 10) as u8,
            b'0' + (self % 10) as u8,
        ]
    }

    #[inline]
    fn count(self) -> usize {
        1 + (self >= 10) as usize
            + (self >= 100) as usize
            + (self >= 1000) as usize
            + (self >= 10_000) as usize
    }
}
*/
