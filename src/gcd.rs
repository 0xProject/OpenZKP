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

#[inline(always)]
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

    if b_prime.bits() < 64 { //If the largest entry is less than a word in length we don't gain but using the below use and return simple case
        return gcd_euclid(a, b);
    }

    let mut even = true;
    let mut U_0 = U256::ZERO; //Unconpressed long form inverse
    let mut U_1 = U256::ONE;

    let mut index = 0;

    loop{
        let bits  = a_prime.bits();
        if  bits > 100{
            let m = a_prime.msb()+1;
            let mut a_0 = a_prime.get_double_word(m);
            let mut a_1 = b_prime.get_double_word(m);
            let mut a_0_single = (a_0 >> 64) as u64;
            let mut a_1_single = (a_1 >> 64) as u64;

            println!("{}", a_prime);
            println!("{}", b_prime);

            index += 1;
            if index > 100 {break};

            //let (u_0, v_0, u_1 , v_1, hold_even) = lemher_loop_experimental(a_0, a_1);
            let (u_0, v_0, u_1 , v_1, hold_even)  = cohen_exact(a_0_single, a_1_single, a_0, a_1, &a_prime, &b_prime);
            even = hold_even;

            
            let mut hold = a_prime.clone();

            if even {
                a_prime = a_prime*u_0 - &b_prime*v_0;
                b_prime = b_prime*v_1 - hold*u_1;

                hold = U_0.clone();
                U_0 = u_0*U_0 + v_0*&U_1;
                U_1 = v_1*U_1 + u_1*hold;
            } else{
                a_prime = &b_prime*v_0 - a_prime*u_0;
                b_prime = hold*u_1 - b_prime*v_1;

                hold = U_0.clone();
                U_0 = v_0*&U_1 + u_0*U_0;
                U_1 = u_1*hold + v_1*U_1;
            }
            
        } else if bits > 60{
            let m = a_prime.msb()+1;
            let mut a_0 = a_prime.get_word(m);
            let mut a_1 = b_prime.get_word(m);

            println!("{}", a_prime);
            println!("{}", b_prime);

            index += 1;
            if index > 100 {break};


            let (u_0, v_0, u_1 , v_1, hold_even) = lemher_loop_64(a_0, a_1);
            even = hold_even;

            
            let mut hold = a_prime.clone();

            if even {
                a_prime = a_prime*u_0 - &b_prime*v_0;
                b_prime = b_prime*v_1 - hold*u_1;

                hold = U_0.clone();
                U_0 = u_0*U_0 + v_0*&U_1;
                U_1 = v_1*U_1 + u_1*hold;
            } else{
                a_prime = &b_prime*v_0 - a_prime*u_0;
                b_prime = hold*u_1 - b_prime*v_1;

                hold = U_0.clone();
                U_0 = v_0*&U_1 + u_0*U_0;
                U_1 = u_1*hold + v_1*U_1;
            }
            
        } else{
            break
        }
    }
    let (final_u, final_v, final_d, next_even) = euclid_64(a_prime.c0, b_prime.c0);

    let u;
    if next_even{
        u = final_u*U_0 - final_v*U_1;
    } else{
        u = final_v*U_1 - final_u*U_0;
    }
    let v = (U256::from(final_d) - a*final_u)/b;

    Some((U256::from(final_d), v, u, next_even)) //TODO Return a Q and U instead of current
}

#[inline(always)]
fn euclid_64(a_0: u64, a_1: u64) -> (u64, u64, u64, bool){
    let mut a = a_0;
    let mut b = a_1;

    let mut u_a = 0;
    let mut u_b = 1;
    let mut even = true;

    let mut index = 0;
    while(b > 0){
        println!("{}", a);
        println!("{}", b);

        index += 1;
        if index > 100 {break};

        let q = a/b;

        let mut T = a;
        a = b;
        b = T - q*b;
        
        T = u_a;
        u_a = u_b;
        u_b = T + q*u_b;
        even = !even;
    }
    let q;
    let test = a_0.checked_mul(u_b);
    match test {
        Some(x) => {    if a > a_0*u_b{
                           q = a - a_0*u_b;
                        } else{
                           q = a_0*u_b - a;
                    }},
        None => { let hold = (a_0 as u128)*(u_b as u128) - (a as u128);
                   q = hold as u64},
    }
    // if a > a_0*u_b{
    //     q = a - a_0*u_b;
    // } else{
    //     q = a_0*u_b - a;
    // }
    (u_b, q, a, even)
}

