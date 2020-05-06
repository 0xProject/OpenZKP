#![allow(clippy::possible_missing_comma)]
use std::time::Instant;
use zkp_macros_decl::field_element;
use zkp_primefield::{fft::permute, Fft, FieldElement, Pow, Root, SquareInline, Zero};
use zkp_stark::{
    solidity_encode::autogen, Constraints, DensePolynomial, Provable, RationalExpression,
    TraceTable, Verifiable,
};
use zkp_u256::U256;

const Q: FieldElement = field_element!("0B");
const K_COEF: [FieldElement; 128] = [
    field_element!("00ed021e66d670608d65fa55597c3da99e143e17bc34a01dd32b352a028ec839"),
    field_element!("05c8707c12896aed50aed74ccab0e11eb2bdf909946e6b6e81c0d2828b476496"),
    field_element!("04d2550233879ef0d7b5b954d88534ff34619e3ef9d4aafb04d4d3d6695098ed"),
    field_element!("023d3c5a0a4937c2db8384f1ec9c8a50bf4f8c8398462576fc6dab3539ec755b"),
    field_element!("00293de4c0b03a5f72dfcc36a2426f159e42229f10bccb8fad1e85688b854173"),
    field_element!("01632e11ccbdee1b0cef5064bfa964642d95525b1cbba2da3b7d1f643d299ebc"),
    field_element!("074bfd772feaa22b3488b0b5ec3b68ecbef7ec91c29739264f67071222955e2c"),
    field_element!("007121b7e548cd5850feb6534d599be411342e9bcf858cc036c7f3a78359bdc0"),
    field_element!("07c83a09664f16634d039af747a439c966cb826e1e6ce56fe2ca32fa95756b0a"),
    field_element!("03ca9222a4e764de5a0414db0037f36e95aecf04dc9fdce9df0d40d98a263136"),
    field_element!("01750b281b640e2264f37b7f80a74fbb3d29faf9b73cf383f4e88b3b7e998779"),
    field_element!("00831bf753892914f58f1511f69c950ea2853eff5a1787f9e80cc65e59633c67"),
    field_element!("02d357332d061d578534a3aec19f71ea58b03d7095c63241042dfdaf332dbee7"),
    field_element!("04c4ba96d27adbd92213f6bbf38cce6005c4557afa50866c0d2d4274d886fde7"),
    field_element!("03044a9dfa132a11d195a7b46ffaff131941a62cc5e675aa30d8ffb5c0b36019"),
    field_element!("0659fd39e0651a2d1622f524ce397ab179b2ec9dfec8c1076fe2a8c46c00ff42"),
    field_element!("074d5503bfc7c28cb08e2f8bfdfc65ff671f62afa36ad26f6f1b65d7850369cc"),
    field_element!("03b91bb1905ff60ac5068e0ffee9f3a35977a894525717a8c9ed600eb2a1d438"),
    field_element!("02744b08fda6c05cf1295f6de7355d721886dfc77476d4ce36a1736c9e89c5fb"),
    field_element!("03d18c0fac30b0ebf20b3b6c1c4b7d82ab77749a5f7446221b188fc38d2836a7"),
    field_element!("07bcf3a1cd6a4bec11e215f814fbbf69e634579102e0220eb4db51f4d158366d"),
    field_element!("01f1ec34374f555a4afc97d09af1c010028ffee0833eb0926dbe43ee96d1a258"),
    field_element!("026092f7fde398226a4eff02a59e35d2750270bf422c5c47f77c8b994e6e2f00"),
    field_element!("042ffbd55e2901c42570740e323ca7ad71df18813136584f4696836d4b70277c"),
    field_element!("031681ead57b167f296b176700b372f3fb77becfafaaf29a42bd61d74b5cf839"),
    field_element!("034096f164304059300aa25d050ff033589e223d220123a85ac1b34959c87267"),
    field_element!("00d4ded3b2ad9e2e47ce09396ab294c35b05f7c4335e3a37dd1cf692d99f0a0a"),
    field_element!("048b9714f4c55b27b15d861cc4bec8bbc309d9f53a2abaa271f98eb4fe4aa26b"),
    field_element!("03751ed12cc60ef7ce9bd757d35484f59b03097c59ba5c45036b65750408f483"),
    field_element!("078d5ccba44d6ca1b78f5943cc7058f2e31b52dba98903e93cd26b9eb73bd75a"),
    field_element!("038ebb3a7808e92a6f3b4484d5edf5ce9a0933e1c95dba2b5e78fab8fcd1478a"),
    field_element!("00b642861538fab992aeae4b4d895f1023440975345d0a5f8bf5554fbe997943"),
    field_element!("03b8878c7490a57720d7a12feebc3464dfcb9a0a333761f0b1b31148b924ef1b"),
    field_element!("01ac69822b7f18d026fb66f5d2fd4b9f131bc3df1a71e9972ca461bf4e3bad83"),
    field_element!("0346a7ba290a081302abd258dfc4efc5b15d42e792434df73178c63561d2bde2"),
    field_element!("073904d320d9beb16c2f949a2b97efb1f3c89b8fdb96dd3ea52643b012fcf35f"),
    field_element!("0533cc56e887dd229a70223207e039681752821e5776b8c52784c23acecc3505"),
    field_element!("06ef60aac4b7e261cf69b51212fef2b8c36eaae6ec8aebcf386c153e1525831b"),
    field_element!("0753bed1a2af9e00dd8b11e098fa49b3697c41cd8e8c2b7b9e4e9b9bc5ccafb9"),
    field_element!("00a7a400be7e52ef9601cc8798882b97fff855ac7ae8c0f9b3f9883b951fb57a"),
    field_element!("01b438f43354a06a8a21c4bd09cbcc7c3dcbab899f464e8cfc6249946ac3b742"),
    field_element!("03e5865adb4110c8b35551f1ff356fd2a46cb5a986ab2a823b610d23d24486e0"),
    field_element!("065dad862b6410a140d638d7d9106239b4fbcb4b6fe5c2d707b70ab097cf1151"),
    field_element!("0655e879e220d013d3d31d2f317c424322591d14f94148abf5cc1f34860ba86e"),
    field_element!("006f477070d30b150b98f871bed32aef711856e24324f7cf62d15ea35333c7b0"),
    field_element!("0593a3ecd5ba6795d7ed9842784b85b8e7d9bef76a3eda0d688d35d98165ce96"),
    field_element!("01de5f8a47945ba0fd8cd8e9a3bf563e940b746a1dfc2c893566c252b230c496"),
    field_element!("029a23e67b7a265242982f5e09e74d443882b71b6e7e2df59bca2208e0a89b9d"),
    field_element!("0446c63fde4e4180a39c5321e98873f4acd9898ae84219b2e0f8601b573c3406"),
    field_element!("077a9bb14b1031021c80770d67cd3a98f37250b46a2559f39fec72d0f6f2999d"),
    field_element!("075364a4dfa99572171b561a8ddf0103fddc33d66eb85a7feb5ec33feb1de170"),
    field_element!("016a40882d59f97d59c4b9eeb5d6b9476e10c6807eeaa712a4de312dc79bd0ea"),
    field_element!("05e030fee6f6366c09de0702f90860a2d651332d790fce53ccc10e6aca6afa4c"),
    field_element!("07577a214fdae34d254429f79f6575de2a4124d7da2de032d78d01f3bd640d75"),
    field_element!("033705af636f06462c246d7b00a46a01f0972648657df19807f7283eb3b432c1"),
    field_element!("0379bd4d9842c4db69d26c986353f5b4d59f912793beab5cebb50537bf210c57"),
    field_element!("019580197cdab83f407746d5f8592a6a9a42ef4c81a0ae9137c52c622c72dbff"),
    field_element!("01c5212fff50ecb2634770718e1d0332c7a2219f8cbba14d4a5deefbd025d9a8"),
    field_element!("04a10fe8472f4c4a48eb30a26c8ed45016d5e53ec216da63adcfc868872e2c18"),
    field_element!("04eb2f6482f922e2d90c1d9df0495b6ac67342b40f7662e201ed717e9cfe8663"),
    field_element!("0014a50583be4526a55ab154d4dd463b26b55a6dc9d0973582e1470a8faecceb"),
    field_element!("04eae53193f3efb0c1e732f36548254e16bfb47ce75c1ef3bd553d44142f501e"),
    field_element!("03e0d6dc8f543b33592f54bf5a51f686f7877d76b73bb4b3879f04c3a2417c72"),
    field_element!("04226e3091c0d5c5d2c8db960ba8782498d04b8ea27b30d6d725d3b355318fb8"),
    field_element!("048cff91d2a6a98f17f5b0b6ed489c54fec59402060f4a34ea3662f7cffdefc1"),
    field_element!("038c49c598516fc4ad4b1ef0186b3cf4ac0d308430d2c322a2292d54d1eb062a"),
    field_element!("07b987f67d23e03deaa818bd32d9cdbe1c9edc753b1035bda12af7437655167e"),
    field_element!("0290d997f5b6010315904fb6c2d81d4dc2199b1223773b3883f5e432f408c81e"),
    field_element!("0162e24764c1bb4fa5f66497b13127d0d32c4fd609075b0b69447880fa0ece9e"),
    field_element!("01ce90d9d258b853aa76fff454f56074ea394c46a8d79e01cdacea7967569349"),
    field_element!("077714979e29bae677b2da9c2e63b4236cad9d99af0e3cb08b468a7ff7c8f104"),
    field_element!("00d576c461448ffbf726cffd161f363dd782e413eae04a909adda1889ee69310"),
    field_element!("07021f6387584c09bc8d160d43a7d1810c1def6f8c370efe53a56ea42e21775c"),
    field_element!("02bab5d534646f6abc1d682d25c6e96f6e368eeb799cee9f5460d8fbb6c7197f"),
    field_element!("0471be0aabd6943fc47bab955497890de301c7e9b7fb970da50a25b83103737a"),
    field_element!("044c8ee29adcb17e0b1e0a0bbdba7d91b12949549c8605a1666aefbca3eb9673"),
    field_element!("07146de2c6d21fa1984aa54cd94305fb8f8c07e25fec94c2688abb6df665cbe0"),
    field_element!("073f7523088bca4a78f4c3e4664e11fba5d0b30807fcd606a7e942345b6bb2db"),
    field_element!("051d08a3da7127d517bb964864741c853263aa4f2ae50999dd47afff767eecad"),
    field_element!("048a3888bade3bf6fe309eeadd59d13fe485d92f6a270231bc96e4ff47a1ec6c"),
    field_element!("02713b83c5fb47cab89498b1975ecdfe28459e2a407a3d3e7108cef62d80d3aa"),
    field_element!("058d0652bf1ae8c01be8800a8f5a5ad714824408b2e2a5cc7cf266914d01ca68"),
    field_element!("0321f127ae1c799cbd45908d7e40927769e6584c5880390f69db411bf1941d9b"),
    field_element!("008990a324b4aa1777db32d07a8cb4420686f4406fc5c4b904da0827987ae675"),
    field_element!("05f8bb2da7edd6778c634efd430375609fc21677576ba7b3323cabf42a9eabfc"),
    field_element!("03eed362c0589ded1a31fb70b7ef988183e5778acf2ade07b732419cdc700cbf"),
    field_element!("05bf5b8addcfadfb03f4da240252a2e58068049c1f4bdac5f7de62a6c241fa28"),
    field_element!("000e6b7c2641a87b807b69bff2098a6efe841d09c109655b0c5db00f7507f610"),
    field_element!("012d7607c7ac0177b6627824ecbed1d4775a6a65e263eb40b2b8addd9c64ceb6"),
    field_element!("07f656dcd2e01fe3a114dc8b6cab25f18c51ec1971b449ea4f0d2420285f2cf1"),
    field_element!("03dacae915c6927f251cd4cc42477027db4672aa799569c6a122402fc2a4d921"),
    field_element!("031330591c71f59dfcd89140f0b800a7f649f552847d460fe84091b46dc4dcc9"),
    field_element!("0571078f6a6ca11b649df1634820f755300443c13b3a0411d3fed348b5cf031e"),
    field_element!("02d53c5c812dfe422422fea8b0ea69c3309728b679ecbfe0b1f152c978534c60"),
    field_element!("0614efc0f3e08fec8cfcc355f3043fb6f411ee302da59f4b0255cde8332459a3"),
    field_element!("04c0e4bf73ab0e263a674b7cc5b047b006e9848bde82d80d6940a7d90c50cd94"),
    field_element!("05d9208830a3fc7a4f4f5ba158fe5ba6c3163f484a75b9a1f6da68363d5fb56b"),
    field_element!("05c90eb0374507d8056a89d6d8b3f2bd1bc3a7711682c99b0cae336ef8227cfc"),
    field_element!("037eed4d34f34cce326ec665e1339bf94af6be83e7f08427a3a0bcf63f24ffa6"),
    field_element!("03ea22d2c4cdf3f5c6ae30807a159859118d370290f47adcea9af1c782f23d48"),
    field_element!("01f36594c56eb1984a6886d572353647ea38fffb24c2427bfe3613a97ee78186"),
    field_element!("0530d74ed30307e48975a7da69e1961b7e70a89ebf8fae7973a15bfa1aa2fa9d"),
    field_element!("06e64c759432600e49647bf5bbde380ec3e2d7654f7a76e630d534e0ad0652dc"),
    field_element!("077c5784ee6f8c779f44bd1fdf3c82226d8b93fb12610442c94ede1afe0cdf26"),
    field_element!("012d4a9993345b954cfa1aca1a54ca48c20f363de9d8dfab902dafdbd53dec51"),
    field_element!("0214fabede1f1baddd115ba466b754de19d6545d93b14202a5357d4e8e3f0a37"),
    field_element!("01113bab27042129816436df1504a556979f0051f4a0ab7d20fe375190b7c57a"),
    field_element!("00b11e4d1e377d2035bf97766616d114c0c1d9e4b75348a887056cbc6ac2ba33"),
    field_element!("03a1815f7f083b90150c024c4c14060eafbb09dfd2b07bd768638595ed11763f"),
    field_element!("03904ba0a6e93fc1ddae983976083b7eddd4fcc42006f2a961ce96f24172e965"),
    field_element!("072e5d33fdad1ad92975875b68f8c319b726dacb3895d3ff802d01bb851eaf9a"),
    field_element!("02918550fa366b3b61b221cfd88f4938a11dd06f43972c884bb6df626b348cad"),
    field_element!("067b73f9860e3bac6da91abc690ba630645414dca40fdddc490e85ae198dfbe2"),
    field_element!("05f2dd5545e8a8e957303543db96c68f1e5ec954b51b5f9bb54ba2c885c5b9bd"),
    field_element!("0181ecb69d33a912e27d3a835ddc18e0f01aaf7209e7b99b029b0424fcca4165"),
    field_element!("017a3d1d0bcf12ac334083cc20f7c066314b928860720ad8fe48a6d1506f45ad"),
    field_element!("0204ee36896401ccf2f12d2d01e6196cd496af3a78947016852a4687b3992b34"),
    field_element!("05cf5bdb5ad5a669bdf206bc64efb9a9bfe706a783e7467795d6c08f062cd1d7"),
    field_element!("02e64cf74037cc255c580769cef1cee03b1a133ea2b30d3256611840553002b9"),
    field_element!("05d6edd0644bc12d45209b61afc2abec8316ee88b2131909b7638f04b1612d8d"),
    field_element!("02c6cf2cbf866d863133ac4f7c9097154a1c092c1d1777b025149e3dca2e8737"),
    field_element!("074cd6c88e9c81b83cfa84d8d6255a0ae200e09df06d9ffc94982137ef7b20a0"),
    field_element!("012583400d604ef0fef0359335e8ea5dad6f9ab9bf3162cc49ca62bf23c4af2e"),
    field_element!("05e678d7e625903ad8c32b5615c371b34fd5f40b262a5da3f4687514cd0eb163"),
    field_element!("03c37c0283e5616fa7dc664fe1efbd6a77eb7cd02722123371388712d27e717b"),
    field_element!("0487466d3255d2f5217c04d0ba64b568abc820e672acd2fce521210479d8232b"),
    field_element!("067de569ea4e70c9d281a7dd00e395f3d57a65fd57f57889991cc5d8060dfde4"),
    field_element!("007be7a90cd16138b5fe780c71b5564b800445b3a8fb1b813d6ed9c1ddf8594d"),
];

