pub mod constants;

use constants::*;

pub const BLOCK_SIZE: usize = 64;
const BLOCK_BIT_SIZE: usize = BLOCK_SIZE * 8;


type Block = [u8; BLOCK_SIZE];

#[derive(Copy, Clone)]
pub enum HashSize {
    L256,
    L512
}

#[derive(Debug)]
pub struct Stribog {
    pub h: Block,
    n: Block,
    sigma: Block,
}

impl Stribog {
    pub fn new(size: HashSize) -> Stribog {
        Stribog {
            n: [0x00; BLOCK_SIZE],
            sigma: [0x00; BLOCK_SIZE],
            h: match size {
                HashSize::L256 => [1; BLOCK_SIZE],
                HashSize::L512 => [0; BLOCK_SIZE]
            }
        }
    }
    pub fn print_512(self)  {
        for i in self.h.iter().rev() {
            print!("{:02x}", i);
        }
        println!()
    }
    pub fn print_256(self)  {
        for i in self.h[..32].iter().rev() {
            print!("{:02x}", i);
        }
        println!()
    }
}

#[inline(always)]
fn add_mod_512(src: &mut Block, add: &Block) {
    let mut overrun = 0_u16;
    for i in (0..BLOCK_SIZE).rev() {
        overrun = src[i] as u16 + add[i] as u16 + (overrun >> 8);
        src[i] = overrun as u8;
    }
}

#[inline(always)]
fn add_mod_512_u32(src: &mut Block, mut add: u32)  {
    for i in (0..BLOCK_SIZE).rev() {
        add += src[i] as u32;
        src[i] = add as u8;
        add >>= 8;
    }
}

#[inline(always)]
fn xor(a: &mut Block, b: &Block)  {
    for i in 0..BLOCK_SIZE {
        a[i] ^= b[i];
    }
}

#[inline(always)]
fn lps(h: &mut Block) {
    let mut buf = [0; BLOCK_SIZE];
    for i in 0..8 {
        let mut c = 0;
        for j in 0..8 {
            for k in 0..8 {
                buf[i * 8 + k] = (c >> ((7 - k) * 8)) as u8;
            }
            c ^= SHUFFLED_LIN_TABLE[j][h[j * 8 + i] as usize];
        }
        for k in 0..8 {
            buf[i * 8 + k] = (c >> ((7 - k) * 8)) as u8;
        }
    }
    *h = buf;
}

#[inline(always)]
fn e(h: &mut Block, m: &Block) {

    let mut k: Block = *h;

    xor(h, m);

    for i in 0..12 {

        xor(&mut k, &C[i]);
        lps(&mut k);

        lps(h);
        xor(h, &k);
    }
}

#[inline(always)]
fn g(h: &mut Block, n: &Block, m: &Block) {
    let hash: Block = *h;

    xor(h, n);
    lps(h);

    e(h, m);
    xor(h, &hash);
    xor(h, m);
}


pub fn stribog(ctx: &mut Stribog, message: &[u8], mut len: usize)  {
    let mut m: Block = [0; BLOCK_SIZE];

    while len >= BLOCK_SIZE {
        m[..BLOCK_SIZE].clone_from_slice(&message[(len - BLOCK_SIZE)..len]);

        g(&mut ctx.h, &ctx.n, &m);

        add_mod_512_u32(&mut ctx.n, BLOCK_BIT_SIZE as u32);
        add_mod_512(&mut ctx.sigma, &m);

        len -= BLOCK_SIZE;
    }

    let padding = (BLOCK_SIZE - len) as u8;

    if padding > 0 {
        m = [0; BLOCK_SIZE];
        m[padding as usize - 1] = 0x01;
        for i in 0..len {
            m[padding as usize + i] = message[i];
        }
    }

    g(&mut ctx.h, &ctx.n, &m);

    add_mod_512_u32(&mut ctx.n, (len * 8) as u32);
    add_mod_512(&mut ctx.sigma, &m);

    g(&mut ctx.h, &[0_u8; BLOCK_SIZE], &ctx.n);
    g(&mut ctx.h, &[0_u8; BLOCK_SIZE], &ctx.sigma);
}
