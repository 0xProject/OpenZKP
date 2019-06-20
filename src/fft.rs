use crate::field::FieldElement;
use crate::u256::U256;

pub fn fft(root: FieldElement, vector: &[FieldElement]) -> Vec<FieldElement> {
    //debug_assert(root.pow(vector.len()) == FieldElement::ONE); //Todo - Add a pow method to field element

    let mut data = (vector.to_vec()).clone();
    let mut temp = FieldElement::ONE;
    let mut pow_table = Vec::with_capacity(vector.len());
    for _i in 0..(vector.len() / 2) {
        pow_table.push(temp.clone());
        temp *= &root;
    }

    fft_inplace(data.as_mut_slice(), pow_table.as_slice());
    data
}

//We can implement this function using an option for the cofactor input, depending on what we want
pub fn fft_cofactor(
    root: FieldElement,
    vector: &[FieldElement],
    cofactor: FieldElement,
) -> Vec<FieldElement> {
    let mut vector_type = vector.to_vec();

    let mut c = FieldElement::ONE;

    for element in vector_type.iter_mut() {
        *element *= &c;
        c *= &cofactor;
    }

    fft(root, &vector_type)
}

pub fn ifft(root: FieldElement, vector: &[FieldElement]) -> Vec<FieldElement> {
    let r = fft((&root).inv().unwrap(), vector);
    let len_el = FieldElement::from(U256::from((&r).len() as u64));
    let s = len_el.inv().unwrap();
    r.into_iter().map(|e| &s * e).collect()
}

pub fn ifft_cofactor(
    root: FieldElement,
    vector: &[FieldElement],
    cofactor: FieldElement,
) -> Vec<FieldElement> {
    let mut r = fft((&root).inv().unwrap(), vector);
    let len_el = FieldElement::from(U256::from((&r).len() as u64));
    let s = len_el.inv().unwrap();
    r = r.into_iter().map(|e| &s * e).collect();

    //The inv fft will give the cofactor transformed vector so we remove the transform.
    let mut c = FieldElement::ONE;
    let cofactor_inv = cofactor.inv().unwrap();
    for element in r.iter_mut() {
        *element *= &c;
        c *= &cofactor_inv;
    }
    r
}
//Using the Radix-2 algoritim
pub fn fft_inplace(vector: &mut [FieldElement], pow_table: &[FieldElement]) {
    let n = vector.len();
    let level = 64 - n.leading_zeros() - 1;
    debug_assert_eq!(1 << level, n);

    for i in 0..n {
        let j = reverse(i as u64, level as usize);
        if j > i {
            vector.swap(j, i) //.swap is unsafe when i == j but this is impossible here TODO - potentially implement pure safe version
        }
    }
    let mut size = 2;
    while size <= n {
        let halfstep = size / 2;
        let tablestep = n / size;
        for i in (0..n).step_by(size) {
            let mut k = 0;
            for j in i..(i + halfstep) {
                let l = j + halfstep;
                let left = vector[j].clone();
                let right = &vector[l] * &pow_table[k];
                vector[l] = &left - &right;
                vector[j] = left + right;
                k += tablestep;
            }
        }
        size *= 2;
    }
}