#[inline(always)]
fn lemher_loop_64(mut a_0: u64, mut a_1: u64) -> (u64, u64, u64, u64, bool){
    
    let mut u_0 = 1; 
    let mut v_0 = 0;
    let mut u_1 = 0;
    let mut v_1 = 1;
    let mut even = true;

    loop{
        if a_1 == 0 {break};
        let q = a_0/a_1;

        let a_2 = a_0  - q*a_1; //Better than going to a mod calc

        let v_2 = v_0 + q*v_1;
        let u_2 = u_0 + q*u_1;

        //if a_2 < u32::max_value().into() {break};
        let v_3 = if v_2 > v_1 { v_2 - v_1} else {v_1 - v_2}; //Asignment of absolute value
        if !(a_2 >= v_2 && a_1-a_2 >= v_3) {break;} //Collins stoping condition
        // if a_2 < u32::max_value().into() {break}

        even = !even;
        a_0 = a_1;
        a_1 = a_2; //Moves the euclidian algorthim forward a step on a_i
        u_0 = u_1;
        u_1 = u_2;
        v_0 = v_1;
        v_1 = v_2; //Moves both consquence calcs forward in sequnce
    }
    (u_0, v_0, u_1, v_1, even)
}

#[inline(always)]
fn lemher_loop_128(mut a_0: u128, mut a_1: u128) -> (u128, u128, u128, u128, bool){
    
    let mut u_0 = 1; 
    let mut v_0 = 0;
    let mut u_1 = 0;
    let mut v_1 = 1;
    let mut even = true;

    loop{
        if a_1 == 0 {break};
        let q = a_0/a_1;

        let a_2 = a_0  - q*a_1; //Better than going to a mod calc

        let v_2 = v_0 + q*v_1;
        let u_2 = u_0 + q*u_1;

        //if a_2 < u32::max_value().into() {break};
        let v_3 = if v_2 > v_1 { v_2 - v_1} else {v_1 - v_2}; //Asignment of absolute value
        if !(a_2 >= v_2 && a_1-a_2 >= v_3) {break;} //Collins stoping condition
        // if a_2 < u32::max_value().into() {break}

        even = !even;
        a_0 = a_1;
        a_1 = a_2; //Moves the euclidian algorthim forward a step on a_i
        u_0 = u_1;
        u_1 = u_2;
        v_0 = v_1;
        v_1 = v_2; //Moves both consquence calcs forward in sequnce
    }
    (u_0, v_0, u_1, v_1, even)
}

fn lemher_loop_64_full(mut a_0: u64, mut a_1: u64, mut u_0: u64, mut v_0: u64, mut u_1: u64, mut v_1: u64, mut even: bool) -> (u64, u64, u64, u64, bool){
    
    loop{
        if a_1 == 0 {break};
        let q = a_0/a_1;

        let a_2 = a_0  - q*a_1; //Better than going to a mod calc

        let v_2 = v_0 + q*v_1;
        let u_2 = u_0 + q*u_1;

        if a_2 < u32::max_value().into() {break};
        let v_3 = if v_2 > v_1 { v_2 - v_1} else {v_1 - v_2}; //Asignment of absolute value
        if !(a_2 >= v_2 && a_1-a_2 >= v_3) {break;} //Collins stoping condition
        //if a_2 < u32::max_value().into() {break}

        even = !even;
        a_0 = a_1;
        a_1 = a_2; //Moves the euclidian algorthim forward a step on a_i
        u_0 = u_1;
        u_1 = u_2;
        v_0 = v_1;
        v_1 = v_2; //Moves both consquence calcs forward in sequnce
    }
    (u_0, v_0, u_1, v_1, even)
}




