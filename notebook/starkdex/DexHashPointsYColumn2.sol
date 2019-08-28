/**
 *Submitted for verification at Etherscan.io on 2019-06-05
*/

/*
  Copyright 2019 StarkWare Industries Ltd.

  Licensed under the Apache License, Version 2.0 (the "License").
  You may not use this file except in compliance with the License.
  You may obtain a copy of the License at

  https://www.starkware.co/open-source-license/

  Unless required by applicable law or agreed to in writing,
  software distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions
  and limitations under the License.
*/

pragma solidity ^0.5.2;

contract DexHashPointsYColumn {
    function compute(uint256 x) external pure returns(uint256 result) {
        uint256 PRIME = 0x800000000000011000000000000000000000000000000000000000000000001;

        assembly {
            // Use Horner's method to compute f(x).
            // The idea is that
            //   a_0 + a_1 * x + a_2 * x^2 + ... + a_n * x^n =
            //   (...(((a_n * x) + a_{n-1}) * x + a_{n-2}) * x + ...) + a_0.
            // Consequently we need to do deg(f) horner iterations that consist of:
            //   1. Multiply the last result by x
            //   2. Add the next coefficient (starting from the highest coefficient)
            //
            //  We slightly diverge from the algorithm above by updating the result only once
            //  every 7 horner iterations.
            //  We do this because variable assignment in solidity's functional-style assembly results in
            //  a swap followed by a pop.
            //  7 is the highest batch we can do due to the 16 slots limit in evm.
            result :=
                add(0x25e821abc4d13c78a3e3e075441b14193eeb108c9c7ad5b054738e00ca94982, mulmod(
                add(0x1d78bd8d9dad4cf28ab619718a1fc0b142c255e4743fa4aca93ebceb175c558, mulmod(
                add(0x2d483a7c8f9e5cc051011a383b647b22dde4dbae4a1351f19a79181467eeaf, mulmod(
                add(0x5895f6474867251db688437df5ef8218113380215ff12bff67cef79cf1373ce, mulmod(
                add(0x1a3d46fb9e6221d060cbb2b1e906a14e4608f136c09a11f4285a21d83bcc44e, mulmod(
                add(0x2a241b307c9fbd9466e75216fdb579662f342395fafcb47525aa501ae8049f4, mulmod(
                add(0x59ea51797ec450219b5fdb65389d6264386a2f1f3506e936d872abf3d251338, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0xc63bcdada4d4776a5e669dcd188603193fa4b679bb0cdacc8e9c918099309a, mulmod(
                add(0x4d2723efda41938d42f247f766cbdd6d223fd07de22d7c07c61c08c39a1b1e1, mulmod(
                add(0x18d2ad85982ed7bd19a8b7233c2061f20dcaea2a73155a26e1177125a2cf753, mulmod(
                add(0x4de37dec345f501288924fbcf68d4d3fe81c7e1e52a1f5a6fa22de6ad5f7b57, mulmod(
                add(0x1abda5103d0d8161c09d7d36739b108f2b29ee3a34f67de8743db7f0435c0db, mulmod(
                add(0x3b477a5d622216eb5d5eece67ccc297d7dbe5aad4d3a9a96670343eaf73df8, mulmod(
                add(0x85b7d34c6075a57fc8ebb65731819b40f2e98ebed3608077ce26b966487f12, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1b38c0f2bfb0bf3ff03cc0b91ee28dd8b5e61d483fa3437f5f9ead37ff0f47, mulmod(
                add(0x126eaedc3a62527f218c32b78eb930af89fd51d7da246e7636eb121635cef58, mulmod(
                add(0x5a4db39f7e1804fb3774be9d7eb97765eba266414847b300bd40fd32860b854, mulmod(
                add(0x77beda03fb4ff7dff934689e151d0a3f9cb70ad66f5ac9ac814ba749dda7586, mulmod(
                add(0x6617f0dbdf773ab8c46f821444e4e7735f95cdb3b204e376e42b646c72c74a4, mulmod(
                add(0x3ee36d3f46c621b793841f837c386c206e352e92f7c26f1304651a211de19ea, mulmod(
                add(0x40a6204148a253399a27f8c0bbd90bcd1f8ebaf7431fc6115fce6079650f740, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x187271b3ce992d218c03c78776d3db9889a19341a8fc6533b21d480f813748c, mulmod(
                add(0x44a4fbada9267e78276df9e72a88eecfe56fce975081d524a6cea6b6750a612, mulmod(
                add(0x3147be887660a3dd68cb4c0f5e6b9065353326f5a2927df5b25ea8868a350fe, mulmod(
                add(0x3cff500fe2c199cac990b32e196601bfd18f2b67bbe4ffadd48f7851425144f, mulmod(
                add(0x60c78f28552cf8c934e3782aa82f848345c158f7cb64690a931ba497d1d75b2, mulmod(
                add(0x10652ebe209ee381ded4dbdb278148194fb98c0aeb8c7498d922eb9faab4620, mulmod(
                add(0x6ae9d1cc10ca9bb11828c5a565ef3014dcfebb677b13cf58baa87cc337c3c09, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3da0803a0e6c82a0d18e0ee6b5727e9895d948409f08faccccff26639b3e57, mulmod(
                add(0x778110c1ca4d923de6ed7c208b368bff2e87ea58c8c21c56b7077363e0cb96, mulmod(
                add(0x5f8aa6f0bf550f6b56d97e5b7e271b83428245eb1ff6f0d8d61f6d0cfda9e7f, mulmod(
                add(0x586c98cf0cff41d146b70df59e589d1cc84e4a607aefdc37319810f453ba18e, mulmod(
                add(0x34b286a7029c7ad85e98dafae5c03d05df6abe760171558eee36b604dff9328, mulmod(
                add(0x68a87454669d37545e13038da056db5630d211d75308dfd2c301a3a5c4c34ac, mulmod(
                add(0x3115d8823c33dac6bd75accb1d114a6d6f7025e9f5417b8a4dc60f7748ad2e, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3d516480716d3f1fc4327f895b1a14aee06072f83356c54db3b2193bfc254a5, mulmod(
                add(0x5b71eecb5e341c6d9d371281d3082509245690dfe3c3a991fcdc8d590233c12, mulmod(
                add(0x4f4782a1c3e4d53ee0a76905b0ef0bf387b81ee3984d22cef85d77b96435576, mulmod(
                add(0x5d2c7607f5231fd406def1a709d1b79699746069f86be5416a8c1ca93d82c55, mulmod(
                add(0x63a950bdd816b7617da85e6a6704d4697105089dce73128156102d22958894d, mulmod(
                add(0x28282469d726000bdd0fecfa15d90218c8b73c281d80d91aa2d49b5ab9d37d5, mulmod(
                add(0x55c338ebfc781d1d2358c3644e73b37d16686f4246024d6f9d7915863e179cd, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x774a4fbd908a12cd4e551e3cbcd32ee5c49887af3be224f4bb0906320e9a3bb, mulmod(
                add(0x7bedc46534679f358b80dff5d5edf4a024fc53dca889ec9513bc3318ea80121, mulmod(
                add(0x2c33dc1aa6a727feb9da02284896846fd0464720a754f226220eb07952948a3, mulmod(
                add(0x631bee8ff4ded42de0f628e5971eaf3566d83e52865f19044a5038d04c1a307, mulmod(
                add(0x574574def1e8dfc18dda711b2b8651d746c83eb57661ed44a2953a83f10242d, mulmod(
                add(0xcf539cc14b70083b31fe06a85b4a051a7e080a49774085d8c13f5e1ae03d8, mulmod(
                add(0x2e9b4732b84a54d1b8201a60380f0caa6ea7af637fded804705625de436b7, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6b90cb307cd524b6a411573b6f605e660637800ac54b748fbc2e8664998c68e, mulmod(
                add(0x40e08aecbd35517d2c69c56323945b9ffb7155eca73daa5df1aaa3e588a6689, mulmod(
                add(0x324fe07fa558390384a89780d91147c09ab70ac957acc4baacbce71fbc44d51, mulmod(
                add(0x64492a733ff0b04028d495eba107b7c78e746c1a88f19b3d7da0cdb12ce5bc6, mulmod(
                add(0x20d2001293a536a7e3e002b82c05c7a490abe5d016dac4c099b6888577e3417, mulmod(
                add(0x45e0da5a22fba24ea4cfcb27d64b29b8dde5238c98be876e71a607b2325c282, mulmod(
                add(0xedf8eb64537718e456f627da5494b9232d3190a560a1a9110f335fbe97782e, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x741cca460b2c0aa6b3ea1f458e137dd6d89783dc3e7a3eb65f107a9370c8966, mulmod(
                add(0x1a66e792e85fefc5cd23385942edae8103c76ecdc34597d5fc8432fa2327b49, mulmod(
                add(0x7419a6452e249ec62924e298f62dc825da123a26c6da9cd7f2bb7f3ab3f8d6a, mulmod(
                add(0x10febfa1210f533351e2733902bfca6fa8df4ef9d382a583e80760913ca324, mulmod(
                add(0x5389a18bc7c66a95f10250b6606c75ed086c92d6d565d0985d61c10ceef2b9f, mulmod(
                add(0x7fe69128aded85482353cde52ab8194c5ce36f579fa53e5e0ceb953ce0224b4, mulmod(
                add(0x35bc2bb6e97cf013ea482b109daac4a77f49095bd1e98d62d52da0d4a8a92e1, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x39eb2d3046a35d7a8b88e8f0f9a2478134ecfb08da3bb24c9ec9dc3f6588fd1, mulmod(
                add(0x2abe1bfe25bec0e411d6824f9003337d1f52b4457c20bc1be148900ae21049e, mulmod(
                add(0x5791a3220af43e7743c501c52e15aa1acbdd1b922e82916ea5f36c937b487e4, mulmod(
                add(0x167d9617921156ca5aa768939b6ccadf5ad3f8481b269993df0f16029bd5453, mulmod(
                add(0x4525487542f1cdd795cc98ba3ac6000ddb37665532dd17a6151c8a04afca6b3, mulmod(
                add(0xa218542fc1c6b9adba18fa6c55d0957141f892244d91c6b562a78176ca8a86, mulmod(
                add(0x31bf91963cbe8bc85fca7082e012af9c4f0849b43d5ba1fdc36e5ce19f27d70, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0xc8c86fcb9c9c4a0e60c994eb35cdb3aaba3f542c1e3043d25a1fd3ef9855ab, mulmod(
                add(0x6a19305a4f4a6294a289238eaae25e7467af0ccebb08eac801191b45b0764ef, mulmod(
                add(0x5940232ee43c5bc47073002eb7dd3812e2c65510b11e17e7c102139794e8598, mulmod(
                add(0xa3cd62305cbb505820c7a76f2f1555a0867fe242f99d2aa647ed0435000cdf, mulmod(
                add(0x2f1a9c810135fdf41d7a30b38a0ad74153a5e188ee84c4a45370593b2d8b6fa, mulmod(
                add(0x60ae38bcb4f2d5d114b7b7e005026e4f93997259da8c84c8fbd3f72879219b4, mulmod(
                add(0x19f33f8958a533d90b77962dedeea3b967f4390aa192247888954c7da8777b9, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6110645f5154526050faf46fde8708db0355cccdfeca01abb585a52f9d21257, mulmod(
                add(0x5326c11b3e44d6de7c8109de83dcbf5ee8c114fe850c9c1a2e2a0b40f5d944c, mulmod(
                add(0x719d7326ba4c92b4a6031b8725cc4259fc4c2a92a3d01f439c5ac2a4f013a0a, mulmod(
                add(0x6b2a742e325cd546f006458d4f38ae148d9fe08fa680cd5efd71ba1d77fac9a, mulmod(
                add(0x7745bdf2fa40807aae927d67f994fcef7b24335b752f26c2e3081e6fb73cf90, mulmod(
                add(0x447e937363b52af558471295627685a4c8178d593796993ab1b24ff40210fdb, mulmod(
                add(0x5dd8fbd123eaf04dd098f596905f1148e5d391b7b40f789842b63358b7bf23b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x4d4d9ec8348554db8122c3a39c5e1ab23a9f051df1faffcff4fee657636350c, mulmod(
                add(0x75cc138867fd6b37de51b7ab9d316001fd51a17f47b599dcf1b27c60095c68d, mulmod(
                add(0x75f619de07545bf2b79bde841c8aa02ffa141b2da8a299162dfe10eecb5af05, mulmod(
                add(0x45bd2ed72b8219d2118fa6780dbeb8b8d38b2101f70e45bc4d4f72c1610e1a5, mulmod(
                add(0x394c2badbe1e7bf905bb1124ff558a32928dbca8ce2fff57dc921ff2cc17bdc, mulmod(
                add(0x1d7863f46423a5fbd6aac54c10b1cabaed5949d5e4f6b93f91b4dddc0c9bffa, mulmod(
                add(0x38265ac2fe188273fa718c8da6a7a82045a8ef3c44cfc2878d65858d1572e1, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x134c9d204db6ee4ea5b4c36064704aca6a4cc6e72e84844525be328a55386c7, mulmod(
                add(0x6b0437919daed92c8f8c46dbfd5e96faac3dc5d342bdff950a8cab624496702, mulmod(
                add(0x3ea128e27c39fee4a52317a3b0da3e73b58656a79c5dd2d5d0a442208f2d0ca, mulmod(
                add(0x341c2626979ea065a7b34238dbb4d4ec1276b87445c4220710512a9f3af884d, mulmod(
                add(0x73afd1e9c05e56655a888a86cb34461b1c6ee779711186e6ea3e718172f871, mulmod(
                add(0x562b22cb0199bd8dcea0307c6874fcb5bee273434140d2711d262ee9d4c925a, mulmod(
                add(0x306591b80718d2b84e4d7c27fcea72c84f524fbff6cc664049dc12b8763ca4c, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2d166f7def8895e0801c8eb245686adb96b7dd9bf6a19ea33361a56b4965a3c, mulmod(
                add(0x38204649eaeec60de8424f8fb42f4307ac128fd68bf621e72fd0a599ed5917d, mulmod(
                add(0x2899b91f37cdf2399fbd47d3594451736ab4a397b2cbba34059b873c6e27349, mulmod(
                add(0x33110b6d398da00c70a35e768e3efedebf59347a0f5bfccfd50062acf23da32, mulmod(
                add(0x4f22fee3d25fa5b7f3f9091a858c66b4dc11acac0831415976f078e9bda877, mulmod(
                add(0x5c55904ddaad8849ea501a58bce55dfc12f62c886426d6f3994952060a16e7d, mulmod(
                add(0x3e39f136e07a8fbbde01e4f3c1f471a19c6b0b62601ee0dfc9769a9b8c475cf, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3b1eab017979a289c2e0a67171fb462f565ca1df48f133192e776317f47429a, mulmod(
                add(0x2a8dd5aa9e318fd44416649494759ae4b5ef0a4bf1536adb2a63f305808391c, mulmod(
                add(0x7d7ddcec0839a7b34c9e16fbe8349b6c41f3a148fdc540efcf767e3971fb3b4, mulmod(
                add(0x24ddba24c682f266ab56b140f5882dfa20d131569981019f53555ad36dcee7b, mulmod(
                add(0x1526d55fe47a90819d43a2f55db7e2014d0a62428302eb6d544aea21f48b143, mulmod(
                add(0x5faa621580b848a6ef2a8b03fb76e82f145d4eb18e5d495ce6515da09b28a62, mulmod(
                add(0x116bb72b6e437517d9c92c9c67e839d5dabb7fe6c3957652a8718431cfa809, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x22eabc3c60595f7d909850ac5e09faee6b926b6d0e6ef95d4d249381eb34dfe, mulmod(
                add(0xf433c9f822e4b7dcc5e2f00d83c612497e2f5fdd821f7e70bae634a80b00e9, mulmod(
                add(0x176bd6fea2b0f8aef9324cd870db87b2e0da6ab1cafecd1ed6a7c13d4debec5, mulmod(
                add(0xbbf4816f6f2ab45a63229e67f9d460eb71c3f3eb78faecf0a43eeb7718bac3, mulmod(
                add(0x1a2c03ff49c6d686a898dcb2f5cd6ab9e380910b984363cb7c292fc91f634d5, mulmod(
                add(0x287e961765cb139a6125b0d3b7e61d2e632419b04d0aefb368cab226b115db8, mulmod(
                add(0x6fff8cbcc955342fb8da3eeb461fc62fe6521472273ab3b850a19f43b220d2, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x207eff9411cc4948c18e5cddf7f23e0c7c00b716eb2b8555719deed13aa3a3f, mulmod(
                add(0x17f5b035a475b833c1a2399209741495125f2f4fec22d265ff5d3ec593a8129, mulmod(
                add(0x3fd0f4c1748e274cc92e5834dab0b53f504c51f63bac7c8c5b43d2ba0a72aa8, mulmod(
                add(0x4cf712eccc54b4d8ea847668c1c85bb444c602e9fe6b6c5e942871f3dbfa0e7, mulmod(
                add(0x3b82383db554d160a88c68eafe4a1e0e3bad92efd6390a2076f371ba2fc4b11, mulmod(
                add(0x7cb99219fffa2e6ee3eabe6aa901d365d0de143dd6d5e18d40c42ebb26a22c1, mulmod(
                add(0x3a9fe4b13b8756fc49b341cccda7cc90d8feb26e71172888ff07a6916351eae, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5fadf298cf37e2d7ee527698b456843d36c157319fc5388418306ede881d4d7, mulmod(
                add(0x3082f0cf11f8f9bf5676fe06c362d563726a362da54645764d10b0d765c973a, mulmod(
                add(0x727d4dcdd3638fdeeebcb3415d165352ce15afd843a1023d32ca86eb8ab258c, mulmod(
                add(0x5809f7d5f0d027944f06ff5a8a7fffd1e439e1dced3622270d859a28cea0f5c, mulmod(
                add(0x5bedff3288a72a92eddcb231bef8483f086cfb55dda51ea575925f86e42b161, mulmod(
                add(0x6d9d10eacbb505b627e7e9fd7b74f92d2d1c55dae8bcafc894214eb3cf6c79a, mulmod(
                add(0x68d026cf8d74eb7f0bdbd4636cb32deecc4d8f985d8453cc0d44f17e5e8aaa, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x46440da6b3c5b71c74a66e1ea5484e1283b2b7f8ef5d8c34d4b23974834511f, mulmod(
                add(0x68e4df6ef0f15386369425e47408ea9327a5605d4bc2c3127a8033c4bf0ebf4, mulmod(
                add(0x343100baa89045cb5180c448de753fcc3bb674efe44aba424ea77000b3024dc, mulmod(
                add(0x4eca8a311e3e118b081ec729151fcf52fe56cdf8fb8522a265bdd6c80899324, mulmod(
                add(0x255e24d58a63c370e242d027f530a6164232acb385dc60184d3b5c265a4e17f, mulmod(
                add(0x2eb938f2bdbc31ce6e12dce219116f3694eb4b0413951ed4026ef9489f3088d, mulmod(
                add(0x400646ba1e3876a0ad65a600818a3bdd95591d77525d5fe19471c5e9ec968c2, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x298fa278ccdef71e503174f58448a8d1585d9b4ac673e27489ace57713f7dc3, mulmod(
                add(0x374dda1b2777c256ff1c94e516122ee490783a9bb629a39b80b9c4e8dc723df, mulmod(
                add(0x7df637038d1ac574a25c0318b57b1e12d9949e98b5536fe0d65cf671017032d, mulmod(
                add(0x4f7b56fd52ded4eca31b6385b01f7067723933698e8b85f775b818b353dbd03, mulmod(
                add(0x5398d5350bfcea9534495e494844bee03c79a70fafd2826d671ff0111282a8a, mulmod(
                add(0x3b26fdb1da8f86f53c24b3508d0b7093271a6863b4b17a0098e27de5d956a66, mulmod(
                add(0x66abf6d1bae4e41a934dfbf1cdd74c29b7414df20c68b4821b7f274aebeace, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x71223e8e9c4ec1c7dcee1d99d10b6e7f9c254bb875da689f1a2b0473f53a6e8, mulmod(
                add(0x27cec2c64f30d88504976396aa67ccd07cd14468e487578acf06232f03d7c54, mulmod(
                add(0x2a720b910575bc31beb02dab295630f8b1de036efd915faacd2a74a609c6a18, mulmod(
                add(0x44ba500a616f51abb81291e7cbfb40d08b37970fd2955c0daacdcc6297a2498, mulmod(
                add(0x5d57a9d6662a4c7002c4b5a6da0fb4e1805bc1f9be03db544bba13f89e5900e, mulmod(
                add(0x1d6b178a0587ede44d4169fc994d06f497adb564a8f0c344a30d9732a7a8d45, mulmod(
                add(0x7ad0462c39304557f87373c775f4c4e900cbc253e355f14af00fb251023e177, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6011bb818407ba18186f7604e1dd546c33dc0907edae91a34168ea25686d379, mulmod(
                add(0x303d7e68947fbfabd8020fffc379a7d9ad120de283e048e8cbd98227492bcf5, mulmod(
                add(0xe39edd7a1da6844ab13e09f8830cf1f19a19dabc7355caa019744c0e823e4b, mulmod(
                add(0x16d478f0d850321787a4372f29a34b906c6cdaab54e318164d282ebcf24b3d5, mulmod(
                add(0x325f9b6d00660bf391a41a14e60cda2929db815cc21dd4094d736cb883f7ace, mulmod(
                add(0x638e11d84916d90565be5e83d405f60f6fb830d37c2b34c9ceec41901b9db9b, mulmod(
                add(0x6bc79acca81d4cbed9d89750a1fcb9f6fd1fb21d0d563035a3e3d292ed71434, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3a99851827d40c48acbc27ea4639e8a17c7f4fb055e8a3744064bdac0bfef9d, mulmod(
                add(0x56f06ce8a2583f46d1979896209fd454fcaa597eb0152a8a8b001ce05560657, mulmod(
                add(0x38ab168edce11bd59a885b7f048bfd58a92196deb09c11a795caa3c92b716d4, mulmod(
                add(0x5b80c0cb41695f7140fc8adf1439afa99280285613131982ff7d05cd62bb358, mulmod(
                add(0x172af342ee499c5ccc12868a2f5638df22cc5e39094fecea8b0e907594c1b6b, mulmod(
                add(0x7a2849313d56b7a318fd6c931d90153b04aa4d6ebfe3e774da10d85c5588df8, mulmod(
                add(0x22224da4c821724f23e7c4370a7e9ec6b7c700bd99ec6b44f82a2d2e3717d8d, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x66d649708f19189f06b3914e9ad35ed852c1b61925c2fd988bdf47c076d2921, mulmod(
                add(0x12f19e50b17d19a2e6d8fefc107035ffacec5192f62681733f46067a3f58ad1, mulmod(
                add(0x5946625cb871fb0bec1f877ee09b7ae46c3a3bb77675b9aa5ea2e7981904988, mulmod(
                add(0x838b2838a542527499c05e7bf94ec2610f9d1185946d985dc75095605535b7, mulmod(
                add(0x75fdf6b45da97d8f4d2bcb876b1829378a9d0bd14d3142bc8050239168bae37, mulmod(
                add(0x593c6a79cc100ffb98d794d50ccc66dc37914b654692e546bed96fc26fde45c, mulmod(
                add(0x2b7a830b79011b4a44bd973afd316f9ce35d0e8a6646927a92870442486b41c, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3c22c8ac9dc656edad1278c3b25cc6a1fe4680e9e30da7c288937dcadee53c8, mulmod(
                add(0xf8e4a97be9a4e6cb5f006fd451d036bcabdfb3587d495098b2b36b6355258f, mulmod(
                add(0x2d0867a5c42af4d3c3045beae878ea2f7cdf44bd1300ab59c37bbe6690b33c1, mulmod(
                add(0x1c1ad85b9804d54169fc51bf57bbc3946f3f83d101d9fe8e8db17b83647305, mulmod(
                add(0x2fdb2e3eb61c9abf48cbf123eb3fdf9f561ec68eb8c5266a4e3fa96919c81ca, mulmod(
                add(0x5da39948464316131827187fcb05fe55432973a3695cdeb974ff5a757fbf618, mulmod(
                add(0x2a5af7e43c086422f39737d05cc3f0f574ff1d043d2ba1d4b095606f56849ae, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x59019aa1ed67fd4b06b064c5add84282f09ceb34ea512b920c4d1c84c16d57f, mulmod(
                add(0x4b0df49bea6fd1bb73ac3d4671a185b7678299a65192d7f8cee1f5539eb0760, mulmod(
                add(0xbcba6a9485c671f9481702fb8367df9ce75ab451141b0d7c8699661e5d54ff, mulmod(
                add(0x31d575fe6faca411b7654115fe8276b0c3758a31f09fd3ecd8f998f91f989ae, mulmod(
                add(0x6e759d77e6febd27f205452db1db20058ac4ebc3850f829132956d5cde4cd9c, mulmod(
                add(0x737e9e0d6a380b7c9e07cee1fdd580ab7bd083f3902bfd289ac1440355f8bf4, mulmod(
                add(0x381a22e509c639ef8975cc17835dd99a6fc9b644b9414b192b3aa079244b5ef, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x32c9434e8601ba7a626cc6f6b4c2f68c7d03b28b5f8b1150782c26e6b34f6e1, mulmod(
                add(0x7629d54edcdb3bc3ebfc87311cd5cea20c825fbf8c7b97d3048c7c881528b9f, mulmod(
                add(0xd866dca3034cb0bccb387c5682fc86c409b29470227b76d4b1c8024a320bae, mulmod(
                add(0x6ae356234d04e457bd84bdaef3913e1be5c6d9672db9d75241f509a4c5b3941, mulmod(
                add(0x6cb185662199038112b31631140e09e560b5964f21186a2b6c4e3ca21a1c7eb, mulmod(
                add(0x3f0e64c3b40ab89da42822e0c20e62089a1d3fcdf7204ea1b0c99ac05f9106d, mulmod(
                add(0x4a740c39ad64bbf2558ef81e11e75f5c10c4cd8ce51f2e0c8ca4be01e36c2f1, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x285e65d615b6aa5207b5fc3d5cc873f65e5ed09f6a68148e28562f41a58d682, mulmod(
                add(0x7994066d529ffe46da6c6ffdf73bbe64b80b13af6c4edd130fed0f01171858f, mulmod(
                add(0x582a3400db04a50580980a1f05e35fd37fbf54aae7f2d7cd1e47c2392066d46, mulmod(
                add(0x155672d408bddd4b78cb7adb1fa6770238b07b1a55e61cd244feba717a9a6b8, mulmod(
                add(0x37cea5a8aa6e51ee24b6ce1cfed6974dc8b90ab2a6320d77672f483dcda5620, mulmod(
                add(0xf30e15991234811481712215f37fbae8884f5fa2a5f13547107510213b1961, mulmod(
                add(0x7a99fca4e86983e301c4b3389eb9fb637377e974d91383f7173aea457b1a327, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3387903993b7d1e3b1bd162788988c95b6c0af90234371526c2cc0c4f49f712, mulmod(
                add(0x67596279ce3340433f42742e31db58443fa90466b9eb3a4de21925a479dc5bc, mulmod(
                add(0x7c59a7bf45ce826df766f6b8b8a617948561b71656f9ad29b3e47f9f9c071de, mulmod(
                add(0x39eac758c69956526f6545ead4543da0b1d533a2abb1694ebca94b66e5df05a, mulmod(
                add(0x3d1632903d4dd5d736210cc80cbcbdfd3db6d8075a3737149ea3d92c32791ef, mulmod(
                add(0x35427d0f45766b76e58448a42bf507a632984ca6e8655a037c444098bbad7bf, mulmod(
                add(0x4ad9cac6f6b243b78ffbd3505cc0491f03cab63ee7493856bf7ec16c1b215c, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2be7f062a008a0c08c7069c68a4cacbc3a02c0fd989e87375b45b62ec5a8c6c, mulmod(
                add(0x1011c6c5634a67b6faed77cb29be9ee57fd41962892040da82c1288dc0be9a3, mulmod(
                add(0x92f15efeaa5a51880aaa8ba36609ea78978f2542d2f0b3a619c88401b6025, mulmod(
                add(0x618e5af136b45fbde11125290fbec78ef1c930cff5fc96a18c82556ae07c27a, mulmod(
                add(0x2122a17024c104f0bc5b759cb3624ae7f42f1d25dcf1419b6f6c318d1c04d30, mulmod(
                add(0x79d5f4ec8c3e5fb2d34578b99b2137267ecfa224f14d686fcfedb119cffb885, mulmod(
                add(0x463f881e91bd1fc4aca363540536d521a8538b2cf37ffbac9d1d3fd950384b0, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x69d6e2a84380006713cd0e57bb6e50e5acd8e9e21794aeda5544ba8d6356309, mulmod(
                add(0x70a54bbf268f2f6453f21997bbed131c771c150eec1cfee8f96167f342ce346, mulmod(
                add(0x5f363d315f0be2010b683b33985ec4f0e8a4ad2b274c5bf36e7a421d78cf1f3, mulmod(
                add(0x69c81150353adc132d055538b122aadb41b773f8eb5f52b9eebc30216c4c1f8, mulmod(
                add(0x24248d9626c55acfca1f88498d583166766c5085b7d46eb22f0648023158b6b, mulmod(
                add(0x66013ea79ad1cfc06b1b1b6c8db98c7abf78ab98f3e021e15f63cf26c3d88d8, mulmod(
                add(0x2a850b5fb21191b50fb1d901a4ce64572e5f310ee4b760dc46380928d03e06, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6602f0dfdb9382899fb7a6e8410dcf8422414703dfb3e4b9ea586cc97e28514, mulmod(
                add(0x3ee154201ad391bac96e2a7faf539c8dadb215bc6aae9ddf4a55ddc7d9c18a2, mulmod(
                add(0x4a090aecb6b4e4c435f02a63b8cfddbba2a3e3026ba44cc4a30e9b5c20de7e3, mulmod(
                add(0x230830a7b34e94c89de44af644012e6614e51feb83e868027d67bc53777c0f0, mulmod(
                add(0x2a94f9bef48cfbed7407f207a14ab27eaef8aa586e9d27935b8ae73fee8ded5, mulmod(
                add(0x2a49b9a4bc1f6a04dba55187f29e35f94551993e26b6bdc277e50ffbf37024c, mulmod(
                add(0x54ee745ff1f450daa8249b0b8cece5b96c4939e84bd5c1a2317ebaf91ff6cd3, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0xc0628946846506132bedd8d36caad0916eff3ef77960ca45e14b4d73d8269b, mulmod(
                add(0x68a00d038e888267e3c2d1cdd4c6cb34a169b0d44bf27abe7f35b207d72451a, mulmod(
                add(0x4c9ae662f9ec9b0ceb914936bef4a3a56196bfdfadb72f075161d9ed3a7e9a, mulmod(
                add(0x510b70db76f9051cd52679358ff70e6688865cf8717afc8d2803ab55e483e5d, mulmod(
                add(0xfb0c5740f3a0454bfa5bcf31de724471fa98a213db4145ec454feb1fdaedd6, mulmod(
                add(0x3c6822480da70c99baddcb0eeb8196bc04b2d5bbd7aeefa3ff07306daea3562, mulmod(
                add(0x4c05859038672457ffdd4bb184339238e9e090446083aa07241e21f0dbf7296, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x40f08146a2e2ea5c4d0592a049f7414033843ea64f2bd8e371dc3ccf094885d, mulmod(
                add(0x3620a0a3b5476e3f1c60dd0fed1eeaec7c86af383afa7eda2a489d553856b7d, mulmod(
                add(0x5fc91d54c56e8f1f80dfd9a7bdaa45100c61aeb6c4953f96bbefedb0d35269f, mulmod(
                add(0x19f3aff31ff83c86787cf450198c3b2707f9101d003f0581ddeddc93a2b8286, mulmod(
                add(0x654fc6716b406a3cf240cb0e7558c1842c766ccdf6ad785b44867f2dd66554f, mulmod(
                add(0x5506dd2eb72dfc9a29257e8f48a74f021d9185de4d4e12c01715d06d0d598b9, mulmod(
                add(0x37ccbe51eca68b9b2f01232e1f77c76390c27b3ca0c763698371c1a2045dc93, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x89d9c19dab73b6f77714ea1bd4d5f7263d44bf7848373ce3c122a09be8af4e, mulmod(
                add(0x5fdd225fee4c8cf47bd729eaecdc9e9241f039b5f01975d0c67f2bbe57c2189, mulmod(
                add(0x14953553b39eb928d601769c0719cf1728b7d03fb2cda10e4b857d921540e3d, mulmod(
                add(0x764454eaf57b849a7ea880bc92f0815cd5b4c07b25afba4f630f5c421790006, mulmod(
                add(0x171a8095b719b3b1d2716a2a1da52690282cfcaa975a895bf7c6b6ca3b3515a, mulmod(
                add(0x15b438c9f5ac374b3be38ce9fee04117e067d3ccb638a379529c95fe58db4fa, mulmod(
                add(0x10fe9e9740f2d8c6a30d627936036fd96fc5e78ece005ff35666fe8ac5df315, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1331d68379246c96ca9a3c64a28921a9811b3f51d26867d14fd8f20f926c2f7, mulmod(
                add(0x1fbcdc420b6d3161e09d6d641006c6b304ab68c4471a45a3440c1843ecb0794, mulmod(
                add(0x2a55b6bc673786479e75cb731ceb4ebb427b1b46e4c261790e461e2ac14a36f, mulmod(
                add(0x3830fc25e4f9917d737ad6758afbded204672dcedbe396bea2d13636673e1ec, mulmod(
                add(0x7d74ec6fa0dc34c911fd197bbaf8cc3efe1dd3fd4094817e90695dd547c7c55, mulmod(
                add(0x281023d5f3a2d094ae210dbd59a2715d5c1d033496fec8b10657bd066e61c85, mulmod(
                add(0x30621045bd5e2bfcf3caf16511683cc7cef38436c31487f0992ad1009b2e60b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5f7221cdfd25c87d541a317add39231df666facf9cd4a81e696add02735a48f, mulmod(
                add(0x103271c97d13aef79fd5f7c086601ab027dc4a5258365d850e9eef6ece45dfb, mulmod(
                add(0x3c21c05262e2113f3511f1145bb088c5cba1bca7977994f5f16ff1929a429f, mulmod(
                add(0xdd8eaee404421f8706e51f114b91040020d36cf55272a261e559c737d921b6, mulmod(
                add(0x1e497e07b8f3b8286c981c3a556f5db23340fe6c9b34b90c6d766232c6b1bff, mulmod(
                add(0x4de19fd9a6ed8ad0e5f19ac23cacac5d7e16284816a8cc077c0a246324bc9ed, mulmod(
                add(0xd36ca5ba510fdbc5ae0c75ec9783e4ec153345bc7e1104cb8b75dc44fe5285, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3357b27167dd1aaad1ba1b44539088ab0c336be3472775990fa1221642a6d32, mulmod(
                add(0x439a58506e17b3c71e3a2b054916f94ac7596a11710f13b0b68128e7722b16c, mulmod(
                add(0x33f3371da994b5cb59e98301b388a4c0e4082431001794c00af87b1e9f61c93, mulmod(
                add(0x5e584177ae3aa2a597100bc3b23810b476e7ae25c5b1775130fe12bb5de8bb9, mulmod(
                add(0x55a5a8af734839035d9016919cea1830bcc7c4e5ba58b82870e7c77b70fae48, mulmod(
                add(0x6b06998e1afe42d75df99d30cecd83bfd5c97df836e168008370509147c8026, mulmod(
                add(0x388976e3c306f572105c27e5180aeb9e128e0b10a3dfb43b6a337aa2424223b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x7c178383c1f57fef0110e090a66cd4c8e2568570c6e766ad437777e70e6963c, mulmod(
                add(0x2f7b7371190e6afee5dc0c39c39fb62854cc2d66b26ad60a43c42d81d97973a, mulmod(
                add(0x661f88e3d888ea6e1e8548ba74d6e48de11bdb8027f936c6f9867e31206d6d9, mulmod(
                add(0x7f2bca938ef75ede5d8a310694fd9a187d737f04c25fa5899d79b25c3328148, mulmod(
                add(0x1650f32ab26d0e427d4c45d9403437e2b28980602f07936db96af1d1794de82, mulmod(
                add(0x52b5716b67341ae75b334d06e3507727c7d920eca2e6c9210d27b2aa29a990c, mulmod(
                add(0x5b33b471588817a26337caa6294689a138a62110dd7ec5a55dd6722a98ca0a, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6023b03f7e4f862c901a8e236053fdc343f689f682bdb3cba35dc086926de14, mulmod(
                add(0xd847b64c0c05ca9038ac4b16cdf348742f93f4556f15f83c5cca30f8afe8f3, mulmod(
                add(0x88cd6332fe14e3e6a77f8a107c33a14efff3a4102c6661f8a46225da7a499f, mulmod(
                add(0x6b0a4d7190e17e887b89798ca7f9452099acdd1f07f1900249438a1e21cc585, mulmod(
                add(0x2aaad0b840a0e15009aa132a4af0ba89cee65c1f8a448f3aa9be4ffd477ee65, mulmod(
                add(0x16e613a31eba906f601de23768d95b679a159c6d349e2b7f5d77c8e65071b7f, mulmod(
                add(0x727fe86926d49e01f25ed4ef4e45ecb045a4813d36ddd9d31337adf776aa73a, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x638f2bfbf6d5296e4405297e679f2440913956997bd117b6921f53a956e3522, mulmod(
                add(0x70f5f0c8568902d9e86b82defc63e91ebe916262d9b4641eb261e93ad641975, mulmod(
                add(0x31a1f6bc57722b1d9a599a7f10093f316efe6bb50ad07908992e13e73de3f0e, mulmod(
                add(0x7e7357ce6c1cde8aaf7288dc9b43117378b3d6a896dee26e579f5c34c31aaa8, mulmod(
                add(0x173a92729c6f7ee8e2da9e396fb40985ce6d6a1695a40afa5fb51b6828c690, mulmod(
                add(0x9f671156597cb08a931504c5250f51c883b733e68d6c0576997ebed3bcf00b, mulmod(
                add(0x5a0ec7764504cee2a3339b6d3baed18dad06470a3a464b8a02301cdcd4e28a8, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x4ff48d26d7e1bd56a499698c94ba4012c40f6d60331532303b5510a54ae46d5, mulmod(
                add(0x2bbbeac7ed248b78310fe5f60a715864e70bba1904a23a21e98ab34c430880, mulmod(
                add(0x7bb76215ba5ec8bf6c98bbe91c425dc55d0758f20ba0ccb89da0a3038444fa0, mulmod(
                add(0x502ebafd6bf7b0a73f8563eecfbc9bfb4859dfa590f836816fabd7043f26b19, mulmod(
                add(0x20c14985087a142f468a04739d01a6fe1ad1f5bf211075f45603431afb967de, mulmod(
                add(0x441fc489a6bd57b232d9953a62d649274587a745bed11b2082737ad0e10f3b9, mulmod(
                add(0x5ee32793482459108c2a751304e8bbf6e742d3cfd2b735f89f71912ff5ce06a, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5e9dc50bf23f623e0f7c055e26f641fdf6bc7380fb3d15e2f9e12740a05f70e, mulmod(
                add(0xa63af2da45ade13dd36cfe742250f6b5ace42fe0ada1e197cdea978d450cf6, mulmod(
                add(0x51291cd239a83707c1d60ebf662c13f3f06848c2f86d4284c6a429f945325b4, mulmod(
                add(0x5b90772e0863a9e215798a52b6875e84b409bc8d519681f6d30f7c69061fd40, mulmod(
                add(0x788cdc8e8f16faa802521019b450f4cf1f777e1ed62886f0a8084cfb63cf22f, mulmod(
                add(0x6e6ff65a6f51682a916121e8762ef43820fd6cb12ba85a863ba3c7d959aa56b, mulmod(
                add(0x59ed5c5c63bbb0bd3ba11143e966259041d29a368d11ab307f3973025c63182, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x67cbfa9207c19e9bde69c24b6c86fa27836448a4499feb9ec918b159504fe07, mulmod(
                add(0x4131f273a49b5199160ade2b83e2c29bc6f3154a92b7f57d58a1bcbc32ea19c, mulmod(
                add(0x351290dc64502b29e5d5497b9d42a5f9309e0739198fe2a2333bc22c32a9ee4, mulmod(
                add(0x13a967a9560b0f243da22a250b54c2443d46402cbf28bb93fbb59cf0b4cfe57, mulmod(
                add(0x409dc454fd616ba7c6bff412ac16a4ae05d2c7b808847b002c8db76fd24c8c1, mulmod(
                add(0xa9708843d76eaed4f370ac050fdfb17a26ef32aa586788c9d7766a56d99db3, mulmod(
                add(0x628b9972335b1ed25402821b29d606765531e4db7b314201fe7ba98304430f6, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5112503838972672d1376ef871590f775d49e52dd0e0a115e4adf389479047c, mulmod(
                add(0x3c6f0325c4f3c2bde649e999bcea1d53db395d2aff86ea4f0dfc38cc64a1de0, mulmod(
                add(0x260e123a8bc81513535e8296a569c5bd5aae49aa39c4d58c93bfe91e00583c5, mulmod(
                add(0x61014f0dff9145d0f937ab88231fc97bacc7c90e6efcba0fdf8606f3615c3b, mulmod(
                add(0x25c1fd73a92a6d52f1d32898ed2a7257d2bb9cafe53e20ea9a2b436f5223a82, mulmod(
                add(0x6eefac241e920de065c87d3a4eb23e0e9aecad9b566463422b715892395446c, mulmod(
                add(0x7e685e30c48b89bc468068ca7343905fd0665ec91fb26f7d229df87f470033b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x90cdc7d46175fab0e57a2e3e71c4ed434044f1eb3d5216020dabe7ad61d96e, mulmod(
                add(0x752d3badede1319ed4b769527c1d5209fc5dcc4dcacb6edefec188b16e16716, mulmod(
                add(0x59cbd9a3d756a6b2fce8e5fce1fe6a4f9100b7d12373966971055958ee0ab4b, mulmod(
                add(0xe4c320c1528044c5461fdcca5fdc626a810597bb0afb1650e877d5ed57f8aa, mulmod(
                add(0x2b069d1a579bba9da9271a4754693d768976858e9ca10fa7c17096eb4d5317a, mulmod(
                add(0x15cf73a0eebc310e6eeb3b419e8234b82c94a7d578eda4297708c90166f981e, mulmod(
                add(0x67860454d36d0c79a1a1583cebe9f0f8634c66f1adc42728508378a9bf048b3, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x77f194c4b5b1fa40d2811f201dc23ce84c518ea12193d4fa4a533476fd5e0a7, mulmod(
                add(0x715a83c63763fa4765fe79d84ede3459f77ad835e4376f41228f9edbc7093f6, mulmod(
                add(0x70f0a5e494915a7ed806d706da396c5780b7277fca3c61475bfe355c7b4dc31, mulmod(
                add(0x4bc00e1b9c0d62b893b12bd304a374c6e28078f69b393f3a7819fae6c622112, mulmod(
                add(0x55bdcc23a5247bcdd496fdb54e8ba25baa9ff6c9c062ba487741008416f1092, mulmod(
                add(0x7b7e2d547a09712bc33a04f0bb4a747bc7ffa192a36678655d838c32f17f15d, mulmod(
                add(0x343c04bfd4bb8929cf3e8b59c6f830ff431513d347c05e2314ed94947def20e, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x51300d65e66b01522cad7abb879fb7f6a281c8db541de28442dd9f7d2da2d22, mulmod(
                add(0x3f7efa5db819d3cecf716807e5874cfe0ef5e0d2f62ca8f8508421a6252d8c5, mulmod(
                add(0x426dac594a31c8191077973b2a056ecd374937b2236236d4727eef4a6b476fb, mulmod(
                add(0x74d803b24a76a59702f2d131cb78fe24f0b68e23cd9f277a3af1b6e72b2e7ab, mulmod(
                add(0x12a64d97c574f6ec3878b497b1a803b67e494f622b77ef682a12592d23ba36a, mulmod(
                add(0x5827697b71bed7fddee3a8455a6d285c031137664c66f742ff45e0a3de7d6b4, mulmod(
                add(0x43cbaad253dfc4191dc7737bd95ad4740c8bdca321a2b95c15876922653675e, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1a49203dd8fc8a02b78c2669774ffcdf490ef36955621f756e18f2e7b00bf1, mulmod(
                add(0x43b97b38df63d95c17daea701fa3299a384309eacac79ba41755872f9edd134, mulmod(
                add(0x1419c83a1537cdecc5fab01a3758d042c9aa2ba6ad7ea2d6f31bdf30542e718, mulmod(
                add(0x2dd8a1e426c57412d4eb85eae4b0742fd331c3896e71d3cf453b375764b876d, mulmod(
                add(0x21267d8b0ca1625b8f2e18b6555543c1d5fb45d6690380167761954d1e541fa, mulmod(
                add(0x201fbe2d9290cba41b9c6a69a89f49c3d4967a64e77a2fca31479ac22004834, mulmod(
                add(0x5be044b336459f4091575ca780443f67ff21af6f2af0b5b08e285aeccc150a0, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0xfb78616c1c60742d11bece89b37e994cebae787dd1aa6aa787662c5d91c0d6, mulmod(
                add(0x6d1b3870f2858cf734bce9f098053f9dbafb501552079c34c9fe4a3d7b9ba16, mulmod(
                add(0x6c1cf1a64d1a31a8e75d7318db6d5e46fbed31763a585e1492d5c7588f885d6, mulmod(
                add(0x78dd74b8d289f1bdfb77cbdeb457725cf97acd6bec77040f74ff48567153559, mulmod(
                add(0x4f55573c028b391b48470cc489e3e6f3c1ba16889b5860bdc1cb40771d166e3, mulmod(
                add(0x60be7e068b8028dad5e3ddf35b35f3a3f82ce555948df4cfe12730b991b82d2, mulmod(
                add(0x21d943b38f7ef19f3cab705e3f7ddf2c07338d5e564007c1c1caa3d95ff99f7, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x69e84a76768c67c0aea91d4b492c99008fa0b5373ed10150acf507f29947eb9, mulmod(
                add(0x1c7e702dccd290cb33398bcae92aa664930bdb934272a9219ed83a913b2bbe0, mulmod(
                add(0x6968ff554c33181d3f6b15c6381fc08ee83b84cc57a7e7b5d5b17fd3599ba15, mulmod(
                add(0x7c7b6251ba85fc215c66e191d36cafdbb1cbe322b76a071df1940b48ccd3a6d, mulmod(
                add(0x5f629b83511c6fe59c23362d74677f7f697f4f696df97f8c827fac4ff286f78, mulmod(
                add(0x30a59c23b86759c95f0b42435b661675c512eaf8275a33a78e30c40106e7b47, mulmod(
                add(0xce8f03d1c7bbd6ddfcf3cfc811fa0076b13fed3b5ef9a98e0d66598d380f04, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x724fa49e6d0ad3aaf25d4a941afa835c29b91984ce6ece5c8d3dbbafeb97a1f, mulmod(
                add(0x2e44549707de9dd5b9059e8006d623b88f045c67256692342d2d33b0427d2be, mulmod(
                add(0x3c3edbb38c6febda3ff6417b2968318c40062e4939eae196b4130fff01cd3d6, mulmod(
                add(0x483bb3d6d0fde3b0487e3e118dd10eafcee406a2ffcc03afafde31d02fd8b37, mulmod(
                add(0x714cab4aa6829f96af5d85b841722057169ff5dfcc5d4c3432fbdf81cbdd154, mulmod(
                add(0x52381bad486ddb2057b398f2076b26b84dc3f4b8ca2748787fc54a8a895d1de, mulmod(
                add(0x32f7595204fd2c88bdb806307cfb5e333bc535df923e4476b42a21b4366fa12, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x4281f5da556e019b785c47dd7f038c060a15ae990b2524be5d81e7a5e4c15b7, mulmod(
                add(0xe458f9d6859a8d0330928ce09d40aa82cae486710fd1873bf15ae500caf25b, mulmod(
                add(0x4252918fb9d9ebe55ac828f1677b8000de536415170d559de70a5b7978c7d0, mulmod(
                add(0x5ba23df1ea44f37bf5914db2c202906dc2a2b57bf1aa62d856150e4810f2c7b, mulmod(
                add(0x38526ed29b7a19e88e399e497212d46893fd806a9720e7094a078e96622d915, mulmod(
                add(0x63471ce2a7ce256f822cbc650cbff92c9ae258a273a26a3cf71592ec0e05e6b, mulmod(
                add(0x1c4f419d51e8b93f971978a5618d175254380176d1b3e164fdae3783dbc9dae, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x4730661359198d82a3f6907012395a2ace9786c15f679d0bae38683de43e193, mulmod(
                add(0x7b17949c757c0bad7efe731402ca57335a7b2e896ec52ee861cdb546ca6c0c1, mulmod(
                add(0x16ceee1c1b184de09453ac3084a18045eb070a462d4bf14b78a42ba5edc81e3, mulmod(
                add(0x71cc20ebffe931ba8d1a2babdc5ab89f70c130395123059c0cc3a6115b3e9e0, mulmod(
                add(0xc423cd57128c8bc447aee6a3bab3fcf33a14cd0f0d59e45163fd45ef3519c3, mulmod(
                add(0x6740316aa7890e9f831f923f2595d6e3f936aeb581d67c39b3868737407c334, mulmod(
                add(0x4bb0b5fe1e2505a15310f62999b3261a462552a0e367b3680524a66dc622678, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x34bbdfec3b095623277ee8c1b89c32d5ce427262b0bb766423d3854282f339e, mulmod(
                add(0x19997530cf8d32295de96bcbe5d7e9994b358964e564c3994b2384924c72c8c, mulmod(
                add(0x53b96d00f96939a1f1787e2da432cde14589e94cfa8825cdf8f5bf0494190f0, mulmod(
                add(0x74556f6968df6431536d5ad5f8a667b9fb2594041adcb9ab96396b2ebfd0894, mulmod(
                add(0xe33f16f74c4a28244baf66b1ace07603bf615b487b338f5314c4d4f0e360b3, mulmod(
                add(0x4dcc651ab40497678e4ac24766b0f72506e970e7ce31a46a04264a935cb1009, mulmod(
                add(0x6ca77303509a41dfdcd15d1c290dcd3e21727de300828efb02e75ab38717ac1, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5b357e71c6423c40a3a0ffceb991ee4447fe907201b0fc7eb9a30370bfcf165, mulmod(
                add(0x5f145b9f6a5f0406b0916043d789f7cc8c202e40de13bdf695a2314716f31b7, mulmod(
                add(0xbe2f807a830efd67512ff12a84d805829f7d1834a92a6a2fffa1172c73060, mulmod(
                add(0x6a27aa939b0d82ed96f7139e4e8e6816d36ce2c421de8a29b05fbd95faa5909, mulmod(
                add(0x71d3a7c0ebbc4b5b7a0741c0a365d8f028acddf7f7505d64deb209303f155e9, mulmod(
                add(0x25fc3c1cedb30953984771cbd3d12a9448a23127ee8f1a4656640ee6815fb3f, mulmod(
                add(0x2ccb56c61cf0d02ce276c7a4ca19628daaa49e86744ea056781deb86adbe3d4, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x7de64010d5e1f4c9864b4ad6fcc6e06f59b4240310d4c7b1daefaca6b8b32a2, mulmod(
                add(0x2d58e3e8d01eec56f04ced5232fbfb0fe2911f9a9812b9209c8204b496f1991, mulmod(
                add(0x3b457041c9fb159f556292621e92cd2dcb99b3b8133ec66728af649309c3ec5, mulmod(
                add(0x8dcb07bc85e171851260ac1e13cc75d83a4c673bed7056dd839e3af1182d16, mulmod(
                add(0x62213831367ff9d45d6d3188b8d2651e9bb673f4a4828e1aa6b8b69b2ce28de, mulmod(
                add(0x19f8728775d11239aecd5320056aedf2b322456679f22b2f2b2940cd3569aca, mulmod(
                add(0x99a409a95a5add3ce3c83c73a7e6ac17ab69d013fe6221629b9bfe89c23683, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x413072fa04af17a24beadd083897287864e3ad960479676aeee511c86c25df9, mulmod(
                add(0x335815b1a1b2af09a76695aa7e6a729a738c0ddff05abe605bf6ac8c89dcb17, mulmod(
                add(0x63d50eb73c9e1fe8c63e82f022f2b8f172b55445f106f4c05376b8dfff8ab49, mulmod(
                add(0x5f83f086b3b0cf88785638b569b1c7f7beb0e446362527f8f6d35c654b7010d, mulmod(
                add(0x38990b2733a0d3f4ca3b760c84570de0688f8448a6358189c5ef4d8411b8f46, mulmod(
                add(0x176a6bedcb9e0ea10e9b561560aa27e3be8d1d7b5f6236e0700f8196b7b3048, mulmod(
                add(0x27f553c529be8e36acc1837fe704059ee63f08f675b9de8e9e492fc9cf51561, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x778e19f24b6e67424140a400b2f26e993a653ee500d17239ca85deb9d97ecce, mulmod(
                add(0x2a646b2be9dff0572b554c4f456aadba34e4a1a8d8d4cc65773b12c72caeae0, mulmod(
                add(0x63c0c3669941bf239b7045479baf5969ed9a127d241b6598e94a6653e911723, mulmod(
                add(0x6895c73c6795c9847ac09044ff2d656321cd4c38d5f7551da7ccb28136bb9a2, mulmod(
                add(0x2f3fd2cd378921a23ed168f01d3de889f148ec283de30d4e5c786375a72ae86, mulmod(
                add(0x282042f42859fb88246c44eaf1cdf99e0532b1cbade03db067d9ed0c2021b70, mulmod(
                add(0x41337b02031755b8ec4b882e05abb409c310d756afb3511298d61eab5cc0fe0, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x13e8378693bebb3338ab391196513e7d860604cc878e707a70760fb2990ad85, mulmod(
                add(0x7edb78dba4cdb31cd91730ab07f28d1fb608ff3b13797e9e28947b8106b9e6e, mulmod(
                add(0xbed6e37d854ecdd7c0949fcf00b95318350fab7eda2052401b7a585ffeaf57, mulmod(
                add(0x7cca6a1621a32714a2cc22c4a9d549f57fe2361f028aebe47a8ac9fe2517c67, mulmod(
                add(0x3b0968ae32ca8df4ae24f329832aaeef93590842f63533e22b5892f4452dea6, mulmod(
                add(0xd55d436f1336ca61f5e5d186bf3cbf3b0225921dcb0580ebbfc35cdc1c221, mulmod(
                add(0x3d2aafa2290f4f72e2eacf9387930a1daebe80388d9bb6e86cacd9856e80b10, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1175f14f0dbbf8f343bcd8522228e333a4618531cddb0e59ae7b3ab5df6fc52, mulmod(
                add(0x73b982f167dc343cc03cea582219625818fbac944246f5697dbc077106cf49, mulmod(
                add(0x7fa85fb77a4feddd02fdfb37941175c6abded77133c3a0d04f4230e0739dd67, mulmod(
                add(0xb5b4a298b35fb3271e337e51b7e2ce4be68130ce6dadf5dc29206ee69b76f4, mulmod(
                add(0xd86c84f58f12d0a61e8a90f0fb9b4f5ab6b6d1c05d0bef2b97e515afad7bb4, mulmod(
                add(0x2db6679248d87e10bcbbbbbfd3141e0871e242c948b67a45a4031c6cf05ff61, mulmod(
                add(0x7f405a2d740f03c37af1b15f2a0b3c4d94912b1690256792f407391fdb4d5fc, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x18063c29c8f1094aebf7b1b6e94b1aa930c9aacd48991b86cecb0eef65d6923, mulmod(
                add(0x7931edecedf073680812a998f023587f6cbdf516d194c268130883012bbbbef, mulmod(
                add(0xfa58fd835c91613ee310d0ad06a514874efc868b118247dcdb1eb150c221dc, mulmod(
                add(0x5175c440e34252adda271e011d727274107eabe30c6a12e3d4e5b1aaddcd676, mulmod(
                add(0x18d39e81192c3093968d5d36a24fb0acd3d1a637cde77c171fc97c81dcba080, mulmod(
                add(0x492b77025d7314add413b7c6d8b12ff58eb6f5e15cabadf063355606877fb96, mulmod(
                add(0x783e38dd1edeff879c64bbf204f6ba2d217ccdf98d0a9112afd4786ae6d6f10, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x29355cb56f48133d43961506c65522cb93f79e4e99836e0a574c14d53623f63, mulmod(
                add(0x7999641e583438b304c0d37190f36092f6c7d79059881900f578d85d1b73c37, mulmod(
                add(0x3d4ed175c5039de8c22eb411c359d97dc1f61f16c8cc10749a1eb0dc2f87ba1, mulmod(
                add(0x3c6daad3d24a5c03b3b262740ac0ded4d9ea8ca0c63108237e07a07e9b85933, mulmod(
                add(0x77152b2ec3b3e4b3818f48da12c2c5a9c62c902dc554d9cbf52f7a03ea02773, mulmod(
                add(0x2cf318d1e10ae466fcf413eefac0f8162fdccca88a49a050198b488032420cd, mulmod(
                add(0x791cae1af2b8d887c452e5cb1b0ff12f8abb10bfb178caac899f940b12ef8f6, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x32f18cdc74c3c41bf40735dbf4043eb16a7f9db273811f36b28fa269a3341b1, mulmod(
                add(0x6055ed00a3abd6c3ca7b3720fbe305a843ff9ac112439fe6b8f10a7c439c1, mulmod(
                add(0x17a4664e354b735b054ba3d1079e721b312e5c726c479362c4bad0eebe14e2c, mulmod(
                add(0x37f0e472eb07e15509b57d63a7f4e4b37b678185b759c15de43abb72942162a, mulmod(
                add(0x793681b47258696a591cefc10a9a75b3079b1d6770f920e7d239b48a26d15b0, mulmod(
                add(0x366ed16623097164818611dadacc9426e4ff5ac873021236ff01c099539305b, mulmod(
                add(0xa5a09ec7e9ffb8e2e974de07bf9226d2ecb5bcaa41933517920667c6f90f13, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3ac57316c25750ba67ee8f1b70b40127ac856915de3dc27121cb6de9c4c8cfc, mulmod(
                add(0x4af805eab396f1e5c8674527e2f5d14e1c91e20c448d500a23dd1eaca5615c1, mulmod(
                add(0x7bd7097ec247bd2dd3f31c5cb641ab016dfec5499f3db2500bdef67b96a0854, mulmod(
                add(0x4ed5cba39fa5bb5c5aaf38b2e8304acfbcb22c9d046f9ea7f622165b4404bce, mulmod(
                add(0x552b427435c31905089e34d0bb5fb0371681a1c3e492c616c2ffbed1a0761c1, mulmod(
                add(0x4829a5cbf6628d2c2028f9848ef12ce70d05260a70084c2b3536d77a4106062, mulmod(
                add(0x7f80a0897ae7e379034ef091e5d359e51acfb09ff260f79a5b8935183d33952, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0xb118548c4669cdcf18d4cbd6c011c84bf993cb365af0401cd192bebc7e2a87, mulmod(
                add(0x13a804fdd5c11ada15a97ff351da0ef6595b0e4523f4fca1064ce4c25a4dd95, mulmod(
                add(0x207cccebc196d668eba122c24ab32307f77c134af59f7e15093eef205b91b40, mulmod(
                add(0x1443587bc45ac760259ea2ea629d33b4cb1ef6b7c18cee5a9d7792d8175a713, mulmod(
                add(0x385d3d47ab609b0bbbb995182694e71b59d1f633afb2cf88538acef9966e253, mulmod(
                add(0x352faacf627b78499c172b4b131ec512ecefce30863e7bea2956814fce6ee4c, mulmod(
                add(0x204c732eb58f3984e1bbc4b34682361f71ef2d8ef6336a1b1edba2304d7d8dd, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x18aa23fe0b0ba847a45861632dc766edab31931b6fbb1ffd69df3d363a58b45, mulmod(
                add(0x69c5c2952062c56086ced218afa6d48f5d501baad2755c31e66ea6cd0fcdb66, mulmod(
                add(0x66418c35a1279b6577e4a033b86b2fcb74bb4ca3f294c3b18e8842a97f4d2ea, mulmod(
                add(0x7d59ab3974ccd085a4d7be60d81df53fa54fc8b41a615bc00efcfb80f95b94, mulmod(
                add(0x56a861c3b7cd2b74b0dc0d772cbe0f7cf355dded7ebfcb8bcccca5b76de23c2, mulmod(
                add(0x5945d4eb8435d62327480a801b19e156fe04ce1b5029d68c5f7ab5f0f4b5af6, mulmod(
                add(0x666bfb99a8d2905b46c0d13f830f758d212e1469d42ae3122ed1240e19a7263, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x45dce57b81fda05ea42886e8d8335b7e8dccd7118457460a65e369c94d8c47f, mulmod(
                add(0x79c0afa74215944095e954d825845ca32f2e1fc77c46305ee14d0c9cbfd4151, mulmod(
                add(0x3665127b3be3f39d7724cd5dd56647e8bd029211425bd573434d08bd46a2d2e, mulmod(
                add(0x6b9e36486904713384afcd30ac5f00f3a9dee01629b04ce38a1fa634fd95f7a, mulmod(
                add(0x36f8eaa4510fe0d4fd62fee18c3975a048a66570e90f13c51338aac65a8b99f, mulmod(
                add(0x27d9dc1089da99b0431c95728fb2c3488849bfe58ada2162f3731aebf0ada08, mulmod(
                add(0x6e9e0bb284ae8f76e08a89a2328a39c8b0111279a080dc7d260c6abc3d2d422, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x46230529bd9d15bcf5b39f1e7d7de643f29a821055508fba28e20cbf411c75e, mulmod(
                add(0xace15952caa2b71feb02cf78e7f702d9f25db1edcf0e33c08b1d50460b9969, mulmod(
                add(0x1498b00fafdc8ce3bdca127554810f6f79fbde0a2288810d2061233548577cb, mulmod(
                add(0x4d31c59c091422560ebc8c3a9550be5af9a865bbba0980bd98466fd45817319, mulmod(
                add(0x984947cfe8405f9d922eaf2dda1dd352314fa7073aad30ae6ab5a725bef1e8, mulmod(
                add(0x29dc8c817ff7d003c677d5a8a27c43dc81bc4e4a55d4b05869a4f6093db37b1, mulmod(
                add(0x2c6219018443f6dc52509751f3f3e9cec8056671f990cc315a9541f4211e58b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x4e93604a16876e2cbbbf9c7e75b0244f218a412a00c93cee0215fed09400b7, mulmod(
                add(0x7ce95dd58108d60ceee243c08cd1197268aab5db70188fdf8e71d3d59d887e, mulmod(
                add(0x571d5d64e1c4fede99ed76b344c589d8678b3b1dca86f8a22c7966e8295727d, mulmod(
                add(0x59fbb3853ad0e402f8bd23c4d91e2e3ba573759d91a62ed1564de96dc634818, mulmod(
                add(0x6c1972711d09a06c6786e0bae1ad5b16a9ca1c3816a903348c9862448717e43, mulmod(
                add(0x39beefd7be98c73c098a96d4d7e06803317894de6cfdfddbbc20d7ddc86b2a5, mulmod(
                add(0x36e7dc049912fdb888fa59188adeba0af4936ce7dea6671c2ac0a6fc30caa2a, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5475d64cc47222ee31eb963ce0e04a28f5a3d42d2f69bc4096388bcf92a81f3, mulmod(
                add(0xf762eba5834d4449a6e36a478d1ee39451310b8b0ac881c5317bd2f16718bf, mulmod(
                add(0xb3287a93471f9ca1650a47faa7b88938c627b3977b094ed2b6ccf574e13751, mulmod(
                add(0x6138265fcc0318332714a9f6a421664a661dc4cb8150c9657bd16c70af91bfe, mulmod(
                add(0x3f5de2246608440809da642b0539560890c866eea68ebbe44af02571de60659, mulmod(
                add(0x78d303f513bf09a3d9c05f4eff2cbd2885dfa20b384eb1ffc336c72c18fbad2, mulmod(
                add(0x5a1246982e5259c43ff905c49e26ef0902d5a74e112dbdb28749e39044906fd, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x29eadf74abc88b7e53f7e1b3809fbb1b441e4c33ba6f42c998081345bff47e6, mulmod(
                add(0x272b2b8f00649964da4dd630a513d16f98f1c623fb5a60ba961f8b99df1ee4a, mulmod(
                add(0x27599e33b9b1d4163a1bc0eca1066f9bc958b387ac714cc51fec7989dd15834, mulmod(
                add(0x9b9db17468ba05a74ee59ffd79088031c8612504a34bc38cee28237afddf2a, mulmod(
                add(0x4c804f1403c9afed9fbb27ff8e8cc8ece4eb7435f39b5727754e50bb68def64, mulmod(
                add(0x3b36e3198708b5ac2b21207d786bad7dbf7235e924a8b6efb3447e38f80572d, mulmod(
                add(0x484485fe82b3d706eb26fd87652147117cd94d4d92d93d1b2e315c9fc9b1aa4, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5232df9f99d7a2f8d32de15f8a8e8e75ac0e322dcf136e51951294cdfb4bfb2, mulmod(
                    result,
                x, PRIME))


        }
        return result % PRIME;
    }
}
