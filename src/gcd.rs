use crate::u256::U256;

pub fn gcd_euclid(a: &U256, b: &U256) -> Option<(U256, U256, U256, bool)>{
    if  a == &U256::ZERO || b == &U256::ZERO {
        return None
    }
    let mut a_prime;
    let mut b_prime;

    if b > a { //Note : Alg assumes a >= b, and gcd(a,b) = gcd(b,a) 
       a_prime = b.clone(); //Gets correct ordering of mutable data
       b_prime = a.clone();
    } else{
        a_prime = a.clone(); //Gets correct ordering of mutable data
        b_prime = b.clone();
    }

    let mut consquences = (U256::ONE,U256::ZERO,U256::ZERO,U256::ONE);
    let mut even = true;

    while b_prime != U256::ZERO {
        let (hold1, hold2, hold3) = euclid_step(a_prime, b_prime, consquences);
        a_prime = hold1;
        b_prime = hold2;
        consquences = hold3;
        even = !even;
    }
    Some((a_prime, consquences.0, consquences.2, even))
}

fn euclid_step(a: U256, b: U256, data: (U256, U256, U256, U256)) -> (U256, U256, (U256, U256, U256, U256)){
    let (q, rem) = a.divrem(&b).unwrap();
    let hold1 = &data.0 + &data.1*&q;
    let hold2 = &data.2 + &data.3*q;
    (b, rem, (data.1, hold1 , data.3, hold2))
}

pub fn gcd_lehmer(a: &U256, b: &U256) -> Option<(U256, U256, U256, bool)>{
    if  a == &U256::ZERO || b == &U256::ZERO {
        return None
    }

    let mut a_prime;
    let mut b_prime;

    if b > a { //Note : Alg assumes a >= b, and gcd(a,b) = gcd(b,a) 
       a_prime = b.clone(); //Gets correct ordering of mutable data
       b_prime = a.clone();
    } else{
        a_prime = a.clone(); //Gets correct ordering of mutable data
        b_prime = b.clone();
    }

    if b.bits() < 64 { //If the largest entry is less than a word in length we don't gain but using the below use and return simple case
        return gcd_euclid(a, b);
    }
    

    let mut even = true;
    let mut ret_u = U256::ZERO; //We cannot directly use the u and v in the sequence because the deadlock stop eliminate them
    let mut ret_v = U256::ZERO;
    while a_prime != U256::ZERO && b_prime != U256::ZERO {
        let m = a_prime.msb()+1;
        let mut a_0 = a_prime.get_word(m);
        let mut a_1 = b_prime.get_word(m);

        let mut u_0 = 1; 
        let mut v_0 = 0;
        let mut u_1 = 0;
        let mut v_1 = 1;
        even = true;

        loop{
            let q = a_0/a_1;

            let a_2 = a_0  - q*a_1; //Better than going to a mod calc

            let v_2 = v_0 + q*v_1;
            let u_2 = u_0 + q*u_1;
        
            let v_3 = if v_2 > v_1 { v_2 - v_1} else {v_1 - v_2}; //Asignment of absolute value
            if !(a_2 >= v_2 && a_1-a_2 >= v_3) {break;} //Collins stoping condition

            even = !even;
            a_0 = a_1;
            a_1 = a_2; //Moves the euclidian algorthim forward a step on a_i
            u_0 = u_1;
            u_1 = u_2;
            v_0 = v_1;
            v_1 = v_2; //Moves both consquence calcs forward in sequnce
        }
        if v_0 == 0 { // Deadlock condition, in this case the algorthim will loop, so we have to advance with normal step.
            let (hold1, hold2, hold3) = euclid_step(a_prime, b_prime, (U256::from(u_0),U256::from(u_1),U256::from(v_0),U256::from(v_1)));
            a_prime = hold1;
            b_prime = hold2;

            ret_u = hold3.1;
            ret_v = hold3.3;
        } else{
            let hold = a_prime.clone();

            if even {
                a_prime = a_prime*u_0 - &b_prime*v_0;
                b_prime = b_prime*v_1 - hold*u_1;
            } else{
                a_prime = &b_prime*v_0 - a_prime*u_0;
                b_prime = hold*u_1 - b_prime*v_1;
            }
        }
    }
    Some((a_prime, U256::from(ret_u), U256::from(ret_v), even))
}
#[cfg(test)]
mod tests {
    use crate::field::{FieldElement, MODULUS};
    use super::*;
    use crate::u256::U256;
    use crate::u256h;
    use hex_literal::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_gcd_euclid()
    {
        let a = u256h!("018a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814");
        let b = u256h!("518a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814");
        let expected = u256h!("0000000000000000000000000000000000000000000000000000000000000004");
        let result = gcd_euclid(&a,&b).unwrap();
        assert_eq!(result.0, expected)
    }

    #[test]
    fn test_inv_euclid()
    {
        let a = u256h!("008a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814");
        let b = U256::from(MODULUS);
        let expected = u256h!("0000000000000000000000000000000000000000000000000000000000000001");
        let inv = gcd_euclid(&a,&b).unwrap();
        
        let result;
        if inv.3 {
            result = a.mulmod(&(&MODULUS - &inv.2), &MODULUS);
        } else{
            result = a.mulmod(&inv.2, &MODULUS);
        }

        assert_eq!(inv.0, expected); //GCD should be 1 showing they are coprime
        assert_eq!(result, expected) //Then the mulmod should be one showing it's an inverse
    }

    #[test]
    fn test_inv_lehmer()
    {
        let a = u256h!("018a5cc4c55ac5b050a0831b65e827e5e39ff4515e4e094961c61509e7870814");
        let b = U256::from(MODULUS);
        let expected = u256h!("0000000000000000000000000000000000000000000000000000000000000001");
        let inv = gcd_lehmer(&a,&b).unwrap();
        println!("{:?}", inv);
        
        let result;
        if inv.3 {
            result = a.mulmod(&(&MODULUS - &inv.2), &MODULUS);
        } else{
            result = a.mulmod(&inv.2, &MODULUS);
        }
        assert_eq!(inv.0, expected); //GCD should be 1 showing they are coprime
        assert_eq!(expected, result) //Then the mulmod should be one showing it's an inverse
    }

    #[test]
    fn test_gcd_lehmer()
    {
        let a = u256h!("018a5cc4c55ac5b050a0831b65e827e5e39ff4515e4e094961c61509e7870814");
        let b = u256h!("218f5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814");
        let expected = u256h!("0000000000000000000000000000000000000000000000000000000000000014");
        let result = gcd_lehmer(&a,&b).unwrap();
        assert_eq!(result.0, expected)
    }
}