use ecc::Affine;
use hex_literal::*;
use primefield::{u256h, FieldElement, U256};

pub const PEDERSEN_POINTS: [Affine; 506] = [
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0463d1e72d2ebf3416c727d5f24b5dc16b69f758cd49de911ad69b41a9ba0b3a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01211aac6ce572de4298f85b038ef6a8aeae324054290152c5c9927f66d85eeb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "033840300bf6cec10429bf5184041c7b51a9bf65d4403deac9019623cf0273dd"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05a0e71610f55329fbd89a97cf4b33ad0939e3442869bbe7569d0da34235308a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "035aa92df0885fd20732d950000368debeeff4924de3c52831fe19ef7e807b3f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "051e9120dbb3de8a06055f47bdbf73e0b46ecbdd7b9728f2ba89f77c4afe39a3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0382d64c9967a1988b6346c265aee724abd5caac208ecefbe4deec837f33b9ce"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03f6c38c2c154983480b2d54a9d5af7c23536cf1caa6a1aac199e12f5f31cb95"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04b2a130122949c2b341405f34f13adafc851cc69e25ccc010b47bd849ffb510"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "023c3adddadec10c99e3e86ec55f1fdfa2f96150f926753cab6d4ef844e2ab95"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05e7a88386446f6c042909de8ae81d919400fdfe2acac5eb0fe61e5bdc6b2c54"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04cb4faae6091a1453bbfa8676fe82d43d75f33b9e7f39ac71a19251fe20ecd6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0189743ef58b69cf5b1b65934122aa88a661680582116df6917ab5089d65ce0e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e92d38b80106e6d9f9170f41c76f9434d59cf31be604380bb9d2c307704349"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "062138624eae52da85b6cee9f0578f5ab06e0cb3fcfc5a291a3ee192226f36a1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "036ca0665864bae3a3313518df2f8a909fb4e62351c39fa9969073cc7816c5c5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01c693230c548b36b96f429ea47dc41c6ce038821a347c1b6fa30c96b373443a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06cb1bacdf31d1385481ff8380a6441af9dd7e01d1d62237c1126d6c2c881af6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "011848c028144e27ea10651534475c786667bc952bf43d5c900ab545094151ed"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02363bc003bd25384a6377b2560cb5b3a1c37b0533a965a3867baf3586fcbe48"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01e3ccbf18e066b942f0773cab9aacd663e827fdbacb6aa1ed61c6e6afc63173"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0752fff5264694433460de0ae07d7a71a1914a77576f4b772bd43e063deefd90"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "068fc1e28d46978d9f31fa1f47963922f5ed91e925553d94c0966c3bd0fb94d5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0630f319865070ab597d87ef212678386e95f7cd424f549ef89ddfdc8c76263a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07e43a6e056b1b5a897f9e04f9cf17c92b7870d20ddc07f0c289a0af87062d34"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0620c7da55deec9d5e375e3887bc17c761944f58f53e4095695b9b27298ae1ea"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05d7ea9384341f2e1b05206549b94da97bd78f6cc9bb80ba499ce132202a6659"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03a13073b4b28e754b03b21b7b5f52cedcf6e80dc79a9a63fc3854d055939436"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "056aa6d92598603a927fd32ca4a5c27a1e8291f43cd77797b5885757a15dc673"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07c0e780fafb764c5bfca5c9f1dd44d8b6bde924693f35b22d7019cf9f216a25"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05f5e2c7c24ffccdeaa84c5f9ceefccd4ba8978fd7bfad8bebd2578dd73f43a6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06dced6f5ab13d722b885e125b1ad2ea3a6c937c62d11cfbacab4fe993523361"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05f2c7e032de9c11bff5bbe43eb2137e8861daa7befa21aaf87636feed030725"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "012460409dd3ae55c89a45dfc51260f022a0515676f271691ae17d4fdb7e4e0c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "037a8da6821f1a849bfe25b7d130191ce376d0614fa7bc5e26693cff453bd631"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02130bb98747d7317d01f86c22d0e25420c3385b2649d4389cf5341aa822d6f9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "063a23f27d524ce163ab4677ea40a378fa48827f084c5f4becafd79af5bf89ba"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0492af6eb5b17e563af79f7d228232bbdc961ce4267e8318b4c56d92b369ffcc"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0553620c6bd37b38279f7470fe8842f35018edc1074d2e8232cb0c81d2a71baa"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05704cd109df62c8c2204ac0325b5ee6dbf29dacc0f751330b3a0ba805d796f0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07276cb41d4a11fea8cf1cf895c6c185dc97624e33a4143d003c3d345a569222"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "013bcfcee1200ed1a77c18639d8bc9ed7c4ef01b574e2faab4a72781bb3d6f2c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0508cdd61408657d31f8cd3f2dc2be630dca784c9880c5add45875bbe1709f1f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01652184eb72773cae8bc5fe6800fc2632464a18b035a601a4826fee229be091"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04db41000cd1e95ca95ec1cab00cdd060b68f79d8cf7f6beae650cbf010c877e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "055ceb6ba123498dc5b10eaaee6ae79b00c57823a1acae9ce3541b29d9198223"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04b1b5a4b5a995d6629645abdd636f2183cbdbad6432f70be353df9b234f9e9f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "018231a3f0ee55f59105053bd0286246f329df3a845b785883a7a6e762cf0b8b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07e329fa9eea7875673ae42d27e1d0ba7b57ac06e471fdcd1176fbc461702df7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01a38665840d8da11ed43d03d084a0ca1598aa55936e18d39c731e10e1b4bd3d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "011173e45947a36c1a6fa71c5047c2a318a2db6b7faf8e2303c438ff4053d8d5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07d1940611bbd6a1f42a57f5a1aba1f1247cd39685cfef08f7a22b2bad881f54"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "026dc800a3cf7b9450de5d8ca2b1c6661bb4dfa548e512040584bea077d2e150"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "073f85d1807dee49e1b17a6c40d77c3e8e64581cf9eaf9628cd17372e313271a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05c8a4a82e93df9eccfb62aae67cc55df52a11183aa2886a7b19277fc712b9ec"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0091d966b8d5df49b5814ab25bf34f817869db6e6a3ca478e45b5c68ceec00c2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0564bc93ea8ee6f76084b0f4f1830a47cddae7e88d7f5fb86e0b38f69c56b5a1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07d97aecdf8bb7889e84879bcf0f11a1db63813d5f23f71aeaa9a6eb2f34715c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "056abbbbf73aecb80a435640101b82afd67716c96aa97254d68aa4f8b5b057d0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "015abc011b92bd71cbc43b70a29781da240a4366b43a500634d9e4cc3a69f41c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01119e5ca9a890ede5d7639129a730f70903ca54db23269d034485b579629d26"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e7ac3a933572432a6559786eaacde3397aaa0246161e8749be25fc8e480505"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06fbb5d8d93b0966583b0bcc66dcca5c3fe162f9394ec16bee07ceebe774c7fb"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0125945a73a64b6414abbaa2ecf3aed1629ff5e063c16a89da7038f201b48456"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03d27bc02721dfade02e9d80ff837cbae33421200b1ba060a3e8f436d5c67778"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "026fe7a5f5247873295b3e8de7fbe79f1d2dfd6dfb01b90af83a502d4e946afb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03f3f9031bbc815076e493badd8eacd109f82fbc030eb92d50df3d1266550e02"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "078f8f9de3b43e2536f4c0796c1b1cb2bbfb5281b903703c2572b3840bacf159"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05a39969bd4344a549f3148fc9bb345d03dc68f2b96d753ecaa3fdb3007125e5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07c2b9d68641bd073def39b440205acb9b22f45d7ac3d6d7f39bdb8dfc7c75c6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06a172f52542ae6abea76194c65fce59c766e2e5e2c9261d28058708bb0f00a0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "031094c476b1e89094d636083f242d03752cedc38d9b7f6bdc6c56b6d9508e19"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03d9b9f4b8f282f5374f982457f7cd6087c3cce88568723e98c60f2e00f631af"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0112e7d11af925afd03a0748af6ed0083ca59cf4bc0e9672fd87da411579ad53"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05eae6df16b73910e5d3d5d42d9d45123a9dc49f0d897368441accc50aa68fdd"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0217640494b584362edc56f840452019e72f78fde5c9281b00f81fa42f4ba6ed"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01af4605e574affa0096b344b64ec8dafaa50bd03970a36bedc4cb5e7a9e88d7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "012387d4fd77dac6e1ed44a3ca57707bb17b5027cd1e249675c103feaa1c4c48"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "006e9f92cd3001d4d2a5039340a4e6751d2ec8a2c5fa5cc5b92789a38ce7c7da"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03bc8992cc4f1325c99af040f5c906eabbfbaebdc576847ed35277295c76eac6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "002a66012a95f32e800a8acdff8ffcee6e1e836160492a5adde5d2d052199934"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "052ce47887fee9921b02a781aefdb63d5282f70d56cd9772d5c218e192911324"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "028dd594939eda50510cd77b89a53cb47dcb95435eb2fe16d754b6aa934f70ca"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "018da35c66413ea6f14ab7fa48497f5217f30f624f10c861e0fafe70d18a8271"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02d8749fb4c7fa3c1a03aabb8ab7d5d2734f4054762b85d7033bd981e22ef07d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "076db42d876fd9502dbd96d37e8f067cc803dbb71a894eb0b4adf3c8a210db67"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04b9d7037e08db02b7c647bad22453551579a77b63c7586d34974381b74bd41d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0689bce06c1a76a86259389e4a14cc36c5c96d3aa0620061b136bd1ce1f94af4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0125d19adc8bf5e1c394bede2d93af7eff815ec22f654c4db53f15f84039be40"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00480d1963b0f26b2abcc0e91a4f8538abc1af45d4cfbd9af31467c3bbcec3e1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "005fe822c5bd344d56bf6962f9fd5e5bdab9ca85f5bd6b13a9ccf8c7833c9933"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "077db0f5358bf5d171387026d93ecac0e27eebc826f884d1834ffed329e533b4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "001a0c1429ea5b0667dde58a6d2d2b2d03aa1baab115802bcd1db07b1c5d9795"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "053f09a4f21a5a1420063eb18fb8751019f62bbb6bd4030fa8cef49f9e4f79de"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01e60c48c0029eeb630a955be17a7436b47e2f85160d9418ae7a37908ad9f6ba"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e17eee7a2dfb740f4abe01b3beedf4394a6f8efb9f0af86e6107c2652fbfd7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0465a1933162493845ddfc9c0063012a83dd24c54d114e7670f450c6cbdb08c8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06f5696b808b637ef962cd11494574515f6b2d77c2843fdaf480041d41f8edc5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07715378f66785d0c1e977428a200ef04ef539ec4f1f56755decda6664851390"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03cd5402238cc1a4bd0357e045bd9a7fc64233e666ecf71ed2c0a9d94e907364"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06979be705200441c66896be73af2573274345a53fc7f305ab1960ee4dfa0ad3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05dcae59ba7f7d058639fbac966fc39c6c2650046beb6f1155c35e00f74ee071"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0480a0b97251dffe53d1a3dd3635c1729937c89462be3b10f6917931d2198afc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02a29ff95711623ab362670cd67a5e342f823c1d2d392a5fccb593d35cc74877"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03ecae85857a10276738aa7bde988bd21d88b7742387ab97878f4ce2bc630389"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0458839317403f88b2bdd103e4f4d1bacf90e984cffe26e45bcfc9790312485f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "070c67addd42febd1e859ba56df067c41e047d871bc0463c3720f3da0c5f627a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0308331c6d5a55e7c433f8193bc360789aa0d5627174ebc1c71f5d5bff36bf85"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06c8aba41a8b30049ef56815819ac85d65db2e7c53a7a97d014f7472a00ce9aa"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00106da39c096add2e0a727b207615bfde3d1243eef5685977628edbe257f5a9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "049a083c37f0102b4aead32c348e190f97b24030a9a703194e1665150535903b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "045f28f450486b546f7860e5e1d9bf6a2094f0828739a03f062dc4525c90f963"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03da9508bc51911463ef1cce828a64db5f5e16228c54c2f7ef0d0aa76a1b77e6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0410fe4af06771f1d7aeb01a0f623979e89df9c80e52795233266d1d51d713dc"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00b20033406822119cc0cd0bc1603665795b8075da153b11d5e121e7104ba74a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01d3e89270382d8c0e267f86fbea8f36a729d5583cdf43484c536f136fbfcd0c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0009478e8e08f9d547941690979649521ddaac887129227bc0e4ad42c7ff2194"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03e9400d0d06a734df1b614d689a9f5d6006f035e64fc31973d917e7ed8e78f7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0209d6dd7ce34db9f702fc456bd66d7139297c83035fe83f29ff3b3c73fad51a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01f1e26eac904c1736e573156322a47f6051b9d78a733672edee41f53f2d848d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06af8c55e8e194739f523b215de593cac907b553ca221fd6db7b82ecfe98cd69"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04f58ca09e50e27522cdd03fce1dfb283620fb094b4b796d23534a36a3b2c998"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0299cbe0fb2a16bf68f8f536df5818c42d748040f0ab48be4bed67b4da276108"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0403958e22f39038ded428feba0dd6af1013c99479905018d35e2aa2d406ca04"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "002316942a8e727db0d72050e8b32bc5a4bd535c32943a04a008fb0c6794a4be"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "073222a66791f304e04364728efbee6605f34ac8869ef4db9cba9198fea99e75"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0748f295133226d3557ef4f30445938cfe555984e68e9f4dd8a5e54a7461c917"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04a96ea827d82b3302cbb45550c15274586aeca0ee5b2b2285cdf30b2b9b53e5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06426ff647273e8621165f5b89073858c08a1f13c45febd5a573c3671f470768"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06c4e10c55ca5bbf579dfb77a91006993f0dcb75a02f62990b586ed1c26065b2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0507c03134c2186e58fa12cb9b57f63841f57e95c856c576e5be57fd5aed1319"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "053faeae331bd925b8d9d6b2eeef8cd6d7d5c0d56e21f9a7aff6f2d7e44dac8b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03be2864c57c17f27122ebed2375b6a470827d0755adcfdeae88a522a9085b11"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "040db6468ebad08b318e3a1d843ceab419a0683020874ea1421972d022053db7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "028d30943e0764eae1de18ebfbccbe91e0f52e2733ed999e8bfb2ad50c45e297"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0350d499f1a48ccf3f6e0bfa97105082bca6fd5d29dfe34559adc9ff4cad2790"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07e30031588f924805969a8ca8033566434b79223773f91eb9adf7262b04f9d8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0695bea2302661cff74a22099ba712b9aa2575328144fcf18feb686652bb237c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "077928b7b9cbca2800e5dc772714b33d85a29e2a2dad0730e0eeb79da73eb135"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05989c8740c65de92c63baa90964caa45f0910e274b3dedd98f163844d099ce6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "024a9465d58b10eb9f852e0de9ba6365ab4d35ec21196d6f5c030b11ab73ccf3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00d328db1bee8c8090b09ec59d75d0713831a563033db60de3b3e1667ddf61c4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01921cd79a295e051e70fd6ac1fe3b5dfbe11c04f835457901a68b3b3ee0b48d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05046fd55b1e24bf58c11128704aaa39c09275e46fdc7246eb044015bc567ff9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "077d453eacd0ebc287ab4cdb1764b7ff28e114f6fa8c34dd206126018dfc5b86"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05ff26af7f6397425784d96522353aa361b4c282e2855df5a89ac45a817ae527"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0525f6ca8de56b8743e75c6a2daa3036f1a96bca35d96fd48ef342777caaa8a9"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02a80e7ab9af741f5479e14956dc9a23fe3d29cc5126b418abec49255c9e5d93"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06907097b2c2f847d882139b91062d501c958b7a5eb334758b2b6f4ecd1c275b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0189ecd5727dcf830a28098714d90fcd69f79e06c06371d85fffdb8129e3e1f5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "045935a990f982322eb8e0b3132920d494dffce8d5a168744c2a98bfc5571e89"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "060c59dcabb4b1fe45b97502241f5b3efc2fc4e03161828fa6abbfd64d996140"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00c14026d5f6c1ad678ebacfd83bccd1e2231bf5c5f4c45401526fcacc7d1c11"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00111a0ef4b95be8e12931c4a02b97e0483b52066ca7cd6e224d00d7b7398060"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "025dab0252884726009615135d412cdeeb5a48f5484f2efc361f59ed500347ec"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07310f1fc8476f46ea3c43f3204fe203e07661400fb60b198f1bf26b0cb49d1e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "044858e614b57ede2b569e092a84fdd9b2cc705d6fedec85e8667eae8a3de0a1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0757120dfd4e355cac63d8f16b52c366651bfd10fa78ab29be66f331d8808574"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03f0e3bf0edddac08b6b51cd4f59b9c464bb3a021da01977505d8afb35dbc89c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0224a4de216618e65368921241592749d575aa93e420ac982650ee04ec6cc876"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "003fdce7333b2070126764def3de95bef71eb4664bde4f3e8dc175e5668dbeaf"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02de6d0dda4da87480be9a03e50683a531950c09f5f98bb277085b128e4e20eb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04c9af5ff18f0ced3af1ef1c4466793880a60eb06e3465913726a66bc11fc465"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01c1b0773ecae99793fad87f28dcada5b2eadcda25bbfad97329f27383be99f7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "025e98b6fa900b8e4c88dbf337a00a9cfaed0301eef32fe0b479f1a7fd869e55"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e4b2bc8b530246bdeaa3ae7d4fa8d9865467dfcff72d08ed65300aef08020c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00f379811b0971ffb62316e22cdac8d4e56140390f8d6287bbcce6e8d3f03d58"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0101dccff323e132dedcc1a621eb472fd0dc751b433045aaa6b206129e88be05"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "016494890677a987e9212a10a391d3b4d16b7ae50d495fe0dd110203491465af"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "025c9ca33b29b52e20c528afefd21e84db3d54d3ade767c3248c90163633f4b8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07dc6d7e63fc727c33bb04a6f8861f99245f4ec278fd37c5e2a57637987eda14"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03cce6da7a32d9e507a130c3c2eb1961f21a3185b4d59a282bd4f6eecde758b3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03917f6cd4b7e96785483674644e7a721314065dce4cba75e4e2b2ede9a1a6e6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0379d93248055dc9df8d06a690cf6cd182e042062df15e1820e11792bfc41728"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "042c0661eb3cab3df401637d39cc9fd89ab76eb0710de10f03a27018a6254849"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00b11dc25e2beaafccf792478eada612eb2f557732921a5cc78c817d1faf70a0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02e873c59a9df965eed5be99bb54f892665b1bde548309f22e5997db3f402480"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01962a5884f2624f8fff1bd8986a4577ea741dbebb2b670c05f5f34aeedcd81c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0281fa5f436319a09b1c285c99e3a1ac81bdddbbaf42dd0dbaecaf7961a5642e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07233049d5bc6007e29b7b0530721fa2260518c85a7a6372ec591457b71598f4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "019c972f94824c6b417ea1b6acbe1d6e7c33f268e4af818c9715fae71c1b31c0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01d940ff1dde74c3173a1ce5b99390c8afe2e91bf811b1213d1ce6df4697aa85"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07bd706014ce45244f9c5b348072443d2071b15a38f24ad9caa49fadbdec2e36"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02c4f5f6c4b415a8c974f732242fca829194b9998727f0a0c72019fc00425000"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03b7120f7d73172ad2ab724afabc7ea32dddb4c087def32dc820fe133326f905"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "043d95226202d805a83a716373c27ac7606b2ab52cb5a91c7426bf73e67c5cd6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "066cb8fb4063a74f41e56bf009b923a2b4044893162c1d6b5ba3c89392acc728"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07b8b5c4870fda932204ed026f9f5a01e426aaf86039d63e5a6f01d58b53df2f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "059f4178a4b48d83f8946c81b05ae55da85b8ec7f51c79f0084ef8cb61ed81a2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07d58b16ae7511b0c52314b3274f57d35c450e444e14cb2b787966e2ac442b04"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07fd8ab260c6b9075ceb27343093f5b29efeb47621db52765a2d9a2f5c91fc06"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "003044df5a8a1b37546e57e45d4192c40e20b6794c83256b08bd938b28704637"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "009038ecb802fb514fb2e1371d96fb69c7860b6311c52577a5191b62a74fef93"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0696d96ac871d9f6f057e0d702cdde2a7071e21c1862fce3016b25d89a7eed8b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "017d6963d4b0ec3b6a669c0ce8f5387cf0c6c8b594623cb1910c518595e65542"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00dbc239c806055d761189b6bbf2506b7971f69c2824e5de592d3823048b7d8b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "041e51d1b947533709d8a57394834836838229a60ef248a88a045a549eac6376"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "052b2ef5ac20f4aa6113bbd79a1519ea6aec92c5561d0f3740362035645ab161"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07a9e28c35c162d73b1b2f9c650974f2125dc474171a609267e1c77ce8d255fe"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04ea1f4b271fd506ccd0f1eaa9cd33535e28876e8ac713436a16a7081df0bf0a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "063646d6e89be5adeb45f669b7ad7e186dbbbdcc59c968db42d15c827a6452ac"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "044641c030cc4878db6ce88a7ff43ad95e61cb9738418d2f9ffe97b152c69115"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01e7587686eeee25720f1398f9058152dc2f90071e63b8c1c2f7321fa902494c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0644f3004a8053c494cb58fe15261fb7b48296a0abba563938fda867f2b097d7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0183f4c3fb42a35c0e884f6befda1f667f5812bc9ac166fc66d8d10599b29ef6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "045285c82a65fa09860ac1d4d98c2599020d4e27c32a08bd26e44a3582ed0d8f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06045c3515905bd657f91c9cd9e53b609a1dcca332f937faa48ba5e13b0c20ae"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03d39d3750f0b8544e2caabfcf9c09032f1ed7f2c9077847d95e9bcde908c0df"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "008a97ae1a851fdafd472112eb98c91c9f1d87e9e10dbade1be117b613153660"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "019a2ac7b25e29c6c34cc88774b8f2c63cd72899155ba97892e68995fcde9e2c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0032d8fbdea47cdd9293c1ea5f4ee6b0261ba21a1392b3a4630aa75297eff51f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02f63969e59e386ce01c1e6857847fba4e1e6b000dc70a91f0e6d460e1d97ac7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01fccb5ec2738f5f67391a02bab345ad6bcd95c708c2fab2bb1732d9967d4570"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03ac9059af3e804423024a36fd445b92c106df37f06329c436844b857f15c670"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07bd59e5f78744b4cbf8d66844277b5c4e84c04d754c6401616216d06570f6fa"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "041519264e83a6dc49820e021c9d7564607c9a069ae8fa668e623eb072629db4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02291ca1ef3b92ec4d8aefe61a002f525692af1673392884a62576a95aec1542"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03534ed33283b4970164b8acf745ffd26c50db281005375f422f48fcffdc3f94"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "042f35f7411e41f937f36e6ab2c57e9d9aa026c897ddbc9adaea592db1b6fa13"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06b403299aa4a30ccef4e0dc159720b3923ff9a00810d4eec2a1148ea6e4ff4e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "025b2c6201f5f8c9ae8c0db85cc2931fa8cd4d6b9f1bec8f2976651f1ae628d0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "055fe05ba9b0a83141f2f39920dc1a8640b1397bc7be94c7c4d1f998c6e042df"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07871319d733162bce2fde71d76eaab7d9189f3cc64edba6b1e65e81bd4a3cf2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06f7d265f4b66768c9036030cc5435339c85554cfe9fd9f51596e2954f753322"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00e29601d64291f6d94e7e672648334ee8a39ec9aa75605804ec22dd7269c8d5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "014782c9888acffa46d9c677638d3639c9d9b7d0dd8f7eb0b691848d0993abdf"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "018414328326243c4fb2d4199c2f92948a3e38ed3502ed389abad16c0c3f2337"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03779806ab7d181b3376e9616ee2eb3e08f9b989b0e10a187f2eb0c5550a7bd4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06644a2d7d3962c71b46b98d96325f1cab80a25f8ae3ceae29e53db2d87f9b43"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07bc5dd7b15e47765a9f11252477eda7133a2ab59ebbae4909df2bdf7709659a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "037cca6ada05d8404eff0bc2e0bbee69beacc21eac3e4ab5ffe684917bf83358"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "012063cabd637292e08e276882a9a71b69126ebfb6f10c00d8e7f4fc6b305896"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0043782897563687118e4b7f4b49eea7570e26166710da3e49f3e26830cf4118"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "031b9b8dd3d91c2ee0643d3de6e4f4aff927f44bb4556b3090fbc8ea71c539b7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04fa9ed902c19bb97dea3f376559069db34b96774a9bbc645e09fa9c2c1152f2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04a0bf9a7461722c4ec6b789d385c601a8784c49c57142d7ad375f1f78a2a9b3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02e46d9d4e5086ec6a51b0441da5a51c406763cce5a9d050a55b4b0eb1f13fcf"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0648ed239896cef020a7f9ce53371fc77a4a496314e9c9af0afb2bcc1ae1bc70"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0502bd5548ceb094b1aab6c515276ec93eda5b7c644f4db0e29dd47bd86cd472"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01c457213355e8da59b1f44887cec08d15d1b9f47e533d9adbc88f07248090b1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03e7d63e1a1d1bdaf0bd7e18566da7c4b0b2589eaeeea002c1f035c423d19f6c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06d30fe9ebb022f5e9bb59b4b6d247b72b764f06341902ebba3681964fa33f3d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01b4f721b86dab882b82570df4b2934f9b35d242d2204ed6b777ab243957136f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "040e3ad1b3b0122b52f26dddcc2736cbf830fe1d0fdd06b468963aac9a76f324"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "022ce272963e6858caa5ebc828d94320ef4dfd15746795f453080ab55b07ea41"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "002cc22001789b68004ccf8d443dbab69a56eb8c50cfe43067fcd8dcbabd520a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "052b343980a48a9e2a565a7c4fcd85c20972e67aca3c3dde35b798c2699ef466"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00a390f62791a8fd7642984cab19814b566685d3277b9bee55c3e2fde6ca7016"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06d1159ef5a6d08b7b44b1a1b8773771f5b41a398dc3c3f0471afcb9577b47f2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00f5afe8e5520df740bb661295f8c046128149516f911da856cbef58e33e80ce"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06eff7f27253f52cce776ebd850e557fe8002cfefbbe481ad3eeb5d41ec7a243"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "028e4508e52d1825ab0d204c52cb98efce9795260c835af2150e1473ef82b056"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0504abd13712a1f432a2757f3b0f470a34cfb0c89088d5f640b4b5538899f477"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "053078cd360ee21810ee05450132943a6e14df56796fcbb7bbcc208dfcac5a15"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0507dc74fba187dc07533070dc3b5b97be2d80b8286a915f9eb1822fc0598d47"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "030f0b17a5e6726dac65f9900be23e6a37b004dce81a88ce4fccd10e70d75b7d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05844e10a4ef359c75a2bef2f05aa9d55f8d051fe7944cf80e167e34a7b16e24"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "075bb5b2779672f9c505d7b9b8be57871c5276933d55739895fdaa31aae2042c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "037af3bb6470487f9e93135e9c32df646c263488e209e49cd124fb1fb818f848"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00b3ece5b72acf04018077709079a6cfbfcaf22b1d549c6b96b291f7b8c226c0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04346271c477fffd134dc5c5d2c86491c7f6ec9c34f177eac526b422b9e04c55"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0205d5807683516c053dd59775e5fb41324fc66f8ea0ec6e0f13ba85808fa7c6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00ce20b91a96abbcff230483564d2a681a3977d888e40b411d5639819e6ea558"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05aa5d23e1e09c0699f039c816aa46e2a971bdc17f0ab8e67cbcdd31668a47c6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05391e00b1795ce4ca48b2b5550f01b6d396c099f3f8e8ebeb3ffa123185526b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06149019bb30cf8db930615d6641999451538fc62762587ba357dbda789606eb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04b1b886c150898e8b49621e7d12468ced2fcb1fc536a23eb39e722b93ca5b71"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04d2c1b825bb80211e52bc788ab09c22a0c203978f61a681f7b87ff19895161a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06a7250318026dc022f14d2f84439f125f8e90673f713aaca4a76617e026c7c7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06cd363a8f99e4cbac42d4edb9cd61d4c9aa5ff0784813815088bebbce8ca3da"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0079e4fae78e424fb06ec79abfa984e4956878bd5894293125ab5366308f4e8b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0455212a63121a69b56bb5714144552e8c9e9a2f4f716489be286816c1882c30"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06a881203ace4015e7070a5ba815ec76b21c212a0f162bd66edb0d430fe36e16"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e9ef1d836ae49ddd97bc309678d01c726b82bd5d5d15d48e7453d8d40d22a2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0463a8e5f14cf44f8e11782063dbc96d6404311a1f9a50ea17d1b089b2b243d8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06aace45176872b12c3825eb4995d82eafa624614a5c80dfaaf7ca96af72a31e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03b20a2a6ca7d936a0c3824669f0f1acacd004df70a46ab5d6aa10df94924401"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0412a011d5613293ddc90ee02883aaf4b5f39b89b7389afe3aa5617ec5688570"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "035e99ee971e434ff48dd88e9c4402b9bfbcce219484ce26c5df482184d5c85a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04678d4ecf51be45590cc2580955127f9e2cc7b173cdd3799fa4d09107da52f5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07909a274b77f622d7aa9985102f1b4d2d46c261ba7fdf50e48faff339f49810"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01ef4bc832c5f84017b6543cb1db5c0bb57cc333c410f9f6812003dfc859eef8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01ab7ebe7fbc494f2794f0190bcd3877f175bdf769670841137fede857bae935"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00058f49032d9c5dc06d16f2f933d2629a3b99bfed0502c79c2228c6bdba4799"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0140d286c1d79659ae023207989d4ac95edb25f87f115193cc6ffd2c6ea698b6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07e593c48ef9dec3569941ae73e157d35817cba002065c7ac935d36d78331004"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0062da109923071f740784076358424fdf001da6d6212aa5b84cae4c2004f203"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06ebd0057b3a7fdfa8c25b4d7f1ec7f46438a9aa2ec9b6404987b716f073a0c8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01b48d3d25f49f0f6481a7a3a2d5e04b63da0977dbf78c423bca15f854dadbfa"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "030eb8dcd0c3aec884b3b8f4d9c9ad3a339b6bd853434796cd15da9539c889d7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "041053cc3f53bbf588d548ad057cb191b21dcb7d612c79b774dc69adc933d937"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0469c43955e06cf39f423880343c0004271d5a0b6e90ea0646be642594909a55"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0744f34ccd666018c36e0173f7bf73c865a725139cd892d76f3b54079c79fb23"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0337a498147c16995bd54d7faed06266989e28a2876a1ff280cb95a7ad7c9ff8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0464ccd29364304ff349774cfeb2223a845d6f80cbee998427e67e5627b02e29"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0351bb13d760c691c6182b2a6db936503b50f3b27dc375d5cbf69a06b6cd86fb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "048ab4461b5dab6804fe191f2e63ce002d6d3298a7b8fb579fab9f30644857c2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02f0498d7311c0162e32c98de100bb304536ab4a0541d785ca23533733e7d382"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "010bae6688c48c5b9cbbc9090b72e4b2bf69ed577fd4b5daab1e4754bead86c7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04734e2a757a2eb50a7b48269efb97e1d7c607d945e62916f515ee4bec946182"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "068200ef8a19ebc584601d543992c05eb08d84520eae5871eaad846d09da3400"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "023f15e10cde3c2332d4e4987a0f25acb9d0ef44efc0753036ec302b1e079f18"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "013b50eb370859e88e6e0bfaa1ee7ae649fb5bc0f69a705fdd0f25a38d8ce5b2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0004299628470c1605f5b812f80bd9dd4018db2b1b1b845f2642f1f623230a51"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01933cf38c72c787e50abd608e5ad53de1e20daf36ab9a987c6b58079c86e306"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "054d456d69ef4327a2fa51adaf73b8fd6394aefd5c45f6d204afe0ab66e4211c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05f2087348769a0ddd5b631628a4027220c2b4741a148a236125d97a6629d926"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05ba3faed9a4203e93bfb254760fd4e1a74206845e8a674acf397378dde48558"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "057c3a27cf8448ee8446b96fc527ceaad1eabc58ef0dc0a0a5b7dc76696ae342"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03d95ff8fa17bb9313ceb5c4e91f353f916a04802dde885ac2647a362ea64e05"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "058fe16f4d5ceeaf5012209b7fa6c2745cc3d00b99fa59c1026a5468efa2f371"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "000d2ef397476e0e9d584b90b7af8e5e0e5c7b6ed4e4cbbf588336f2e8f37c04"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0456d9396b488c667269961cdf93640272fc88db758097efaa2dbcc7435e8e23"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02afc0c7563006f257fc7107d6824c23c71223219912e5e1e62ce64831a281c0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "012f8577fdfd054cc9ed476029ba642ff3c10d071fd2098f1955be11e4ed6bfc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07fdbaa0dff05f93159beb9b3c8c89aad24cb30ac3bc125593578e0731200e0a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05ae5aa07dc38bbc1ef3998499f20e6186b9efca61435492b802880dd4b6dcb7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05a683dde604f04af4979d761eb55afc60923d2500e0f04726ebaf3d8a65b9b5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01ccff944cb44b5e98330eb42c8dc85481efd9498ab93a66c798b5661b26cd7d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0714684a98d488911d92eacc91b753c13fc94b0e0e8318f1a8c942bca377c2a1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07e34ccf2cbd225afa301b5eb658ebc56258be15d4cd2f191f5ae36d37a14e87"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "024155793f7adb55460901293d0b5f6bbabb1240039a4fa8b9f89cb813963558"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05ba0aad02e33b73f73e200e93db8c536d37829b62f2cdac0c0c5ed6647aec25"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02a4fa7a2c2fab922910fb4f1046ffe03a3f485ba2b45a0d2c119750e84dc5ac"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "077c9ba72af8841a3fb4b5a1af0d8c14c264861d0309718964ddd4482425239b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0165f5f67eb02a34c54c038fe72f8be95b2b0bbb85a0a9087da1d9cc1c5627fa"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0495e526a588c28eb182194b004902bbb2fff29bc48d92660c54cb076440631b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05029e7899d614d2bf3b6e0e65ac9d3690b7de676dee998ea56c8e1c582efdc7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "072761212bc355b3d738e3692c7fdd2c189ceb833f04b823fef4954e34e2c4c5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07980f8e56769127e1e9c05bd0c8300d35ec1f9744003dfd82648475f6ab0095"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04794ef8cac6cbb3c1b7cb7561a94d9ccee3db166d13fb2e20a01cf633dc4d34"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "076bb24b6f03a089366d11b573870918d81d5144fa040aef5ac5d3101f78f10d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03b607912911cad502c99d28c70feb78e051f037f9796af95db4da50fd61351e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01a78614ba5950fc32c33b6f984af0fa5a48cf462358baf6f5e6df46c3532be5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0381b3a76187ca8836a7a718e04ce23a776508a083830990cceabb9f0571c060"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05d41027165e1eaec1b6a853ecbd875c00c16bed36890ea04c900a44ac479a7f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04ffed0fb2e1ddaf90d171730e226ea10d1902584524cc4da43165bd533ebfb6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02ef3aa61045c48c08fbf2929741c20d24c4ba215a5754f12daf17f78b27854c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02db439e94277448332bf9d03638fa8b7022963ae7c81fc92ab7105dded7ca1a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e03ab5c21923272737beb834faa9cb61945d1c45980ad42dc0262c6fa1e8a2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0140ba3fb21ae7e20b9e099bf7437505d216af9fef61c1096b07ef17c6eb3934"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01ec1e70acc07c38421c2da215f106e643d72908384188801b027abadd86650a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "068830f08710334c1ec35e3bd11385f74d2045215678a650d3816a7d5ae8bd2e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01aa1500d031d65c88ee2df1a2a0ba79372c9bd86ef1ccc57e958da1b173e9dd"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "025da3bf608d0becf8a80c2af833dace11e4ee04097bc574f4f71a1e48806e5c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0259307875be70525a57ebd0952e715e12afab39e23d4d1486d224de90f5d38d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0557bba8922c00a49ea17cdb2d438ff5320df12f48a2230306247255fc378c84"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07fa6684a5fb0eb2eaf003e02b9688a3027a89d32406ef9b4dd468dea522e529"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07dfb7b59908447918a7b169409e74d1076560e771612e52bf5059ccb92285ad"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "022256d285ccbee1e61e1c3f3aed4728505663e728bf6b8e867a3eadec96f430"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01b6050f83b6f9a01b28cf0a54e0e31e688c9632e08377492d5dcdbf32a21080"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06a3dfac6de139f7a2841cba1da48b7d94fbebb985f99ee3ef4a7d5e0b1af667"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "036a2d9c8ec7cb4d7ab7d57fa7d3960df75d9e57910de11dc659337f5db23cb1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02ed90cbf73296cdc2078a56ec97c437e25beabdc0c50a349085ea0eca4e697d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03aba5aec32a662dbb414ad091c1da1d1c5692583183c8e290bb44bf17d1becd"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0299ec874abecdd250577f58253f1851f0959969ad29e0d01508c6383b27f9c1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0084b6d411e3f2c5bc6fc22833f56dae7de95c7216aa9c92a03cb6ccd4952e80"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "073bce68ce86c146de19a0381f9e3a25ab6a32a3543ddaa52e604d6908687526"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02c9738b3c24c2889d902e7a2fd57cc3dcb0e7088e1d4dcdaad4a964c72c0dd4"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "071e88b7ca978ff3a0e578fe594853f364de0f1c6937816e8ee9b802495b90ff"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "069937a4935a5495ef121b07793b130171083fbed5ea7c8568d54cf983d857c1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "031a302466b67cc637a2338f96a468f436f9aef41617e60b6a17acf9416009cc"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "077c9b31ac91e9d1479ae6afa9bdc062d4a9a7214fd226bc6e309198606fa25e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06c4d179f7888bd08aac6601c554057e3b759ad550874721ac3fa2a6f03f0382"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01538dcf47800f5b6e64a8080ce39896d84a5244e2f33e006b4d04ed381877bf"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06c99d4a2e1cabf591834f7b489c3489d5946ec7969a318f7553eb356fe42cca"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0531f41f2a4aaf428a54c70a8815b00aa2ff0370312d47f4eb5c8306244fd7df"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02ed3b9e6d5408f9b67a9b451d5d38f5d7e52824952f1436edfa53bd542f5151"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "047ef3d5612c46b49cc4001d81130f348083e01043385e0982e42aaf9fde693e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "021e74cd13d5c3ba086552955f5e2596e3ffe916fa859b94b50562d2fc965caf"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03276c4991d297f5f2af9424371698edd789802fecaadadf122344b4dbb9022d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "011ffa40ded787fd84ad15a5bcde03dc4b17480d4c3da4a9fdb122978f93a873"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05db330fb998df84ac130ac9b0aface0a8bf0d691e15cc859d206ff278ce2db3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0621654a871e16238b4f4f6b51904ed38db3e8a231ab6c65f06258d1ea855927"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0460e637c0a8c9a3d202c9b0374a1d16b54c4ea095e1e37a2af3e02734f0c85a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0124be96571795cc872924e8ebb11b1f4b82efd964546df559cd5cf8e93d92c7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0490d13c157ce328be75f0eb5de9450818244bb59a4cd7e9ffed7f3b69a062c7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07b9ecc4ae6011d40187e49fb5d07023a7ecc4a58e99cc9fef308306da1be7ce"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00eab8af617a6e960564d7e53ff3e8534ac5c1df2216d7108f0f91fe09a3dd2c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0351bc96f2b04619fc8bf969e631f3e03d51c71fed9437bcc96518f77d0e0a00"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "069a962f72814d46302644026ce5f323db823c4a38aa00ac2f92ba647dcc3ded"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "054948c5a8331a6564bf80b02e1c62f1990c1c6bcc62f18f28d882ef1b2fc2ec"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "021f1e6d3b4e38aa16d9014d4a1f24044a1ef06fa1abe55e97f90c3a1fc12473"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "072b43b8a9db5228061d4ea7b07093e6275ed02f984870ed939f4bc44fbef9b0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0472b5d04bfd84af6223e3875a67c4d84b2530b5f041eccbd7a1ff02cd2e9c0a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "050f4138f3e3c6a3b76c66d7f24790f3adf5425327eac74077eee5898f459a05"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0378872e68833ec09b494788aa4b3bde3e6d2669889948c6a65e55a61f3862da"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06fd14a8334e5daafd6bbb6cfba1a2cdb4e29c3df8724ae37679a9b67b5cc50e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "053cdd917cf42b5c6db08fe3dc6da71821f2e8a87af6c8cf0cca5aaba872d9ef"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03f295f3885a2779fa8eded41384da9b8d994f11c7f27ef10a64297aad9eeaf9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01055cb48d300a76b95a762d59759d73e8276ba92d54db0dd81afee9005d599e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00264a607564c1f36eb77a475f5ed772ffd41526da2739f69fd3202317472e78"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04990e11b69a3826c60ed6786115bf0b3faecba852351e6138bc5d8b34ac4c12"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "069f504bfaaf431a076c778250d9f697d764963eb0f7dbe068709da9f0d6ba31"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "053cfd37bf960d70978c5ec074a6602ba90333ed6111324276d0c864e89498f0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02bd4cf1fbf92d5a14261604054a0621e679fb326649f8d2129e9ece4dfb8f81"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06184b442cecf1f75231e6de061a6c457c55413bc9f661240383d41b21edd7af"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "066b0fa43ef640b999284a3c041e08d2f74794a92c06c8e8fa537a36e8af9ff4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "072f15f2d8b55f684b3c6794c91af5606d722abd6012a8d9bec62baa06d0d7bc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07d5a90c87739b0354a32fa0cbac4b5ffba13a44a99b931665fad402a8c8130b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06708f57eef471fb782e9fd266e75695800110e19fc7e6f0627baeb6a3b8bcb5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "063fcb3211bd5e4c6693caf726dd0a8863807ce0634c9d82bafc85bbc40d3222"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01a3b8ac0777eb82e380c6539549f47639da5803b65b2c0773abde61a6c6287e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00712c1661668a195e4ac1444e14b97ca9e456945d366d9eab305280bec5045e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01568cf919ce97e7a67f75f290571a1df6bbc9ea54f41675db48c2ad99289fa2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "020250623a7acb2f656abfef18d283bf99de786c8bd293ec26e52bd32c881f78"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "034ac9eaa3f729d9777a35a5b57682aa727c88f132b63e2bcc68e8dfb38ea723"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05e61310b15f5bb9ac0b6849276adfca3001179c81edf3308c02a3a107785507"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0768fafc59ef7ded655abb2a9d6a529071fca7dde0ba3afbf910e332877b7c1c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0325d705b366d0878a57b786f4639793af72fdf206b5ee5e6ad944723876080b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07252c4931fb25761063d791d517091801f0167072273762ff979e43e68aeef8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "010fd40884bfcaa000c73e46dbc7ebdac95c72f18bc9e10aa93822c19a9f1a68"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0693c9bde1261ae5f15279f486da904f183776cb4d0782a1e31c5d12c5aa9d14"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03616d387bd0064b74d8d30f9329441dc42c6000a82c96faf0be4ea5b27cd22a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "004b2b50977bd52e1776a5024f25049ed5e02718188232637e60f6bbc89400d9"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03f07d6e26f56245995a545ea88142b9911ff8a449fd30e25d0c61042c9adfff"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "073263864fec9c651fd1818e152d0a101bf0b12b3f77cdb49b995160b4b92478"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01177fcb90e3e3cb937d17e84ffc445edbb809d12d741e7547785b4c1bc4c2cc"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01b89681c578497931debbbaf44eeb63bac5698742033f0a1076ce6334ad4cd7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0421622bb282dcaea097b9052ebf4bb7bea67417cc9c05b4d6c37afc65c74ca4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "053741a80c42128627770513d1cfe514348e28d85a50a9031b63e3cd0e795e10"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0227b1826edc6610b30c3ee211fa99b96b6e139e0cc6fa21388a9f6ce04046f2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0492d20ab55a06964cbf36dc1c7f9d1f7fe009dde6a6e0cc6409450d8f1b633b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e0954b1d0801bbc47993e5fdf2dee181a4205d326d19d7de71641d132a730e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03acffd36a8fc89cae38577e79f107849939df545fd7b058438babcdf0ab0693"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0670361dbc04254fb55bf299904ee006b17802b8104d599dbc11ae4bb4357a34"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0299d311b367f954ee643fc5f96ed11954b14d69eb425087bd560fd466ed4d42"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00b734de2d68adfbaf6280acd16f7779f4231ef7852cc5d76e312929b287efd6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03c0573556bb556e545c83bf49b9eb9a52452f5083e3619f7633613ed7d7924d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "015208263eaad7dea4bf37d32a9ce23a781aeda75b59f88282ceac52ac6b3450"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0579cd92887860e27a1c2cecec9a8fcb40a1f7cf4694e305087db6167cc5f10f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05be7a68f959574faf3b58ceca2f51327917b61b412da153e69ef7cb455abd9b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05009c140df863c354b7df7e7f543424e8aa467624703b9ce644b5e8940d928d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0671cb8b983afde4aa68867e6e1043b54fbf9604e108328cd8ebd80f3883d543"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "063a43a154bd5b09dac1616517dcafc505e1542c5860420c0a2976f920be18f0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0048928fefd247e8cc61c143adac4e4752b88ebcae0d806e5693d682d2153314"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "020f81ad613cc5ce43be9beb3582ca5645da5ce5422c454a74ca106ef4d3382b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "017925f0d22616720995d65dae3861140adb9b88ef7ac30db39c04f196346dc6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00339514086b57dbb76a7f1c2a41355e89f126b88b57d66ebd05598dac89dcdc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06018a68bad9128da26b0d1c58c189141ab8d826e98fd7bc832b8e33acfd382e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0145f040f1ba1cdf5593b86aca1463b554e4e82b9ec515c3dbd600ee1dda0374"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0312a31b862a85f65fd705ecbb81df2e4bcc8a81f73378edf1aa9df29c67bca4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05c98289429d3b7d7d27e0c7777798d58e21c88268c364703d5b32e86f0991f2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0069d6e7ee3815756ed584567febe6e95a00cca07103f425d7a945134e11fc7b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0684fa0d53cb4d51afad25aedc4820d1db8b6351ab631e4b4f40eac9cad85db7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0557675a84f0ff93f6cf1baca237a19216985a514587498be19192f4b904ab91"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "061df76e8314613aff37fe62edc01d65742807259947b6f1216c3ba972c60621"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02d41cabf747c4a6d1bb5b1c3a9aa560928529a652977e871e9cac4815af918f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0433bb009a97e60beb3bfd39be83893a5a536c9baf355a52bace76394e2e7c05"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0640d502710102f8e0975f67a3bbe900615a3ab1a9146e677f1843524f5ceeff"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01f5c8f72f462c0d710aec66eadbc8777f76ad5efe543f27544429252f4c5163"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "021b3ce358cb4d32987f766f6d8b3ed204fbe517fa7869dfb9d5c50920e3985b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01810ede8266ea08c0a5997adf577157645d7ecab0f9973db742156e482a03a4"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04d89abbb89c2ab403180f52ddd24b61ef2d02ac3d0eef02ca9422b7676bdb3e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0207a5b82c7bc32f20046eb821fc204deb40a911dbdb4c0696709285b3e30b6c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04b9e3a672a1840c860db9317365d86ef206af8ccce3bef3a69f0a8b406730cd"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "052d72d487e9875736ce97c1ef4b1efb5f9062934cca0e37a5b24631b2b19cfe"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04343455e1fafe4e4a0054030ac66b5d55428380e115a1fe6d6a57523362627b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07cd3b79c1580e38a4747d1561d50cb7e6dca681f47bd26a553d5ddcec57f698"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00eba89fe818b80725885326a144638095d3f90c3361d0cced8be6205006bb6a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "069195882ff4171326534d39d0daa931631748374570c2e073e765761aa181c1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01178866241eef1c9bbbda2fdd6945de13579d2fdb481c73734742f34ef85e13"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "007d10d996bfa408da24398e2e729ff91281e83c53179ef51ee1e7b0339cad01"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0255002119826adcb7d60dccaa328f11e185de6934b5fa30ddef798b5b8babef"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02ae1d7ccc2d4de7f7210668136038bf1807bec62143f25bc2f95085e648bb2f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05724d3e20bca1cd9bf7af8e830b35389b8547bf68c11faf37b68ea670d585ec"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "017bb4611c4594eac75e36273277e3ef7d6178a1cc3f72c4b73a3b9285cfefbc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0043256d48c88b902948953d8b6797818a7b12ae863370baad05ebc275abbc9d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0666d021fc175665542b96b993aa452d4cd820851a73c1ac78c794d91f73fb83"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02a6da4127bcc7013c77adc3fc8ffc38131bb8e1ee0e05cef540036a00531239"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "017646c8d22747428e4550b79f14245f00e09d28032794dd620014c7909857a6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0268b26c1a089e40aceae89b620ad63b37533f7d0eace62a890b7d7b650fb9a0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01ffac342830bb46789adfd8ab3f1856b8a2325c8989bf42d579181c22cb18d7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "036b5c294f230c552b35d2bd2e620e9193bb4ff1b111981ba02f0517c1e4cb3a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "013c2dc54e17a8ead36afc2b0cb4dfa5de310364078adf0119a63de512f7de79"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0411ef649af77e9984e4d1c595fecdd9760aa6059981f20222ec0e980c163bb7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0075049cbb173bedb1ef04c211da852babb02f1f597f943756a1a3b81c876753"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "021756922096fb0e90d834bd241b1e0660ee6ab20573a1a194d89d03bd5a5fed"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0335d63a9cb795c7c2cf41673f244d1176d44f0d22e604d1b596048480dc8ab0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00d5a56221f370c433a0b5cdd9ad002363a24001e8357c697cb1116017d90938"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "062804d73bb5d9852be9c1f6f57a4b563699921e9ef62d6756821d91466b8d07"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04cb02a2e0f02713f0800f84cae773d6f470974ea5783c1421675949cf452a00"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0250cdae0f88ea045ea09fc1bdacc730a9fa604edc865bc8e1a91d5d06677042"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0775d5d22f4794429bc55bbc6830a3b3b3df97f38d0ad4959bb39d96290ac5d8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07ff39fa3f54b006189f025f8a6d4ea00e92d87c12f74f501bca5ed424bb1c07"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07c3fba409551414341b36987694953c155c88636979c9df16fdbd0e0744139a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02e3f07f49f00a31db924f70783f8666b2f3a3b62866e40fe60206d30dc3b5ad"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "040583131dc8bee889237e859414bd58e89bccbcae2f84ebe187077333ff8d88"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07ce38951b93a6c7f9cdb48cb7d1aa7d8ee61d2b5d7ebcc4754f26c1160934ae"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0725ff15b35eecc9284b33fdd94982d5f76f39037f6f7886d5c729b9c381f4a8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0377936c288d9e6eeecbef5013853854bedc9d2cc7d35092760a2b771d656769"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0386aa942c6c41b4f177f977d9aa2f49b4a780b26df72f17274dd6682b11b497"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05f319b87230ea8486b6a3a1b53f09a189905813f231b25aa4ecedb10f25cd87"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0575072c15aec01ab22c796e13e1f6ca3e32c179447b53b20f1bde96957c8683"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "041600f68d7d700f71a4f6b42914c1b993c50c2353fd299092ad6ebb9af4ad3f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07fc945301283f6b2ce350c90bcfa45cd22491f20b55822c6944891a32c898e4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "021a1761022d9d59d2797837fdb19968e3f9be465439806bd24cd3c33d382c63"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "017c0325d78f6821524d0a4704bb486f98b64764f19df68bfbce5aa516fbe8d0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04a67c82a2fa3a08dc10ecad5958f6fcd6c20560fc6090df8634a489e2e3fe78"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0534b4899c2528459c12505ffb442c4bb1b2991601c6700fa9c969682f9a4b5e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03f81f6b47068c6eb4a79d3bbf045649bf78639840699b9c5783388839041671"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "069f15740862100687dab99ab2376544249772a2eac5caec0ca7a9e5122fc714"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06c6519dc05b5dd8194af57fac9dab308b8d66e644ea4a70e96c7e2cf47415cc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06cbd8fb73ae2598b97109ff73e4c23b4cad09b6858f5dc1db4ac73e20dd3175"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00c4183349e6ad8fd81ac350e146adfef7093d0fc7743416365da74fa17c48ef"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00ccf701ac0e989aa39a85118d8fb044f74f87b5b48f6ead41b572667dc5a37e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0578514ef152e0c61855967fd50912f4da880f900ddf2f4d74f99d629a4a6299"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0441fc18d42a3d1d59f6a680334f862dc6dfdd2ed599fdd27d4ff8407017fba1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0045c3cae95e88d39b6b39a1adf8423d89c5ba1ea6bdb2debd033d98c8d63795"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07f0901b006fb26ce7179e95d986f9cf8088a18da76fce0324aedca829affc7f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00ce893ae147253f48414bba8b5c0a2dc543cf3f7a8e512aceb819657091137d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0596977ca9c279f1cf9d7d81f84e2175e1cd455ab5c8b59c2972391688ca4e4c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03790eec7e5174d127d1f10f46c360ce7317c4b733d64683aa9f8193aeae2786"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0004f59f194c003e60c17ef09f33f8040272c5c5f5417b8e3f0ebcdc8f6bc1ff"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07155dd55a8c8bcb62de11204048446ca26962d0005208eda0c5b3c4d8b4926e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "026bd59a3dd06f09d2fa75507fe69fac5d83f3d571d6dc3ecdf0cd32270f4ccd"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "049b9202c47be382743ed7230c62b8160a71cf0b65ecc8444913f41700aaa108"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01537b857b30a9b214569cc1841ecd5d48a3ac70b9c294b05b52753ad1706aa2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03a73278b9b9a853e6689bcf0dd9062e34d4d2fdae7fd1b38f79641ee371dd31"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0437f67aac315285347b60cca768d40f77f50c190bccacdf41da1ecfd0f41136"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0188fa39c7df89700e84ddbaf55aa09ca44f4475fe0d79049976450da7786788"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0070652a2e4bbb5e65d9f54ea26262b70210d61e7f9a5124c7f83c9a52374fa2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00dadf4be67bcf7198d65c86d0b3aa0eef4ee2fc4dd6087d790c73c09e29c6af"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00d270a7c9a8d8f6664a6fa38ed22534d7a984b534a446c8cfb2a0b95c1b4ec8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06f9c8495fef94c2f918520d1b5d71300cca056d498f9518f158c5a9900b8d48"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0260c2499e03d38a8ce7c032cc23bbe7320862fa827d321153ff0a416738794f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05037fbd16a808d052a131c17abe958ac6cfb98a9ec40c5fd07d797b0508d530"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0049f081a98b64a2298121ef6e388af2fd8a29fa2831990cb5c520ba911f15a7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04cbeaf55e220c630b7b2fe88ed58e337a2d290cd711a10449583ca62d532252"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0583a8ad4ac672037d6029acd888faa28e6f472ca471f4767e12285a61f72956"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05fd59e92a5187afd6f94cb0c40bf54d4e4254f866315f1b1a721b7512c26cee"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02f7be5a6fb1ab1cb43a8575716e76e695051ee1bdf883b9ad0387602b1e3400"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "066611ecf791313d1e68bbf50fb6d01d6b49524a5d24d6eeea2f09fad211081d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0629fc53c959f6c33bf2efae924441e0b1f192f92f5f50431cf9804249508971"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0643f3a68797a72d849ed784af8741c2c46d8b6f95be58e9de8721d1ac35bf84"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "021c7dccc258bcaf9b5e57ffa82be6c387cd9bec3982973822dc2e5019a46445"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05083be6f0bc36cca0e9f91491eb4a7cf78d99959fdda3d82f38a5ffedb97d06"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03625f65dba1f6e5c79f82aaee2224b98af5f45af9101bbcc65203e7b1d3071c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02ad73ade1ce826d88b9bf0166fc01b72caa855e032858647b7a3f6f4637677c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03d715c187ac304ea3819f9970846926c8b8e566cf0d6a7ae0c59fc7b771450e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04f74795261ec35a70e936245d9c055f72ad17e95966044dd11aadfe639da651"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "048727960639c797aafba42034ba7f707464c359cc90b3066d38221c312ff240"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "013a6456eaf8b5d06fda2c902410a1a32dafea82b9d40729d49a3e63811eef7d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0740cca9260808d92c5827b84920aa2bb4489553d0eb9867542a9fcc241cefd5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "044ff251344aa347e4391d88c7ca0abf13a14ff480c1717c653f8a877781bf37"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "029f3569257f0dbd7b21faefac0d2e1d7761720b01997773567026927c913ff5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01e326b2259e16b501b0755c89b18527df3f7cacb7b052f6b4b8ebc1f645dee0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "027cf38f6e7b06354071e3ba4bf9e977401f87d661c7a6a187638ad1bd4e1687"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "048661b2e4f8b31f278bb57637d2e4ad3184db5d6af460ee36a0906773951f0c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06c3244f2556817f361b9ab5cf6ddf0dca7697d6be2002ec59b129ddf96e8b84"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "036ba91ce280e77c7e07061fd9d15fd8ef2eaab4a488284f1723360c5ec913fb"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02d672351c10a29f29aef616db32443b11e43934aa548fa309916539a7cd6b70"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0511c457a51e44b5f097874776d5a32576fd2b8b486fac69db36d62ffd202783"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0130b7b9ceab58038821bc2284ce9b3ac7748662c281466109b33471c29a6558"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00e31f1f91d0b1abf14770b841fc59f5d70e8191105622e5f73caf5b3f58c07b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "026196828ded4b165da84715a0f8a70acfd08954b8bb4a60c27c36e6ea637a4c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05f21e992f75ba7d6cb2c77ec9702ed4d7083e2676172385c4fef5a396900bfc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07ff5447a1d27260ae6009447fcfc4ae41040c46556e61bc018b7acbbeb0fe9e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "049f3ced61464b90bfbf274d7a66e7d69c2cc5da927e1e1d350d11b3e61646d2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "033a14b1e55c86dec9b6aeb5d454df84e292684a45f6694d4ed03883bc825cba"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07738b980805d4433e841f592472201c45c20e505908d24a8019a95b8ac57357"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0219848fe115a871381c3047ab0073723c15ab06992f1aecd64875ea8f411479"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "053db4e4c9b9009d3ec7f325e1f0908ad58b1cf7bea1d4af1621395c6d4c8c39"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "032286623c51b42df6cdf1e0025e7e6e6d4aca6e062b656b82441af947b0311a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "070cd00a98873c63c2ba1f91bc2be7f6d6b8e2de4c2d54c1712d426f6d8e1ad5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0138b2c20fb314de644b8d377ab539f96a48d410a47c68af1417a85f48443983"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "078b3458759db3cacf74f266027259f30b3a00d6a0f5a1cd1686b7e5f18fec8e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03e3c217be40668fd46df1612710c7673d9758330c3cd49fba04b155d7aa0976"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03ce0f18e403a2573414a7216e2c8d88686d0429c8a4941888092858e3a929bd"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05559136ea21a3ab5bc5c02751c8917960615b96ec837d8727baa906613328bc"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06a4c59f3a4981d4b08923fea44341868a59bddae04d2dfbcaa2248abf85018f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02779b53deb8bcecf9c0515bcdedb67d400392e40b9669af455d29218937821c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0670034c292a3c97ae717ac26d316849e114fcd9ffa2010dff0ad175a1d48828"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01228641cdd08978ba3908fed031b1489c40adcbb824b62cc09e3dc7e8e56a53"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0158560da6d163bdc737cc5fecde54e6bb0db1680e1acc9f909906a4d90aabd9"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "074a7127f8bf965679a83f12af816fa97a35b5886c9b2ff3deda2f974a87e5eb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01e2dadad1d684bbdd54992104210e197d257475c6fedd6058d6e0fb544c0280"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "048ffef2145fe49a933ef57f7e149d021a4786af267ae4690f8349ed79dd5fe4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02c06a76a4432fcc4152cba19aa20b91784f1cf79153a6f10e4055b6b04850c5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0110798078ff3d76901d04f61e253ce07ee8d1586aa81733a0fb5e10b5807731"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07956a35755e0bcdc048ba64255bf9e22b7b201f4dc22e9eb9fe19a1557a296a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0414fc04a1a585ad9f7b16935c62ad03eb58c70b22fbb314e3f0dae02f06b842"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0752898cc0a6daab458e245e127af964476bc535c238712145e3a39ac026fcbb"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01ed38981481711d313884365ae31b77379996072c1a2cc22349c32252ef1af8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "024323191a9ba5f0ea4c199512fdddcb122cb08a859d8893f048203d4cac358e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0633a59fb3987a322ad8e32697f4f54ce074404cfc55f2262452d801535d2fbb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0578066f64df78c12ca4ed3024b0dc8ced4aa9f541ab87250bc7b8457bd64e38"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "022e4809b716bfeabf21878ee4b309e35c4b0ccf7ba611c5cdfef786d4a7b556"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "071488320938d4ac3ed469b69afed7ac58099c050655cf3411e52259707c1393"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06c61f824bc890b54cae620f40b59c3ea7f399438b5758ee32e8c6a7cdeeae1e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "000ee103685a886cd56e4f794d1040c709ed5f395749451e93a066dd353edc50"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01036bcc76937f443baaa862690665ad1a2f031eeb9ba5766338a6eaf168139c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00520260a416be83e45cb94911736dc6d3e4c461110b2847538bb9eae25329d6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0345709c1c896b22e44696963d6f3a28cbd2ca538bad7bf0eb4a56ae709062d7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "009bdb644126f4e2c47643914ad07107c6ba63973605c647ce19b24da3ead72e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03a71733205ded955239ba9eeb0c0652bbe0bf01923065c079cee1a731fad886"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02b22ce5c16424c6a1a68702e9a0075ac41362b279dbf97e3a89ae5078b00c08"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "013729f9d1978f49a5a29de05dc5a3c561c4e5627b1db12e92aeff33a8643b2a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "039e0d88c7796f85b1ae5d54a0d5b6bbb7c2809a185e60cfdf024527d22fb179"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "065be751e83b97bbb80c0d096d42a0614c0a25d8e186f2007c19cabf3a1f9927"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06b47d8008ea7264092fa9b98bca16bb3905b22d4c9f83277d7f6b00224e91d2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0477a2daf5a60805635170f2dcfc085f4c314d53d1b3f588f9a387c18822c46f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00c10d69d3d84ce2804cdde1742b5b7b43258dc31440016d439c3e5d01b50b53"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04a94315fefa0809a83bcb189386c8f9ada96e9903310eb6bfbfc5b10f71af2c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "067307f884cb461485c700d3a77f8127e16944f04c8d771f961d2d6693522340"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07ccb389df1d01beb6925ff49a7f31d0428796ed10a47799e8953c18d9276cfb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "055b1a635199fa3b85e7176dec47939fa01502afcb4a8fd8a68e57168519b750"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0271f65372b5f64d0eb3fbfc9a3aaddb70b8295a3a59b92f662f484fcdcc0730"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05bb6aa0a0b02860ec095fb6475b0cadfeaef6b57fdcf7a774777e3874b8e1de"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "001a493c49d2d8cdce85807b5bfde2e612f00b8127c5134d14c463444eedf17d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06d4dda2b469848d9c4c25b3f0b0f3e2e6aed6e6720575e2661ee3ab6dd686f3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03a01c1f18696cf07e2d51f8aaa7326dc1f093890351ebce6fdfbc15046b72a0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "061a93c84ebec8924288a96cc65342af88bc29f5cf7bdc4a442775469e928e97"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07dfe2e2e16d8a2a89883914af1dc739ca8d4468fe50bfa55853b0977b7c1466"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "014b2eb9ce830de78a477dc3fa0225fff4625ef905a43a71b54408731bdb956c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07a9e84075381f0de40d500a9b9b9d26e552ac83377c6c7182bfeee6a3f17a56"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "013d65ed56a2305da421c389b286eb2c6a8b00226d73680ae9d08a2b7f743c61"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05459fb8afc9912f7d116ad9a18463db939f3e1b7d493dd5f1d169ea050ff987"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "073723793cb690cbf4a6a825bf6b267d2dbe199a4eb37279fe571976668e1b49"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "060f4e5298c536d3f74256f2f943681d6433c2870195167588edc2d4d5ec6025"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00ced5c897cbea3ddd5abdf6cc9b02c3c02bdec57ff533a754d09b07ccd39ccc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "037c62f59f1c5afc6236cf56558bf683aa3f3965b8926c86a444776e55cf8d72"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06313c6b01142ee5acc1da4e1563e4203caef5f0372cb09503061950d1d74253"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07a7d4e28537086f57957cef4e0c51d9c684f0f93c3d86ad4fff1d7b6eb4fefe"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03a839e9a755f3cea8e5bc312e6b596415ff337e3a4912ad2ff0617dbd72cb21"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03ab0a95ab1e3b1a2c3d06aa2431b99019e043a035604aca22912e3d56e3f141"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06b49ddd40e4edb5b68d66fd4b8985b5561f9a02a4f3b2c236f1607fa65f0ef1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02bc3e533f939742747c7dbd7ff333daf233599a46b43702a32ab4acf52d4007"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "052223b07c974494be6aaccf03b7c729b0ebd85b98a26c4f79d65867569df2b3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0275caf4c9285c037ed2bc1733e5bb9e90465c349b26d41c604ba05ab08cfe20"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00fe17f1031ca0ecab7b61d8e90629bda3e141d6a2fc49a7b5339869b0507fe7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0241f384531d009e56345ed202e7cc3c209d66083e8ec8f9e1c6314e8718a809"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06917c92710309f7497e6f52e4b7c49435cc7f23e8c6da2182ef6c745a6773f7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "068872b715b0c04459834a2b9ae40cec90843b61ac7889c4dada699441a00b7a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05543c6adf1ab3c7ef3077509d4f9c6b5e57e25d4d04ad9a2f48190cbe71451f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07aa4fb1aba6d743b04e5b285231a98deb0125519996991222b283f718119421"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05b5fc65d526b18b5dbe103ad4018e597e8c09bef37c538d0322b4420bcfeb19"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00b9f7b43b511fa936c98200e9e8afdeff24e1a9f9bb6ea0dc647a09078323ad"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "027004b437d5ada5f4df7db40b9e03fb30f93e11f3c3b2cde585f9cdfa4a6de6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05335c4a7b7f0e827be3b50979abea8652e2948d06697110bc0363bcc03295fe"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "074cdcfad647c878b1f0c9e589cc3f819c8b24de113a4ca8fd2402b2ab69e51d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "070705698aa5ae1cd8b58a3e2038f2afeedcccd662848916320a8fca29e45fe0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03591dccddfe44d8f38a994174be38ed2eb7267e28c67472d44ed9c096480a93"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0777e1fd7f45abd237dcf7178b0bd7ff16fdb8f4c0381fbffeb54ce7a91adbab"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05a2d5c2ca0ddacb8b633542532d0df901a7b6519841259b45a15b1fe7df1258"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02b0db3160e8f5cbe8786c792cfa41848546de0681643ccf6a2c956370c6506c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "007133beb82b6e7f7871aeaf6fcc49dac1784116ae2ad301af0d5de2dbaf7a11"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0035c13de88883d0c0c1f4a78958326703eaf9fd90359b80be2c27992b14994c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "024841a77cb1ed6480ba67dcee4531111b17a281be526c8d4368b6c32c932fa1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07f44a5d3416cf56dd91dd9a17f7b3df5c8dbff71fc2e61c44c30b7916093037"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0435faa5ce65868399e80f81d6e27341120ea16dd4acd90db95c27077e34da7f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03b03f3da97d9d404eb3dcbb77c5752be3c620dbcffa858aeecba37a7168e011"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00fc9cd561a0a34ec1d11f45e96483d7814f45c974a12fa0acf70ce5c445e342"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "033b7fd167a14d2f16fbde4f896b1016307337efec7d050612f51322ed6768bc"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00fe07af8332a694fda87e1542439461ddd8e80de05ca3fc4baab825540e5988"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02cb7cc4d31757b75f62eb2ab1f707656d4ed7af175769d16bc94ef366042729"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "001b17427dbab142a398948268d0555d147c5682041af9da5def81d8809c5c73"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "073230699f464bfc738fe1cf24c98b45e13e6749071417a513a4ad347faa1599"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05a7d2b2200cfb3ea0cf08c14b465c1f06d7e65d77a37b077977fc408962a78d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "000f7961d3d36330b5c6e346b385c81de8c76a48e93c7ec8cff87ed2a2bfdfd1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02199f304ea6787ae1cd83178d99b17cc02f3929c8d3065b9fdf3dd7766c7719"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "028a1bf314d83aa965486ccf1a82a9b4ca9bf2ec7db766335772d4770dc7d811"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0604b4be9daee557910c144eaaad55226399ff625ab99a9e92ca8e3ec06c4ff8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00983dbf3843032cd03cda75037793ca1c62f4d413784b1a051f651732f86d9f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "051f311f51dced320726e7526bce7f78dd78282bd3485c76acb3bd4f49bb3af3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "034c45df13dda89b28a5a591c571c3f768199b296d2cd3bbb2e6365a701e2c6f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07b79881326371f40180a0fd434babd4f7274760cb9353ee063898b85330c1ea"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02ab673dfeec8f4ed73999e78c1851cf85be136957425e1339e5f18640ef80a9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "069e9a8eef4302d44fc9f97f992a730ac218514babc05cea6e56fd0218a77ebd"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0208fa99ba6c0efa83e8c8d7674c65164424db0910a6499c2dd0d8c480abbc0c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0483f6895a35a8e6f5a1097338d1d9734d74d784ac4fcef12e869713bc57dc46"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07f486764ff3bb0b165726f5b4c2069ef166d6443cc167d777190f0b1ff6222f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07dd5bed8e9210ec9e0e7319aeec5a27083dd300fac2c6b7d006506d7149c9ed"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00371c588e31576e08e337bc806537b98b32b2a543a8d334f170a7ed69925e68"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05f336844c5a73a5e476bb59817882dacfbdd1621ebcb72ed6963817a1402c06"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06fbe6c8dbdf021f028b5e782b34712013d41f02f4e26c0a74d19438231c7fcd"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03c84918cafbd0288ea6fb09cb00af57af2ee3ca32043b09e780cb007e65e68a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06eeead2440fcef7ae0f888f7d1b2f9d3b8c1a2a98e421836235011153c3c712"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0658426986de2d595c3f29975a67ec90b0ba90af3b10417f4b898bf6c4ae319b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0116284d7a126da70777972c7c5f21c513b320d479460ca779f510422ce424eb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "007a9aee8bd22dbaf4eb6e1ca5026c48f5545272b8fc19a887af8334b206c21c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06b973dda8bfb5912b2da803a50514cdb4d8c87a169b1703916e412d18d1213a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "004e77a970ae6257d026b3968b1bb584aa45c7af4b4d96777251fa8da942f4b0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "073e7219d660b54ef0d1729e1c69957ad49cb0898da272b31aa077d3683eaae9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0770992db59aaed4138d6e9c67898aa47a88243e6732e27bbef3011b71514b34"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00c291f0a410e84df20ebce6507e985541e3b0dbc0e41afb63cd47b14853e671"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01d94cc54b8dde110a058f68457a29a9bfa03353313a48f39b62bce946f92fe0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03d2535854d182f98ea0892ce7bc3dd3effd3442f11f88d3ce703ae574dc5bf1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0103bbe682920c582f7f42333993773985ee508638a3ddf14218bdbee4c06155"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03690be4893beca4d508bd0eec2ccfdeda6daca3859c0ec3b19f19bb9927ddf4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0013e1978d0452e48f6660551543f5266fcc007264b14b24be0438204af6700c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02e459cd5feba8c8672eb4942d708613c82737b204ac8aea51a16c32ba05d2fa"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02cd395c4da0eb4dc5272ec5896692345a350ab219e00764669e3a093f74a6e9"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03101d0aea0382fb328160af35104177055e8952af984f27328dd25181747236"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "025dbf83bc4ee122ea1f2ddf2d7684bde8c242656043697ce8a9ae90c6bc8707"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00017d9293a0d9d180b4ac5404f34393efde547d0df2ca72a8b9e28c0c18227f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02e1cecb90a9981c6f0f75e56eca59553ca6f3f4ffbcb10675e0c0b31e2527c2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0679831323af2dea208ea992e38064d3f874994e80accb9b8970f0a9717e6a29"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00c1a0f374700d83ae0b9a043224180e5de4190912636f844e82f2dc4a10d788"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "055186c2e24929482fa9e40b6464cb6bd94615b142fba5040cf778cb4cead5f0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "071547ead6c223edd38bfcbda7033c9feaad24b62dcdb4d2e62693d175c9f4ba"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "018185517b1b77d6644aa96f505aa07c35418ab12fda7cafa506d1cfdfc30566"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05fa232c28a61d23a913c46ba31dde472601d93ef9a0d86999dcfd0f71b0315e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "029ea90291cfe47b506a3354af83fe419db5c8cf128e0bd2c6f5fbefb1422fdb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0428974a21a58e78bf99a2329fda393c876b58640c4cf2a5e9c256321d3efc58"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05a5737d6ad224f6701da05f7af5dd181e64fc4ed07d67f9173208d476ca6791"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0737cd1414481f21219c70e0a5ed16a4d29c9699640551f5366c8492ea20b31e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "023e2a266c075b6a706c71193c70a4d7c03fa72f080e569ff96df0085793e7de"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02c23b3dd6b2b44313d7bd992c5793c7764c9cde1676cc58bc985b884677dfd2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07757fb565442159d8b8e98d0b324053ca83751b76de9df016ac3f3435478476"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00e55e3c1a2e705126cd0691d6fb4e8298f8acc9c0bd6c9bfb1d01b0e05adedb"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "064d3ca0f76849b265edc708d2c72870f1ee7b9ae93b062fd0a310f7f2e2fa30"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04c8bc43b5138390b9d834b77ce9b47772b8a9255e08347fac013287aac73d50"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04ad5fb351a993fe2f7293fb467973e406edb5714cba0606c7f6add97d933ae5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0668352472cc20ec772c877b2afb3ac31d566625fe4288296701aab2b8371acb"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04d88de28295e6c595beca08b7150e49af495546e278d0be9961b5d53a7baf9f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0167d8d71b48e2a0b0baafcb4e34cc43f64bd2a45f40e9790f306d0912924976"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "005cd34aacd6aa4bcd230a7320564307e39d2bde925e388d17da2e97e2b8f1b3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01dfdcf167db009d678f74f1abdf7c262844eb2171ca0f1d1e21ae8bbe94fc70"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06b09459a78477788f9208f5d486c80e5973d3cfdc7f35fb4af81d18d747ceab"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "020ae1ae44c852db131ef15ce963e97c3fa5d420fb4513a68076947d6783b0fc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0607f635551bffe5f44003420a20052025af0b4fee78b049f87d48ce8dc47d13"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "065feed9f3203889c5812ea23e188cfd0b29f6946770da814e10abbd85409c50"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "027bb2be6e1a567c520f363ec6129f4c41ec8e65cace4efcd0167b415e4ca36a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00e598c8c4cdf9feeb52e1b2521569c0fa9a2db6c14493ad54d8366b812ed413"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "072db8817d98f60906106dfe2e9ee2992d8153a510dd313cb67ef123b14e7d3f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04f34745437efdbea89f603a952d3d642f1b6c11a2889c940ad09b1cf8942e7c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "037dede27078005a50f7b8247b11c0d5b2be0cc2945bc4502f1c413cf0b2fe9e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02666c5d79deb614d4ad59b772699f1cf8c38aa64fc62d3a6ce6e6693c17d5d6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0755fdffeae6c9e070ed9f11c404c0efc4ae136956cca07865db5f49490ba44c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00f735009abc9e3e6afafd7bb9a58ffa242e24fd7b088920b1764c4d338b8408"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07d1028053e44b096d2345a65e849b36bc31ace7bca4f901b8dba64da35d8cc0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04ec63f3e052b059b1cd9cf49ee94e29fd71d4fb5698f6bfff24e055ee0c1b92"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "055b17db315317d34337b28697a7761dfe348837e90e8c4c9737a66a87f07ba4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05147e9b3a823eac502aae672c64c3aa858c5cca7a3b0fb503f90c86dca19560"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0109ae3deb1ef85754b91fe7bd5ed9fc7444344c59dcf20550e80d1d83a374c1"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0782a2ec71468369c9bc927d15765553902f75e9412b54e6eebd0154d7376d66"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00b7b4bc42d000fa93437c26db8101ee8c77c15295b532f3edc0068865928116"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00da66dad099248aa0f080c632d407fb038f3f8c569192e4a4474de4b5e29171"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "023470b8d808577ee7f69663f10deb4d7f79969966656f6155be22f7633e66da"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "076e2731d79658da9aadabe73f509ceab683cb81846d08c709716cd5d2c9501a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02b672ff9a1bcb4e785a745586dd03c08ff03ba3bc236d5b1ee8f86ab9065905"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03a4f708ee691a04168682d06f4de5c0fdde1cf785859d93b0a777bbd1817dff"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0575c787f96ca2651c16ed90b9df1a32447f0365e0b3d2ae0a0a1eee083729b8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "055e5f305ded46119234ecec9e4bd8588a3ffe6f7c1a3ac859ff069f93c6bbc8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05e5c1cc33c78f330568fc0ab5da2d566f153ef4fe4c63479013c63718c0903e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "024545d4214b7854ac75f0e0200bef6b1f973cd2ab8c55878cbb37cad3cd46fd"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02097f540985381a2064c240d8d986299333e4f405ff8492171ccc13f6a221d3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00a4088b0e29a640101f964a0b6aa704c3edb9b84620f0088b33281a160089ff"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01ecb72d522a04296a41d010893dadc740753bfa582c9936e823f4d4d380a0f5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01af5772be8280f03ce7d66f88479f4c72eb836e52524be1933b7b310ec371e8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "078b8ca14ae44ecab036db84128591308285e0714206cf5285c2273bc9c0750d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "020103971aae7b8402c05285b33dbbad69de03391eba1a80b90ab904e5a05ef4"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01d2a188c045fa7a4c28802d9267437eac5a983e76ff4a93b0e05d7f1adbbe31"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "005d54dd2d6a50d8421297044bf00288c17e5a698cc2d806ab455e03eec41193"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03320b11ee4642c5a50bb15d0ebe86bdf0c54bffbc077684344e349df45c9c9a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "036c39c2b54ef8ebf1f1d931b521b4b0b3869dc3318a8e52bf1e0fcbedf58482"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0617f67cf43e56cc71a397309bf2547091d2717a00e41dc5dabacef50fe5dd95"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "043929f8a2060074c27227539594d910eb5f9a3840af183bbfd58b4682c801a9"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07104712ca7d320e08ba344b4a4cb64896d05de2904689cd6316b71360b0eef6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06bb51b036c335f925130cbf9b5a069f332d4c914ab3bdf0f998b6a586a8d87a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0030064ef0a59b27cf15a0e19c44db52498233ec2a897cd9000e94b4d62bfcc5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "013bbc2e17e7e139a79fe193288ab88dd6478071ee386557b60401c594045c41"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "008fe702742604ac753ccf0d456212991bb7023b13d76f52286ef1dfcb7a58cb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04472d9a31210e764104e7d31db3b7b4922a0cfd6b4354e63fe61fe9f6df2044"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01590e9be2caca3fc2cc302e517b62823cd0b7d6accaf9fc62126d40761493f8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0781b1f52e7c576f1110a31944d6bf7363e0348857a134d111a1712e8980cdd3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03300333064e2408a3c966a10f6ab4d8f431c0c6884d6bab3bc6e9d8df2eb6a6"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05dad6411d7ed3c30a4b6bbd680262232d944dec43f3a5e9cfdcc1978f25ac9a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0770ac572fe6d17efd91aa4d28460985b261309f5e3969f6313e3d009c6ecd62"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "009f41e712983d93e776e3ac5b11248a2b8dfa721cd76e66d6d39e7d4cf44d39"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02520ad8c44f053efa50ac7e78b1a536c05a21ed67b8b071b9e098ffac68bf57"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "040e91f881e05624939367403bf17e6a8eac00e759bdff325220a165aa9a9706"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0309f225996b46cd245c438f4f173b81c2fa94ede1b987524057a2d400c9bfff"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00295fd3293c52fcce1b08fe74fb6d4dc1c6f028a7416b297e76feecbaa1cf3a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06649445c816678f03ef4b1e2620969bec5b90b676dc95ce06304990632b955b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01798157a7f3c5565cab345cf9c2e198b8c3b5627d8d58f5cc8335ada51bce2d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07de82b72c528ab0f7469739e2102523570b6f491d646bcdd2b72324e8b77ca8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02df94a22a4e538dad82ead293d18088a5ea00022a1b3d42b63ecd5d59cf7c61"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "010c8d0bcb31fb4e460367c91da2b3375990cf9f8c3c20735828b4a13404d14d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06570ae92ab3a270f95f437737fba45ad5f12d37697e65c4e2db8ef1629a7dd2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00c4d9ec60fea9864c56ae4d59ad7cd9440dcf6ed7703008235faab1e5c1dbb9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02c4c64f04ea3fa4446195b76ae40eaa343b0ba4eac54cbfa688221685708130"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "003723b736ebb3f9eda8a985d601f741bd419d8f1d8d3d80fc38466fb355f4ca"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03649a07118c6a7e2d5f0203c054725bf5843954b4e11e2026c3f89cbd4481a9"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "040a36e9f2a6518d69ef7480af5380278e4bb96d81b2989b3e13e969defa0ebe"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04b9cb34bfcfc9e93291e12997865707dd0607716792776f1965f61fd4635aa9"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0405fcbfbebcd17fe60019e6b6397928a7d11bba4ea6ca7045f93c358711217b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05b2c1e1074d560ff055040b6a3ff9e2b6d1c118f6356935ff9038f569be90d0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07bedbec6c49c6c3ef255693cc9769303573f34128dd3a8e6813d3c8e3b49fe4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "051abd674b1c7be11680dea89f21e9bdd4ba4c7ffacd8dc7439460463eb11bad"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0309c2c0a9ae3db18553d67fb0fa95e0b4e0d363df379544233136355d617bf9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "004cb2584ff904fbc84e33e1cc9e28b2b5c59083a517512a3a7c7783f6d4c6bc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "031612760c3c8d35ecd87618b19253b06cf636d75749949cf65eb55f6a1f2d9a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "079e53827c04053e385cab3e19a8c7809c5ce2df4b92396c47292ee97e30e14a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "070b9967a3df2143f31ce3127eb0413f26858c6f8c7562e63a4350f785ea0713"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00705cf9cb727c79e74b01a3ac4ef80753487a2ef5f1706474145bc7fcd25522"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "009059cc1059576704b61edfa211320a704f5f2172b0bd82b984ed5665a427a0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00a12db4b7b5a21e9930d5591c47efbcf09b834aa4cf8ab47a94421d93ce0aec"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07fa72b83bb3196729ffa1b088b267258eeae02919ecd5b0636fa95b44aa535a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00e5fbbcdbf6bd6a820c912d326a755c4d42aa73b38d8b9352b333551c301c29"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02b4c39b8bc03e1afdc1095d31ffefe8f82802057a9366b29f984445c26750f2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07d1d4d1336544a4b8b6d3d62f911476b0c8c80b0e66f480861bbbe353f889ce"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0195c2965fec72c40d7f059a38b016d26f04bb298db9b23a139faa03c4f35020"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00f1350544c537b32d945aaeea324c59679d365ecf82fe4b41c89486a8bd6ef6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06a2c682cabccebcdd8f525e14f53d3c6dc7d7a785aa596dcef9d5a8cc36e938"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "059a0da5498b4329eb626648e7d92150abd7f334379e4ee5e30d4d852c612bd2"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07ea17d45f4a3a6cea28d405355d8c4b14f02689f789a92b95384e7dbd9f410b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02521a53cc3460a687b25af86a983b969aa92a17ec9a4e8272171df242c457e5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "034bcca06b228b6c590d8cce15148d8ca04fd4a79377317f8c1fa75f84497c41"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0079c4d4079a23997bb6c291ea1c535bfaac6b5125ca05bd3f3e526d5e6fcf06"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06a7d94faa9db881b475f807b48bcf43cbc19ad73d6a9b5b0d5ceaa679334431"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "036a3fd86a5f41a4250d407500562e2a94ff75b56c2c1d17a138663a5afc1dec"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07120214da19a455b57849a55f591e1d8bc652472f4a01d6d903f68267c87afe"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "071997131bc0cd85371fa8e886969a0509a3cdaa21126d2b15f3f83c332c12fe"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0254b4cefd7964bd1c04b0f3afc9074e6c8c435e9b53b55344becbdc5f2e20b0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06ae02d7e4cc9f9ff60eeb51be89107e19f8e7276a67a3dda90b3e89425a199c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02811b1f18264cf175d5acf6aa28eaf5fb634f5475ab581f414d3bcb51dc20a0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04dc0f3e4aa14ef500ef11e620a7e82d8f90eb91fd24ae79ed654aee03a2bff1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05fc2c6074b68a2e4bd9e4aae1738efdf010e9f62eedd224f35c00c54936d80a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0147c384078e2a66a3bd9da9ac8d82850ad36aa86658ce3db6761f194c2de90c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "024759f676a67b4df5cd13519ebd91451662ae0e3a416ff12f7264811420e5a9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03261b6aa6138b6e05aa92995e2759cf0aa061948da5a26ee1279dc29a206b22"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "031adb8377c8aa338a0f169dc87923019c131c04fe165ae991b8daa2fc1c5375"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "052b27f8cf3afcd07dd434ded390301fe04bf1afdd02be40935029dceea505fc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0636dfa66f0102c9b7565a3f3cfe8faf5fa0a48190e130cff882a06aa6b3359a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "000a1cea75706984e127062db3b8e7c9a81ca880e08194a2e3b872b6d2872efb"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03dfc4166f31d11e6295fb64c56e4d157855143ef3336c78874123b85b51ac30"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "067ccfe51dc6c0658e97015fbf8c290528e8acff6db6afb71f9ea41af242aaf0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "053f6ee6a87c6a5c24a5deb296b2d21937e925d4058d22af2cc3de91dfae0d27"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04aa2f70d52f24be4f2f50ae2bff73528b3c4d1220168afb8dae1b3dda639934"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0419217600208a66be9636e5540d9734e129fd89dcf383152299f7fb28fd5866"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03a418627225df42297d4cced283fe172c3d37479fa9ef02cd99fca6116b9016"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0048191666acb0c6a96fe556612788f19dfc894915fd063d4e681623d0624dc3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0455ef9eec491c7c43330460e3f00816284b47dbb458fe3b64a312b84afe6389"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "079bb89dcc2b5c385de9c64712b28bb3a1b78dc70cd5c32244f8d8480f971bf0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0367b7a13b5691621378b80e0904863a0598c99d38360d1ffb327612c2f0a80f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "022fb17dcebbddac50e1c0b6ee54b2bda23f631c5630ebebc2034e3f30d82ce5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0414d2f68302788fe4ce822a991c313db81482f40caa4c802d005163d739c075"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "015a8bbce514adf760a0bcb10583e4dbdcf5049abce20758ce66f79cfd47ff2b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "027da57682e8a88a22a3bb6c2ef8ef821207fdf1096d5fb3bb9322387109c91b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0657cd64cf336a7a2e2efc39ca17ada3b28d9cbafb100b9d9c42c2a0461146a0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02e5c75c7b883a81137454eb26e04bb19e45aebd31701ff97860354a7d9afaaa"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "078baa1a092211212bd64d54e0be33911b26bfca41490286d382c36b49af2e06"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "055f9e089401f93197d11b29589e9788942d6d12f7d61f5f5cad74e5bc65fc5f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "072a19f190d7302f761bf5fa9ca7ba67aec1649de81128f50a50514a7424ab15"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0227f42d227742ba16be8fbb1cf67c0ee65d5cc6fa6e75c3f9e72fe02b9bba97"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "076d0248a12f3b2ec0c4560a29e9810f2d1ba950e9b2b2c9fb1ff19ad9525191"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "006877f44c426f4595010bf64a0a5136021556b2f3c9e263edd2c9158b9274df"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06a60bffa610670764605c1e13ae4d790e0f21d13348d06e843647b8dc8eaaf0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0651fa5799af86b346713d5d333c4605a79c48ab4fbcd64f1ab2896ce009da21"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05a82648b3268b7107feef35b720fd7cf1efb64061bb0ec00f68f11feb0163aa"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00cc964b4be4bb85ace20c785519997d64cb31dae680e288804c274d9124fba1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0406db7c9b520d03da5eb5efccbace8e5321b1f08f82ca3239cbef4b24c0faaa"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06b5dfbc95ae6cd0f43a1170e518923e01677859e6650b88912b29f93fbaa081"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0771df6af194a0b9f91963fb0edbb5a63682347e266483d0401820dd1fbc32e2"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07fc2a0448afa78e646f8fa84841114c0e53d879ccbd6752aa6074e48ce89c6b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0172900df01ce7f868e13ecb087d9142df2de3eab4f1c74f395e19fdc2a6734a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06244416c9d39914701f18a67364033d9433d5facbede68b0e23a6a55f5cf9ed"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07790c0436b70fcc995e0e6c3417ad6c8519f87fe5b192519fb89212ff5bdf39"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06d44b09416bd7465db5b29d088868008ec4cdae38689b10ef3ad01348e3e834"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04b57b4673c7c90486fe08b9b455c5f27c0f9dc03b2c26a754ca40ffe64bdd0f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0482f1c0e8fbf0d96bbdc95e0bce7c3ce4530ac684c1598adf09e725a65edae7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01eb3b55435adad19c6e4cd66cb2a5e81ec65bc4b194bed948a99b4a0214c506"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04c399fef4aa1251b7a279dc14e0572609b86fb5ea2b48d38dab0f95b2110a19"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "075fd95662d08cab549074b06e742aa16c9f621ffebeaa911d5e8db8162cc812"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "040e76a747d48990e1c8873bc44efdbd64399e53c2320b473a0c5667fe9ed78a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04270d0862c78482acee2b15631bf77542eb8a34c297661232e938917e7a3161"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "078f2b5e12854eb1f1f1bcb1fde4ae97b8df11b6e40b7c4c7c8df5a4887fe9ab"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "042a8a9675038fa9725bfd8eb56c5aed2622ee0f1fbf96fef0507261cab88749"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02dad91525c2d51ace468290466480f929091d3c8914f72e5095ba1ace46112a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03d006ff5abef2ea01bc3f0a2eef6e9b627714b7ba65db107157fb256b350cd3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "063775dc30351d03c5488c3b34334d91d3a1cbb61535545d4eb18890187fdfdc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0171ee389ae1f42d8479d98443de46693c36c940b99d657b9a0e6993b52e6983"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0382978af3f7cdfee61ab1908b74a30a8c0de21b759c84a2bee743ea8dcda21c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0632e93b52d03cab89144b287207e3254201f77070ea8417f957b9f51fc50fcf"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "001791654db81d6037354a4597b23b558d1ea2a395838cc1ec9e319ebac574a4"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0354441fd46cdcef6243350004fdc62a815b948f19c73aa55db3a3d185b516fc"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04b13e17c90d9f62ac4bf3ba7446bcd0fdb3f5fc51ffa63e15f4e1e2e9e506a3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00b2c85177f0651b819ec6c81b3d07751743bc4b42006dcb3800e47be5f2c1d9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "051b2e58c85518774cf839cb0f53422fcf9cced9af22e4b82fbfca745cfb3cb4"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0403e8d226b2a448eba57f12125d4a7b0a6b105e6ce33c76383d5f0ae0d10dd4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "043a5a0cb1907c1bf271892ccdff268fd722fbb570cac462b3393af56ad05504"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00c8f203697f128b5ae7e5e234074f1cd722e16c2b35dc4a8724912e72556a2b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07227bd77e11fd4385aca369610fbe4950d434135dfdfe621673a440bf71871c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "069436140e46a5fdefe9fa4516d4eef856288c5a2fd375de2f5b8e01f16636dd"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "026c3e24bb4561270c5ad4723b714a550bfc7d72e5d99a4e924da1d0fb47998e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0281e6ad67caa4806999c68b45942e60f9882b84ee633352273a86f8801a3bf7"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "034580f34a20c811adde488771b73290f82d3c53ab13392023077d0b94be96f4"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06d1f27eb6ed62c57e11f16f16919ba07063e11d439d2b60a5920ab5673a728a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07e78cd52aa6cc1175c8e6c03406a0f4c06895c29c338cb7cf33f0a76e7e92ad"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "067dabeaccf7be77b0aed8b5f1db7c1a9c93b072b83637625e6c027017ae853d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06234a5c86bea6a256cdf62f2342c8aeaab066e6a1925d79f40a370ec055b75e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0012802cd9c290be6c2bedb95e8c48b8034d5528d0938350af8bcefecafd5557"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "013dfdba61c43edf78b53ab7503193f81ee3fda7c45d977bb6a0d80eb32eaea6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "032129f3cff277047b8b466ed8ba5dd562f2053b96ffbc8922543d67cedf98dd"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "005ca8316b50fac70a2d77dd7e72602d23242d722d7cf856b6a49cb8679db4e1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "046fae4f1f88677412b759e02c90a932a96030a5da143160b1b17111eba504f3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06b932ff67c75cb20a747ee954fea395bd180e9e1ffd48d98692f4add83738d6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0696b8b8a1a45ab777468bbe44156f133eb98007db7cccaf0369dd7b4453c3c4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06ec1c811bd4e96be2820d6f754600c0df3260615fe83c0b1e85a2c010c80932"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "055cdeaa2d1e21ebb359a11326c303793d60fb3ebe426048867c39129abff345"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06f865f97cc708c3892de1ab0eeb66c0d777eb34d5f26a814fddf235b5225215"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06320a38bec9fdb02731a63134b4b2eb421cd3b3e05a0dff01822974fa6e120d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07708d907b07e9a0a81257deff05e66465fd4ed6ec7ae2b645a00361b55ae93c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03aebd5467245d5939b8fcc591f7a64d5f5f084c81cca6090c65d3bca4f6748f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00ecd3fac2a25f51cd1510ca415c93f3fcf5df3a00413e8700e7ac6758c18959"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07ba363918e4309816fa974fc7c5b99416acb5b53810e6c1925f33049b14fd19"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "005ab37a89b40f68fd4babb3c5a0bd986124790d2eb076af37341d2f323ff4a0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07a18280e670ac412c9dbe63a4c71e0496c4a6e448bf049de0d6b31280f2b08e"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01f39f2462814733f95c9b9bfb9f9beed498aeba0b3fce711e06b23a544bc52b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03104f44e6bd2505001bb8ee18ce291b78cc4e7e263c30f1996b4b1b5d66ba42"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0272ab62c2ebcae6f1ae6172c111e57e68e398e320172eceaeb0f8426d20076f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "026aec76b75665f356228391c879683ca801d7ba8507b290d550fb9ff2b42a8b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03fefbcd5f2c7f49d03865982548f830451c4be64c4f218799c43b27a8556d1e"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02c3d66286eb7136d570ee477a853b865d3be04bad9dd2f83f2b9141e9dc931c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0730649710172a97ac69660990c370daf1adfc75dd836c8a0f10dfc999c6786c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "057a008b3c848ba8413325ec7da2968eafa2c4825b3ce7459d218c429a6bd1be"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0767c6a85f70759bd9a2b9805c31dcbc4d8282c2a1bcd8e177914b274cf4a62b"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "005f26bff8d7bbf7a14da7ec90a93b23b64507438f3b17fe946a0807a479044c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06a30e26a74cfd705e067a78cec39cefea912cb513d36a807a3e2eae8bd28663"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "012f420f6c89463e78098297b7e048a50a08b96867ba64d34dc7cfa26873be7b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "059cc6b60f340470c3096968963e93b849cfeaa36b510fbec651654c656f5356"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00c5256f4f1b29bca432890b5549d56863ecd42f18d80862e6c4a9483176823b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02533315a3f28e6ceee6f207b1e9a2442ac640290754625378daa271791c3296"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "012ba79884ad58755e41f635991a5c06236f6bc281a084d949e6b172cd91bd70"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02ff2ed9a9dd217eb9c8d399945e9a9db35d505fc9d66465c5c8d2038dd90613"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04e4eb2a097bd75d1218650993d20ad0da4ad6d60e376d19d361486d63cf9006"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03b2bd5eb9abbb01b37f23d8366e6b0c351be4ec54999d6f16e9bba26c8160a8"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00e539cbd520fecd636f024cf8febe5cda33706a404ca18a211a798a40e15c5f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05e9433af0e6b7f6983fde6e38448d0ab57c1df21323ef556cfc9d607dd1d7f1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02be7dc4e94fb768c895d6fd0c71a868eb17a64a03f36c9a41b47b122d7219d5"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0052218326ffde213cc2447b4826c2a1d080000068171739360b25153bab7a57"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0652ead8cab9e8767f5fb2b70ce734a27f4ac1b8096de87721f506366dd85887"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "028bec09b826f3d00b965fcfa2932eb277af0682353254419fa88333c4f5f813"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03a375558f0cfe5ec34ab4931d1f57ca825804cfce963c4eba93741c5f163c55"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "01d2f3aa572f46f9769dec91a69d2a8167cac154d799cf42219e73f7428e17b7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02bf89d4e5ecf4922127ba0dea5176c02425ad890a43e4d3f8a36723678ad651"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06502356441e4852e4764a4e1b9a4f35b9074ad7595f92f9cbf848d0181af706"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0502a9a9701dba07cc582e21bad40f53efccfbeb79c4e4b1c636820df00f96cb"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07f8645b4f62cd55d32a6fa7d0a6d13c152b436d7651d311832740cc7c910f04"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "043fc238265fcb989e5d74bfa7f7e6f46c64da15304412d594db7b81496fd18d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00260d7e7c2c66e7c012774f905d345e0f4597f3fd740c01618083e86c26fbb5"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04b1b15172584c38ad836d62e423d9a7c9d99633539c5da32c316324c95919a8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "05352ad39170224a7903b01c0166ef54c2be4acdc838c32fd355e72fd52390bd"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "040b88ca455dc2107893397c027c6983ce034f929e70e3ae5d48d72a78aa6808"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0191f6e20b76e06f2a6eb0de6276c184c743979881b8ba1b0c46e77da428e357"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0762ca6f0282354032173904c0d4d7a075d5feb20fefb07944a46a91fd4cd2d9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0039498611ada5db0103c84edd2fa3e36f38fbff38d7dbd6374bcde3b0162d99"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0405a2a30da27d4e09a5985f8925b58f3e4c219e9f650735f21df27fe7ce1a6b"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "068173f4d7b1aab809ec79a5e8ae7305fdcdb5a4056034ec57d4d48027282e6f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0600932274e1db4b164b21cf684507de3abf5d9fc549fc28393254615b599d0c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07ece4236ef015b277f9ca2b35b8cfb7ff25be254ed23cdde8c3d8db2fdd4e47"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06fecd8f6e3e74102b22b41c628d8f9e3c02ea4d6254f3d61107c1d86531e9e0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0521580b074563e56998771bf0d7b3501216ef3a5b30ce77b77f1a6deea0c859"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02c5d90277e32572fefb1442b4ea2fe06dff9803ee412a4ed274be3e4ec5ddf0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06d280b728184575181b323f8f2e9fd6d6a85bdd1590cad78c0ea0691d9aeffc"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "03566bccd5cb61bbfaef57fd0a7e364a7e4bf66a2358f07cae9781370dfd5216"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03f907b6f2c7237fe6d56d598d8f9657939566253147745e68274bfeb7264d10"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06d23fa0b829585802fc11628a0757ced87682f52bc2b00a63c6ade9b43242f9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02db2e43e914c5d8817ff47edc921159163f7b65c3324c3eae8917af23b7b016"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "001fbafa65c3f394f1b9cb888673e04b226a8b4a27b608c4966fa313d8f285c3"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0511660bc76f3fee54b4287cfc620a89675009f02aa1572d36cf37afe5c4e95c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06e222a5c42ef04ca66b6e9a7f0497ccde0768f0d21e89f54d3f0d8a7441e04c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "014e69c36fd30ccc834899548fdd90c2ce6e7fccac0441dd1b11032b4f7fc1a6"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0105bcdbf6472e39d80e497dfc54c394af6933bdfec20b107d9e0f5e40ccd781"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0612e37eee25c65f2fb4931fedf885f35c53b9f106c1a6b874fde7263b9bd3ba"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0215ef327da9ce74b55b9b0193a22e85216fc8c85dc256732a020b8fa413c72f"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04b212bc0bb1a71d14b72b8e4048f01eab84a443837249630c0af1df50ab7e81"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07e9b9ef128ebe01dce58ea9bd5fa0ccfdf33e876785a9671603f456eadc8835"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "054aea3fa50964b81eca3d630acc8855a4dd92275609fbf20fbea9908735cb90"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0399fc9b5ee0e6aa704d1da1370b3823529c504b6924644d130a85e65807159d"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00b63e4aa77e45c307141bdaebeea87bf05a592b223d5be4a32af65b5733496a"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "02be23600eb49ef93795894d0e83b2d9c5dd70f3daf5b689bd01fc25bd72ae39"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00d854a6ce7a58d80a82731dc12fa966b7b1cb5fa11e01418d25c43bb7ac0183"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "079930227fa7ed38a1a80c9b9d19aec5ec24e443c56f419bc294b541310384a8"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07c5355dbada0dcabae10cfcc75cafde72ab0e5bdc9daaff4d1d211e30499a8f"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06359b335a95c3a07e88dc80aa126c8d8e2fe0eda95ffb728ca397da3e9cf125"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00162434228142edccd317e402b86d7af8541bfa6b6ee8c5fb17cfba2e31bb90"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0302e219a4910bf5db39bb98619461f181856e7b70fed251015cb2b60639b805"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03ea4e516190864f9b093eeb1132377d3aee4a757cae79fd55899fb5b0ec6a74"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "07087e38b8acd961e35e506febf03f0ee9ddbfcad414f90078bd62b958c8d8b0"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "00581703d2a0dae9b7bab3ac07c8f4949fd146c3a243a32d19c28c644a69a28c"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05eb667366bbcc829877714b3da2c5df1c3b7828d6899b9f1cfa4446afd073a4"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02469ab4a98a62edc1f1be391f2fd9a9b7b062d960e96226656e70841b87ef76"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "077df776956982ee81d6638c8ec030737710649bf368687c93877753433ee137"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "03cb04d2a8ea4e006e9f5b13955335c877fd514b014927fb3e07edbf90686ce3"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0207541eecfc1f9a685d499dd50cda85473ed3dbc9ba0aebb2d62320a856dd8c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "067cd9e728a4315c444f5f80a9b07731d8ae3b88bbf9d57f936b95880cce9a20"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "00b8e367119718b90811ec62ca72fbc71b6ef74d0aafb269f51e954bed269506"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06aaba7f34a8b2ba42326dbd0c6aaf533cd63f9afaef71119e7d60da92fd40b0"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "016a9c8c1c9c48d909d9a4317277e218a74f9ee290221c681652f6e9f864761c"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "025d746782bf3bad9bc5a75487674bb53a09270af3de16539c48ed80e34af306"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06c3701cb530b646106a862c70dcc05feaa0b962cde43d9ee7ddab4c3b96e69a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02f0c64fec7fe06ebaad5061f0290121bd3fa94196e26e0393f3474923fc00d1"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "04324ad44458b264d0428f92d6c08a5c4050199dd1af444939271b30052760ce"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "009bc9ac81d2db1ab72011c093f4e3abea6568aa4434c0374286a0de7589b212"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "01ca1a4bb7fe5e8ac25c3ba30b44aa6e177ae52ff6ca2833797d400999191025"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04bae69e5b30f39254916ca8628c2613701c783674496d504d9532d540dd8319"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "0408bfadba310b7941232940883e4815677d9f8174a2a9348712f6813eabbb65"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "04ae35c858461bbf045978374dbf23a5522f4e18ae644611c84e82e03a9d7bc7"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "06f9787526d389ff74d81c07edfc02bbedc66853b5e088b37355f8f7ac118bb9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "02b93566cdfb5c5ad2c64e4143b19613b3afd6e51a882e5d2c9a4208ad250945"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "062be9cbf38022633e53f7e67d96ad7d77d88c32df5a48fbf992656b8a78f976"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "07ba2b99767ca92a33d4c30e6f86de5f1dbf90ba9e97322a5130b4d1cb255d6d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "05b95ab49cc93e9eb683343bc68b1a67e676b9c6eb14c54fee932009a687476a"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "066d66224e3ccecc4f6c0ecc559de1729b2507c0e19e41b4aefc628ff42952ba"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "079c720174357a93231fd76e9bbe08a003c643642d54429efe223d2d9143ec72"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "0618430857f2fd3f6ec867639a924d1f6b58b03b04ab9c70c17e652b7724b90d"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "017734b8920fde4282a50492ffcfe7b689178ae5b111f399ad44af7eb9aeb9d9"
        )),
    },
    Affine::Point {
        x: FieldElement::from_montgomery(u256h!(
            "06129e26fa6e4438a80c9c4963004ba3c044d6d322b9154dd0a71aec18f59329"
        )),
        y: FieldElement::from_montgomery(u256h!(
            "076af414bb295c8e604021e30114490e00609226df09d9e91717b9a733fe5296"
        )),
    },
];