fn reverse(x: u64, bits: usize) -> usize {
    let mut x_hold = x;
    let mut y = 0;
    for _i in 0..bits {
        y = (y << 1) | (x_hold & 1);
        x_hold >>= 1;
    }
    y as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;
    use crate::u256::U256;
    use crate::u256h;
    use hex_literal::*;

    #[test]
    fn fft_test() {
        let root = FieldElement::from(u256h!(
            "063365fe0de874d9c90adb1e2f9c676e98c62155e4412e873ada5e1dee6feebb"
        ));
        let cofactor = FieldElement::from(u256h!(
            "07696b8ff70e8e9285c76bef95d3ad76cdb29e213e4b5d9a9cd0afbd7cb29b5c"
        ));
        let vector = [
            FieldElement::from(u256h!(
                "008ee28fdbe9f1a7983bc1b600dfb9177c2d82d825023022ab4965d999bd3faf"
            )),
            FieldElement::from(u256h!(
                "037fa3db272cc54444894042223dcf260e1d1ec73fa9baea0e4572817fdf5751"
            )),
            FieldElement::from(u256h!(
                "054483fc9bcc150b421fae26530f8d3d2e97cf1918f534e67ef593038f683241"
            )),
            FieldElement::from(u256h!(
                "005b695b9001e5e62549557c48a23fd7f1706c1acdae093909d81451cd455b43"
            )),
            FieldElement::from(u256h!(
                "025079cb6cb547b63b67614dd2c78474c8a7b17b3bc53f7f7276984b6b67b18a"
            )),
            FieldElement::from(u256h!(
                "044729b25360c0025d244d31a5f144917e59f728a3d03dd4685c634d2b0e7cda"
            )),
            FieldElement::from(u256h!(
                "079b0e14d0bae81ff4fe55328fb09c4117bcd961cb60581eb6f2a770a42240ed"
            )),
            FieldElement::from(u256h!(
                "06c0926a786abb30b8f6e0eb9ef2278b910862717ed4beb35121d4741717e0e0"
            )),
        ];

        let mut res = fft(root.clone(), &vector);

        assert_eq!(
            U256::from(&res[0]),
            u256h!("06a1b7c038205cb38aaeea38662ae2259a19c14a7519bd522543f72dc7fa74b2")
        );
        assert_eq!(
            U256::from(&res[1]),
            u256h!("017884f169b20153de79a9c642d4e3259263f2e7ac5f85f5a8191f28d8f14544")
        );
        assert_eq!(
            U256::from(&res[2]),
            u256h!("03112a352e474819d491a13b700a07161eee580ff40098df978fa19f39b4fd2d")
        );
        assert_eq!(
            U256::from(&res[3]),
            u256h!("011606a821f418d13914c72b424141c5b88bdb184b0b5a55fc537587346c78a2")
        );
        assert_eq!(
            U256::from(&res[4]),
            u256h!("00dc2519322c102b8ad3628106a3ebef7c39f85215203bfc820c7a04a9645419")
        );
        assert_eq!(
            U256::from(&res[5]),
            u256h!("01df6a70d033d89376c96c45ce8dbbe4eeedce2d32636c29d3cb87b9e2074d00")
        );
        assert_eq!(
            U256::from(&res[6]),
            u256h!("00ee6a5e89e9307e64789e1a71c42105de12bfa104e32c5a381fe5c2697ffeec")
        );
        assert_eq!(
            U256::from(&res[7]),
            u256h!("048bad0760f8b52ee4f9a46964bcf1ba9439a9467b2576176b1319cec9f12db0")
        );

        res = fft_cofactor(root, &vector, cofactor);

        assert_eq!(
            U256::from(&res[0]),
            u256h!("05d817ee1af8beff1880aad163a9912704d66e0c717a670c52db93da5ea34455")
        );
        assert_eq!(
            U256::from(&res[1]),
            u256h!("0631b16aceb1ee5711066df1ffafd9f5f451b0dc44c86e90005bc78e8bb4f861")
        );
        assert_eq!(
            U256::from(&res[2]),
            u256h!("01a30c98c149179cd16059ba201b99cf629d3e04844a50936006a185a67ad354")
        );
        assert_eq!(
            U256::from(&res[3]),
            u256h!("07a17b9035ff1ffd1f9e0bc52982effcd957bc07230830c10e51e906ed092f9e")
        );
        assert_eq!(
            U256::from(&res[4]),
            u256h!("01381787eccc6c77b0c5dff0b4b66dc0bb7d911bd705baf85f62001976e6ff27")
        );
        assert_eq!(
            U256::from(&res[5]),
            u256h!("009defa0822d287ce55035bb705319eb34e78180157e5297e6a46df9af8ef042")
        );
        assert_eq!(
            U256::from(&res[6]),
            u256h!("020b8317360c61abbc0bdce513eb42295402eb5dde3d13abfc0325f277f507bc")
        );
        assert_eq!(
            U256::from(&res[7]),
            u256h!("034738bd5956b1df55369cdc211109fd67e6ffd2ffbb08e856b1b4d1b1a2c6ae")
        );
    }

    #[test]
    fn ifft_test() {
        let root = FieldElement::from(u256h!(
            "063365fe0de874d9c90adb1e2f9c676e98c62155e4412e873ada5e1dee6feebb"
        ));
        let cofactor = FieldElement::from(u256h!(
            "07696b8ff70e8e9285c76bef95d3ad76cdb29e213e4b5d9a9cd0afbd7cb29b5c"
        ));
        let vector = [
            FieldElement::from(u256h!(
                "008ee28fdbe9f1a7983bc1b600dfb9177c2d82d825023022ab4965d999bd3faf"
            )),
            FieldElement::from(u256h!(
                "037fa3db272cc54444894042223dcf260e1d1ec73fa9baea0e4572817fdf5751"
            )),
            FieldElement::from(u256h!(
                "054483fc9bcc150b421fae26530f8d3d2e97cf1918f534e67ef593038f683241"
            )),
            FieldElement::from(u256h!(
                "005b695b9001e5e62549557c48a23fd7f1706c1acdae093909d81451cd455b43"
            )),
            FieldElement::from(u256h!(
                "025079cb6cb547b63b67614dd2c78474c8a7b17b3bc53f7f7276984b6b67b18a"
            )),
            FieldElement::from(u256h!(
                "044729b25360c0025d244d31a5f144917e59f728a3d03dd4685c634d2b0e7cda"
            )),
            FieldElement::from(u256h!(
                "079b0e14d0bae81ff4fe55328fb09c4117bcd961cb60581eb6f2a770a42240ed"
            )),
            FieldElement::from(u256h!(
                "06c0926a786abb30b8f6e0eb9ef2278b910862717ed4beb35121d4741717e0e0"
            )),
        ];

        let mut res = fft(root.clone(), &vector);
        let mut res_inv = ifft(root.clone(), &(res.as_slice()));

        for index in 0..8 {
            assert_eq!(vector[index], res_inv[index]);
        }
        res = fft_cofactor(root.clone(), &vector, cofactor.clone());

        res_inv = ifft_cofactor(root, &(res.as_slice()), cofactor);

        for index in 0..8 {
            assert_eq!(vector[index], res_inv[index]);
        }
    }
}
