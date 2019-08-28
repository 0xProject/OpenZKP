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

contract DexHashPointsXColumn {
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
                add(0x5da0d0779df2b558e244a518115a7ee5d885de473a34009ec215e84e6a83a4c, mulmod(
                add(0x76b1574f0f2966b8b43fb3ba4a3052bbd26f3ac1e05f8e996378d4f061f9ba8, mulmod(
                add(0x303eb50b931f5691d5da0a6f00556a3503728fd5fe14d663e81e2a00dbe7051, mulmod(
                add(0xe656b7c0ed3b676a24d4b3dd71d1064054f6151d3210d3486b87f520355dd, mulmod(
                add(0xa50612aefb6d593b5aa8da8722dc74dcd27b5399e9c61428c30fa8e3d9f2b, mulmod(
                add(0x5ef21b0c0cfe40d8cfeb36c05d8dd79ce4fca795734323748d96dbf6171f84a, mulmod(
                add(0x50ddd3fddb79ac66f391f4c26e8913ee933a5ad38f0c672916b157447c75846, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x683442f71b54416a56bf38e5b80c990618c3f3662f0fc33e5240e0068042e0b, mulmod(
                add(0x20a210e7c75feeb3c45730debf581aa58299aff97f14c9e5d64d6760fb67c0f, mulmod(
                add(0x5630fba77bf25c20816fd47e7bbfc74c6602284f4cea1bf17268ce13005b07b, mulmod(
                add(0x135eb4ae01daa17b23cde7e6ecc8f18a6e320a8194e58e31d161fb6361ac33e, mulmod(
                add(0x60bc3d6436b0ad174ac38177f931ae860af842025aa51f0638aa7d18d9e5114, mulmod(
                add(0x24eb997161d5dc5465ca8291f30b4241d38d30aadc7325cf7f1cbf5113e7b07, mulmod(
                add(0xb03c883d6ba332af7001a37988402c631d1b017c6d090f1ac0210c054448cf, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6961e5f13f0446cc73c0a24a75fa83d8e21ef288b6b2529e8b7e53d91716c6e, mulmod(
                add(0x63d6bd56614167217477c47535f527713cb8f732c9f287d1d594a9f71961dfd, mulmod(
                add(0x2b03125f18313eb3118557cff1abf931492a6a4a5b756b9303c713dc4c88425, mulmod(
                add(0x49dcf72ede8f5591d9df3c18310add72d5d71c7ed3f6dd4a1c9a5028548126b, mulmod(
                add(0x685ddfd70f439851782905718f4fffd8dd537eea7c19db6c8fa19b3e1a469b9, mulmod(
                add(0x26f6babba9acce9fc4ba50bf8b79a46d55273086483e1cbe63c897f0b2e5aa9, mulmod(
                add(0x47e4fb89a0fd92725654385386b3a49eafc9eb9f2ce39a3d6e0b75fa9ba6e7d, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x24e4422dac00db7ea595bd63b409ad69082d8ae47b6a19c8ea967d21e2665d3, mulmod(
                add(0x202e0d34581c070e8dfbb551ac5db5eab4cd271c4a5775377b05d8faaf93793, mulmod(
                add(0x584c7da3c6ed324822c1e7c94dff9d3981a1a7b0182ec66b89c14e34dd1148e, mulmod(
                add(0x50c42b6132f3c6d97adbca283a35a3bbcab0b1c6cdd84e986ce702ee99e0d66, mulmod(
                add(0x4aa3a64bfdffb62510912ebbbfb88bf5fefbdc7ab4d5aac4de06f392366d30d, mulmod(
                add(0x70823e579c0c9a73db3c41fbe2b2b61cc419385c64fde1ee6dfcb50a6782d3a, mulmod(
                add(0x72dfbe43a4b0d65fd959d583d243bf67e8b2bf609a465666581a536f16e40f8, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6096e23a8be5f0822b4d386259c085185385d7e0e02ec8b58f80defedcff775, mulmod(
                add(0x14861ae90fe75799db29eb71fcd5def0f87dc5d42f2c5c40c517dd0978dd538, mulmod(
                add(0x647e315be8858b3ff72e64614601a1cf182cdd84eb2a51bed86189777f385af, mulmod(
                add(0x1d622474570d406df7e9389f1350b093ce0445b70b1dd6d7fdd4ffabf05fb48, mulmod(
                add(0x14be7d5bfbcf464e9b221dcf7228a723e3bc637c4af27e12d182bb2f2cfa641, mulmod(
                add(0x33b686caf70798b1aeb36e0782b00cf25f6a56ee14d0f682ab0d739c97b0549, mulmod(
                add(0x7bbddffb5d498a3583029d8841980f29e7d1487a16158e2c6790f3c536b801d, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3e40109501dadfb33e4b367dd4f09382fe9363feb715e9bec8a0264d7c6a3b, mulmod(
                add(0x5ebfb2d643768bcd77df8b1933a591c42fec469b3e394ce13a79fab53fe01ed, mulmod(
                add(0x1359b59675ec0f7e57479e94b5bb0c7a708bc32a87eae64545b918164c1a7ae, mulmod(
                add(0x43744bbc11ff5e38a08aae8d366667f24fa787ce91dd68febb6ad866cee0a19, mulmod(
                add(0x553f266425f75f126ec183011251b14c21021ad825904292b207a144960a845, mulmod(
                add(0x52ed3da8b58fcc75efe613d624aa614b3924329cd362c41ce986e6fc9e2622c, mulmod(
                add(0x5831b73201cbc75e011c229f0a0d7ac6b00759db2c3fa6b61f8df59b040386d, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6f1e861942847d04fe1cc9082acbe2e0a4dea4374f87cfb63060a64910717a8, mulmod(
                add(0x484331da66ea6a0960183f3a35117ab64deea7322d4b004cf13e4a9f24d2b8d, mulmod(
                add(0x2a54b0b87193d139be42bcebee85875e8ca98d777a9c4c4c9c51c1ed7c2a4d6, mulmod(
                add(0xb7b4499d69c8b39e22c462ce40bc7b64b7544a418a873c21716d907a068d39, mulmod(
                add(0x5a35529f54904225d132b541f4969278845660ae848fffef02737934c22cf1a, mulmod(
                add(0x6ee2711dabd772a0021e4ccaebcf3622022edce8804b590c7e612fa87f762c0, mulmod(
                add(0x58b7c86842b5453aa9c26ac2e85f0ed9591a12f812a9ff6b4d4dc972af615c0, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3b9315b9628afd458bd2c6cc7da25d4bf60f4caf16b2f6cd6638b310098f7c1, mulmod(
                add(0x2dc0b2f77724742f9b3674f2a1e9c2c231016927822d91fdb248926c58796a0, mulmod(
                add(0x10a765e19129ff4e437c3fba4abd5899178531cd5561a883d0d4a415c14ea64, mulmod(
                add(0x5b8bad1121a6ab3535a57b26a2338bcdac00d5fd2e10a28091921075b494e86, mulmod(
                add(0x4e3ae344d7bd43e979b14f0e73490192c250f814cc7b58b929e8d5f2c6b80db, mulmod(
                add(0x10965e70385e6cbed5bb82c0f682edc5fbceaa2ef8c2d1f80951e2660b2bfe4, mulmod(
                add(0x62d242740b82ec454201295e185a5bdf4077359195248dddf4feb3c0d146247, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1938d6c94ae8f556f94400bef8b42fc9b13eda17be250cdfc4ffa56ab7c8b8e, mulmod(
                add(0x109985071971ba9d7c9f7d768348a4b0ce5bf2fadcc30d43a638d7cec04bef, mulmod(
                add(0x7a94500b290040c799b4205c16e6d5d570cdfe3e62c1ee0d33be65447e2e251, mulmod(
                add(0x7262875e78e2dd900606a3d7f9c32f096f23102cc72f75e9fb7310ce0a0a8bf, mulmod(
                add(0x48f672c2b3b641a7f4dec91a25500ab22e03e4c3dbe4d5cdd7a5837089ee0ba, mulmod(
                add(0x28fd39c5e8b4fcd1c567942e635117c3794141ae33ab1bd3c1fb2cbc54e5451, mulmod(
                add(0x1d245dcc2d5c1901e87e8c1149bcfbc2bb4dffda5508857907a3ca04e6064d3, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x535518fb5214586100f30ffe477f01d80d03a87ad6c069fae64891da08b0f32, mulmod(
                add(0x792ebe0ba587318a6d2320278d7963b9379cfe411b9acfa46a986259bcc5209, mulmod(
                add(0x5b217d74cd9140f380a1e29cc1a9d542fd56c94275f7d043fe4faaeeea63156, mulmod(
                add(0x2863df231c2fb77d41f91eec59c4de6ff5e55565ebae8f726759c36a687bde6, mulmod(
                add(0x7b5b6d056eefee43cc045e4d9a8c696d842c3bd0363f5ec050736fcf0a574bc, mulmod(
                add(0x5ee6bb0ef1fe5fb2aaececa04514c87ad8fa6190bfe4a469641a1fdbbf682ec, mulmod(
                add(0x3d30cafb90b9f58c7fffe372e683c6c32ec705e1b86ff4a67700bec96da895d, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x7bc0b6569ef3c75d18a485318e094187a11153dc30a0c037c3a01af756fe6cb, mulmod(
                add(0x7fad4d86eeeda8dc3a3e4f853f1b329bf748e2eb2853429e254ef0b5b7a0a82, mulmod(
                add(0x746bacf530520867004dcaac1ac5b25833d4d9dee78d302bf3ea9c23030155d, mulmod(
                add(0xc0c656020274b15410828da987fb031323b835399e975318dd8183bca842fa, mulmod(
                add(0x44e3548f6527b1bdfe905b7b7bb9168da4eb28c2e82932f97d1e82b6a5176ac, mulmod(
                add(0x447a24a269f12555eddd49f3f8d6ae06a116dfeafa3613f8df658f62a5a500e, mulmod(
                add(0x4d7ee5c6418f90e6221f681f85e8bd19b3658d34d71642d610ddc064d925a00, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3e3be015faaa90ef82e370e3d9dc41a6b3bda6cccf4e1ca808a0ba7c4e79b61, mulmod(
                add(0x738ef17fdc934acb7f99e61f7bcfa14b50aa92cc32ded6443d849a6354de380, mulmod(
                add(0x2b3c292c719d01e872855bd0cde1c1c4996fcfef981835123ba38a5f7b880c7, mulmod(
                add(0x24cc1845f8c501a357190f0e7bc3a0b4cd9ab9f318dc4c86e4899ddb1d92482, mulmod(
                add(0x4b8e8264bd54859473baf627c6b950681fae9cd4ff6c197bc1bb848d9c22ed8, mulmod(
                add(0x175c1d9a83f92c51094c0e0c2772b19a264ae6d4a4fde3dc89505adae2a70f2, mulmod(
                add(0x74e1bcb383e752b8a9b8153c4ef005cb2f721e936818c827f72950f0fe14ae9, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x225f8a97c2c52c608af29ebb52fa61753c65677a767c3e8e106570d0dba1ea9, mulmod(
                add(0x6c1d0ec105df67a80a1fca70ace31811b8ec9a730a1577d96673e1b28503821, mulmod(
                add(0x69691d5c68ad47d736332ae39e9b82474e212516f0fd959325c363f5761071e, mulmod(
                add(0x63efdd46f101b6a841b459c996eeac158f3825818157d656a00caf52ad16a32, mulmod(
                add(0x4b27f8e11bda66d1359a108ca4783bf91bf1182dff7fc91598191b9f0a3a65a, mulmod(
                add(0x7e0f4c312d346c060c19be90d0828e6a2f1a4fca01de4059a7183fa10d95a19, mulmod(
                add(0x455f955e0520f3093f9a640f8faaf094df03a05ee63efd121f17a9e0c0d41cf, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x698f10837c3c3b358e0d439072e4df1d5fa001e25e74397d98d39277ca7cc91, mulmod(
                add(0x6b6a3d637c0a20f5a21dc5e3be12df9387957d6ed40a9d7cdd2faa86613b999, mulmod(
                add(0x5683d367d2d188f9a29eabaddcae74ffded7ba172622338a5dbf07383a59469, mulmod(
                add(0x5c2051d931f5603a855694b273930a52c5ace1abe764ae40a148f12c3d5373e, mulmod(
                add(0x583083af08ae05265441f8f0d7d990a2f8b6eb163314ba72a70d1446e280871, mulmod(
                add(0x7bad58ecd51ffb5e568afc047b909a5e027e0fb4309724b4b4eab343cb853d7, mulmod(
                add(0x47ee27dc0f7d981e80710fbfd779b49d8223b95c3bfb7e54d5a5711d0dc4d05, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x4c621588a9b4331ab2d6c7b5a954adcf8436f8119c04d02a32f45c00eafa924, mulmod(
                add(0x4eb9926e6fcc74a9d963162f40ad16920fad67c85065f67bc0b607143c26392, mulmod(
                add(0x58cc0ddc727a13afc7954c99215c11e4ecc211efae7b8719f60d503b3cdf084, mulmod(
                add(0x7edb9a87458063f2206e91804a1e4328e7d93aac62ea697c264ca2b2c9ac7f8, mulmod(
                add(0x4999dfbc39c2302f351140e7c00cf607a914315507adda43f6b8b989e2dd01a, mulmod(
                add(0x26ff1bbb338f0b5aecc3fe6cebfc3c97ba547f846b7e4d45b9894a58d14136d, mulmod(
                add(0x57a4fb3016c561ed775be1602770d30a401b1fd22c07165c7831c1305d3122c, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x304d0102df767ac2b66604fbf2b87d2f4c44c374ca5faf5a8804f701d69cf62, mulmod(
                add(0x1bc45c80e58588c42f60ead841fe693b2e7931fec4c401b2e1a43e72009b6b0, mulmod(
                add(0x7d327eec36459bbfbba42b8054657f0248ef22a7db8b21a1000c6fa26aade29, mulmod(
                add(0x29a7fe4b0a401fe4c7ce859e37ffd7dad50bbef4e8dc353d9a80120940049b9, mulmod(
                add(0x5f1271097e673d7903ebc01ce7e9423db8a05abf6fe1e1e3ac926a2c2d7f189, mulmod(
                add(0x44e4571012dc05cb59cc74c8972e0350c9a5d7490932f5f39fd2d6447c4b643, mulmod(
                add(0x7c0d1079d0762fd4f958a772fd750fcbb8c9c0e9e8910e23cadce79a240f3fe, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x33035adbc2f9917c25a9cff51cc0c9ecf8e89d4aa18e31eaef75f8ae74c1fbc, mulmod(
                add(0x6eddf5ef0f93bf7075a1b5457b5223f92319301fbfcc7d0df591020738b514f, mulmod(
                add(0x3e61217fd4aa6f5db41ce7108305aeabad210cdc12bdecb889e2e08a89d1133, mulmod(
                add(0x759a33baca57823673d9cb7b302f1903b714ae7c251448bf853c7cfaa4d337a, mulmod(
                add(0x519f0e900250be02d1f8ae1bd0f1f17378ee28364ba39dd8b988f8c957a3ab6, mulmod(
                add(0x4e95da2d7c790d17e33a9e01cb9d4cfbb564ff44432abf7195ff0b718d31077, mulmod(
                add(0x3223ccb0f0c7b396afbff215174a59e4707dc3ec4101979f468bb0488becdba, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0xd46939b2b4fcf0af6b654ed2fc4f668f902b7d7ef1c84901fa6dec79c85da8, mulmod(
                add(0x5e4b12b1ffc804e8ba93602194b4228512f9d9586a20a26824a673521636927, mulmod(
                add(0x28ffa72fd365fb1a52f61e893ebda03afdd766c18f2ccf98c48988c1c17962d, mulmod(
                add(0x775f494b922774c2972d08138d811d284d52c42886fa85f8ef2e42653830b68, mulmod(
                add(0x7df639cdcdce2e01a3e2d88f2e1da5b95c780bbe5313e17b934887f9fdbba63, mulmod(
                add(0x40d64b3172f0efe230fc001fe386da1613452ce24aab75ff283ff965f7090a4, mulmod(
                add(0x196e1ad10cd1b61c53141d5306f146c66cc6b0c2305d77ca818e3015b5aa33b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x429d695f29693fa1cd2ee50392e94f721218780c415d76782b4c7429af897c4, mulmod(
                add(0x77c0a0ba06bd128cc78eb9f2bcc1573bf8ea77403af51f4732bec2a1510bdf2, mulmod(
                add(0x7fa2a7f35be2ebcd8007f24672ce368a8955d8dcb21eafb15e873b1e73c3adb, mulmod(
                add(0x1ca461c969624602ae2e6c225289988f05c97e727c0fb50725fc2c5f912a5c5, mulmod(
                add(0xcfa91d36677415ea432cdc78175f590691dc3db18f4b60908ce827697cff13, mulmod(
                add(0x243486d7f5b53dfe94c9e675071c84c571f54a486d2280734e112c603c7e5dd, mulmod(
                add(0x503409ee3c75afacc8661a208ca61f7eb5f38b8c7ba6c90dd48ae26b6035f87, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x12e5d0aa5784953cea06dcd55661d519dde874ea13ff8987db858fc362e5f02, mulmod(
                add(0x47865a36e5ba4ba0ffa035ae571e915be018528083d8a91c8772be74c12cfe1, mulmod(
                add(0x46af11684190d642a93f52b554d5312d308163576dd4ff619fa8cdfeae1d3d5, mulmod(
                add(0x79c7693aef679cbad9fcc98368ff531526965a18701335623acc09fc085de40, mulmod(
                add(0x2667bf4185ecc01626f9588209b83e31360b99213d1a58a775f853d6504dbc3, mulmod(
                add(0x90f4b8bc5f6d9af26c3580a7d564c3518eb7dbc6d9f637b548d0b834aeb12a, mulmod(
                add(0xa0304ae77cf0c8c27b6b5617ba2758e3ab2eabd9ad63a1001b25f7b924759a, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x8272e89b1ee3917cc3532f1ce4baa617fe7d84eff7c5c4bf66e9da91ac3f3c, mulmod(
                add(0x65352c42b953b76c67f074245528d973f99fd1e579f745302bd5b6fefb92961, mulmod(
                add(0x4f36de813b309119c1c07b59102db32d011da4c53d9528ebc2fa5f41c3e1e4f, mulmod(
                add(0x7376dca742b6281a7450e758475147c6b39db77f838f3208c97475d629530f0, mulmod(
                add(0xf0174a686c7b5d20fa7a204433205cb032eb15cecde8a402f65a0f3b63a94e, mulmod(
                add(0x61b005b48a26f4eac4d96682d9dc6f008c13e73b1b701f0cc62746ca18a4936, mulmod(
                add(0x2e4912abad7b0cd8434c1bae87bef7c22c861fb0ca70d678eac0e095a3ff716, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x7e956ae78fe7dfe33c78454d071c5b28ba2f1e6796a846b8f3c9770a2cabc40, mulmod(
                add(0x7f9184f17e9b735728cccc6e0c3bfd162015490e5166ddc367590c57e15179e, mulmod(
                add(0x6922240cf0be01d9131545c8e41a37611dc8804c59d54e9e71be9945592a023, mulmod(
                add(0x262d9eb9f79b7e3c5d515aa8254f46e2c1ce332d2faf5472ffc8e22a646fa54, mulmod(
                add(0x1ce50811281ebadc4911f51c67b500f716b01fee9fa90b74b8cbd48b49eaee, mulmod(
                add(0x392d61b103e2a98ba374fe3454baf192973660da1fd60e083fed9c2f8383294, mulmod(
                add(0x4b9cea7e0a337ff2be01b388554dd2ef595647bd105265970507e129146264, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x352d3f8a562e1203cdce00c34dd66071013635c18c6b5520f5f356099ce3100, mulmod(
                add(0x5a3a1b6cd2178b031d10fd48c0f9c77b84e6c1e628207bb4c021ed5f3b1c6a, mulmod(
                add(0x488c4b1518856aa0b6d486b155b59754425ce51e16fdfd76c8efab5f033c261, mulmod(
                add(0x489f37a49ef361fb0bb0ab3fcd27a587892c40757c2d5336998347718154ca, mulmod(
                add(0x23abb349bae7d7b9f2e24ba007937500295110ae1cbfc150e6cb6cb5477599c, mulmod(
                add(0x9b40370c330bf94274805a0df22d36733994d24096573dfb13e1df56e0fe39, mulmod(
                add(0x1bd6a197e987ab4c8b3f25ccaf5c0314e26f77c70bc8bb7353bace7c2af980b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2739d0c60cfdba9df785c8b2a8f1693c1fdfdb106762825a3ed1b891a07dffd, mulmod(
                add(0x679c36c59ef9261443372deb5cd88b410c8245f49f067a5d1a1f8eeedc33fb3, mulmod(
                add(0x170471587a894d81141399e3cd37bd01c127697b5cd855e11d44ea93a788061, mulmod(
                add(0x79767b2d9c67fbdbdb42f4dd12a1f217613219c14648565cc9f9081fce0d53f, mulmod(
                add(0x1e0b17799370c236e0f6f4b2a5b622e79ab57d55eb8661239c6e7e07c1beac4, mulmod(
                add(0x7114593da3d3d801ecdf626370bbd620ab4345d190700ac42f98c64d7143158, mulmod(
                add(0x79cbc0564441e4d34082b508dc7e6114c2c7e60f4d90b1027e351cd5b4501f5, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x45c603f0a68b8aadb48b942157be05a6f400716b0f5515b23120e9550da201, mulmod(
                add(0x504c66333ee57e92d1757f96a30521d01eb9d9baa5d84ca647067679d8f3ec7, mulmod(
                add(0x4b0123f54824c3fc128051387243af02298c17b09c63d08cbb3e536b47a41f8, mulmod(
                add(0x4c7067ce731ad9e42c5454fe3519b0c023eeb8611f8f0eaaeb1bb99826d88fc, mulmod(
                add(0x117e52fa4b3949e0fe996e85f3f7650879836a27619be4ab42264fad6e78043, mulmod(
                add(0x2a37863ec20b6756847f4e278dd3aa663c6e8d39ea0d392df23e36e787d0124, mulmod(
                add(0x99e32ff98aeee639aed22395067853e1c6857a340195c10ace4aab3dcd2dc5, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x21aa789048f2d4091d7e5bf257f1c4435e6dce08be4aaa140de6bd14e8d8957, mulmod(
                add(0x42c3d6f91e09928c5fe3b2d3f7344214c5deea39f0cfe8d711ccdd91f7125dd, mulmod(
                add(0x65571cbb2e3c62d7c79bd050c5c61efc18f16894516d9bfc669715c6f07108, mulmod(
                add(0x2b86b557c617d036a3d44f66848bd60bfe910f36e57536b45763502afb62d60, mulmod(
                add(0xbcac47b631cef9af8e6d6346dad25887d0d38c35cc28e4fd5540ad82f6e562, mulmod(
                add(0x4631742ae052a050e7aaf8ca0f94c30d002a31f2c8ad541ea115942f2ca32ae, mulmod(
                add(0x682a5e3a3b632e20dd078cadbbae01a6ac0a239d5afc8b52168c9c72b7a9a50, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3f23815815dfb6510e4d16a1761deee03e92bc4ab678cd04c1d4873fc8d8d3a, mulmod(
                add(0x7ba1849bb0f55b7b45b2f7c5eec286771d17b7355cd2d427e9af17788e8402f, mulmod(
                add(0x6680a2eab148c635906700f3d970f7c382d84754c02e94acf953dd463a5eb39, mulmod(
                add(0x29360d97e3e4c861158e5c4145adcd1686eb04373ba5cad582e905802d5f721, mulmod(
                add(0x7d6704f24294db467c4ab17f9ab24cf0d3865379baa84b3ff21bd7833bcb3de, mulmod(
                add(0x19fd8cee353c0471780c19d9dc4c6e0c892d478b018a838c30bbefb06155516, mulmod(
                add(0x322f05346e355acce47792541e42dfac1cd497410c059b1d64bbfdaeebd4029, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2010014f03fc63308c7e025162c5c1e2e9ca6df1b7358fc5840f76d07d3a755, mulmod(
                add(0x18d81ab7c04bdc69796db2f8519ef3be46ce20b06da2e33c725ab8209157bf3, mulmod(
                add(0x65c9504466049685cbc1e4eb1bec344d9a1b7ca677339ba35ade33a8550d894, mulmod(
                add(0x340fd240d829dcc6a27f6ddb7551795a3fa47dc2a98ecfdcea352943d041f8b, mulmod(
                add(0x75b3bd863d4824e7a01ee6e393e941e21f835ce1d45fd4fec655db607cf38f0, mulmod(
                add(0x3e8c793f227b0d33ffd36b22a8b06add64ee2a24187cb816cb03ab2e417e48d, mulmod(
                add(0x89ce7f5968b960a715f189ead7c58cfa96015aa615ca96293a5ab2215f0c7e, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2f20627110e9714b9ff74ff83aff7e6f11f6eb14451da877e929bf74f99dd52, mulmod(
                add(0x39849008b6e32c167eb8d3de4aaf70f7943cb717deabbc13ec532109e522972, mulmod(
                add(0x6b6264442ed87651e68361c468b0d11fd579fbd182e9be5a15944d20507426f, mulmod(
                add(0x66d4967e706a68c0d58510c128574813108c67867c87c6cf906a2710fbab381, mulmod(
                add(0x607b6c481954be31b98f8a2d1cb0bb1f3853b6f998bab3267a63df2496035c9, mulmod(
                add(0x280ed59c6aae5226ac8c607bf7bd7268bb1ffd88b3a5db0bd2b0e9419cb546c, mulmod(
                add(0x3ade1b2dd462fda315f9af01fca48d31c0f5301e0030ba769dc52d55e483c90, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x132d4bbec3732dc433bba9349ec5ace6341e79d74636fc1fb94d23a9a78c330, mulmod(
                add(0xc7f6f72d565afead463ec3ef46c83d214edf17dc6969bc53a0c4cbf64ed90b, mulmod(
                add(0x556ebb259aa9501a6bd9e23f07ae9758bb17cff995b4dccc9cbf197805dc701, mulmod(
                add(0x73d5bfa47dc8fdd9d8779b313c105f30640750bb6a3d16495cf4c647149ef97, mulmod(
                add(0x3dc3685afbc6470f54c0bc55b64c2d2d5150dd7362707a5fce5608d9c470788, mulmod(
                add(0x38193fa536191f13c98356fddc4b3b6f9dfc638731626d8dd7db15abd9f1211, mulmod(
                add(0x698ce264a969803836af0b105944b79ffb7cb02a146cb5f53b20167d657bb92, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x146f820030421a0c216be5672dd59cce4c09bb08ca5c453f4af5c71af273e96, mulmod(
                add(0x7fb98dd9de5e5d95a15d257ac421b7a73648bd67ec1439dd6c978f0bad33b9c, mulmod(
                add(0x53807bed439fae9b5a322aa4f531820339504bbf5a35019aec778aa2fed2dcb, mulmod(
                add(0x11033a1310f4a1672738d75797d9763447420b0a963d46498c6b497588f797d, mulmod(
                add(0x61725565655814287790f3b1b169f5cfc821680fc445024d70e20657ac1def5, mulmod(
                add(0xd092307a9b47d574425d3d2451d3ebdafd2ee3c34406aa986a5c23d2c381de, mulmod(
                add(0xb5085bd3a0ae37b4c4544156bedf269daf1335491fb05d08acef0adfcd1bcb, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x7b9e6d9a7f4b37fa3b843da2fec328b58417927450f4cec9c4bf2a55198de17, mulmod(
                add(0x45cf5b4687d8331bfe2ff3ec2b3afd9a07edbd844e6b448ce83ee5e02ba2e7d, mulmod(
                add(0xfdac327fbec5ff010dafadb34c5dfb376a4723ca990d0e533ca0454c878eb0, mulmod(
                add(0x40ec03c166f14db6e00c2a5b93e9ea21980b702b11c57edca4aae15132b411c, mulmod(
                add(0x1e015c7dc86eef2f9ac1fc665e9c0852b37281f3fbb8066994c38117fc5e203, mulmod(
                add(0x12935d67521a07668fdecf6bb799059e3375e130e20d1834895b2d1427da090, mulmod(
                add(0x387e3a6f9ac66871496c6e630f0625d4874aa426c52b8d88780e43e52e0eb49, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x794199ddb8dedada05fb956f7977e20b1e070866df5437f3ae41fdc8a44067b, mulmod(
                add(0x4ad2908cb341c25fed99e27ef988bfa98f65f330a7becc64a3cb79dc08c0e95, mulmod(
                add(0x67268302ef20c6d710f2e40c58077d5d790669c4bf46762d97c203815e77aac, mulmod(
                add(0x19071dccc3ea683659422abf0ae5ea7590977ef55bf61f7c5a5bcec969324f7, mulmod(
                add(0x57ce2108b3032ee1381fbeb363883b3f9c40553e7afc96069e4753ca18d2ab2, mulmod(
                add(0x2bef0f96aed4438040d4f7a7a7a0285b387150a2edf2b0fe5112a2bb3e44b5e, mulmod(
                add(0x12b6ecac135023d610478d5651849e2e9cc391a15a08a4db0f9a0d2a6f53097, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5fd6c694dc8c08d431f1d2026717df096f48081577262bcd117000cc76ce901, mulmod(
                add(0x58e0cd855fe5f815ae26d57df2b0219e0e7d25d3c9fced1d52ab42bce70011a, mulmod(
                add(0x62200e0ed1667d1cae87dac1a56a830b28a92d925ba8a1072ac6809a4b93e2e, mulmod(
                add(0x48f9008e13590406aef8f0fe6dc15d717af49fe1f6601553aa59a057d670fb6, mulmod(
                add(0x1cd5f46b49799927dade0f30ce81359c2d6e321cd3b3f1e92cdb3c8393f455, mulmod(
                add(0x1e26db11f193e1861e84a2450b766bdda85da527ee9a851923ed53dfb314821, mulmod(
                add(0x3292fbabe5b2eff0a6b7c566e03295835211d42217ba2ad6c3634dbaad6993f, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x49f6e5e7dcc1927589a96f7b0fbf3d7c109401dc50da88bb5e1e98d0ac739f1, mulmod(
                add(0xd4997986d51d092510f42ff8c023f4250819967f8bafaec0486e7c47e2a946, mulmod(
                add(0x7f869d50581f95ded10bfeb13ee1b530dfb427e3fbb711a2d38c43f04b2b2f9, mulmod(
                add(0x2430121bbf4972c29b4959f648b88f40efafcd0dd7e9908b5ffd8a7dfa7e129, mulmod(
                add(0x4a3d4d79753a6bb761e83914ed535800941b60169d36611b75c7c16a76dc645, mulmod(
                add(0x567a89c34ed8f409b85ce337a21bcb0daf544f79c1366fbd5dcbd8953a733e7, mulmod(
                add(0x34e44c8015c1b51ef7f2cafb04671248af09f57ffe0ab67fcd4fb80aa166d1b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x7966b0d4af2b045fb94e81c186ddbb9ac685b97b453603f845e00d6e87dc8d2, mulmod(
                add(0x24fa9186ae3ac0def7e9e9db06b4ab4971ce7ae46b5423e8947d00b65e9874f, mulmod(
                add(0x3f6a7c5aee7bc6228768809d2e9c33995821acf813837ecee65a8b469240e63, mulmod(
                add(0x2306f73ab4dbce371f7e6918891fd82e9b8c61a3c1a2067c56c515e351fe8a5, mulmod(
                add(0x737f914ffbe55b4c2e9b33aecda1ff937151ef817d70a3b5d3e59190b2fe3eb, mulmod(
                add(0x2f9d7f34d6945a91cb5d676d2d5859902cf78ee842e1ceb8700478a26a624c8, mulmod(
                add(0x3641dd0c477f6c783f8ae45a238c87843b5e9ac9f9eaf549de4f496af639664, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3e7e030abe16ce0a443285e4c3e850cd31aa67649929e16247bbd800a5684b2, mulmod(
                add(0x491db605bb0edda78c134558b96eed5eda50e485afdef6f5352d62a671b0eff, mulmod(
                add(0x2e784ef1077ad6552022acd8a5ae609150c0559c700439bcaed18bb30b1d8c4, mulmod(
                add(0x6e50e377f69fe61f5f7b2eb7cf978b86b6cbee1f2be0ab75bc41048cb8c9437, mulmod(
                add(0x1be09d2fdbe7589222be9c1d6543b9904f80403b77c37db37219d192db76e3d, mulmod(
                add(0x4cab311c825005ea40a9a7eff5686b8909b3fdf34a64c1d97fc03b1988e1f07, mulmod(
                add(0xe2dc556d532b7e4bfd37a1842502812f2d4a3b7939a569af4d760dee5eeaee, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x173170e1dbf7eb799f7392e67b165dcda70767af0885ceecbcdbbbba25e808b, mulmod(
                add(0xbdc33c882cb6a16d836fcb476266edde746cd2a6d3c08c0d15bbdfa6c64143, mulmod(
                add(0x1c2cb45a877fccc1b3e5d3a80962cefc7ca2a4c746dbb608ae4b42153159359, mulmod(
                add(0x7e89600e5aed57a03f38f8e809b70501fc89047ad671d480bb64d793524ebd9, mulmod(
                add(0x46cce8fbb386bf465d742ecff28ed64e8c4a56629be4fa4bb00af0967821de5, mulmod(
                add(0x48ce847fabd0b122f35c696c4d4350a8118fa344ad02c5855a06ae17474bf26, mulmod(
                add(0x3fe196515c7eceeea3cba01c7e1792ba8e5f9472a4af667b3bf2002a77ceb84, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x70f0e768de6ec42afa18cbbcfda32da16adad8facb7b76f2a141603c3a2f23d, mulmod(
                add(0x23d99653e3398d4429c872f581807baff2c3efa190a79cae16e17c198e01606, mulmod(
                add(0x2d665b7813839767669bbfc93c3b195b42e943da3400b587d7ea0f669465865, mulmod(
                add(0x4fc818bd8bfe5e8ed7b6d24b8b8815aa27663cb2ca5bff2c16a1f2a744aa9c2, mulmod(
                add(0x423dffe9d134439a1e66586888f8d0ea8bb37b85f33b783aa414e7341d14e9, mulmod(
                add(0x313a50eb66f68a5707f3f7553af362f2ae0c6a2e26a35715dd1dba2df79a9db, mulmod(
                add(0x564dddd0f310e4227f76c4efaa2cd8a0a36822578988f854cf1f2c4d45c12f1, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1ccf84ec98749e4bc4ec3ad4493ad578abc9f79f92e62a9bc10383c2649f335, mulmod(
                add(0x1eaead52d5ef47c3a0ced81ffba03fae9e82328d8923850be81aacfebfe1e0d, mulmod(
                add(0x1387097c0cc5963d30fe5cc7620a92479da2b2b345f7495f592df4c801f636f, mulmod(
                add(0x2a975c44f188c18602968f6080b00d939e402f31f4cf0c16f8ba7c9d1212e17, mulmod(
                add(0x46cb36e1f90c58e1fb2e5ca92c91a4e62ea7dc569686b6faeb6cda153720744, mulmod(
                add(0x5c7a483ff76fd1549eb5f5c496bb7fab03e62d1f924a1d17119b87caf9ee5e1, mulmod(
                add(0x2bfb0c99daa5c379c76493a08a3a367cff21ef86d0da383f85bf3ebb37b3c5f, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2680d3f26866d0a8e9a4d5bf0164dcd9d5156aa0ca38a51bb4c47090b6d32aa, mulmod(
                add(0x1d6fbdfe2a0bdea254166a6657ac56b0b376a17600ec3fbf172d19de8ee3062, mulmod(
                add(0x55940a4f686e51d1ceaa201de4155ba309adb14c2401bb3a4867a6a2c5094ee, mulmod(
                add(0x293b9b51d65b55d0840530442e15257e29d2ca96ef43d08cfa1d7b7ed9ce636, mulmod(
                add(0x1936561497d586da6e146791a560a4c9612a472f1ea651bda05a8959da96554, mulmod(
                add(0x64b8444ed4a096de1faae4125a6719359ab86161b888bc96c70cc6ad777e270, mulmod(
                add(0x6f9d8c8b1b07e16b153afd5bf4d11221322d90da44ad71382ae1643a930db0, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2412b831bd6bbdbab5fd773ce1a3984a9ad5e68c21efcb438bb9be900696594, mulmod(
                add(0x6be42da3cfb42ad973c226b9b1c07ddc087bedd604aa9852e1d883cb28d8859, mulmod(
                add(0x1802c63b8e6f11a8230cffbdbc1a9b26d090fcbda38e215f2e3f96b56038f44, mulmod(
                add(0x20b31a9b34a15563f4c59c59a8b9afe1a76e4783ed54b9050203e099559d002, mulmod(
                add(0x3571e14b3a39cbe7e81eb3ab0e20789784bf06e4e431886070e001bf51358b4, mulmod(
                add(0x135de021a4685feab0d9d5d46bd67cd5a69d508d0252df3909b28acd489041b, mulmod(
                add(0x60037f9c7daeba565338a718f3a32048932b2ee286ba6d86347a74cab1c99c, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3756e31248c97c3aae74da20c57669c1b90d99d26c6dce067ed9313bd846d0a, mulmod(
                add(0x35e6d15dbfadc45fce62bde7456964da04b6d8975794e222b9bf6130aa51570, mulmod(
                add(0x6b46133beba1cccb3339e99d96212e1e0d48cac74e69ad879c2d0b908bc4e8, mulmod(
                add(0x774c19208ae8e08b28b6ccbc97c46b56037daa56726e8d52b85557d2619d5bf, mulmod(
                add(0x3988872c41fde316da865063d4645d0c4281d3eb4a5b081765156d8013199cd, mulmod(
                add(0x4f212508f137d1ecedca957be8d47cfb551ae03aa9c0dca5ad894222ae3f303, mulmod(
                add(0x3a76081ac28d0ebbb4d00880d9bab7a19b405b1af6a0645890930e7f58e434c, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x2043ad0ec01e4274d3eb485eb55fc96cb668614a796b85b94d9bd4f0ec04197, mulmod(
                add(0x2707a7b8660e646e6de4715aec687c28df6d59d6669bd9f86b90a997b4083f1, mulmod(
                add(0x19c51e81eb35852906f49f49c9d12467d6f0d3353e3878acc72ae5c0edfd08e, mulmod(
                add(0x3d3e469e26ea33359baae5c99b700148122bd0cd9daaa052d502f5c3350e974, mulmod(
                add(0x2820d56aae7c8959942f2d6b49fa6da3f8049f0935e99e48b727d99359ab7b6, mulmod(
                add(0x56840f587abbbab9afa770002b9904aaa717041cabd405520852656b046cd5, mulmod(
                add(0x30ab73aa5adaf303d834c464a43c87e79c545d8da8664eed1f137d2ea9e2ad3, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6c9b38a57391b18ece8a7263b1bc6bd2620e30cda6d89f2d1fd40b5f712c68b, mulmod(
                add(0x7382ba2ab198ff4ea770a82556575ae45fb64c22f63892aeba494d68b484add, mulmod(
                add(0x315288ae55a9416f263b7bb13a750628c0b078bb3591a8d9ddfb7796f9a7e8, mulmod(
                add(0x5a9fdbeadde70c463ac6f2e39fb8b494d07c8f8c4cefc703f28d733adc688fd, mulmod(
                add(0x47447d81c236bf550cc835efc782575e110bde29b3b7e8cb19d897146a863fe, mulmod(
                add(0x4addfa4993b18794a561eebd1878170493bb17af9cd8c30a4dbd9f4f455790b, mulmod(
                add(0x36aa2e9afed6c16e9d2322d9d08a922a986c0de754344275f9ffa66ee12bda5, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x4561ecad6fab60d69121ed47fc409cee04148b344f74f8c59c3f0372797a32b, mulmod(
                add(0x3dce0157a238c08b73c24750918927323319571f05b3ddb211db651db31f707, mulmod(
                add(0x2ea9693d28596311a05d9f701b2511b7e53eb6a5ecc8639504e320db8fe4faa, mulmod(
                add(0xcd35e2b098979c5232f69f16b47d841fb0766a67df68ca6ba1da7f84f6e8df, mulmod(
                add(0x35865c8d8f829f876324c24d62d61ebf9446a33c3fc41fbfd6798daf4b97325, mulmod(
                add(0xf6b74f5964f1e1415d3ab62844660aae46ba37b9ea61469daa2d6c667c1e12, mulmod(
                add(0x1d8fee37a4407ab26b2e5728f0842f40777a5e26076060c6bcbaf1ec5ebbc89, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x764ae1c95b127fe1ac1662abdbb90007c213fd2b62ed66e13250437c1746fe, mulmod(
                add(0x237e9f49c0ec039818c5b9fb79f23626741a5c542044219daf65912053b7a55, mulmod(
                add(0x4c67e4cebfa390f0e2ab6db3f5ac8abff6c4dc64afd01fd7e5929252e573758, mulmod(
                add(0x16c0029fbb876e0dc4c41f3b7f22fc392ac271e91c4551f09d5959d5ecbcecc, mulmod(
                add(0x537a6d40e4004af560093d75417c81e16facf56f7b1a9d74e6b1dbfbe9a507, mulmod(
                add(0x631860f2b525ad2bd160c32df80f3557d381a6813049fb1e50472a3931d1f0c, mulmod(
                add(0x6c7e6586143ff913ff59b8de1505ec3c01b3748cb0eb56db3db8d16d4229e1f, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5653477ccefc1166bcaa34281561e343daaa3619cf7891ba07e65250fb4ab14, mulmod(
                add(0x7a61f01d4b2cbfd4718c512bf538dd490e414b47ad73b2dcc03c0751f6279d3, mulmod(
                add(0x33fb9f77fe3b0d6d5896bb0d11bb6375d6e00a7c85af0674079e2076a11a7ef, mulmod(
                add(0x8cd29a69168967160af7ef512f31ab9d1d4770162c760278366bbba1056d8b, mulmod(
                add(0x135a3d019e8de537562bd5ed20d483d711fbee22b58e4f10f894d04d9fe0f57, mulmod(
                add(0x6b887e47668b3bf9382321d4935c2b2bcf0ac71fb2a448711502585eb0950cf, mulmod(
                add(0x629a2bd17cec72371bb3308bf174988a785086a48594975446f7827e1c6edbc, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x65e955d5ec64f61d44a2fa624ff34144e06ff1356a84dc0d57ad4c0c6d05c84, mulmod(
                add(0x4026c0140feb5aa9e939ae86ea2e4ef2b80396a89d9d262a4384cd36123c684, mulmod(
                add(0x67c21f172629aaf354527cd2d3d5a897c7ec340b68893dbc19d59bf3cf13499, mulmod(
                add(0x421952885c3c6f007e2d8606e7cba39ab99f69b29b924fa0a4277b2139c1e78, mulmod(
                add(0x5ce5b9d1ed952873f102c102dbb70f7cc1ebacdf4f70be62c4773634400b6e4, mulmod(
                add(0x250d6893fbed09a3f40a25a32833bf5647c9a9c67889b7f50834b4ebfd411f1, mulmod(
                add(0x2ad2a6263575df51285a95e036a193805ffd4c82582367e6c0342175f496631, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x5fd705b0012ea9a1aae07588bfaa11273309e45ee2b3720303cc8a4431ec9fc, mulmod(
                add(0x452880c65ecc42f157ca54d2574282584d9ed0dfabf539a5053b306aa784b88, mulmod(
                add(0x19976a86f7e13a62cbe1b261d3ff6ad6a0b5f11468f434915a78a34df1bb5a8, mulmod(
                add(0x71428fcf3eab7213550ebec82fc9f019575502f56f584bf3474f30fb3adf128, mulmod(
                add(0x4a5e8b4951b5432873eeec1eb1e655d94695a1bf0852e40efae8d2e19fdbeec, mulmod(
                add(0x7906191dccdded27d02f39d56f8d726a5da3e9f1bdb684d8c1cc26f01e13d1a, mulmod(
                add(0x282d021e1a0a3db422b7aca23d0c25c9b06d98f0cb9ef5147d6ac87e3035f19, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0xc209de4375dd0a813614527f66057feaad7223541f7c8de0dde58caf78a3d3, mulmod(
                add(0xf5bcddd9d411006217a888bf1d0c074bb748f6a799f2051d365d733045ec87, mulmod(
                add(0x6ca3d4983ffa3e8723f75bb37fc9eb7706fd6647082c281425287b8dc566a7, mulmod(
                add(0x89735a82e7274b40f88a5f93b9ce7719579fe21e4f20ce44c946c50a700804, mulmod(
                add(0x4fc8fb39578c33d05fc6539d6c2fc8e3428d3e55a3796b31a00cfe1743ce26f, mulmod(
                add(0x30cf169547cc81093e7da99e7924472e30332312ee82385b7a3206ca02a672a, mulmod(
                add(0x5fc4d6f2d2f6d80b2d64b3976df55032c9606b8fce079401de09392c1cf7c39, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x66a735e554574d3dc04f5bf97f76a4ab6de025c5f80b503928d8ccc31e9ac47, mulmod(
                add(0x6234470dea6443c0cc6e0ddfa20b7f25ebd801e2a25f3f3fb9804c01f17d30, mulmod(
                add(0x71e38d13f1d0ab52191fb782022ea027268f3087ef51b3ca6387fd79b9b53e9, mulmod(
                add(0x227617d703606d033eae7aab69ac1c72f9763673130e48eee78879a4c750f74, mulmod(
                add(0x552b0f6a0fca8334455161be4ee6a5ada33c638145f14b2fb6b57fb7f035cda, mulmod(
                add(0x1153b7061175722256fe6c82ddc78ff4cc0d779046ac049cd4b365dae56d172, mulmod(
                add(0x558bbcb8157c99fbddfd5d3d8ced71153e8560a9bc2ed593fafe192720936fc, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3dc31ffa5e9dfec9a91dbb8e7c6ad245dd353b6ea7bf8e6ba6cde90ba40a2d6, mulmod(
                add(0x10cc158b97456366aa5ba3a0825b65f683c8505e639b27914c7d8f67511d0cb, mulmod(
                add(0x5ea307039ba102694e58162d2a061f769380081471a1fdc819e44242a530986, mulmod(
                add(0x76862a785b882336c004c91f1a9dc0a9ef62ac0023bff84e58866da8eaafbd1, mulmod(
                add(0x6275ff8623e9e6548a9d40d817765762deb504208dc386048a44dccb879adf1, mulmod(
                add(0x444091e204efb956ee8ee013ed22e7decfd7dc920b2823d8b5ba9d4e5557688, mulmod(
                add(0x20cc179d999de1ff880e2616b77281087ad09e67ae1fcaadde44dda96efd79a, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3a3edd1ce8f95d6ad2818ec77a209fd8318f50f80bbf1f5713609ec52e99f7b, mulmod(
                add(0x2a635afad51ab52b5f7dcf829cf0af9b29e8b402e0900f3defcb608a92f9e3a, mulmod(
                add(0x35f37f06484bf6baec1414a783de3c78e08e1f4a0553107edcf9382ad8e9fac, mulmod(
                add(0x611cb858f099455bc4390a441ee5650b0fc4057c0b284c3e3354a3b2d024e44, mulmod(
                add(0x345505eef1acbe563e7f9d80633363406d01bab798bf96b76eb441ac7e17de9, mulmod(
                add(0x51c69e10f4d89074d3b80a3ddeb47a3b7e454f32ba4a209e53ad687e61160b, mulmod(
                add(0x2edc355c98f029cf683799c6627207b44796d1bc8db98398ebfd720fe657f2, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x26766914344d8f6a768786bcc461b707d64c461baa813d1a14b03b9b970236f, mulmod(
                add(0x617d7513a1af1532fc5a46247e9808f812ae72b0117be1a2cc8d7aedd552829, mulmod(
                add(0x192c148337549ae70fca36cf2ea0f464a4454c7dfadc2d167886b0512be8a8e, mulmod(
                add(0x7796714587f837dfa94eaea592cf5a23daa638c5c797b1d93a17f3ac0a8621e, mulmod(
                add(0x2de11301538af15b6912114fc4da38d617ef2474cc0ea8dfdfea1372a09f17e, mulmod(
                add(0x855e7b0b87d834863953d64f97fad0cfe70a72b61d39f5335410980da5b5aa, mulmod(
                add(0x2fc4b1a8813a260cbef0b42d9040e2968f548698819e44d8040bae00cecf328, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x78fcb75db89388c46179aa063b8964c39d819f3aa2a8cb22967de38a0a35a73, mulmod(
                add(0x3b3aba96664091cd5e6d0161df4ee7f745e7a490eee50bdaac1239bfce97aed, mulmod(
                add(0x280b578d53befb0da877432c3975cb0edea2f5e73ee903ac45f9fca15b1da7b, mulmod(
                add(0x3bc76a56046439cd38ee5d5efc0ec010433f0147bf5f1ebaa74effbee9e34d9, mulmod(
                add(0x1d8b5fd6389969a08852aef57c6de2024c90e3202466cc405a5c11c7f3187e7, mulmod(
                add(0x7cf2c7e300297190e692a6cd7a87288dfef5fe5ec9f969f322b971c5e543f28, mulmod(
                add(0x22d698208bade1663bf3457b2fd786e7d52f590446fb8a22f4bb2c7ab1790ba, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1286820a5797b089a1cfd943c04b70735943868cc4c2fe86c2e8422633661af, mulmod(
                add(0x3d8c1789619a89e49a5d0c42247f7065657f1c5f7aafdc26cb59772564b8a90, mulmod(
                add(0x3780d0ade79f7ad3f3a6d3e08a9550cea709f1379a971da697cf64c1010adf4, mulmod(
                add(0x47d64e6c2415a43b703dbc5a14608ae8c5a6f05b56512caa4dca4fa089b9b9d, mulmod(
                add(0xa90446a7c15e4d5e92e19ffcc43136d543d76e072c716dc2754209040801a6, mulmod(
                add(0x1dd57384c0383896363ac913d802cffeed74d9016f1fe0cc455e0be13f82262, mulmod(
                add(0x6d786501dd0f3b47385150af7ae394ef0b3dc7bc5130d4f74bba19ab9bf91be, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x614121ecab55682694a35d9f610ffafef75b463f8715f1b57356102bbfa1d43, mulmod(
                add(0x2142b081b9a81658dceb9ee8e3e0bbe9db3dd1ca071eecc7c841f164d44742, mulmod(
                add(0x718f309b35dbf217d8064c89dd2ae7b1b27c3ded2eaa4d22d8fcbaad959a52e, mulmod(
                add(0x34cb48aa1e1ae9a347f3b5b8c6ea47b88f376df1b42380662666994af968a8f, mulmod(
                add(0x6534e7966156435257e9c876fd1b588cbfb2b3754c679f59a6d776b5831dbee, mulmod(
                add(0x43de23790a125cfd61cf43bc15c3783ccd917b40ea18c755624ad73c661e66b, mulmod(
                add(0x7ee45e8ee67c1abdac1d5b327fb7c6e271ef4b81a8818b24c44a5c398d55545, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x6118643bd4e42614c55b4f5d489b95940288027fa08fa1062cac235146aa6a2, mulmod(
                add(0xe87a6ecff814b6bc457c10aebe4ddff65fc88caa8445918a2dd0530a442dd, mulmod(
                add(0x6c663d8f7dd914de8842732c15ea8f0f11df9054110d39ad314bafdbbd00a91, mulmod(
                add(0x77bf7abd99cb9f89cefe82dba148345620a2949cbe31d15d3e8134ff1fd561e, mulmod(
                add(0x1a50d0d5e1b8e21b4af8e7cd086e94252e0f4c350b35f8ea8d781d5b40a2f9b, mulmod(
                add(0x7d0791997cef2265e29a2407cc3f6e630dfcc43014a75c7e7b3a89e3000f8f5, mulmod(
                add(0x5481fafa7795994981dea40bbf5048002f8c52cb9ae2507e6801b047c856900, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x669d7416b479f34a4fb615cf2786efabc2ee020ac4bc45d1acd7f2718be5565, mulmod(
                add(0x5bfd931322b9c8b06c29277a4d9542680b6a3f2fc60731ef57f6a3c3d68e05b, mulmod(
                add(0x2e0a9130e73039bd446ded5a8aa2c6b625cfabac66649a32237b11b4effaf09, mulmod(
                add(0x42b626fb35c0c361535fdb3ef9c6f10e9d50403ac7a1282aaa793d3d20eaaa, mulmod(
                add(0x6305b7baab0c629ef1249342c43e0fd5acd55e10b3f294d4aed5b90ee6b08f, mulmod(
                add(0x2d8b681523131c2a5bf60555ad4e1f220cbda801d19b62fdf3d325be037dd0b, mulmod(
                add(0x68e0d1f6280a66c0872a515be49cf3968d323b6c47518dbc44be2521b55e5b9, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x47c0d70c92908b98b18b4a56f92d7b3c79c63b69652a558e825a79fe6ba3199, mulmod(
                add(0x22999ba6ed105495f995180294bcd2fff4418e9ac8640eb8f2a8339d6c299c7, mulmod(
                add(0x591b715eb381d73deb97da6bf9e8bfd035b07e2c9c6a7c6a1ed4891c8d2cc2, mulmod(
                add(0x2cb8776daf6ef66f6fac81b14876f1a7131fc3f823169422c978e58e2188bec, mulmod(
                add(0x4559e9b86b12dac0b2f5ec9146d4e7232c158063dd6f5426bc00bf75f35dabc, mulmod(
                add(0x4ceb81a18bced3c410638175bad00f2b4db3d7651c28ddbed31e15cb1eadca0, mulmod(
                add(0x69615950be6baa2b1922d48ab3e0ba95a2cb5afe252d8d6d48e552a2f0415eb, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x7b62f603a16fff26491b5cbf4bc78b68b428a75a783c71fbb96a8c741cc6094, mulmod(
                add(0x2a002cc03c12557639154a56793c8b1241b56379b80fb8a3288bf91cfa799b4, mulmod(
                add(0x1cc6cd6862153d8bc39b47538087047cc5b38afdec45ef8c08130f6492511ac, mulmod(
                add(0x1b2bb56142c23ad7f62f4031b9bd188a1ce5e038d72eee787cc2b07db74510c, mulmod(
                add(0x4447e77ed01f6ae0a4d3824209a92a0606ee3427b4986d5955b56a8c0e6a280, mulmod(
                add(0x11e5dc7715fd5131843253d42212797fbb40bcd8d14e25abec2ed315884974e, mulmod(
                add(0x62d71a31fdad1b78ec77a838e89640372cca12fc4e421903834566eb75f3cf6, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x398e8b8ecbe6527b4f965b0aa21ab9ca9cd2153e5dc4c4ae7984ff948254f3, mulmod(
                add(0x77020d9b5b2092c42484f59e7a9fc59cb6951a0c7f895028c049ca8829e58e, mulmod(
                add(0x332ebf59a625a55bdebfef011b09d1266a28379b2813031c8d7c3cebd74094c, mulmod(
                add(0x40088507438b96ec8d4775dc2de32cdc61b17be89bad7ce2107fe2b9b43cd72, mulmod(
                add(0x61c1803b8aad70963b8b59a73d0a8959338568e8a8b817588005e96468c1cc6, mulmod(
                add(0x2095f025b2dcbd657387a2f363d31842e90012f4cfc4e05988b437c328d9e9e, mulmod(
                add(0x443b9b9bab962a5d0d798e2cf95aa97eb8af3d0b120490d664397735a5a54ff, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x63c92da79b21f36ca983ac54157ab7c94aae9c5a6c022f058ba58cd68c00fdb, mulmod(
                add(0x7ce35e131d3218033fb805240c209d692ee267bc9f73bb7ccb94636b13f2ade, mulmod(
                add(0x666f2c5f85b167a09b8aaf6dd213d2973b7db0a673d2e54b58be947cac36ead, mulmod(
                add(0x230d574a45abc7560299feadd12e3dec99654cc63b6cd1b3b3c3bc2a2976287, mulmod(
                add(0x41a60e75b68b2c0a8d53e40da2883e393a9bc01f69e9698a6baceabf28b20d1, mulmod(
                add(0x15aaa68263f5dfdbd7040a694bcb7c71bda244c1417aa610266332f19a41c11, mulmod(
                add(0x7b2a7cd90e63b8fe7ea3d2f5da538acc69215547680a0675911d5ee194f1ed5, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x386095e9a7206b18a692ee480475405c7895fca0fe1632aac2519b65f0498f4, mulmod(
                add(0x5aad30772231eb8b5023c74da6fbe11f1b07732aa9dd2e4b495d90505d5afb, mulmod(
                add(0x6b927eb6bd570fe49c27b8d515281801a59dc04e8c5f13ab71b41d06ea2b821, mulmod(
                add(0x5593761c8332d745c5359aff586b92edea39e02e570c6c538e4dc089b5ff0d3, mulmod(
                add(0x55d43c373778c5ad37165493923ba5a51fb285b6c0ef2a618639ed1c1337a11, mulmod(
                add(0x6462621c9019d6bbdef8af0476ceb8a46a42df827e245229f7d703a054a16a, mulmod(
                add(0x6d913791c0feb3c00ee06299bb863c3a1ce09e6da2cc378b84476ae9e2a8ce5, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x67926adeb91fd7fb37ebe0869a9bef15e78e3c35a71adf6e1cb914b1a662772, mulmod(
                add(0x133065cf342ff43ea247acf38d197a6e86c07266371044daa65c8743417f7a6, mulmod(
                add(0x51623187863ce5e5b40f6ca0db0525264c8acd4d7516b89498ad8d8c5a1d0ce, mulmod(
                add(0x38418ad02d08d2bcbb08ceba5276ca82e77abdd3d5832cc97de7fe989e85016, mulmod(
                add(0x129493e8d8e0942b1a413f0cada530ee8275dfc3ba0756c77760147722fbd00, mulmod(
                add(0x1070028baae7a5145cffd44b4f59f66082689d04dbf90c23174b699104caee, mulmod(
                add(0x3d97845cab79f03680aefc5d6a26ac310f2a6edfa6aead94da5d5c963ef2de2, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x37130bcc92f4b744055d3236494fcefaf2ec3aa6b0d0d24e3e7b0b85dfe8081, mulmod(
                add(0x48ec1669da828a7272736f502273e0bac0e53fbf0d5a85d4e62e0b660ee28ff, mulmod(
                add(0x11da5782b822707b49fc301ada53925c2e320e7e7facd3b4da77bd51ad1e5d9, mulmod(
                add(0x6b7178f6fc4316cbad7d5d922ad578bfcbfd7e7b6394102ed9f9e67c4fa5248, mulmod(
                add(0x69230f0b4786cee1db98ca0130fc6933f53396a4fc7a3026adf73b7a5a571d7, mulmod(
                add(0x4461229861fd99837544eb8e7c8a72f7acf0db44bf5701f30dc3e1bfa2f6430, mulmod(
                add(0x62c0c342e82edcdec576ac158be70541e8f4485fcaaad0421e2aa0457db46e0, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x14eda602a35956e710cf3b2828f6899ce8db8ae380d6b08a1aa0818f925b254, mulmod(
                add(0x5d9aa0afed1ce4e9805013ac65af8d621520a9f2e5a9ed9eb59d687675855fa, mulmod(
                add(0x74796a4a8bd315ac3d19e0f5187fe866262d4dc8f23359ff61d61272b39cce1, mulmod(
                add(0x7fe1e8d12c9780db2ba0c0d1071d32b4eafb496deaffebfbeac548317cc09d1, mulmod(
                add(0x5abe167ce98920884c6c28ce9d0cb0bf9f044ba3b72e53ff09c27b1a16035e8, mulmod(
                add(0x38ad01085cbb07a25fa775f19fef1e6b2e3a5e747d46b6aec696c2da29b61f6, mulmod(
                add(0x12925a7141881081364e24465ceaccb1eccb74c5c72162161cc8fa269a2b67b, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x289c729f6fb98c2eaddce0242c5a410a7b0e1c9ce283e396aa9e45563400f93, mulmod(
                add(0xe85e10e250d3c828644826c5f6f700a4d8d9d050e2605b2584275f40130d84, mulmod(
                add(0x74832f998e8d85031f4ff83b6d34b4035fd6f35e6aca01b400885b0a889a904, mulmod(
                add(0x6e46488dfbd98ce0d7c34dc5bdf7cd1653b55d0945507fd94131d2e9e6bc20c, mulmod(
                add(0x6d92882e2725989c3b570c58c342abde4d2470fbb98c2d31799ff5003fb8ba6, mulmod(
                add(0x780b994cbc4d5a58e84cd705bc1babe2ad75db76722bb933375107afb4702bd, mulmod(
                add(0x38b0ca611df09fb0d7f8a94c9311ad5e55775adf569b36e0f5dc3f660f9e22, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x70f991a1bddf68d4094d5ee820938053593ca8c8017d062e9257832d7f8e986, mulmod(
                add(0x238923279fa0ac9c95c8e62df2b98c0cc1fa4754abe2f2f7979227fc2127f78, mulmod(
                add(0x430af73865976c509e40a8dd4019d068b140a9d268124c1b204543695ada9a9, mulmod(
                add(0x639d51d2691620e9fc105df745c4116526f0e5d710e727f3c4a2f9ec06ad3b1, mulmod(
                add(0x601cf5e9b7aa51251fc4e9a3084526a005a88132dd6a3b57a42552306e13e5f, mulmod(
                add(0x20f3e804575b24e0468cd8e739d058c973a6e911777fb4ee60dfada70fd5749, mulmod(
                add(0x709f19a75ffdadd2f857c55c0c6a89bc023662d85b08e33c6f6624a091fc8d7, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x3295d0eaec01b0967c0e860aba89559e87e49a33650e2680a2fac10ffca3c0e, mulmod(
                add(0x61100e0b3a1a6bc1651f4053bc78526dbcd1cb72f5c66a2b51e8f5188fb628c, mulmod(
                add(0x422f8dc3f9be2e46a834c8e42c22c562025fe09945da2ecd6649bbd66caa87c, mulmod(
                add(0x20a1f02cbd026c9444509c56c06b34bbd08772335859d4b39bbc57583eb836b, mulmod(
                add(0x32c0912c62c3d896652bbb2b02e3de4c5a07cb102ac981a4ca8ee15919c59ce, mulmod(
                add(0x203cde2cc62afa8d0ade6eeb8e3636fd9ab65be0dbd13f55ced174c59a1f11f, mulmod(
                add(0x7a1414aa6d147c89d6f7621546fba717f4fd78abbab47b77476c9bf93cefba8, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x1563352a5af6dc8f6339357d3deef781e0a0a29a3e9d25db35b837355f1c3a4, mulmod(
                add(0x2fe2f165e9a5354f4f3e0da4669829e12aeeacb39c1d6dba916f4693b179f0, mulmod(
                add(0x36c9ec85427320f62b00c292980c02bade29102b5ee9fe81d8a0165f1ab92bf, mulmod(
                add(0x27d2a9a2d31fd2ed801d3ac75960c13a1724831d6a8c4428b49807e03eb42f, mulmod(
                add(0x7126fa704bc54e852603106c921c72507420415d454d199e5c6aacb705538ff, mulmod(
                add(0xd34f514e1adec8b2ce6dda3a8342f3dddc9e1424f19239cdc8b8b8ecf402b1, mulmod(
                add(0x12abb9d5310525bd0e1bd2204cdfb4fdb6d52b978b4c6c2637751d053f27995, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x32a63065a7e21c74a934bbb0422e6736d337bfdafde7471f5d423f4b61a9753, mulmod(
                add(0x523741413f7b2a07d7e0d95722c60d692618a8d6bbc6b8ef526e618d070fe8d, mulmod(
                add(0x4ecd1f186da05c52f0f0a84bfd27af2b752c67cf88435ea7d24533719ebd8fe, mulmod(
                add(0x7d5ec6b4005e4535a61267146fb3d54e0cbb3480b92a7cd72c7374f90adf8f9, mulmod(
                add(0x31992d759eeb77a458be2d1be91d91131c99fe4bdb1b0607281432aaefd4996, mulmod(
                add(0x6502f780a400426e02ef8c55664ecb5eeaef3a24e0fdc9aeb7e22cbdde2a652, mulmod(
                add(0x7069d52ca75b62107fa5a37a37a2c5d4a17c809c572814238783a3d89bb2216, mulmod(
                    result,
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME)),
                x, PRIME))

            result :=
                add(0x538fe2aefdf83883ae888822bbaabf5fe590adf209a1d276047d33a45c213a5, mulmod(
                    result,
                x, PRIME))


        }
        return result % PRIME;
    }
}
