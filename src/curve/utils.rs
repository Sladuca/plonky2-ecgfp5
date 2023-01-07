use crate::curve::scalar_field::Scalar;

// A custom 161-bit integer type; used for splitting a scalar into a
// fraction. Negative values use two's complement notation; the value
// is truncated to 161 bits (upper bits in the top limb are ignored).
// Elements are mutable containers.
// WARNING: everything in here is vartime; do not use on secret values.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Signed161([u64; 3]);

impl Signed161 {
    pub(crate) fn from_scalar(s: Scalar) -> Self {
        Self([s.0[0], s.0[1], s.0[2]])
    }

    /// Convert that value into a scalar (integer modulo n).
    pub(crate) fn to_scalar_vartime(self) -> Scalar {
        let mut tmp = self.to_u192();
        let neg = (tmp[2] >> 63) != 0;
        if neg {
            tmp[0] = (!tmp[0]).wrapping_add(1);
            let mut cc = tmp[0] == 0;
            tmp[1] = !tmp[1];
            if cc {
                tmp[1] = tmp[1].wrapping_add(1);
                cc = tmp[1] == 0;
            }
            tmp[2] = !tmp[2];
            if cc {
                tmp[2] = tmp[2].wrapping_add(1);
            }
            return -Scalar([tmp[0], tmp[1], tmp[2], 0, 0]);
        } else {
            return Scalar([tmp[0], tmp[1], tmp[2], 0, 0]);
        }
    }

    /// Export this value as a 192-bit integer (three 64-bit limbs,
    /// in little-endian order).
    pub(crate) fn to_u192(self) -> [u64; 3] {
        let mut x = self.0[2];
        x &= 0x00000001FFFFFFFF;
        x |= (x >> 32).wrapping_neg() << 33;
        [self.0[0], self.0[1], x]
    }

    // Recode this integer into 33 signed digits for a 5-bit window.
    pub(crate) fn recode_signed_5(self) -> [i32; 33] {
        // We first sign-extend the value to 192 bits, then add
        // 2^160 to get a nonnegative value in the 0 to 2^161-1
        // range. We then recode that value; and finally we fix
        // the result by subtracting 1 from the top digit.
        let mut tmp = self.to_u192();
        tmp[2] = tmp[2].wrapping_add(0x0000000100000000);
        let mut ss = [0i32; 33];
        Scalar::recode_signed_from_limbs(&tmp, &mut ss, 5);
        ss[32] -= 1;
        ss
    }

    // Add v*2^s to this value.
    pub(crate) fn add_shifted(&mut self, v: &Signed161, s: i32) {
        if s == 0 {
            Self::add(self, &v.0[..]);
        } else if s < 64 {
            Self::add_shifted_small(self, &v.0[..], s);
        } else if s < 161 {
            Self::add_shifted_small(self, &v.0[((s >> 6) as usize)..], s & 63);
        }
    }

    pub(crate) fn add_shifted_small(&mut self, v: &[u64], s: i32) {
        let mut cc = 0u64;
        let j = 3 - v.len();
        let mut vbits = 0u64;
        for i in j..3 {
            let vw = v[i - j];
            let vws = vw.wrapping_shl(s as u32) | vbits;
            vbits = vw.wrapping_shr((64 - s) as u32);
            let z = (self.0[i] as u128) + (vws as u128) + (cc as u128);
            self.0[i] = z as u64;
            cc = (z >> 64) as u64;
        }
    }

    pub(crate) fn add(&mut self, v: &[u64]) {
        let mut cc = 0;
        let j = 3 - v.len();
        for i in j..3 {
            let z = (self.0[i] as u128) + (v[i - j] as u128) + (cc as u128);
            self.0[i] = z as u64;
            cc = (z >> 64) as u64;
        }
    }

    // Subtract v*2^s from this value.
    pub(crate) fn sub_shifted(&mut self, v: &Signed161, s: i32) {
        if s == 0 {
            Self::sub(self, &v.0[..]);
        } else if s < 64 {
            Self::sub_shifted_small(self, &v.0[..], s);
        } else if s < 161 {
            Self::sub_shifted_small(self, &v.0[((s >> 6) as usize)..], s & 63);
        }
    }

    pub(crate) fn sub_shifted_small(&mut self, v: &[u64], s: i32) {
        let mut cc = 0u64;
        let j = 3 - v.len();
        let mut vbits = 0u64;
        for i in j..3 {
            let vw = v[i - j];
            let vws = vw.wrapping_shl(s as u32) | vbits;
            vbits = vw.wrapping_shr((64 - s) as u32);
            let z = (self.0[i] as u128)
                .wrapping_sub(vws as u128)
                .wrapping_sub(cc as u128);
            self.0[i] = z as u64;
            cc = ((z >> 64) as u64) & 1;
        }
    }

    pub(crate) fn sub(&mut self, v: &[u64]) {
        let mut cc = 0;
        let j = 3 - v.len();
        for i in j..3 {
            let z = (self.0[i] as u128)
                .wrapping_sub(v[i - j] as u128)
                .wrapping_sub(cc as u128);
            self.0[i] = z as u64;
            cc = ((z >> 64) as u64) & 1;
        }
    }
}

// A custom 640-bit integer type (signed).
// Elements are mutable containers.
// WARNING: everything in here is vartime; do not use on secret values.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Signed640([u64; 10]);

