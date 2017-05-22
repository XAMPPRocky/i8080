        use pointer::Pointer;
        use register::Register;

macro_rules! ops {
    ($(($name:ident, $num:tt))*) => {
        use std::ops::*;
        $(
            impl Not for $name {
                type Output = Self;
                fn not(self) -> Self::Output {
                    $name(!self.0)
                }
            }

            impl Add for $name {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    $name(self.0.wrapping_add(rhs.0))
                }
            }

            impl Add<$num> for $name {
                type Output = Self;
                fn add(self, rhs: $num) -> Self::Output {
                    $name(self.0.wrapping_add(rhs))
                }
            }

            impl Sub for $name {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    $name(self.0.wrapping_sub(rhs.0))
                }
            }

            impl Sub<$num> for $name {
                type Output = Self;
                fn sub(self, rhs: $num) -> Self::Output {
                    $name(self.0.wrapping_sub(rhs))
                }
            }

            impl Deref for $name {
                type Target = $num;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl BitAnd for $name {
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self::Output {
                    $name(self.0 & rhs.0)
                }
            }

            impl BitAnd<$num> for $name {
                type Output = Self;
                fn bitand(self, rhs: $num) -> Self::Output {
                    $name(self.0 & rhs)
                }
            }

            impl BitOr for $name {
                type Output = Self;
                fn bitor(self, rhs: Self) -> Self::Output {
                    $name(self.0 | rhs.0)
                }
            }

            impl BitOr<$num> for $name {
                type Output = Self;
                fn bitor(self, rhs: $num) -> Self::Output {
                    $name(self.0 | rhs)
                }
            }

            impl Shl<$num> for $name {
                type Output = Self;
                fn shl(self, rhs: $num) -> Self::Output {
                    $name(self.0.wrapping_shl(rhs as u32))
                }
            }

            impl Shl<$name> for $name {
                type Output = Self;
                fn shl(self, rhs: Self) -> Self::Output {
                    $name(self.0.wrapping_shl(rhs.0 as u32))
                }
            }

            impl Shr<$num> for $name {
                type Output = Self;
                fn shr(self, rhs: $num) -> Self::Output {
                    $name(self.0.wrapping_shr(rhs as u32))
                }
            }

            impl Shr<$name> for $name {
                type Output = Self;
                fn shr(self, rhs: Self) -> Self::Output {
                    $name(self.0.wrapping_shr(rhs.0 as u32))
                }
            }

            impl AddAssign<$num> for $name {
                fn add_assign(&mut self, rhs: $num) {
                    self.0 = self.0.wrapping_add(rhs);
                }
            }

            impl AddAssign for $name {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 = self.0.wrapping_add(rhs.0);
                }
            }

            impl SubAssign<$num> for $name {
                fn sub_assign(&mut self, rhs: $num) {
                    self.0 = self.0.wrapping_sub(rhs);
                }
            }

            impl SubAssign for $name {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 = self.0.wrapping_sub(rhs.0);
                }
            }

            impl ShlAssign<$num> for $name {
                fn shl_assign(&mut self, rhs: $num) {
                    self.0 = self.0.wrapping_shl(rhs as u32);
                }
            }

            impl ShlAssign<$name> for $name {
                fn shl_assign(&mut self, rhs: Self) {
                    self.0 = self.0.wrapping_shl(rhs.0 as u32);
                }
            }

            impl BitAndAssign<$num> for $name {
                fn bitand_assign(&mut self, rhs: $num) {
                    self.0 &= rhs;
                }
            }

            impl BitAndAssign for $name {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0 &= rhs.0;
                }
            }

            impl ShrAssign<$num> for $name {
                fn shr_assign(&mut self, rhs: $num) {
                    self.0 = self.0.wrapping_shr(rhs as u32);
                }
            }

            impl ShrAssign for $name {
                fn shr_assign(&mut self, rhs: Self) {
                    self.0 = self.0.wrapping_shr(rhs.0 as u32);
                }
            }

            impl BitOrAssign<$num> for $name {
                fn bitor_assign(&mut self, rhs: $num) {
                    self.0 |= rhs;
                }
            }

            impl BitOrAssign for $name {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0 |= rhs.0;
                }
            }

            impl BitXorAssign<$num> for $name {
                fn bitxor_assign(&mut self, rhs: $num) {
                    self.0 ^= rhs;
                }
            }

            impl BitXorAssign for $name {
                fn bitxor_assign(&mut self, rhs: Self) {
                    self.0 ^= rhs.0;
                }
            }

            impl PartialEq<$num> for $name {
                fn eq(&self, rhs: &$num) -> bool {
                    self.0 == *rhs
                }
            }

            impl PartialOrd<$num> for $name {
                fn partial_cmp(&self, rhs: &$num)
                    -> Option<::std::cmp::Ordering>
                {
                    Some(self.0.cmp(rhs))
                }
            }

            )*
    }
}

ops! {
    (Register, u8)
    (Pointer, u16)
}
