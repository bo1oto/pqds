#[cfg(test)]
mod tests {
    use pqds::stribog::{*, constants::*};


    fn gen_table() -> [[u64; 256]; 8] {
        let mut table: [[u64; 256]; 8] = [[0; 256]; 8];
        for i in 0..8 {
            for j in 0..256 {
                let mut c = 0_u64;
                for k in 0..8 {
                    if SBOX[j] & (0x80 >> k) != 0 {
                        c ^= A[8 * i + k];
                    }
                }
                table[i][j] = c;
            }
        }
        table
    }
    #[test]
    fn test_table() {
        let table = gen_table();
        for i in 0..8 {
            for j in 0..256 {
                assert_eq!(SHUFFLED_LIN_TABLE[i][j], table[i][j]);
            }
        }
    }
    const M1: [u8; 63] = [
        0x32, 0x31, 0x30, 0x39, 0x38, 0x37, 0x36, 0x35,
        0x34, 0x33, 0x32, 0x31, 0x30, 0x39, 0x38, 0x37,
        0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30, 0x39,
        0x38, 0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31,
        0x30, 0x39, 0x38, 0x37, 0x36, 0x35, 0x34, 0x33,
        0x32, 0x31, 0x30, 0x39, 0x38, 0x37, 0x36, 0x35,
        0x34, 0x33, 0x32, 0x31, 0x30, 0x39, 0x38, 0x37,
        0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30
    ];

    const M2: [u8; 72] = [
        0xfb, 0xe2, 0xe5, 0xf0, 0xee, 0xe3, 0xc8, 0x20,
        0xfb, 0xea, 0xfa, 0xeb, 0xef, 0x20, 0xff, 0xfb,
        0xf0, 0xe1, 0xe0, 0xf0, 0xf5, 0x20, 0xe0, 0xed,
        0x20, 0xe8, 0xec, 0xe0, 0xeb, 0xe5, 0xf0, 0xf2,
        0xf1, 0x20, 0xff, 0xf0, 0xee, 0xec, 0x20, 0xf1,
        0x20, 0xfa, 0xf2, 0xfe, 0xe5, 0xe2, 0x20, 0x2c,
        0xe8, 0xf6, 0xf3, 0xed, 0xe2, 0x20, 0xe8, 0xe6,
        0xee, 0xe1, 0xe8, 0xf0, 0xf2, 0xd1, 0x20, 0x2c,
        0xe8, 0xf0, 0xf2, 0xe5, 0xe2, 0x20, 0xe5, 0xd1
    ];
    #[test]
    fn test_512() {
        let (h_in, h_out) = (HashSize::L512, 64);

        let mut ctx = Stribog::new(h_in);
        stribog(&mut ctx, &M1, M1.len());
        assert_eq!(
            "486f64c1917879417fef082b3381a4e211c324f074654c38823a7b76f830ad00fa1fbae42b1285c0352f227524bc9ab16254288dd6863dccd5b9f54a1ad0541b",
            ctx.h.iter().take(h_out).map(|byte| format!("{:02x}", byte)).collect::<String>().as_str()
        );

        ctx = Stribog::new(h_in);
        stribog(&mut ctx, &M2, M2.len());
        assert_eq!(
            "28fbc9bada033b1460642bdcddb90c3fb3e56c497ccd0f62b8a2ad4935e85f037613966de4ee00531ae60f3b5a47f8dae06915d5f2f194996fcabf2622e6881e",
            ctx.h.iter().take(h_out).map(|byte| format!("{:02x}", byte)).collect::<String>().as_str()
        );
    }
    #[test]
    fn test_256() {
        let (h_in, h_out) = (HashSize::L256, 32);

        let mut ctx = Stribog::new(h_in);
        stribog(&mut ctx, &M1, M1.len());
        assert_eq!(
            "00557be5e584fd52a449b16b0251d05d27f94ab76cbaa6da890b59d8ef1e159d",
            ctx.h.iter().take(h_out).map(|byte| format!("{:02x}", byte)).collect::<String>().as_str()
        );
        ctx = Stribog::new(h_in);
        stribog(&mut ctx, &M2, M2.len());
        assert_eq!(
            "508f7e553c06501d749a66fc28c6cac0b005746d97537fa85d9e40904efed29d",
            ctx.h.iter().take(h_out).map(|byte| format!("{:02x}", byte)).collect::<String>().as_str()
        );
    }
}
