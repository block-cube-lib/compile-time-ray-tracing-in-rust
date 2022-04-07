pub struct Xorshift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Xorshift {
    pub const fn new(seed: u32) -> Self {
        Self {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: seed,
        }
    }

    pub const fn gen_u32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        let x = self.y;
        let y = self.z;
        let z = self.w;
        let w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        *self = Self { x, y, z, w };
        w
    }

    pub const fn gen_u64(&mut self) -> u64 {
        let v1 = self.gen_u32();
        let v2 = self.gen_u32();
        let v = unsafe { std::mem::transmute::<[u32; 2], u64>([v1, v2]) };
        v
    }

    pub const fn gen_f64(&mut self) -> f64 {
        const A: u16 = 0x0001;
        const B: [u8; 2] = unsafe { std::mem::transmute::<u16, [u8; 2]>(A) };
        const IS_LE: bool = B[0] == 0x01;
        let v = self.gen_u64();
        let v = if IS_LE {
            let v = (v >> 12) | 0x3ff0000000000000;
            unsafe { std::mem::transmute::<u64, f64>(v) }
        } else {
            let v = [
                ((v >> 56) & 0xff) as u8,
                ((v >> 48) & 0xff) as u8,
                ((v >> 40) & 0xff) as u8,
                ((v >> 32) & 0xff) as u8,
                ((v >> 24) & 0xff) as u8,
                ((v >> 16) & 0xff) as u8,
                ((v >> 8) & 0xff) as u8,
                (v & 0xff) as u8,
            ];
            unsafe { std::mem::transmute::<[u8; 8], f64>(v) }
        };
        v - 1.0
    }
}