#[derive(Debug)]
pub struct Claim {
    before_x: FieldElement,
    before_y: FieldElement,
    after:    FieldElement,
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        // Seed
        let mut seed = self.before_x.as_montgomery().to_bytes_be().to_vec();
        seed.extend_from_slice(&self.before_y.as_montgomery().to_bytes_be());
        seed.extend_from_slice(&self.after.as_montgomery().to_bytes_be());

        // Constraint repetitions
        let trace_length = 256;
        let trace_generator = FieldElement::root(trace_length).unwrap();
        let g = Constant(trace_generator.clone());
        let on_row = |index| (X - g.pow(index)).inv();
        let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1);

        let periodic = |coefficients| {
            Polynomial(
                DensePolynomial::new(coefficients),
                Box::new(X.pow(trace_length / coefficients.len())),
            )
        };
        let mut k_coef = K_COEF.to_vec();
        k_coef.ifft();
        permute(&mut k_coef);
        let k_coef = periodic(&k_coef);

        let on_loop_rows = |length: usize| {
            (X.pow(trace_length / length)
                - Constant(trace_generator.pow((trace_length / length) * (trace_length - 1))))
                / (X.pow(trace_length) - 1)
        };

        let const_before_x = Constant(self.before_x.clone());
        let const_before_y = Constant(self.before_y.clone());
        let const_after = Constant(self.after.clone());

        let expressions = vec![
            ((Exp(Trace(0, 0).into(), 3)
                + Constant(3.into()) * Constant(Q) * Trace(0, 0) * Exp(Trace(1, 0).into(), 2)
                + k_coef)
                - Trace(0, 1))
                * on_loop_rows(128),
            (Constant(3.into()) * Exp(Trace(0, 0).into(), 2)
                + Constant(Q) * Exp(Trace(1, 0).into(), 3)
                - Trace(1, 1))
                * every_row(),
            // Boundary constraints
            (Trace(0, 0) - const_before_x.clone()) * on_row(0),
            Trace(1, 0) * on_row(0),
            (Trace(0, 0) - const_before_y.clone()) * on_row(128),
            (Trace(0, 0) - const_after.clone()) * on_row(255),
        ];

        let public = vec![&const_before_x, &const_before_y, &const_after];

        match autogen(
            trace_length,
            public.as_slice(),
            expressions.as_slice(),
            2,
            2,
        ) {
            Ok(()) => {}
            Err(error) => panic!("File io problem: {:?}", error),
        };
        Constraints::from_expressions((trace_length, 2), seed, expressions).unwrap()
    }
}