fn lemher_loop_experimental(mut a_0: u128, mut a_1: u128) -> (u128, u128, u128, u128, bool){

    let mut u_0 = 1; 
    let mut v_0 = 0;
    let mut u_1 = 0;
    let mut v_1 = 1;
    let mut even = true;

    let mut index = 0;
    loop{
        println!("{}", a_0);
        println!("{}", a_1);

        index += 1;
        if index > 100 {break};

        let mut alpha = 1 as u64;
        let mut beta  = 0 as u64;
        let mut alpha_prime = 0 as u64;
        let mut beta_prime = 1 as u64;
        let mut internal_even = true;

        let mut a_0_single;
        let mut a_1_single;

        let bits = a_0.leading_zeros();
        if  bits > 64 {
            a_0_single = a_0 as u64;
            a_1_single = a_1 as u64;
        } else{
            a_0_single = (a_0 >> (64 - bits)) as u64;
            a_1_single = (a_1 >> (64 - bits)) as u64;
        }
        println!("{}", a_0_single);
        println!("{}", a_1_single);

        loop{
            if a_1_single == 0 {break};
            let q = a_0_single/a_1_single;
            println!("q{}", q);

            let a_2_single = a_0_single  - q*a_1_single; //Better than going to a mod calc

            let beta_2 = beta + q*beta_prime;
            let alpha_2 = alpha + q*alpha_prime;

            let v_3 = if beta_2 > beta_prime { beta_2- beta_prime} else {beta_prime - beta_2}; //Asignment of absolute value
            if !(a_2_single >= beta_2 && a_1_single-a_2_single >= v_3) {break;} //Collins stoping condition
            if alpha_2.leading_zeros() + a_0.leading_zeros() < 65 || beta_2.leading_zeros() + a_1.leading_zeros() < 65 {break};

            internal_even = !internal_even;
            a_0_single = a_1_single;
            a_1_single = a_2_single; //Moves the euclidian algorthim forward a step on a_i
            alpha = alpha_prime;
            alpha_prime = alpha_2;
            beta = beta_prime;
            beta_prime = beta_2; 
        }

        if beta != 0 {
            let mut a_2;
            let u_2;
            let v_2;

            println!("{}", alpha_prime);
            println!("{}", beta_prime);
            if internal_even {
                a_2 = a_1*(beta_prime as u128) - a_0*(alpha_prime as u128);
                v_2 = v_1*(beta_prime as u128) + v_0*(alpha_prime as u128);
            } else{
                a_2 = a_0*(alpha_prime as u128) - a_1*(beta_prime as u128);
                v_2 = v_0*(alpha_prime as u128) + v_1*(beta_prime as u128);
            }

            //if a_2 < u32::max_value().into() {break};
            let v_3 = if v_2 > v_1 { v_2 - v_1} else {v_1 - v_2}; //Asignment of absolute value
            if (a_1 < a_2) {break};
            if !(a_2 >= v_2 && a_1-a_2 >= v_3) {break;} //Collins stoping condition
            // if a_2 < u32::max_value().into() {break}

            if internal_even {
                
                //v_2 = v_1*(beta_prime as u128) + v_0*(alpha_prime as u128);
            } else{
                a_2 = a_0*(alpha_prime as u128) - a_1*(beta_prime as u128);
                //v_2 = v_0*(alpha_prime as u128) + v_1*(beta_prime as u128);
            }

            if !internal_even { //
                a_0 = a_1*(beta as u128) - a_0*(alpha as u128);
                u_2 = u_1*(beta_prime as u128) +  u_0*(alpha_prime as u128);
                u_0 = u_1*(beta as u128) + u_0*(alpha as u128);
                v_0 = v_1*(beta as u128) + v_0*(alpha as u128);
            } else{
                a_0 = a_0*(alpha as u128) - a_1*(beta as u128);
                u_2 = u_0*(alpha_prime as u128) + u_1*(beta_prime as u128);
                u_0 = u_0*(alpha as u128) + u_1*(beta as u128);
                v_0 = v_0*(alpha as u128) + v_1*(beta as u128);
            }
            even = !even;
            a_1 = a_2; //Moves the euclidian algorthim forward a step on a_i
            u_1 = u_2;
            v_1 = v_2;
        } else {
            println!("got here");
            if a_1 == 0 {break};
            let q = a_0/a_1;

            let a_2 = a_0  - q*a_1; //Better than going to a mod calc

            let v_2 = v_0 + q*v_1;
            let u_2 = u_0 + q*u_1;

            even = !even;
            a_0 = a_1;
            a_1 = a_2; //Moves the euclidian algorthim forward a step on a_i
            u_0 = u_1;
            u_1 = u_2;
            v_0 = v_1;
            v_1 = v_2; 
        }
    }
    
    (u_0, v_0, u_1, v_1, even)
}

