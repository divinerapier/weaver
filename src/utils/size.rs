// #[derive(Copy, Clone)]
// pub enum Size {
//     B(u64),
//     KB(f64),
//     MB(f64),
//     GB(f64),
//     TB(f64),
//     PB(f64),
// }

// impl Size {
//     fn clean(self) -> Size {
//         match self {
//             B(v) => {
//                 if v < 1024 {
//                     return self;
//                 } else {
//                     return Size::KB(v as f64 / 1024f64).clean();
//                 }
//             }
//             KB(v) => {
//                 if v < 1024 {
//                     return self;
//                 } else {
//                     return Size::MB(v / 1024).clean();
//                 }
//             }
//             MB(v) => {
//                 if v < 1024 {
//                     return self;
//                 } else {
//                     return Size::GB(v / 1024).clean();
//                 }
//             }
//             GB(v) => {
//                 if v < 1024 {
//                     return self;
//                 } else {
//                     return Size::TB(v / 1024).clean();
//                 }
//             }
//             TB(v) => {
//                 if v < 1024 {
//                     return self;
//                 } else {
//                     return Size::PB(v / 1024).clean();
//                 }
//             }
//             PB(v) => {
//                 return self;
//             }
//         }
//     }

//     fn ext(self) -> Self {
//         match self {
//             B(v) => {
//                 return self;
//             }
//             KB(v) => return Size::B((v * 1024f64) as u64),
//             MB(v) => return Size::KB(v * 1024f64),
//             GB(v) => return Size::MB(v * 1024f64),
//             TB(v) => return Size::GB(v * 1024f64),
//             PB(v) => return Size::TB(v * 1024f64),
//         }
//     }
// }

// impl Default for Size {
//     fn default() -> Size {
//         Size(0)
//     }
// }

// impl std::ops::Mul<usize> for Size {
//     type Output = Self;

//     fn mul(self, rhs: usize) -> Self::Output {
//         match self {
//             B(v) => Size::B(v * rhs).clean(),
//             KB(v) => Size::KB(v * rhs).clean(),
//             MB(v) => Size::MB(v * rhs).clean(),
//             GB(v) => Size::GB(v * rhs).clean(),
//             TB(v) => Size::TB(v * rhs).clean(),
//             PB(v) => Size::PB(v * rhs).clean(),
//         }
//     }
// }

// impl From<u64> for Size {
//     fn from(v: u64) -> Size {
//         Size::B(v as usize).clean()
//     }
// }

// impl Into<u64> for Size {
//     fn into(self) -> u64 {
//         self.0 as u64
//     }
// }
