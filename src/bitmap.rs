use std::ops::Range;

use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitMap {
    data: Vec<u8>,
}

impl BitMap {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.len());

        self.data[index / 8] & (1 << (7 - index % 8)) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.len());

        if value {
            self.data[index / 8] |= 1 << (7 - index % 8);
        } else {
            self.data[index / 8] &= !(1 << (7 - index % 8));
        }
    }

    pub fn slice(&self, range: Range<usize>) -> BitMapRef {
        BitMapRef {
            bitmap: self,
            start: range.start.clamp(0, self.len()),
            end: range.end.clamp(0, self.len()),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() * 8
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitMapRef<'a> {
    bitmap: &'a BitMap,
    start: usize,
    end: usize,
}

impl<'a> BitMapRef<'a> {
    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.len());

        self.bitmap.get(self.start + index)
    }

    pub fn slice(&self, range: Range<usize>) -> Self {
        Self {
            bitmap: self.bitmap,
            start: self.start + range.start.clamp(0, self.len()),
            end: self.start + range.end.clamp(0, self.len()),
        }
    }

    pub fn len(&self) -> usize {
        if self.start >= self.end {
            0
        } else {
            self.end - self.start
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> From<&'a BitMap> for BitMapRef<'a> {
    fn from(bitmap: &'a BitMap) -> Self {
        bitmap.slice(0..bitmap.len())
    }
}

impl<'a> TryFrom<BitMapRef<'a>> for u64 {
    type Error = anyhow::Error;

    fn try_from(value: BitMapRef<'a>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<'a, 'b> TryFrom<&'a BitMapRef<'b>> for u64 {
    type Error = anyhow::Error;

    fn try_from(value: &'a BitMapRef<'b>) -> Result<Self, Self::Error> {
        if value.len() > u64::BITS.try_into().unwrap() {
            bail!("Value has too many bits");
        }

        let mut result = 0u64;

        for index in 0..value.len() {
            result <<= 1;
            result |= if value.get(index) { 1 } else { 0 };
        }

        Ok(result)
    }
}