fn cohen_exact(mut a_single: u64, mut b_single: u64, mut a_double: u128, mut b_double: u128, a: &U256, b : &U256) -> (u64, u64, u64, u64, bool){
    let mut alpha = 1;
    let mut beta = 0;
    let mut alpha_prime = 0;
    let mut beta_prime = 1;
    let mut T = 0;
    let mut even = true;
    let mut q = 0;

    if b_single != 0 { q = a_single/b_single; T = a_single%b_single;};
    if T >= 2^32 {
        loop{
            let q_prime = b_single/T;
            let T_prime = b_single%T;

            if T_prime < 2^32 {break};
            a_single = b_single;
            b_single = T;
            T = alpha + q*alpha_prime; //Plus to get the abs value {otherwise will underflow}
            alpha = alpha_prime;
            alpha_prime = T;
            T = beta + q*beta_prime; //Again with the abs
            beta = beta_prime;
            beta_prime = T;
            T = T_prime;
            q = q_prime;

            even = !even;
        }
    }
    if beta == 0 {let (ret , _) = &a.divrem(&b).unwrap(); return (0,1,1,ret.c0, true);}

    let hold = a_double.clone();
    a_double = (alpha as u128)*a_double + (beta as u128)*&b_double; //Matrix unfolding step without signs
    b_double = (alpha_prime as u128)*hold + (beta_prime as u128)*b_double;

    // if even {
    //     a_double = (alpha as u128)*a_double - (beta as u128)*&b_double; //Matrix unfolding step with signs
    //     b_double = (alpha_prime as u128)*hold - (beta_prime as u128)*b_double;
    // } else {
    //     a_double =  (beta as u128)*&b_double - (alpha as u128)*a_double; //Matrix unfolding step with signs
    //     b_double = (beta_prime as u128)*b_double - (alpha_prime as u128)*hold ;
    // }

    let bits = a_double.leading_zeros();
    if  bits > 64 {
        a_single = a_double as u64;
        b_single = b_double as u64;
    } else{
        a_single = (a_double >> (64 - bits)) as u64; //Shifts over enough to put the msb in the top slot of a u64
        b_single = ( b_double >> (64 - bits)) as u64;
    }
    T = 0;
    
    if b_single != 0 { q = a_single/b_single; T = a_single%b_single;};
    
    if T >= 2^32 {
        loop{
            let q_prime = b_single/T;
            let T_prime = b_single%T;

            if T_prime < 2^32 { break};
            a_single = b_single;
            b_single = T;
            T = alpha + q*alpha_prime; //Plus to get the abs value {otherwise will underflow}
            alpha = alpha_prime;
            alpha_prime = T;
            T = beta + q*beta_prime; //Again with the abs
            beta = beta_prime;
            beta_prime = T;
            T = T_prime;
            q = q_prime;

            even = !even;
        }
    }
    return (alpha, beta, alpha_prime, beta_prime, even)
}