impl Provable<()> for Claim {
    fn trace(&self, _witness: ()) -> TraceTable {
        let mut trace = TraceTable::new(256, 2);

        let mut left = self.before_x.clone();
        let mut right = FieldElement::zero();

        for i in 0..128 {
            trace[(i, 0)] = left.clone();
            trace[(i, 1)] = right.clone();
            let new_left = (left.clone()).pow(3_usize)
                + FieldElement::from(3) * &Q * &left * (&right.square())
                + &K_COEF[i];
            let new_right = FieldElement::from(3) * (&left.square()) + &Q * (&right.pow(3_usize));
            left = new_left;
            right = new_right;
        }
        left = self.before_y.clone();
        let execution_increment = 128;

        for i in 0..128 {
            trace[(i + execution_increment, 0)] = left.clone();
            trace[(i + execution_increment, 1)] = right.clone();

            let new_left = (left.clone()).pow(3_usize)
                + FieldElement::from(3) * &Q * &left * (&right.square())
                + &K_COEF[i];
            let new_right = FieldElement::from(3) * (&left.square()) + &Q * (&right.pow(3_usize));
            left = new_left;
            right = new_right;
        }

        assert_eq!(trace[(255, 0)], self.after);
        trace
    }
}

fn mimc(x: &FieldElement, y: &FieldElement) -> FieldElement {
    let mut left = x.clone();
    let mut right = FieldElement::zero();
    for item in K_COEF.iter() {
        let new_left = (left.clone()).pow(3_usize)
            + FieldElement::from(3) * &Q * &left * (&right.square())
            + item;
        let new_right = FieldElement::from(3) * (&left.square()) + &Q * (&right.pow(3_usize));
        left = new_left;
        right = new_right;
    }
    left = y.clone();

    for item in K_COEF.iter().take(127) {
        let new_left = (left.clone()).pow(3_usize)
            + FieldElement::from(3) * &Q * &left * (&right.square())
            + item;
        let new_right = FieldElement::from(3) * (&left.square()) + &Q * (&right.pow(3_usize));
        left = new_left;
        right = new_right;
    }
    left
}

fn main() {
    let before_x =
        field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd827");
    let before_y =
        field_element!("00b74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
    let after = mimc(&before_x, &before_y);
    let start = Instant::now();
    let claim = Claim {
        before_x,
        before_y,
        after,
    };
    let proof = claim.prove(()).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", proof.as_bytes().len());
    claim.verify(&proof).unwrap();
}