impl Signed640 {
    // Obtain an instance containing n^2.
    pub(crate) fn from_nsquared() -> Self {
        Signed640([
            0x8E6B7A18061803C1,
            0x0AD8BDEE1594E2CF,
            0x17640E465F2598BC,
            0x90465B4214B27B1C,
            0xD308FECCB1878B88,
            0x3CC55EB2EAC07502,
            0x59F038FB784335CE,
            0xBFFFFE954FB808EA,
            0xBFFFFFCB80000099,
            0x3FFFFFFD8000000D,
        ])
    }

    // Obtain an instance containing a*b (both a and b are interpreted
    // as integers in the 0..n-1 range).
    pub(crate) fn from_mul_scalars(a: Scalar, b: Scalar) -> Self {
        let mut r = Signed640([0u64; 10]);
        for i in 0..5 {
            let aw = a.0[i];
            let mut cc = 0u64;
            for j in 0..5 {
                let bw = b.0[j];
                let z = ((aw as u128) * (bw as u128))
                    .wrapping_add(r.0[i + j] as u128)
                    .wrapping_add(cc as u128);
                r.0[i + j] = z as u64;
                cc = (z >> 64) as u64;
            }
            r.0[i + 5] = cc;
        }
        r
    }

    // Add 1 to this instance.
    pub(crate) fn add1(&mut self) {
        for i in 0..10 {
            self.0[i] = self.0[i].wrapping_add(1);
            if self.0[i] != 0 {
                return;
            }
        }
    }

    pub(crate) fn is_nonnegative(&self) -> bool {
        (self.0[9] >> 63) == 0
    }

    pub(crate) fn lt_unsigned(&self, rhs: &Self) -> bool {
        for i in (0..10).rev() {
            let aw = self.0[i];
            let bw = rhs.0[i];
            if aw < bw {
                return true;
            }
            if aw > bw {
                return false;
            }
        }
        false
    }

    // Get the bit length of this value. The bit length is defined as the
    // minimal size of the binary representation in two's complement,
    // _excluding_ the sign bit (thus, -2^k has bit length k, whereas +2^k
    // has bit length k+1).
    pub(crate) fn bitlength(&self) -> i32 {
        let sm = (self.0[9] >> 63).wrapping_neg();
        for i in (0..10).rev() {
            let w = self.0[i] ^ sm;
            if w != 0 {
                return ((i as i32) << 6) + Self::u64_bitlength(w);
            }
        }
        0
    }

    fn u64_bitlength(w: u64) -> i32 {
        // We use here a portable algorithm; some architectures have
        // dedicated opcodes that could speed up this operation
        // greatly (e.g. lzcnt on recent x86).
        let mut x = w;
        let mut r = 0;
        if x > 0xFFFFFFFF {
            x >>= 32;
            r += 32;
        }
        if x > 0x0000FFFF {
            x >>= 16;
            r += 16;
        }
        if x > 0x000000FF {
            x >>= 8;
            r += 8;
        }
        if x > 0x0000000F {
            x >>= 4;
            r += 4;
        }
        if x > 0x00000003 {
            x >>= 2;
            r += 2;
        }
        r + (x as i32) - (((x + 1) >> 2) as i32)
    }

    // Add v*2^s to this instance.
    pub(crate) fn add_shifted(&mut self, v: &Signed640, s: i32) {
        if s == 0 {
            Self::add(self, &v.0[..]);
        } else if s < 64 {
            Self::add_shifted_small(self, &v.0[..], s);
        } else if s < 640 {
            Self::add_shifted_small(self, &v.0[((s >> 6) as usize)..], s & 63);
        }
    }

    fn add_shifted_small(&mut self, v: &[u64], s: i32) {
        let mut cc = 0u64;
        let j = 10 - v.len();
        let mut vbits = 0u64;
        for i in j..10 {
            let vw = v[i - j];
            let vws = vw.wrapping_shl(s as u32) | vbits;
            vbits = vw.wrapping_shr((64 - s) as u32);
            let z = (self.0[i] as u128) + (vws as u128) + (cc as u128);
            self.0[i] = z as u64;
            cc = (z >> 64) as u64;
        }
    }

    fn add(&mut self, v: &[u64]) {
        let mut cc = 0;
        let j = 10 - v.len();
        for i in j..10 {
            let z = (self.0[i] as u128) + (v[i - j] as u128) + (cc as u128);
            self.0[i] = z as u64;
            cc = (z >> 64) as u64;
        }
    }

    // Subtract v*2^s from this instance.
    pub(crate) fn sub_shifted(&mut self, v: &Signed640, s: i32) {
        if s == 0 {
            Self::sub(self, &v.0[..]);
        } else if s < 64 {
            Self::sub_shifted_small(self, &v.0[..], s);
        } else {
            Self::sub_shifted_small(self, &v.0[((s >> 6) as usize)..], s & 63);
        }
    }

    fn sub_shifted_small(&mut self, v: &[u64], s: i32) {
        let mut cc = 0u64;
        let j = 10 - v.len();
        let mut vbits = 0u64;
        for i in j..10 {
            let vw = v[i - j];
            let vws = vw.wrapping_shl(s as u32) | vbits;
            vbits = vw.wrapping_shr((64 - s) as u32);
            let z = (self.0[i] as u128)
                .wrapping_sub(vws as u128)
                .wrapping_sub(cc as u128);
            self.0[i] = z as u64;
            cc = ((z >> 64) as u64) & 1;
        }
    }

    fn sub(&mut self, v: &[u64]) {
        let mut cc = 0;
        let j = 10 - v.len();
        for i in j..10 {
            let z = (self.0[i] as u128)
                .wrapping_sub(v[i - j] as u128)
                .wrapping_sub(cc as u128);
            self.0[i] = z as u64;
            cc = ((z >> 64) as u64) & 1;
        }
    }
}
