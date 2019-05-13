use crate::field::FieldElement;
use crate::u256::U256;

pub fn fft(root : FieldElement, vector :  Vec<FieldElement>, cofactor: Option<FieldElement>) -> Vec<FieldElement>{
    match cofactor{
        None => return fft_pure(root, vector),
        Some(x) => return fft_cofactor(root, vector, x),
    }
}

pub fn fft_pure(root : FieldElement, vector :  Vec<FieldElement>) -> Vec<FieldElement>{
    let n = vector.len();

    if n == 1{
        return vector;
    }

    let mut index = 0;
    let mut even = Vec::new();
    let mut odd = Vec::new();
    while index < vector.len(){
        even.push(vector[index].clone());
        odd.push(vector[index+1].clone());
        index +=2;
    }
    
    even = fft_pure((&root).square(), even);
    odd = fft_pure((&root).square(), odd);
    let mut power = FieldElement::ONE;
    for i in 0..(n/2){
        odd[i] *= &power;
        let hold = even[i].clone(); //OPT: Can replace with a mut borrow if method is added to Field class
        even[i] = &even[i] + &odd[i];
        odd[i] = hold - &odd[i];
        power *= &root;
    }

    (even).append(&mut odd);
    even
}

//We can implement this function using an option for the cofactor input, depending on what we want
pub fn fft_cofactor(root : FieldElement, mut vector :  Vec<FieldElement>, cofactor : FieldElement) -> Vec<FieldElement>{
    let n = vector.len();

    let mut c = FieldElement::ONE;

    for i in 0..(n){
        vector[i] *= &c;
        c *= &cofactor;
    }

        if n == 1{
        return vector;
    }

    let mut index = 0;
    let mut even = Vec::new();
    let mut odd = Vec::new();
    while index < vector.len(){
        even.push(vector[index].clone());
        odd.push(vector[index+1].clone());
        index +=2;
    }
    
    even = fft_pure((&root).square(), even);
    odd = fft_pure((&root).square(), odd);
    let mut power = FieldElement::ONE;
    for i in 0..(n/2){
        odd[i] *= &power;
        let hold = even[i].clone(); //OPT: Can replace with a mut borrow if method is added to Field class
        even[i] = &even[i] + &odd[i];
        odd[i] = hold - &odd[i];
        power *= &root;
    }

    (even).append(&mut odd);
    even
}
pub fn ifft(root : FieldElement, vector :  Vec<FieldElement>)  -> Vec<FieldElement>{
    let r = fft_pure((&root).inv().unwrap(), vector);
    let len_el = FieldElement::from(U256::from((&r).len() as u64));
    let s =  len_el.inv().unwrap();
    let mut ret = Vec::new();
    for i in 0..(r.len()){
        ret.push(&s*(&r[i]))
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;
    use hex_literal::*;
    use crate::u256h;
    use crate::u256::U256;
    use crate::montgomery::*;

     #[test]
     fn basic_fft_test(){ //Small number test but covers
         let a =  FieldElement::from(U256::from(2 as u64));
         let b =  FieldElement::from(U256::from(1 as u64));
         let c = FieldElement::from(U256::from(3 as u64));
         let d = FieldElement::from(U256::from(4 as u64));
         let root = FieldElement::from(U256::from(3 as u64));
         let mut vector = Vec::new();
         vector.push(a);
         vector.push(b);
         vector.push(c);
         vector.push(d);
         let res = fft(root, vector, None);
         assert_eq!(U256::from(&res[0]), U256::from(10 as u64));
         assert_eq!(U256::from(&res[1]), u256h!("0800000000000010fffffffffffffffffffffffffffffffffffffffffffffff7")); // -10 mod P
         assert_eq!(U256::from(&res[2]), U256::ZERO);
         assert_eq!(U256::from(&res[3]), U256::from(8 as u64));
     }
 }