fn cohen_exact_signed(mut a_single_un: u64, mut b_single_un: u64, mut a_double_un: u128, mut b_double_un: u128, a: &U256, b : &U256) -> (u64, u64, u64, u64, bool){
    
    let mut a_single : i64 = (a_single_un >> 1 ) as i64;
    let mut b_single : i64 = (b_single_un >> 1) as i64;
    let mut a_double : i128 = (a_double_un >> 1) as i128;
    let mut b_double : i128 = (b_double_un >> 1) as i128;

    let mut alpha = 1;
    let mut beta = 0;
    let mut alpha_prime = 0;
    let mut beta_prime = 1;
    let mut T = 0;
    let mut even = true;
    let mut q = 0;

    if b_single != 0 { q = a_single/b_single; T = a_single%b_single;};
    if T >= 2^32 {
        loop{
            let q_prime = b_single/T;
            let T_prime = b_single%T;

            if T_prime < 2^32 {break};
            a_single = b_single;
            b_single = T;
            T = alpha - q*alpha_prime; //Plus to get the abs value {otherwise will underflow}
            alpha = alpha_prime;
            alpha_prime = T;
            T = beta - q*beta_prime; //Again with the abs
            beta = beta_prime;
            beta_prime = T;
            T = T_prime;
            q = q_prime;

            even = !even;
        }
    }
    if beta == 0 {let (ret , _) = &a.divrem(&b).unwrap(); return (0,1,1,ret.c0, true);}

    let hold = a_double.clone();
    a_double = (alpha as i128)*a_double + (beta as i128)*&b_double; //Matrix unfolding step without signs
    b_double = (alpha_prime as i128)*hold + (beta_prime as i128)*b_double;

    // if even {
    //     a_double = (alpha as u128)*a_double - (beta as u128)*&b_double; //Matrix unfolding step with signs
    //     b_double = (alpha_prime as u128)*hold - (beta_prime as u128)*b_double;
    // } else {
    //     a_double =  (beta as u128)*&b_double - (alpha as u128)*a_double; //Matrix unfolding step with signs
    //     b_double = (beta_prime as u128)*b_double - (alpha_prime as u128)*hold ;
    // }

    let bits = a_double.leading_zeros();
    if  bits > 64 {
        a_single = a_double as i64;
        b_single = b_double as i64;
    } else{
        a_single = (a_double >> (64 - bits)) as i64; //Shifts over enough to put the msb in the top slot of a u64
        b_single = ( b_double >> (64 - bits)) as i64;
    }
    T = 0;
    
    if b_single != 0 { q = a_single/b_single; T = a_single%b_single;};
    
    if T >= 2^32 {
        loop{
            let q_prime = b_single/T;
            let T_prime = b_single%T;

            if T_prime < 2^32 { break};
            a_single = b_single;
            b_single = T;
            T = alpha - q*alpha_prime; //Plus to get the abs value {otherwise will underflow}
            alpha = alpha_prime;
            alpha_prime = T;
            T = beta - q*beta_prime; //Again with the abs
            beta = beta_prime;
            beta_prime = T;
            T = T_prime;
            q = q_prime;

            even = !even;
        }
    }
    //Clear the signing bits
    let alpha_un = ((alpha << 1) >> 1) as u64;
    let alpha_prime_un = ((alpha_prime << 1) >> 1) as u64;
    let beta_un = ((beta << 1) >> 1) as u64;
    let beta_prime_un = ((beta_prime << 1) >> 1) as u64;
    return (alpha_un, beta_un, alpha_prime_un, beta_prime_un, even)
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
        let b = MODULUS;
        let expected = u256h!("0000000000000000000000000000000000000000000000000000000000000001");
        let inv = gcd_euclid(&a,&b).unwrap();
        
        let result;
        if inv.3 {
            result = a.mulmod(&(&MODULUS - &inv.2), &MODULUS);
        } else{
            result = a.mulmod(&inv.2, &MODULUS);
        }

        assert_eq!(inv.0, expected); //GCD should be 1 showing they are coprime
        println!("{}", MODULUS-&result);
        assert_eq!(result, expected) //Then the mulmod should be one showing it's an inverse
    }

    #[test]
    fn test_inv_lehmer()
    {
        let a = u256h!("018a5cc4c55ac5b150a0831b65e828e5e39ff4515e4e094961c61509e7870814");
        let b = &MODULUS;
        let expected = u256h!("0000000000000000000000000000000000000000000000000000000000000001");
        let inv = gcd_lehmer(&a,&b).unwrap();
        
        let result;
        if inv.3 {
            result = a.mulmod(&(&MODULUS - &inv.1), &MODULUS);
        } else{
            result = a.mulmod(&inv.1, &MODULUS);
        }

        assert_eq!(inv.0, expected); //GCD should be 1 showing they are coprime
        assert_eq!(expected, (&result%MODULUS)) //Then the mulmod should be one showing it's an inverse
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