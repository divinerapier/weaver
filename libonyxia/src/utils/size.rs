#[derive(Copy, Clone)]
pub struct Size(usize);

impl Default for Size {
    fn default() -> Size {
        Size(0)
    }
}

impl std::ops::Mul<usize> for Size {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Size(self.0 * rhs)
    }
}

impl Size {
    pub fn byte(v: usize) -> Size {
        Size(v)
    }
    pub fn kilo_byte(v: usize) -> Size {
        Size((1 << 10) * v)
    }
    pub fn mega_byte(v: usize) -> Size {
        Size((1 << 20) * v)
    }
    pub fn giga_byte(v: usize) -> Size {
        Size((1 << 30) * v)
    }
    pub fn tera_byte(v: usize) -> Size {
        Size((1 << 40) * v)
    }
    pub fn peta_byte(v: usize) -> Size {
        Size((1 << 50) * v)
    }
}

impl From<u64> for Size {
    fn from(v: u64) -> Size {
        Size(v as usize)
    }
}

impl Into<u64> for Size {
    fn into(self) -> u64 {
        self.0 as u64
    }
}
