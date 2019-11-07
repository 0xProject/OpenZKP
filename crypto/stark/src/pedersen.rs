use zkp_elliptic_curve::Affine;
use zkp_macros_decl::field_element;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

pub(crate) const SHIFT_POINT: Affine = Affine::Point {
    x: field_element!("049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804"),
    y: field_element!("03ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a"),
};

pub(crate) const PEDERSEN_POINTS: [Affine; 506] = [
    Affine::Point {
        x: field_element!("049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804"),
        y: field_element!("03ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a"),
    },
    Affine::Point {
        x: field_element!("01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"),
        y: field_element!("005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"),
    },
    Affine::Point {
        x: field_element!("0234287dcbaffe7f969c748655fca9e58fa8120b6d56eb0c1080d17957ebe47b"),
        y: field_element!("03b056f100f96fb21e889527d41f4e39940135dd7a6c94cc6ed0268ee89e5615"),
    },
    Affine::Point {
        x: field_element!("04fa56f376c83db33f9dab2656558f3399099ec1de5e3018b7a6932dba8aa378"),
        y: field_element!("03fa0984c931c9e38113e0c0e47e4401562761f92a7a23b45168f4e80ff5b54d"),
    },
    Affine::Point {
        x: field_element!("04ba4cc166be8dec764910f75b45f74b40c690c74709e90f3aa372f0bd2d6997"),
        y: field_element!("0040301cf5c1751f4b971e46c4ede85fcac5c59a5ce5ae7c48151f27b24b219c"),
    },
    Affine::Point {
        x: field_element!("054302dcb0e6cc1c6e44cca8f61a63bb2ca65048d53fb325d36ff12c49a58202"),
        y: field_element!("01b77b3e37d13504b348046268d8ae25ce98ad783c25561a879dcc77e99c2426"),
    },
    Affine::Point {
        x: field_element!("00bce48a9bf1ba2a868ccb5ec7a281b4adfb35f880a2217d3efc41fa17ec8430"),
        y: field_element!("00436e8dd6339b88207b24aeb39c4497e4cecb1508e26706bd72c5450d7e362d"),
    },
    Affine::Point {
        x: field_element!("06005e128fd5a27b749a30fa56766ec34958d2116a479828ffdf30ffa22a8991"),
        y: field_element!("012158816d08f33bf1c2895eec0b5df2a4bdd2281349af68184834092e41af8f"),
    },
    Affine::Point {
        x: field_element!("01c06a16f6f297e8d315f6b7ce5ed8b6cc3591b302d4563be99f26f78ce8280c"),
        y: field_element!("03db714410aebfd11faca0a7575258d78b8f1c546666c923aa420e75af637975"),
    },
    Affine::Point {
        x: field_element!("07bcee06abaa11251a294b041fc21102b1b60bb2dde33a164e9c246dec7de24c"),
        y: field_element!("016003366f985bef69b3b7ef9112f0dd7256f1501f8aa3e525d0f72de5885a32"),
    },
    Affine::Point {
        x: field_element!("02ccd344949a471541296e01e27a2b1edae8d3b8ace42a6e4e2a49ef62ffb9b4"),
        y: field_element!("030e28f3a7b4c218e473fed409187b45f884b6f35cef64188bfb6c22ffac852a"),
    },
    Affine::Point {
        x: field_element!("035b6b70b29b725bdd4abfaa9016424455fef75cf5db8ac105349c22a6cf0c79"),
        y: field_element!("026257545e6c40ce862da7092d0078574b354960625f899bb0459c5fb457b040"),
    },
    Affine::Point {
        x: field_element!("0419fb89cf8fef3182ea87e8ea0dbdf2952abe24d8d08b6496ec3507982b479a"),
        y: field_element!("0247a42dfb62d0a43d4b26c46dbd71f883461bc434d879bdd9dbde3e7d915f05"),
    },
    Affine::Point {
        x: field_element!("052c984e04f4e570d5eb7cc75e19adfbe8ca1ccab463fbb8f46292bd098b206f"),
        y: field_element!("0360e38b0fd0d250338b69f0bfe3e0958dd974ec606eab7033b8d46f3c0f22bb"),
    },
    Affine::Point {
        x: field_element!("0505192fc48d0e75ee4186e8469975db748ee5796f175a273ba80c0d38a620ba"),
        y: field_element!("0125867a44b1e4b3b4dddea3aa770e8c1e830e4dcd0473ee4bf803c685d463da"),
    },
    Affine::Point {
        x: field_element!("06980f6d1c471c17c3cb85791a341cd30075a8ea156338f19a6f5cb63d8b7f99"),
        y: field_element!("03c41b089b6e2167f75d2f59b72e1554ca1e5c2c1021758a4164da3e238281af"),
    },
    Affine::Point {
        x: field_element!("050f36f86c52a6f5da5522070dad22fb4c6b77029834e38b99bcc92459fbab6d"),
        y: field_element!("013ea79067ac7c0656df40ad505ef4d22cac5999360ffc826a838d579b0a8595"),
    },
    Affine::Point {
        x: field_element!("008c13fa68b6d0f70cca82f0c5f289b9bd03c84811511731f3e8dfa1bbb29f0e"),
        y: field_element!("0191ab505d18437e11a3fb027278ddbd11d8670ed6eea9c80e2198519fd7d186"),
    },
    Affine::Point {
        x: field_element!("040b996abea32ecc5d88a8a15b715ae699ce0df57972c1fd01c9d6d39b3fafa8"),
        y: field_element!("02e44cba79a2c3f7d34b4f4690cc4ffc36a9b7c99426ad6d9461e96cdbfdfb5e"),
    },
    Affine::Point {
        x: field_element!("05514394948a2ae3db72a0181da6ca17c3e87d1110b924639cc1ddca754ab07e"),
        y: field_element!("01cb53cabacb8d1d0a010c76eef8faa3e4fb744f41e6f821cc89b729ef64dee5"),
    },
    Affine::Point {
        x: field_element!("0066c51e2f13ad12f51c673c4521906c9b5a586b9c30057a71f096e4ef47f362"),
        y: field_element!("02eeee7e746fa29f0e7eebf4504d267074dfe9e1aefd25bd691cd4f325f4ea31"),
    },
    Affine::Point {
        x: field_element!("0066b81d1f23915d68b777783226cebb960c0b0b914d553ea6eb3f3c00675a7c"),
        y: field_element!("039a226a5b073b9aca14c3d17c1e90c83725a7ca526d94469c3100dcdb5b1ae0"),
    },
    Affine::Point {
        x: field_element!("03fed4bd782818b888391c90ffb40f2e9059d9cd62bd92e348946623ab80391b"),
        y: field_element!("0159c712de4c72d908997fa30b00afdc0737dae50dd939400bc864c8f4bf9b84"),
    },
    Affine::Point {
        x: field_element!("005f011d4514ff968ecdac056db402c967e700b8e144070de05bca7f7f831b41"),
        y: field_element!("00a4ff1dccf68b738a15f729ad670f8a2711580d592e9d87161aefe5fad689c3"),
    },
    Affine::Point {
        x: field_element!("06c81c707ecc44b5f60297ec08d2d585513c1ba022dd93af66a1dbacb162a3f3"),
        y: field_element!("02500bc5cbc7982049226268e99f1523a33b2d6c7a95e1cdf3f91760aea2b409"),
    },
    Affine::Point {
        x: field_element!("0737b2a877624e251d8828c4ce63b03225d0d0434f3e96ba45114c148b54f0e7"),
        y: field_element!("004f9a4f03a271f0398695280c24c003400c42138f6bbf844bf8e1a7bfc4df6d"),
    },
    Affine::Point {
        x: field_element!("06db17053a898be46f09f82fff339843705e22ac07eaaaa808d10011250b36b2"),
        y: field_element!("011549267c9e3d55ce2de5f56e3b95900ec3bc0ec9e1680d76ef9a77d04cd73c"),
    },
    Affine::Point {
        x: field_element!("043717d6a04d53d15eaaccff4d1e23b573a5221defe1d07f645c14281cfc5b1f"),
        y: field_element!("01cbfa4c7826b17093c7d829bf596ca0e6a6bb4c7185d34339d592c7ab81b3f3"),
    },
    Affine::Point {
        x: field_element!("028ad05e63b9994b33f6a30c42d99c0158c1687cb1e0563d91063fc6e5a7ef14"),
        y: field_element!("036b54d58a177e83df2021889374503c6a57e29d5506b487970f1db55522f3ac"),
    },
    Affine::Point {
        x: field_element!("05ce51c640cbd74d161baa97b0a88d81d2d38d08a2d706c01b6327782511e5a0"),
        y: field_element!("0308ebac762bb3c9c496ac9f97c942fea6d7bb69fe4e8e0369aab707a64d5ff6"),
    },
    Affine::Point {
        x: field_element!("0333da15832f1efb8f00b29a3ad48e92046a4509950ca6637ea56c2fc6df5aaa"),
        y: field_element!("0035c6cc56fc971900f3476d154bbdc0157939b09477a2f4f48cc4a97415b8dc"),
    },
    Affine::Point {
        x: field_element!("04fc483e8c2c31fcbd57ac8b7ab33c3c42152c55618c19d462982efbd2898196"),
        y: field_element!("0182719dbd4ab3f5c48c115d2756c5fa15839b7f40b358c4248d2a51d01a48b6"),
    },
    Affine::Point {
        x: field_element!("04f52ddf20586084da000538b3d0e80645cde96607ebda93e46a853d8c7c25da"),
        y: field_element!("028b7b5ae215b517271b472bfee0e678480768a8286d90a6b8305930ea6e12ee"),
    },
    Affine::Point {
        x: field_element!("0356f8a9ad7f4b6da51943f2d793c48392826f19696e1054ebfdda9b38d30b8e"),
        y: field_element!("01e872e137163e4a2c3c789143e37c1ac3df5d5bc46afc9fa323efc696682477"),
    },
    Affine::Point {
        x: field_element!("04ba01fbca52020688f49e696ff65ee82ebcbf2c04c61a28c807e7f417b7ada6"),
        y: field_element!("02acb6d4b9f010aea5150a3d7c22643486441364d203d9c3f310e6ee691ad7fb"),
    },
    Affine::Point {
        x: field_element!("050de424fa35e2ec678f93870c4f211688a74e07a48d02d8d8eac283aacc000c"),
        y: field_element!("0341096affbd3c3fad8d51af3026d737021d0205b22c26c9d920d34de1e20566"),
    },
    Affine::Point {
        x: field_element!("04ca6a40cabb36577c674cedab13a5db4f78c72ee460190507393415b408b84b"),
        y: field_element!("036794b40a90daedd59a2e5c2e55b385ea405744b3c3bac0611454ed7a5c34f3"),
    },
    Affine::Point {
        x: field_element!("033ee591b4f2c2abc67936f1e270b4352b19dec400221e2ed9651b09f18a2195"),
        y: field_element!("03b315af147a849413173153c25780f31424bdbbcc3a9506cdd136269c29f8fe"),
    },
    Affine::Point {
        x: field_element!("040311da01f8933aee8139aa0a8654d61e28f01162f92459fb9c10d13bb0318f"),
        y: field_element!("00856de0a27349b1247a133416a9da8f21b2969e51c896a48321a53b6baeafa0"),
    },
    Affine::Point {
        x: field_element!("00d98c224c71a21ef7b4a0c5d77356c45a214cf39edcdf52e21b70e3bdf898a0"),
        y: field_element!("01a5eb0162bcb42382468b6fc9507bcc46d62e39bf7ad6458daed1ac983d7c3e"),
    },
    Affine::Point {
        x: field_element!("02e0c4cea248e9d049a1e2c26af44db60bdbd436fc6dead7d90a17f21893632e"),
        y: field_element!("0265b614119d1ae3a74ad56960612b3e4a3ec8aaf6707931ee9f87562fc468c7"),
    },
    Affine::Point {
        x: field_element!("04d0925df9a53d2308aea0f6a2d4aacefa375421c1116012eec070d16182a4f6"),
        y: field_element!("03b8ea110c1f8b5eb9670e79f0c5cc80694a2fe620a6054959d4228539565d6d"),
    },
    Affine::Point {
        x: field_element!("00e107e80282aff26560afaf79602070b2272f89261970d79fa2db98180aa535"),
        y: field_element!("0180fbe129dd75c768a951c25aa5c8c01028dd16dfac47413218f938f7d13f69"),
    },
    Affine::Point {
        x: field_element!("030a8d24cd7001ddc8adbbf6b772c6f423296f5c30783c96294726b25208c21f"),
        y: field_element!("02f0c69054ab9c229c9328eefa5b15e3c1e4f2133f9b6059d0160932726e2f2c"),
    },
    Affine::Point {
        x: field_element!("0040551f0b3a4d637a983c00ea77d4c60b234848e7b62e2e9abebc6b184420e5"),
        y: field_element!("016096b730a8507f1bea5f401cd88bb96419230b1211a94fc08ab91ec055f297"),
    },
    Affine::Point {
        x: field_element!("06f1e4e8e78a47cf6f5073a0ee088e7d5e743219367ed08ddc5b25547251fee9"),
        y: field_element!("028691d8849563ce726c289886aa14f5742c960d519cc50a6977d7bed0300479"),
    },
    Affine::Point {
        x: field_element!("0236f560c5c265b3a98a7e77e9aeaa6eb22670f16a777601634b3c2664af905b"),
        y: field_element!("0103f9a7a48965ef77725248177ad49f5ec624d7996199d5c960a0e8d9f95027"),
    },
    Affine::Point {
        x: field_element!("0321bd2283c3b6221c9994ad20d726b6b6a82369de2ddbbd3e2b066d04c25aa7"),
        y: field_element!("002c8027d1efee0ef615feae103be7925a3f0e6698753a706e9b8f433746c782"),
    },
    Affine::Point {
        x: field_element!("05de9ddd02c14a6cf6e085c0fabaf5020823507dcf0461b98ac8ec5fab7af1dd"),
        y: field_element!("02ec56e42f07e33ae66d8e7b6678a453e575086e644f2499d23dfaeb2b27fb08"),
    },
    Affine::Point {
        x: field_element!("05aa496a9db3764203ccb6d5090a323b9d076ac57166075716c7c07b8bc71999"),
        y: field_element!("00d7ef9b60a05741c4f50f6e528b7b1f7495187c2b3155cadf0488645fcf1359"),
    },
    Affine::Point {
        x: field_element!("0770be9f6143d5add961cdf8bc8876a939d3e32d276043c2372f94ff51490c4a"),
        y: field_element!("01a0e47db58deb5751b3bb4468e0941632c51709df34a72446c694c4cda328d6"),
    },
    Affine::Point {
        x: field_element!("01d9435cac0b02d9db1097ceadb4588cda4a600ac358358b4b97225be2cd4788"),
        y: field_element!("026b61887e91537286942ed48e1d757f74fb1644d875f535d9f45571644df560"),
    },
    Affine::Point {
        x: field_element!("0224bb6447e8c9641ad46fc4efb726ac9450680a96c279af4daf56e28e1a82d4"),
        y: field_element!("00639eebb0e7b477061bc64fcd37d49d4ae89771a53e21611481ffe26806d893"),
    },
    Affine::Point {
        x: field_element!("03911e4e67a894627b6c39bf1acbbdecd83086177a2b19c81ffaa26857d85aed"),
        y: field_element!("02f067fffc2a4315a8bb60370ff065bd33b77fa7f47af09fad18d98b9742a226"),
    },
    Affine::Point {
        x: field_element!("02fbf07fc43282a8826764403f48c25886853d1e821b748054676e1adcdd5766"),
        y: field_element!("01abd877b35e6f2e6bbbaac94fa88a6af25c7d40b8b0e2ff7236b50642a15262"),
    },
    Affine::Point {
        x: field_element!("001715714904f41e2eb63a21c8e24ef289aa680efd431dde22b9099c13bf1ed0"),
        y: field_element!("03e557041eaac12a3bdd25b9fbcb9a2c47fce937ec507cf1ddea99327247d406"),
    },
    Affine::Point {
        x: field_element!("00601900aa7c0b2a5915a883d383652482242598406ca756eb481f5f730e31ca"),
        y: field_element!("020be2053bf70701aa82d59e9c73c880de215e76ba079b1cfbe9364e21f738ec"),
    },
    Affine::Point {
        x: field_element!("041a0a0126a4efd77a6f3c09c07295ab6b67863479cbb96a623d6f1d4c14b697"),
        y: field_element!("037db3bdbfdc23896aacbf3b7bec1fc55a38395e83858344a3d639ab59ae89cb"),
    },
    Affine::Point {
        x: field_element!("037931cf05cf420f574d3737d3af9e9fae3bab01ab7bd878267365866604bcbc"),
        y: field_element!("032c0ab2cdcb7b104e9de0eae5209f59442e525a5e88ecb3d3eb43ac75479788"),
    },
    Affine::Point {
        x: field_element!("04bf48c131b626d216c282a777b3b5950f03f37fbea7bc393aeec172e301bd26"),
        y: field_element!("019703f6d42bac0d8f3140ea31b4807e5fc0cdc5ae5f79d7032336a0245ff63a"),
    },
    Affine::Point {
        x: field_element!("06a4212b37fa208a47cbe9fde895db3d1541d51e25701e89efa119582bc71c51"),
        y: field_element!("009c93080204740663a202824b059af76ea423eb05a7f688feb4ba104adf5974"),
    },
    Affine::Point {
        x: field_element!("024150b90c0b9adb0f83c18fe1ae468be60c135923a100b308e410725dcef055"),
        y: field_element!("0129e9fa0f8bfed2d21a921913ad0c07bb68a8c901483bdf2814abced6ca780a"),
    },
    Affine::Point {
        x: field_element!("0483aca9f5b283fa9a4830279412c6c71490f005aa85760fea6d2f0840eb808c"),
        y: field_element!("00498c44140d8c969d47f5870852df4eeb56ba0e3d103392bdf23422477c7077"),
    },
    Affine::Point {
        x: field_element!("009dda3ae53a619eecc3c57f83ed4251ea9f1110902cbea8ad8fdf025ce45d8c"),
        y: field_element!("00301da8adebc3387628a8ceea78cc8e2db5aef09bb6aced0cb7d9b7d2954816"),
    },
    Affine::Point {
        x: field_element!("01b36c4f166481feac3d7514b94bfbbd077f59f3486242bc89a85194bca71e95"),
        y: field_element!("0250d3d70c94e32d8105391eff1a36086489deb218ee14be07bab9b9cec770a9"),
    },
    Affine::Point {
        x: field_element!("0024e0bd0ea7e1622395c128c7e430335555148600e5f017e784a54dc7657a82"),
        y: field_element!("0011338d3fb064e0cec6bd71bf5ee10302ca101776a695ee6d4e7a3cf09bfe79"),
    },
    Affine::Point {
        x: field_element!("00daa67743a08fa34518a6104c158962592a41586a81709591e16d5a3a2cef5e"),
        y: field_element!("00bd174c5d9cf246704510760160eb0314c120c8b6f02f954248493755f39d5d"),
    },
    Affine::Point {
        x: field_element!("0009053c8c94e538f41c2fe35c88b2e97138aee30dc7eb831b58160ee4d69d64"),
        y: field_element!("0389ccaad12ed8a58ad77a9caed3cd48b3cbfc710e9ba8374e1bbe803dbc7f29"),
    },
    Affine::Point {
        x: field_element!("0065534abc162829572449fb49d53c646ee560dfc60dd46c922f85011ff44d27"),
        y: field_element!("034061814a7237d24914e1ade5d95796c6c291b315dd99c42703dc6237d55ada"),
    },
    Affine::Point {
        x: field_element!("04830846030aefdc7e6a65e2be04fa93048f47251632b0b0992c09760a489038"),
        y: field_element!("029377fa0ee6e8705ffbd58a6262bcd4ea68523f4ca8ec1ec39e55da1da5e662"),
    },
    Affine::Point {
        x: field_element!("058f493e34692c658b7f7abef0306b923acd876419969ad424ff52f93ff41d34"),
        y: field_element!("01f0f67799ff37903b863931116a5b36da83e7b573133acad596656194f7ee06"),
    },
    Affine::Point {
        x: field_element!("0413e910bc961503695bbba568dca2fb0aa02308f65c64967dffef51aa7f5d53"),
        y: field_element!("02f21e8ce13502dc8fbae2d54d23b2124d2948a1678bce97feee09d3e479b54b"),
    },
    Affine::Point {
        x: field_element!("0277b206de0789465a1cf430f748849b0458dccbba7b6667600a926950409a75"),
        y: field_element!("03223f1a98ce2d094e09f1b98d0dfbca151fea34a362f036d5ba3f2c2118eb87"),
    },
    Affine::Point {
        x: field_element!("06080b9925cb7e4a5ef96a8cfefef14bc6159194a60400db4f483d7eb0aa3d9e"),
        y: field_element!("000046680366a6151df1cdfcef6a080d43f620d2d8d08dfb5890d72571891770"),
    },
    Affine::Point {
        x: field_element!("007a8f9d1f60dd271dad1d45d14dbf8b1e78064f8ba1640a6db861653fb0a2f7"),
        y: field_element!("00339d35f2ad218e7d5cd25777408562e807a1176792d7cd338e82b130165192"),
    },
    Affine::Point {
        x: field_element!("0293d778944e7c2cb0b419358e2e8a4d411214347397c69e7abedfc3d21d5753"),
        y: field_element!("005adb1af3dbd61acee3cd9e2205fc8366cdbcb4985fe4bcc39e8f1e0619896c"),
    },
    Affine::Point {
        x: field_element!("04edc16285f2c0e615fe7b862517f44690747f52f5d7bfed5ba0912f845b0d8c"),
        y: field_element!("02385fa221b0c15870c1eef97299752a0a01d76129b5abafc2fd258c0b218fad"),
    },
    Affine::Point {
        x: field_element!("07adba6dffed7c0c812779173dbd6e7efeadc76f24698bba7d502ce6c83dc702"),
        y: field_element!("034d504fe07db804bcf8c79d72247c1852fbc9d0da344d654baaab2b9c839e65"),
    },
    Affine::Point {
        x: field_element!("0394308588cb3366318c66d26679011151a91de141abb526f12955d61ce15c17"),
        y: field_element!("01ebf8a20b19d53ed00d39900b8a86b15a35b80f57ad40d9fb30fa45cf389e0f"),
    },
    Affine::Point {
        x: field_element!("01922ab77c25d3c0323f6b5965e723849ab9b78daa1da929950e39842eaf70cf"),
        y: field_element!("01c9d94bac8e99bb4a527c3e1cfbdcb027629f7509f35eb411fd24bdb27123a2"),
    },
    Affine::Point {
        x: field_element!("070cc7baa4e17458729531776778276fcf69578e9bd91bd7a287f985cb084ee4"),
        y: field_element!("01e57cf96cc7143d3ba39d3707e2c57149cb1d5d7ae175aa2161f07e15fb481f"),
    },
    Affine::Point {
        x: field_element!("06663aee3f0c67fbd765aacd1839a67a51e9eea9f1c05012eb3fd115507c4339"),
        y: field_element!("02e03b49a406ea8497de25b8956b6758fec048a7782a982852ce05efe26662c0"),
    },
    Affine::Point {
        x: field_element!("050dd39b86db03cb72fd639ef7aaf92d0311fcc13ee7b214509efd51e1ed9d22"),
        y: field_element!("00e69befeaa1a906141c08b5239cab9297387e2aaf361025ccf9f0a2d91dc258"),
    },
    Affine::Point {
        x: field_element!("01bd3edfa7bf550a7474828d652dd816408070f93cfc3ec886ac54feb0fcf312"),
        y: field_element!("01c68453649f85ecd6e24b28a1ed350ab4d0404ee546c3a5cc26a8fff4d719bb"),
    },
    Affine::Point {
        x: field_element!("0125e46d769f18bb293c5f4b8cf7f9eade40d90dfae6a5ff10f44eba03d7fee4"),
        y: field_element!("0157c7a6b2f24c5b298218f6c363aab3ac0e61aa759cbdef57c8f970c1f07c65"),
    },
    Affine::Point {
        x: field_element!("0529036195e38051b58957394bac6557c771b6736b4ffd16669699643a6b550c"),
        y: field_element!("03eabbf8099150584292658e36b351126ebaf173e0bca3adaae40e6787618890"),
    },
    Affine::Point {
        x: field_element!("02fbe312fd7821654b1846a6fc2475f2959bc212685778990c847e2ddd9a90f5"),
        y: field_element!("035bc5bc63a001661407c81e7f7c6e599b8b8ce030ca3b258e1494da5cfa9fad"),
    },
    Affine::Point {
        x: field_element!("040ceb5a08e9e91573d414370740d64d0712a939a5e0f39541b492c7b8e2e3bf"),
        y: field_element!("01fd1bdc65e3d483bb7377c647ae21c2cc35ce93cce367eb7b9eef09bb4ec452"),
    },
    Affine::Point {
        x: field_element!("0088550e4cc52399f23e106e5510fcf6909b7eeb9cb765f7a280597d895255c7"),
        y: field_element!("021df7ed0abdbfc0e32eded80e741e27b0bb85afeb1807d10e1a56bb3aa74173"),
    },
    Affine::Point {
        x: field_element!("06b9ad6581c4c479b5b79670dd5247364f1362ebea682ca2f48ce10d918226f6"),
        y: field_element!("0256394c4a9596465824e646545bb9f0e27cabe18598131fe246920f4026b897"),
    },
    Affine::Point {
        x: field_element!("0036c8914a4a600b1a5e3fb028a625b532b5a34b403a8f9d5bd4373197778c1c"),
        y: field_element!("01a8909ea4919ef41579803687a522aa2fa4fd75cd72d52348059ded8c9e7f77"),
    },
    Affine::Point {
        x: field_element!("00e989514c3f06657dc6e9e861ea016acd9a63f8a6a1e59a0a1e1404c9f9a618"),
        y: field_element!("01b2e61e29fd99d655dd585fa9c561768aaeefdecc9a4dcf4ee6abfe2656f2d3"),
    },
    Affine::Point {
        x: field_element!("0373ce69b7411b98a78145131284b0d7e92e7b5f553c9f908605f7466098a22e"),
        y: field_element!("0312a90c7f61fa980a6567039496615f991b1f0fa6b02b79b540f3fc68bbe236"),
    },
    Affine::Point {
        x: field_element!("027b9cc68be8ac980c9dfa89ce20eec78a23b2cac2034146b0da0ff6ea7ca5a6"),
        y: field_element!("038d334b4dcc0fc3eafcd89d3e247579ae8b0d7ed369372de69b2d72efce26e3"),
    },
    Affine::Point {
        x: field_element!("04bcfa2046e0887c01cee1328f9f1af390e1eeed0a4de0f851a90cf25e0c7b03"),
        y: field_element!("00c1d24fb6108e4e35b5d0bd49f3e8084dc977a8414a849d83a6423fbb3c60c5"),
    },
    Affine::Point {
        x: field_element!("01284d939627581d6abd80657216999b3f6a6fe5151dd48fbe449756458af644"),
        y: field_element!("0263343af2a75478902b8a8480a968d4443b2422044127cfb78df555920701b3"),
    },
    Affine::Point {
        x: field_element!("0470d4c503225f74fd468b9cf01e0cb166e2dafbf06b24ce02f07590bcae0392"),
        y: field_element!("034c354fbbcfdf69226e8a6861a44193b6a682a7ef9127477064a2f39c993d81"),
    },
    Affine::Point {
        x: field_element!("0582a003d8fcd5f4ea40d34cb1fee5cc07a96cdedb142dce76444964f8cac58c"),
        y: field_element!("021927c0cc29ea8b61c3ed642332cc32cd0a4b28a3a781adfca535e03d686050"),
    },
    Affine::Point {
        x: field_element!("06b15d6e66815a4b3097c56e363ba74ca480f205f2d0a21c000b19265cf3c9b4"),
        y: field_element!("023367a4afef269ae233a0171f6156613a59221899e85ef52c488fe39cef27ca"),
    },
    Affine::Point {
        x: field_element!("0672a2e5fa01b2a5e9c14f36607337cb829ee879a9a668e83486e60e8ff90398"),
        y: field_element!("02b4b7c1f6aee6dad99ff49cde5c1b3bcb2eab2f45095edca3fe4c7087d8328e"),
    },
    Affine::Point {
        x: field_element!("064d86ee99c06ccb55648b536b29a7ae9cecc50c6aac5519dc71da89f5cce6dc"),
        y: field_element!("021d0e8832731ad3dee37a26e09956d7a2d18070c7060b273e818637b90f6b4e"),
    },
    Affine::Point {
        x: field_element!("028417dd1d91bf848d223d420deec244d61e36573daf4bb952bfb5f5fe18a8d8"),
        y: field_element!("03d499df3699ce1437209f8c162ca8ab60db026b931a121b398c1211590ac175"),
    },
    Affine::Point {
        x: field_element!("04170f64d9cccf60808c73d373b9c99dd4f7f93f2daddef9a7effaf18c5a9446"),
        y: field_element!("0046b33e2c3abf64420c8e0f560a52a05138b66c4f983090f3e6ad0a5e1e0304"),
    },
    Affine::Point {
        x: field_element!("02ba0d3dfeb1ee83889c5ad8534ba15723a42b306e2f44d5eee10bfa939ae756"),
        y: field_element!("031ba5dba94520ba704f0d71b4689dddb58fb283a05b2b07a028b4023738bd34"),
    },
    Affine::Point {
        x: field_element!("06cb3fdb494f1bf297fd78f330be7e8c494de17f0bd237427e2af0ba5d7ded8e"),
        y: field_element!("00fda7c170c352bb44ea14cb636a01e2d5fff5bc770d038a311f1988e502ca0d"),
    },
    Affine::Point {
        x: field_element!("045c121c01c2565acf11dd7469fcf612c2ff425edb2618b05511e70a1e82926d"),
        y: field_element!("01a2ef0828ce9779cb3b10637445b76e4bb107ba2fc54c4c4073ae5c0eb98079"),
    },
    Affine::Point {
        x: field_element!("07546ca8a1def078182d1d87e3e943ecb9d6d92361d247a6418054319e596ff7"),
        y: field_element!("00d8e39c1e099d6b2bcb996e1c845d0d16d0db5f6ef2d1f30cd70aecdd718bc5"),
    },
    Affine::Point {
        x: field_element!("01fe314f8627c91365c90249d26a045d92059d110db66bbebe242a24b2c1d431"),
        y: field_element!("008388848a504691d5360935d9b107e7fb592d0be4b3ec826346b39a32f44a43"),
    },
    Affine::Point {
        x: field_element!("00b29e24e8d38eb52923b6ef90bf3c5647117c3f926312441b1133cd0e5cee87"),
        y: field_element!("000e844fce58e42206ce03d1152a980e54ed35aa53d9d94bd48ff80f07a870b0"),
    },
    Affine::Point {
        x: field_element!("00eb05b08c6042cc41ac7bb6f6f7c1f15c00a9b3dc23a9a3e48619e47cb8c1e7"),
        y: field_element!("02aff2d27bd4828ebf6c584042e8aa2ea52b119590b64d6bd3ab63bb01aab42b"),
    },
    Affine::Point {
        x: field_element!("01f40c55763a95ec5925d52d799878d26b6cac567a1b5b81511228b1764e1938"),
        y: field_element!("00248d964c599e453b9d47c5756971a56cd84908f324486f76149efaf6211ca5"),
    },
    Affine::Point {
        x: field_element!("02c6299c1dd5d7b516a6eccbd7aedef390097576cb7432ed5fce3a02c6d91807"),
        y: field_element!("03c069cc22908ff6c298b6c7eccf39c309e27218067378c2f2789ece6ce6dd21"),
    },
    Affine::Point {
        x: field_element!("07d65dd596161e3218bcf1ebb5cdc5761a847e3027e0ecbfccfb1384c2a14be2"),
        y: field_element!("00b31254a0d21cd0225025a20360cd6e7d3b0453dcb2ea3412d63efce12bf8e5"),
    },
    Affine::Point {
        x: field_element!("07e71e9fd26859a420a37cfe50dcdc8110cdf885372f2cb90d0861b41a6c4625"),
        y: field_element!("007511289fd1a908bf28bf089210192ba4d2b2bb55675890a371385516660081"),
    },
    Affine::Point {
        x: field_element!("06cea6759719ff73d6478414aa8d24714af1a8860a5090a32b260dba301b4996"),
        y: field_element!("0380ac5ff9b20bc58e99536971353490c2d60b32e7135a7570d4309179d121fa"),
    },
    Affine::Point {
        x: field_element!("05d69019f30bac91257a3ec661c8ba9f731ad1fe1e2338a4a687e7d203fd4850"),
        y: field_element!("00b23707c306937c93b07f4329e4185c3b46ce9955045ecdfeff659cf6a4b12d"),
    },
    Affine::Point {
        x: field_element!("05477486153368ebbf4b2a9112e48daa9ad48b5b4cdea01145bfd96fe65b07d5"),
        y: field_element!("02ace5e01dd1fdcd70655a556f0420d4998eaf414d2914f6efd0e8b1b2c5b7f9"),
    },
    Affine::Point {
        x: field_element!("064d782c2f53274028b00bead58883abb5d7709e8ac1b1a3f50ab23f2a47f2bf"),
        y: field_element!("00a0648be4735e970a5f7389268ccff1b78a0d76a5f24fcd12993420e7b57a60"),
    },
    Affine::Point {
        x: field_element!("03815c72e415a5b60791e6a28d12e5d30e90105ba0d2ba9d33f7850c4a29a0a7"),
        y: field_element!("01a327db518335a779af723ec438437f9906170160af957368b9470d3ba28d7d"),
    },
    Affine::Point {
        x: field_element!("04bbc2f720b35b61f970f76f5478df5be9b30a0db53f2a354c6afe1977f8709e"),
        y: field_element!("00f46942c1c99d993c921e3988306b4ca280395f57529cb4b634624b6354e432"),
    },
    Affine::Point {
        x: field_element!("01e85dde423673d696418c5bf102b223dde50d9d4d4066f02e189bc84108d2d6"),
        y: field_element!("01f65c588af13884e43661599ae17f4ac1335887aa6874bebf530c89d7d9b45f"),
    },
    Affine::Point {
        x: field_element!("0529eb3230b2d501afb7b7ef0c5ef7a99f7e8980a993353ffbb14c36ea100b23"),
        y: field_element!("006f5d552471b09ab20ec17e83360d0d1e252c5488e6743ee17a6e3c8d7ed66c"),
    },
    Affine::Point {
        x: field_element!("044ade9cc3aa4138e0c3ddc18883bb628380e7cc9def4e207939f13d9a46cb75"),
        y: field_element!("01bced244328cebdc17586924c955d8c4613804215601abe3cfa74e7bc5bee39"),
    },
    Affine::Point {
        x: field_element!("05e02f857c174d8dbb1248d3cf0646771ff8668add083c2e6db0f5127eb2ec9c"),
        y: field_element!("030af40f315efe4a3b948b86848383f6cae0aeba0a128d1fd082fde2719451ac"),
    },
    Affine::Point {
        x: field_element!("020fcd957ecc47b895bab8206454a53340c29370f3a7334f2cd16a47cb4422cc"),
        y: field_element!("0194a9a7f18ea40111e2a9b0ba6cb52082fa2e797efeb26b82b3c21cd04313cc"),
    },
    Affine::Point {
        x: field_element!("011f50d91cc76e8c941811e73ba272fd4ad1a402322e6040c15d8d62a766f926"),
        y: field_element!("009b08b5501404edcc1a1847c7f419ce25ac1e455016bd3bb746a7d58bcae06a"),
    },
    Affine::Point {
        x: field_element!("07f7daf5d6d934e0851c36bc93b7bee7518fb506cf7801a2cc2f9d03caa4fb03"),
        y: field_element!("00daed85048b18cc38b14a7f7f8b197a0cc0ff7037d8a9c0f2beca3d7fae15fc"),
    },
    Affine::Point {
        x: field_element!("05e60e45b65286db60a3ba9fe87c9cadc5885c7b9ecb96616204e7b3d2e98c3d"),
        y: field_element!("018c56b6eec3350a44d514eede719ce13b7ced484eb906abdad146cc22b62f71"),
    },
    Affine::Point {
        x: field_element!("004fd09b73eec8d6e5ee345ec3e7f1f18534e9b50abe166ce731f4eff9e95816"),
        y: field_element!("0273e4bccf0b5ea4c81de27a2a632059b1ba18f62171846a59d632e75ffc42bd"),
    },
    Affine::Point {
        x: field_element!("00ffcdcafea9c1a462a0b6c233d9e1d354da8513598edb2feaed5b0fff9709d4"),
        y: field_element!("00093e246c6dce1df39809aae4d232456879b9f50f014750e6d056934f9b8ba5"),
    },
    Affine::Point {
        x: field_element!("05daec9ee49b6899b09c63779ba7e902299626781f09c692627113ff66280707"),
        y: field_element!("01724c6ae8983698e2b8748187c4ce7f9d5252bdaf454948383b34c35b4bcf73"),
    },
    Affine::Point {
        x: field_element!("058f5d8713561284a8ada6e4fc34d777cddae78880990ebc394458d2dbe7cb83"),
        y: field_element!("039702991f6a71489c160394817b57deb1c09bc872b8f52b8b8c7f3d05156674"),
    },
    Affine::Point {
        x: field_element!("079033e9a4c308f1c41c0ae873ca678bda8f6bf08a611b9493a335f63aec5dc3"),
        y: field_element!("0156084b42362d3341081ce1c7edfa15113eec4e0469ba62bc58b4f88eb36f93"),
    },
    Affine::Point {
        x: field_element!("03869ee2f078fc4f11b7572e0d2f85e558b6fda0b39f9b924a368f528ac0e6fb"),
        y: field_element!("01f36dea97e157c0ec56fc9efa84ec45e77b6be20dca947f435ffeefedbf48ae"),
    },
    Affine::Point {
        x: field_element!("0103b514965486b5dec94155f77c20ec1afd6b54fd86e5f242632df40a79e44b"),
        y: field_element!("01123ba1f13de7e7b95ca17935be5c84e9428937259a4042912b5b1224c24f4b"),
    },
    Affine::Point {
        x: field_element!("06bf3526728fafb5b0cd7b769b57f77923f97af35af98b888ae2a0d1eca4c949"),
        y: field_element!("00e2ce42c3b7c6a8144af0345c7bfe817f73fea92ffaccc45b867f9b036c1179"),
    },
    Affine::Point {
        x: field_element!("002f1b029e9a607b64e12d3e2acc55fdf2f76ee086129abf78aaf3cae707fb52"),
        y: field_element!("03f67d12448ba3498cdd08f0240723e5655924b0170f456dc268aece063123ce"),
    },
    Affine::Point {
        x: field_element!("02844881f7644c6d9ca9886cb2037f4425b6e4508d6acd350afa2fbf992b0e56"),
        y: field_element!("021e4d2de479a316e7024056282dd8589ac84a3d12b9c0df589fd7355a5fdc0d"),
    },
    Affine::Point {
        x: field_element!("022cc8a57388755b722443834ea9d0f2ed9dd92bfb7ad032d77a5ca2887057cd"),
        y: field_element!("030746caca347d5550882676ae462a469ff666410187e0e7f8fd80cedf1270f3"),
    },
    Affine::Point {
        x: field_element!("0783a98259d8a0654823c63a77729534f496b81692d7ab390e4755841707a12b"),
        y: field_element!("01ec526295b16e241366ab6b58bf5b661935b1e9d5bb56eab20996b2c7162255"),
    },
    Affine::Point {
        x: field_element!("052fd0a7fc6e31226ede102c41a6d8e544419816df0b30e577289a0303b286e1"),
        y: field_element!("028987b0a0d997372382a4b3e8d26ed60f3881de924e5b693eae0970fc9c972a"),
    },
    Affine::Point {
        x: field_element!("078e9a6d7a9d69a9bdcb8123c621fccf9f9060667c1407c6ee926322bd6a9c65"),
        y: field_element!("0131f92f4cb18928e42e1bcb6179d2ec23ef9d0f7426df5d8529b8fb0cbcfe82"),
    },
    Affine::Point {
        x: field_element!("021177eb64b9c47247aec2a5cdfc7f2b26e3d44f19957e14150efa962c64606a"),
        y: field_element!("03a56bb403635193c4d01479bf6e3ecca511e50d4df8f8135021cbc471d24359"),
    },
    Affine::Point {
        x: field_element!("02a5a5c7ec39885e3c600cb5bf26fa39cc883a9530728f5d033ba27966cc036f"),
        y: field_element!("00d8bac403e3996d0f46cdadd7ad0dae27f62a8d3d7e6b1355f07dfa2d1748b5"),
    },
    Affine::Point {
        x: field_element!("0635b74203e49ff187015f0b28bd1631f8450dd670ec5e9c4ca70e5aa1e8d8bb"),
        y: field_element!("029b2658602c6bc97d8bde04b796b2fc5e73dc5ec70164e5dc4527cdceb9faae"),
    },
    Affine::Point {
        x: field_element!("03b49585f0e8ed8d6b77125a86ccd63c5c453e26b7aed185705e995f1dc24c08"),
        y: field_element!("02b1d7122c3dce9b44eef4da01b51dc74b6154ac2c8f786934d7121c7bbbf0c3"),
    },
    Affine::Point {
        x: field_element!("039d5d09a4b59a71804a4472622a253fc284ae23ef628b1a846caf1258ba6fd9"),
        y: field_element!("02526a5c5386544be44359c57ffc93b5b210e9404fee26304978f087c487fbe6"),
    },
    Affine::Point {
        x: field_element!("074f4cb7307a7712d0e8b98c9c4755e803fc28670fcc4551148c647fc3ea4cc9"),
        y: field_element!("030073faf26229872ac5b3e9bd2a67fa568a9a9122c548fd61401c55704ec096"),
    },
    Affine::Point {
        x: field_element!("052ed6ef846b814ed360cc306aff91a9a9b4eb5926218a223ec246e359950c71"),
        y: field_element!("03cb41e8e7fa9a646e7a78a0857ffe6f6c76e839bec9830f390d857bed017182"),
    },
    Affine::Point {
        x: field_element!("049fb871d1060b40ae821393e47c3d6c018c56fb81602a72749e4af5dcea8045"),
        y: field_element!("02900c270203025cd9c5f8a3116d1cfd3f120530a71bb8b7f430cb1ffc52a26e"),
    },
    Affine::Point {
        x: field_element!("04781e5270eb1c4b531d6e4551cf75df4019a1459ccbc6f2b744ca699e1d4ed0"),
        y: field_element!("031b80e7df8d1aaa40db4596480822943a0e4a65bd807bde2768baeabb47fad4"),
    },
    Affine::Point {
        x: field_element!("01e63177897a973b54b5ada44f71e1a95861597d3775033217666f0283522275"),
        y: field_element!("00936f3e1168651b911cb6246616792bb0adffe42e5dac2bc5275a9382131d5c"),
    },
    Affine::Point {
        x: field_element!("038c6cdc6ec412667b3efe805c343315db3a5c2912427a0f3c19bc9d21b684c8"),
        y: field_element!("0153052fa359b780735332adb12a4d324243edff6e6e8775d2fae22784237b05"),
    },
    Affine::Point {
        x: field_element!("04ec8b456c02869637bd186cd418c8e6bbe855e3ac4b321ab808d57e717184de"),
        y: field_element!("03e8b35abf1445dd19df23436763e589230b183005fe89c98e23c599f1dfffde"),
    },
    Affine::Point {
        x: field_element!("005dcb4da984a674632dcf3cbbb87735e0be59b120e2d8a30a3bbf346ccade3b"),
        y: field_element!("006695ed00f79b96e96021f3e6e1a536b2ba9dc8837e828481e28fab42c3bf89"),
    },
    Affine::Point {
        x: field_element!("055bfdcc44e3a9e9f0fac10d87cbc2d90be5b72c291d4377ceebdc977ee2f294"),
        y: field_element!("00054bb96f071e71b65627af696335189cb0178856b065631117b3ec4586dc6b"),
    },
    Affine::Point {
        x: field_element!("03059580106ff725d04a637206f08795536523ce4b8a5716dfe97b86f85af4a5"),
        y: field_element!("02325c501facd4bb184446bb108ff1470a03ed61b4905a70b89a43929431a7f2"),
    },
    Affine::Point {
        x: field_element!("046c10cdc1ae393cdf544b0090ebba4117a03a3501a0c8845763749653e5f220"),
        y: field_element!("0086959ecb198479d5511212bc551c383a12f518a48cfb155c4d5fd8c6f9c7e1"),
    },
    Affine::Point {
        x: field_element!("0753e572551308f2e94ff628a3a352a5e59cfeb373e2f0e177e6ff4f1be1ab5d"),
        y: field_element!("01c000f9c255e329885d55d1458d0582dee9f93ecdededbb5d12931104bcdd79"),
    },
    Affine::Point {
        x: field_element!("013ab8619c06fad2100636f5ed708a92408c873ebe3fe89028d5cc7298d9e8c1"),
        y: field_element!("01b24831e210fb2499ecd2a9ff87eb7fff401e11e25f5763365bb9fb242f99df"),
    },
    Affine::Point {
        x: field_element!("02292e6b8c27c8a977b123aeded796fe37bb68c53571ee6baf20897174a2d4e1"),
        y: field_element!("00ba0896006bf57da121f6b79b814cb2fb9113b0f221d75b53c404da02240421"),
    },
    Affine::Point {
        x: field_element!("0778c20c7d4a33a7e8e26989c90ec6aef97622f3535418085eeff0b7babc8033"),
        y: field_element!("0254308f2a07d3abc19ea85ed81f8a78440d557d0e335f4bb86ba52faac396f3"),
    },
    Affine::Point {
        x: field_element!("0419f3e919dbf6d23d28a9a9e5f3ffdc1eeccc1278016712cc2c29708b3e4cc4"),
        y: field_element!("0132cc04bb1843edb5c729f11456d7599b1a6b13c9aab038ae3729bdbf721530"),
    },
    Affine::Point {
        x: field_element!("05285cb2b815ee9cd838eafcd698adeb58c452701d0264f5ec9e3d4fb7774421"),
        y: field_element!("0179968b4a0a462e44660c8ce2a7b6be9887f218041a1a93ee9ed6b0d0665599"),
    },
    Affine::Point {
        x: field_element!("0078b44882ba5ddc09c552fa82a3728a5473fea7f18a2a513d9cb79e5f08af76"),
        y: field_element!("0248a5e2385723cac35132b4daa5c30e95cad8ea6894cd2f7f3ec2d0cc468a26"),
    },
    Affine::Point {
        x: field_element!("063144eb0536d2b32aa4b50f697256573c545e5bd05938bcd1d9729b100d6f87"),
        y: field_element!("033dba8e35402311817905a37cae3e479c00e8692c3845022b376bd2e4bf7e93"),
    },
    Affine::Point {
        x: field_element!("012a63be7d5315e3c1fda7e10a55ff5d099a00c9082591a14fa16575e704dbc8"),
        y: field_element!("03d56a6cf1f164d1ebec0da6a433d53dce36c71cb1eb3988497b055d7100fc3b"),
    },
    Affine::Point {
        x: field_element!("04c0a60f3fd82fe8d7aba32ee46352428b22bf1416c8e1d18be3177a4c20cf60"),
        y: field_element!("003de4f342d3191e8578cfab612f738942d8c70dddb5ba6ef0c354ed3b83ca87"),
    },
    Affine::Point {
        x: field_element!("04923ea416a246f4ea3e91248065a05859aeaf494d8d80ebb4354cf57df478db"),
        y: field_element!("032d8ae6516ad0b2c07f49d86e415f6efe1ebe72e9cf627905822b0943da1fd2"),
    },
    Affine::Point {
        x: field_element!("051a6694e9c167c666809ca2394aa6ba3e93547a8ef91bbf445e4d93acbf2554"),
        y: field_element!("00971271ed3d3a7e17cd7422607473fa0a3fb2db992df927e3b7cfedfd6a38a4"),
    },
    Affine::Point {
        x: field_element!("05d23132a41c1f30ee4b9b4fdc0812090bf0a727950a2c94c12616e85c4ad497"),
        y: field_element!("00c9f03fc04961cd3b0da60b8b4048566a023ef4493c7e717dde329e6cb9bc56"),
    },
    Affine::Point {
        x: field_element!("03a87f0b141322edd517dc98ca218cb03a9dbbde130055615a3f5d755725aeb9"),
        y: field_element!("02722295dfc4058fb6420d0bf376c0841e758e156fdc0f8d2a14fa339465c794"),
    },
    Affine::Point {
        x: field_element!("011058c21b7fd243baf75c1917dd2c490c4809b564429a25282478ea6c83dfc9"),
        y: field_element!("02ccda127954a4e6487ca7e34f34e92a0f93d9cc48d88e581e1f100105663307"),
    },
    Affine::Point {
        x: field_element!("04189ee0696c22a9af5c90f92c5f7995cdb645e8843e5f9f94e1d8497d59f069"),
        y: field_element!("03590482228ab14f4dd2146a655c05224dfbe9ce81e0b5affb6e35b3a660ff87"),
    },
    Affine::Point {
        x: field_element!("00a3b1f7159214e17b14ef828c67689bd4a20cfb79ef34abec4a0dd312c2d854"),
        y: field_element!("035fde95d181cbc99c880d7b39f992f6757d6caddcf84af6360f8bb0dbe523e8"),
    },
    Affine::Point {
        x: field_element!("000c252516c860f718f5a858172705c53f05db9ec60ccf40c53f18311db003a0"),
        y: field_element!("00aa739c9277491865218bd16c8e2c79489238463be2efb1fbcb04c32a139b2a"),
    },
    Affine::Point {
        x: field_element!("058411eec55071ef54d05cb74db366fb6a0fe5931e6a9a09a495b29363267b21"),
        y: field_element!("03efb7d17bc0135c575660d440c454515e6cf0e45be5cff853613a15bc97df37"),
    },
    Affine::Point {
        x: field_element!("00061f580e8e1a2b872890c869698d92a7c75393909465ca09ba6dfebd811afe"),
        y: field_element!("0357d9e9a5174647102c5484104b4f271aabde5a39661388856889c9be5ed261"),
    },
    Affine::Point {
        x: field_element!("051c1b6630602128add6bb158aa6b3f108a4d070fbfe62850e7ef6b61806f0d0"),
        y: field_element!("01e94aac8603483608cde19b05fc786012f10ab0cbdc34aa747fa0bc1fa1375f"),
    },
    Affine::Point {
        x: field_element!("07aaae612fee4154a5c60d1208cc055b6a8762aa8e61cc8feaea032369d474ea"),
        y: field_element!("0373357b78a3e88f2fca7a986983de11b59c06a32ee8b0cad8c9b15cf1058b21"),
    },
    Affine::Point {
        x: field_element!("07bc872769701c0a4926a2e7d2719a541443f282fc646903532d19ebff4ef867"),
        y: field_element!("0343c263478551551e40015b7e97a2daac82c5ff5e644897fe1aa462be5eb8f0"),
    },
    Affine::Point {
        x: field_element!("01cde87f65d6bef73b513782eb07d7352811611cafe7a839fd4a623cf007f06a"),
        y: field_element!("03cba7dcdfe535869c6f522bb8e70cbcd8b18d0b3ae76864ffcebac06f824ea8"),
    },
    Affine::Point {
        x: field_element!("00b64e2f8b90ff241ff2ffc853b70cc7f48e84e4fdcb440b9fb6214584181e3b"),
        y: field_element!("009e480a1200526cf3ccfaea666a4317079b1559f0ebb14c557a056554439413"),
    },
    Affine::Point {
        x: field_element!("049f95fa49f8153e36cc0da4daec61d37b46d0317505e50b85b5e1748adbb9fe"),
        y: field_element!("0295929fa14e82f0b823be5c9deb4a14331c8727e825bf0af25882f42d3e2dd0"),
    },
    Affine::Point {
        x: field_element!("04e53f0ea1e4c5bdb23fdf171a1b8485f2f7df09eba23a932b26fd960c42ce15"),
        y: field_element!("03d9e8d9ad91332756cba3fc2991b107ba2584cbf2eb541e36595a62c53bf83b"),
    },
    Affine::Point {
        x: field_element!("04c9811dc82bb472649ce2a83b26a8b0739add96d81a815db74d62aba1c9af1f"),
        y: field_element!("037d1a8e7d4fb54495b3061d1b7a31de4e28d5be0adfd139b386ae09336b7373"),
    },
    Affine::Point {
        x: field_element!("000a61e05260c14b08af6458c01541dc8d76502ffb64d6b09e6718856cc6f9b3"),
        y: field_element!("01433405305de6ae4a74108b6cfdaa34234e1a9a83dd6890678a7a742449a272"),
    },
    Affine::Point {
        x: field_element!("052e87e3161aa42db450ac4176e278d0582638d74f65026224972854d2585ea1"),
        y: field_element!("016bf36f892ff94e592c03dc3e9437390be875098e1a8f7c517fa7a88f1ac61f"),
    },
    Affine::Point {
        x: field_element!("0191a50daf714d63c78b0d8d75d522ca1969de0116b003b715978cf6e598a660"),
        y: field_element!("0253f6357e6bd29bc25ef287a770fcf8c8ba6b3fc0b3bf3cd9cd6ce950f88a23"),
    },
    Affine::Point {
        x: field_element!("06f8efc97a133a1980d265643edbb77309f77bb7c7bcdbbf8465a39ecc2e98fe"),
        y: field_element!("02e3275ec2d9f32552cf4992bff312a738c9e1beb7422c5e07469e197d3fdc27"),
    },
    Affine::Point {
        x: field_element!("04511e53e7407684e2df402b5594974aae9969f472f58a990add693f18c792a2"),
        y: field_element!("0088bb48d92fc6bb5767fec804156898c280136991ba56685ceb58bff4947767"),
    },
    Affine::Point {
        x: field_element!("056b558b770429a34b5f00527e8769675945f25a96e13c8c1f3240347297bb95"),
        y: field_element!("02334622850e9a18ecc41ee58183e66866d2d05bcbc3ea2ff6173278c0b2886c"),
    },
    Affine::Point {
        x: field_element!("04ea79cafaa0d6ca4731b0385af5197db944ba7d4f99de3ca3cab3f2f535d197"),
        y: field_element!("00ce6fe74c7ddb9f40f2684bcbe0723e345d8196500009976ef7214e0cee97e8"),
    },
    Affine::Point {
        x: field_element!("06c65946cd51da9c30a36c38089e1370be2ed6b964bc3bf7a58d8050bcd867b9"),
        y: field_element!("0238b983afeed5c6d09ed9fa73a92f5cbe3178e15a809dffe13fd294cc8e20bd"),
    },
    Affine::Point {
        x: field_element!("0261993ea595627123f820a0be8db872f636da9918e0311f467839fa78591371"),
        y: field_element!("03a5a1598ff99ba03822675b51ce7d2a8016697225ede7b4107e9e85e3789aa7"),
    },
    Affine::Point {
        x: field_element!("02462c8623c8174d0f53c95a9e65cc9011bceb9fc5c5e47526a101ccb55a8948"),
        y: field_element!("02e6f6bfa0b2aaa64f569b6340af9d920dd2409973e2778d4debff6a6aaaed2c"),
    },
    Affine::Point {
        x: field_element!("027f49416646b6ada84b3445933b77c67f064ba44afca74dbb9e96e630d3992b"),
        y: field_element!("0386d93fe81c98d8b65244112a3dece1c310eba6abfee10baf0e80e8f003b048"),
    },
    Affine::Point {
        x: field_element!("02f5028bdd9240da43e90640d3248f7492659b98d5eb1387b31306f9669704cf"),
        y: field_element!("020e59f6098ddad615ddfa242d46ad749a44b9e212e347f921515cb0c2a69d91"),
    },
    Affine::Point {
        x: field_element!("056d4654489ef310c5a3e3e3619397c05ecd0137b9ef1c599e3a8c66bd941ddf"),
        y: field_element!("01c9007009a59af79e57abc29fc855da3c98d55e70a133b64c6c84f25c472b88"),
    },
    Affine::Point {
        x: field_element!("041fce82bcd947a9613cf294a76f5da3fd918dc8ba351f51ed74f80f4e8c42cd"),
        y: field_element!("00ae9359056ead47305403ae84d2d1d7614ccc8054422207971b4ad0a764faeb"),
    },
    Affine::Point {
        x: field_element!("015e3056478280b334654259621a3bed58e75a801da500dafcda038eae9be424"),
        y: field_element!("03a428b22f9917b769a1dc4d6c1b6cfaed1381af6837e8054e9fd0eb89c920c1"),
    },
    Affine::Point {
        x: field_element!("04e6305e805d8631420ac29e2a12c2e932a401b2f863eac5ece2c8cdd6bbe23e"),
        y: field_element!("026e2273b24d76064f16dfd0c22a1ca2d7ba07c22db5fb4bc27cbd91c96c9054"),
    },
    Affine::Point {
        x: field_element!("04f3f304708f201959306dbf9ba166af01d34c95178fa34eeead515aef013d30"),
        y: field_element!("0299acc51dd47a63cb63ccdfeabd7434471a4d5bf5a6aba76702d76c1d9fec56"),
    },
    Affine::Point {
        x: field_element!("053d656782440f86eebbaa47a2f532fcbd4e8bb7698ba0bd0757a0b4a44969df"),
        y: field_element!("03f23655066cf2564d53196b5e478e9441be0a677dd7b3f94b8d597e56f7171f"),
    },
    Affine::Point {
        x: field_element!("0315b0daf52a1e0602ecfb6fa4946c43c169acec9ceab2908e70532a7f663964"),
        y: field_element!("036a3da480637dc8dd703fd49bc81eae32b1381a352d3fbe660658c9fc277b7b"),
    },
    Affine::Point {
        x: field_element!("058f62b1c8e9f5d3e02e9b0e628190d8294f2e9d55e72bacc711c5964c30faf6"),
        y: field_element!("03dd53d695fe87ba764bee0c86021b691c79f9b3db5b07704b929cdfaac31578"),
    },
    Affine::Point {
        x: field_element!("0572732409f972a703aa18c58ddcb5cc80c42b0cb1142eaa553839974c52fe58"),
        y: field_element!("024a0e5e42f3cf78d1b7c1b5c97f8c464f7898f73501154e2187ad7fa77612c5"),
    },
    Affine::Point {
        x: field_element!("016129b93dd2c1ccc5bcf47b4d62a4c47a70099ffdc69fe0b02a5fb08116075e"),
        y: field_element!("010f0f9ec67a9bef8b365ec2cf474e2a047765f1ad1aa82649f81a0ef4cae9f8"),
    },
    Affine::Point {
        x: field_element!("006fe01f2d82f67d84bdb63445a260706236e0073e06968080277195c3658dde"),
        y: field_element!("00ef34821239685fffb3aba7777de5a43dd3a94426d3d4bfbe8b96419a6bd420"),
    },
    Affine::Point {
        x: field_element!("067158f0a5d04e941f4e09828304664f925f5d498cb2446848604791842365ef"),
        y: field_element!("02f6ce44b87dc9d7f771943fe1d6e32d97f7830726e7189eb03c2e9386756499"),
    },
    Affine::Point {
        x: field_element!("026356f7af4e696961dd162be81175febf7438684e5f446069b42a839830fbdb"),
        y: field_element!("02786725c3ab42854a42ce48bb5d7b4850d4625fddf592ded3ec5d85bfc76194"),
    },
    Affine::Point {
        x: field_element!("03c0561ede5bbc8a9e5107e1a20c98b6eb4e5b132c5159e21eff0e1b42723754"),
        y: field_element!("0175388519464d14aaf44e3685742e5ee7e00cb7394bca37a5264eaebe27f512"),
    },
    Affine::Point {
        x: field_element!("0084cb46cc865d3a4cbe33b6a9b6914f4978619f43ffc2af110bce073327f1ef"),
        y: field_element!("00b5e2e749dc47c2a4d74be827960368bf5923b7aa40b4c1492744548d3b60b5"),
    },
    Affine::Point {
        x: field_element!("0500e9d04912d14b0a7db89d1831325560b2396f204a0acd224f2b78bd613c93"),
        y: field_element!("00046437356cf1f9dbd9e14c20f6b7238ad45b8e5b6faab80e396846da729d33"),
    },
    Affine::Point {
        x: field_element!("01ed48efb7f4ab118d981948ed9dad3c934916d196883fb38db6409d4727c7d7"),
        y: field_element!("01733b4ab9a39d0e9fb8050be258d1753b8c738cf0ba2a59f21fc28059a325b3"),
    },
    Affine::Point {
        x: field_element!("0016fa4f5e056509150307406411de668a1bf485cf87cc688cfa9d8b9c36bd22"),
        y: field_element!("01167fd0b50d6a022a55c0ba462fc526c9465c93b8d22e9b5637f07251cd7381"),
    },
    Affine::Point {
        x: field_element!("056c7bc2ee3772c853e13f3e41d92f933884d665da4507afa57f2c784feb9e01"),
        y: field_element!("03c9bc6bb94431647f5d20b5bd786256049114ea89e33f614d7faaec94b085fc"),
    },
    Affine::Point {
        x: field_element!("013435174ebf1269c01ac67c2d4835ca50f97da8079551da3bb81d565895ac6c"),
        y: field_element!("02fc4561001cb0e9b1b060161f8a80ac32ec660b921e61400f09e6c45a89cf14"),
    },
    Affine::Point {
        x: field_element!("00badcad87064fa9f2f6fbdf93bd2f85ce28d27a483f2e1b08d984432368cc4d"),
        y: field_element!("00df1970d13aa99f79c96109af36abeabcb5e6eb26c5b792d9cdcbc08c2768a9"),
    },
    Affine::Point {
        x: field_element!("03693aeaa365e29d41e1dea2572f7519dd9e05eb6d50b605611442f251c673bf"),
        y: field_element!("00ff17382e0fbd4620fdfef8feac243d960e5a5ce3d966ec55a048f8e00ac2ba"),
    },
    Affine::Point {
        x: field_element!("01bf80fc784e00f25c457abc4f715801a68999a737af38473dede51303fabde3"),
        y: field_element!("014a0ed006e5c80a2a60c1df426d00ce305a36f37ca6922587dd786f00b1ba4e"),
    },
    Affine::Point {
        x: field_element!("04ea26f730e82bb715a98e41b1a1121603e9047cfdc8beae0dd6d5bfc78e07fd"),
        y: field_element!("014e9dfb0207cd78739909419c953ad759022fdc7dcd8adfd7351f2fe44c3999"),
    },
    Affine::Point {
        x: field_element!("07a654eac924976eebf709bdf6df6823c6a42448d1dab91e1144603d50143a13"),
        y: field_element!("03ea9ad72e22f330bad29064a532f8d10b8a66b6b0220b0020d57402bc3bf85a"),
    },
    Affine::Point {
        x: field_element!("01d08ff690e0903b8b382c00c5c4756ae1a6db350973ce8bc04c24a386a6a9cc"),
        y: field_element!("03ec9aa0ebcb0737183abb1c2d9a6e300dfac9d2e92648a20c33569243431c7b"),
    },
    Affine::Point {
        x: field_element!("045c10f66cb3f2bb8c4a2bbd306f9e908e3193c57bca384d6a9f55f793038ca2"),
        y: field_element!("0312dd1ba6a156e639111fc6d9babc1012e7d848933ce3864d0958193329c3a9"),
    },
    Affine::Point {
        x: field_element!("0574879407a659e7c417c32d0a28159ac24a97bf6f750e174e7c5b67f2ed6bd9"),
        y: field_element!("0168b6c5beb28e89c1c06881c00284684f614370f9ed10bead771b4014f9db62"),
    },
    Affine::Point {
        x: field_element!("024f12eba2431aa3c20307ecf79dd60479dde2e0d5a9ac78681083decd2ba4ca"),
        y: field_element!("02f7ed9b912c6b299ec02a19e77518aebbed9369e320870707ae317a36c6c1f7"),
    },
    Affine::Point {
        x: field_element!("02db90a9462cbffdf993443f5e080dc8705aa07d2f6300732c01413c38d0db35"),
        y: field_element!("039eae7cc6a1c5d435025b5c8c2923d4a1867e3aefd582aa50595f520add184c"),
    },
    Affine::Point {
        x: field_element!("075d49b3c2af0f3bdc86293fa485e4fb814b690a819ca0dcea3be20658ff8167"),
        y: field_element!("00bedcc0fc3c6c607b451777e810164d2b5057b041ca31bd6329f1b67bf52b07"),
    },
    Affine::Point {
        x: field_element!("030a30018f83fa9570825f684d65c76f550ccc607136493658af066fdc64473c"),
        y: field_element!("00aaf4fc2b42cdd1821c5542b8eb37147d43daa6e8da55efa58016ff9400ec89"),
    },
    Affine::Point {
        x: field_element!("059e613a7ca62c41df5c01df12971d51e95f24e7674f3da167d4f928b2ae030b"),
        y: field_element!("01f51f4e10d2fa855c04a7124169085db7ebc5e8cd281b5d1b9455bad5be5965"),
    },
    Affine::Point {
        x: field_element!("00a41a8eceb53ec99de74daacfb1d48abb041e4d07caa37e1c6884c0f9d18c51"),
        y: field_element!("01949090ffd620036df483b141b26bae3bbce67568fecffe7f2cbf37250e0562"),
    },
    Affine::Point {
        x: field_element!("033f01db3bf539089e2ae9ac287e21e19e0e552085921bbc89648a539b4aa20f"),
        y: field_element!("006d695e7aeb8a17dba02b48c06d171adfb12d4b50fb6e40326887d4d7a6392e"),
    },
    Affine::Point {
        x: field_element!("004b1202ce344ab5294b5444f58b008546d35ba097a7e18286cd2b64db9ae172"),
        y: field_element!("03290c13e43cf7dd9def659b15336bee35da3e5449ed54dff86b930b02e825b6"),
    },
    Affine::Point {
        x: field_element!("00b6d09248d4e2ebf1d9da24f7c4dffefeb43ca908ddc9f081df9a82755483b0"),
        y: field_element!("0074cafe46ee3b77364b9ee89a59db623b7498352affd018437eb8e3e10f04b0"),
    },
    Affine::Point {
        x: field_element!("058458d3be3fb461d4ca976e8a6695d7955e9ee52f90590a06058fb18dda5a90"),
        y: field_element!("00bad5162ce015ac879d77da811db4a2aaabf3b294d25edfff306cc62b5c3960"),
    },
    Affine::Point {
        x: field_element!("0718576d513a5f6717f4f14dd6b0a24647f21109bc831443a68edf19fcc3f3ca"),
        y: field_element!("037d9fd35488739b9199f27db8828a5d94cf26f05df47b94538e38ca7fc5bfd1"),
    },
    Affine::Point {
        x: field_element!("03d66f7387a2f0f119117d9ee1da4803c6f3b90907c0cf0435f1946c572f71fa"),
        y: field_element!("002cd845a19413c4cb246d692dfa0af6611dbcb0dd4844ff1833992c0dfe3bfe"),
    },
    Affine::Point {
        x: field_element!("02c236d4e94f350a87250c49b7c1d5c0d1c32da3d5b12b77de97d1cd4da2ed19"),
        y: field_element!("024258b56b934b224075bfdeafdec5a52e9d0c1b0a0764868384254585677146"),
    },
    Affine::Point {
        x: field_element!("045e03b48f8f6d0889b165a5d6ce7c03848605ac657fb1b08e3a0ffc8ab2c9be"),
        y: field_element!("0037640ffb9bb16014179c483c0b832f7736db547c4151c31654375493b3f4b5"),
    },
    Affine::Point {
        x: field_element!("0783df8e738b742f86dbc0a5d0aaf0b463c55208bde94ddb25193b0ae7739fbc"),
        y: field_element!("01988f5a1f153d1ac4ac9f05e6ca960e4fdb737411758cd9237ab8fcb489d95a"),
    },
    Affine::Point {
        x: field_element!("004f512bfaeaac539597787724c0c9fe3205911ff0a292e66f5251d872e0ac27"),
        y: field_element!("007bbd0934537c79b21388b14e26fb4e0a45ebbd785cca6c61494be6f0ff8be2"),
    },
    Affine::Point {
        x: field_element!("00438890e188c16416bd3bcc4647890a5cfc40e3ee260ce487614ccc1c6de2ce"),
        y: field_element!("0278ac92539181ce6ecddd3cd17f26d8a9867b9d04d648b39c23037d8eaf7793"),
    },
    Affine::Point {
        x: field_element!("0559f41c2500783db84a29531d3d6eca35b11c6a5284dc0d9ea210b0b8bcbd45"),
        y: field_element!("01af8fdc9ab17d3c90c898d48c89732c1d5fbabb452df0f12e9b2f3cd5382fea"),
    },
    Affine::Point {
        x: field_element!("01939ad1ead846c989da906ea5d7d7d27cdfd1be7027b7c35f0d96e8c1e52d4f"),
        y: field_element!("03589e320242129a130dd0a4dd9d2c35a346125c7622d4065aeefd76bb7cef67"),
    },
    Affine::Point {
        x: field_element!("00d9aa7be65971fe1f13e7cefd760e687e7e36c54de8a7dabfa93eab020c7b0b"),
        y: field_element!("02852492b57e05e7eb61f7f5ecff2ff8a9633304f84322a247a60573713d51d2"),
    },
    Affine::Point {
        x: field_element!("012e45784c7a8dde161dea8576c37a4d01929f82573eceb30842bf3f6239f98f"),
        y: field_element!("009b26069ebcb1760c1f6a45e05a0866d91a59a4e3dac1f44e2847fdbf238cf9"),
    },
    Affine::Point {
        x: field_element!("058b72cbf5e61094c4c6c9b4e79a7d0aa76d75a82aff3960ab54c6318f5897fd"),
        y: field_element!("00ac843352f14f3f3531d289f869e6231e567e16a6db69c4a8482aa46dbc1646"),
    },
    Affine::Point {
        x: field_element!("0067ffc4b0746aaaa66bb63537751c00becf3655f49304057dd7f6b64e44b774"),
        y: field_element!("01b9c00e857958712025b68969b60e560c1c9f0d880340e27c06cf76cbcac16e"),
    },
    Affine::Point {
        x: field_element!("0526c3852b714d6a6b7ef396e5f9a3297c2a3852093494aec0ebff6aee7f4135"),
        y: field_element!("01895a5b0c68cecb06ff6ef75edd471c453b14d7a06adb78b5d58a3ea055bc9b"),
    },
    Affine::Point {
        x: field_element!("029df50d0411f78e22037ebec835df84b2e2cbeb8eb2f83d6645491f773489eb"),
        y: field_element!("0384199611665149e477891823daf1fad954f12512fc1822318c25df404c081b"),
    },
    Affine::Point {
        x: field_element!("06cb707af3dad0b40f84ad9a33967298cda9474e57604f733450c6c404cd6dd3"),
        y: field_element!("01895b1125d8111ca615fc206f852fded518edbb99a29f21d9a804a73073dc95"),
    },
    Affine::Point {
        x: field_element!("009ba020b37ad153c3f36f62ddf299d1ae80bdb88c12d070e6b004b9443ca83f"),
        y: field_element!("0261fd0e65708a1b2b1cea6f02b8f176fb02ad97ebd0cff629aa515bce8b7963"),
    },
    Affine::Point {
        x: field_element!("0438d911ea6ced64100a995e28d018f9bb7cc10659f6c443114f2a83ff5b88c7"),
        y: field_element!("02b06c0174eb6bca5415cde472aa859c9867f293210b47073737d021cd456529"),
    },
    Affine::Point {
        x: field_element!("01a78a9ad366d9440f2b2f342d1de715c192fe727d4dac52e3f68a0a27206d79"),
        y: field_element!("02fd60b87e2200966b34cb175974fa552b6a330020cdeac85dc7ceb2a045d1e7"),
    },
    Affine::Point {
        x: field_element!("050b59340cc2d93b25db843d49b2a697c57fda22c30ee06c90097bfdb108138d"),
        y: field_element!("00d7f998135d65ca9c75773d16b5d9b1877c4033ad21817f2012d33139659e2f"),
    },
    Affine::Point {
        x: field_element!("0408f45f10d9f96139c8ba7b54fbc15be7763fd69367e0163e16f3c8f14fe006"),
        y: field_element!("013faca83366df102c48d975b5a05cfa7312fbd73de27bdf646867f3d22b6d7e"),
    },
    Affine::Point {
        x: field_element!("00b05c91e84b896d8badc5849e81c3890a2d91c53b30e481c9fa4291d3c7c3a5"),
        y: field_element!("00948e55d2d19d55d516a2b5796f1ec89a721608d620aeabd3cd4869df82e56e"),
    },
    Affine::Point {
        x: field_element!("00adbd1c2236c6719d8fe50f9c02989d97802833d24260472c28a09d83415e6f"),
        y: field_element!("00f9326b136a335a90e86ea3358c25a3bbfa79c2b4264828aa46f8d9d3889cf5"),
    },
    Affine::Point {
        x: field_element!("005b439ca306b0402abc67fbf5c7be8d416700a827229860e16232067267842a"),
        y: field_element!("005a20aa91a01e9145e18f6f0f05ac7d080eea0819ab85920eab615fb8e0e4a3"),
    },
    Affine::Point {
        x: field_element!("037a8cdd72b8fabc23f72fa5f169eb12240acc72050ae9d0f1f0c201b3df3376"),
        y: field_element!("02b85d08aa3b0478e496e2bf4856618784762f16437a9f4fb6a20614b42ee212"),
    },
    Affine::Point {
        x: field_element!("0407c683f4d25319fbbf2584086ae7b3624963f441a304be2ffa89a0cfd82176"),
        y: field_element!("005051dd3aa13b13377ba29f7a26f6418e5149fdda66654767409af1b6367234"),
    },
    Affine::Point {
        x: field_element!("01ff5abfc986a66c66d814fbd546b1cbf4fc3cf5c47eff265f0add90d0f6bcee"),
        y: field_element!("004c776071e5e1d646166ef0721ec81c6b0bf9889d8c9c1522d1750b4a75e90f"),
    },
    Affine::Point {
        x: field_element!("029a8c8b71f7710085d59ec421fa20ae2d15345a8461f165cdb0c1ff9cd2e6fa"),
        y: field_element!("024e69df0fcc28608d54f0f21663fe6aa6806dc9dcfd3914c4250d5d802bdd7b"),
    },
    Affine::Point {
        x: field_element!("035080e09d2319df82b603cea6203f8ca10068461d19018cc0b584408669b5a5"),
        y: field_element!("004d882a99b51a6638d659393a13d0059829c14c39c3cb90d6abb4c6415f0ad7"),
    },
    Affine::Point {
        x: field_element!("015cb3c6e8a4bda3d6463c48d7f296564effd59b38032ae4f4c4caa58e229bcc"),
        y: field_element!("03d221f8436a7e4f38e65b6396c0e360d374fa3c69ba56924d40d6c761264f0c"),
    },
    Affine::Point {
        x: field_element!("0375ccbdd7dc82ccd95fcd8b0c182d2d0fb13a27a1f3075468d010bff9fd31e2"),
        y: field_element!("03733c4098a7f955430071c6f05fa38aff4e88d07ffca592160d7171c8612740"),
    },
    Affine::Point {
        x: field_element!("0705aa5a2f3fd0939fe508e8421edff01b5043d4db56af49a5c5848880fa6a7d"),
        y: field_element!("03b414ac278c8f6709d0fb140d6f4b05927dbb267349984a3a41c3ddad86d224"),
    },
    Affine::Point {
        x: field_element!("026dd650bec4a8b9f4601bccbd5725db68ba08d5a2612094ce1f07e062da52bd"),
        y: field_element!("00407f17e79b0c103ad116b6d2089033f210a30357f30ecfdf7835d6385f903e"),
    },
    Affine::Point {
        x: field_element!("00838615ae3efae78b37201a3a9d49a83244fcbd392aa012e4d152c650206618"),
        y: field_element!("02ca44340d36cb32e8b4e19f48eb9c4bee1e9fa450bc9918d4b2972e026063af"),
    },
    Affine::Point {
        x: field_element!("032e72327bbaca46f72b6195ca00e84484edfed67dd87486f3f894111d1cc9a3"),
        y: field_element!("01e198e249efaa79420d03bc1ba4653221a6f3c1b13903f011e545f888da9d01"),
    },
    Affine::Point {
        x: field_element!("00281e022540a98c39c0f32aa847c43879158b198516a3f0a8ac9b5e8ce28ebd"),
        y: field_element!("01bc9e7f29ff1f260f6e0cfb7bd923b7b0831818864d5655cc8cc070bf39f9a5"),
    },
    Affine::Point {
        x: field_element!("036742056dda9882344a829d2a45b4fadf97ff6b178f3dc3cf1fb46f376b68c4"),
        y: field_element!("03169ad289013192cc2fef8ba528e08ee7034a853b13244f77d3ec1050ea8758"),
    },
    Affine::Point {
        x: field_element!("07f3240979a82dc23c9f1077a1138cb1a44e38f57578a5c27722f129bd984ea8"),
        y: field_element!("0198fa22f11da3c9730ffb228c9c3591fe6ee462415941dfb1acde6cf3d37e92"),
    },
    Affine::Point {
        x: field_element!("07e119a95e3f72f6251a2a3c95159054f62ccc4b5fa72d7844786a48aa28e737"),
        y: field_element!("0070051b735806131459fd7f6c917798f9d7052e83c32ffe2b52616e701459f2"),
    },
    Affine::Point {
        x: field_element!("02ef576dab46cb67b0a990daf7a1a417da0f228ce3f46155a52d9050afff0a20"),
        y: field_element!("00fe8a34f8449c00a39838dedaf1eabc62d286b1cc56d396b78ba856e3240ad1"),
    },
    Affine::Point {
        x: field_element!("050c93a6c15022e2a33506208e935db4da381ae4c18cad1fca56650eb0ac8bbd"),
        y: field_element!("031dae1ea0ecab1d42d0f334e2c51b421f86dd821833a6e5b8f579cd87558064"),
    },
    Affine::Point {
        x: field_element!("0211242f7d18a0b85ce2a96412459ba04b890fb715bb5595884f7fe3104ce8b7"),
        y: field_element!("023ef07698156ea472d1b9ca80834e177154c345a9b70df52d12ffa8843cf3c1"),
    },
    Affine::Point {
        x: field_element!("0777e18a1e80d899d359af44db80a3b121687ea36bdc2ac18a4be7c50bc223bf"),
        y: field_element!("00121e9a003f73b1110e06f662d759ccf997421150887d5fceaed37df5b5fa8f"),
    },
    Affine::Point {
        x: field_element!("0089f7802731a3442f2ff718dda7b7368f6cfdac0a96a9ceb75e73eea66f4dd9"),
        y: field_element!("0044cd606cfa5d0c6be6b771e563123273c9d2aa942d74e289d0914d3cbeeef8"),
    },
    Affine::Point {
        x: field_element!("0192059be178b6a77457ca4457608bbef70bdaa211a3978d0a1c3f5f3a13dd73"),
        y: field_element!("0005b2801d11e59cca5a6ebb88afe978b4401de5702edc929bdd60e07f652da2"),
    },
    Affine::Point {
        x: field_element!("01d7b8d1f3c2e27d48727e8ccf11bd637e0c819441115822591209c173d67bfe"),
        y: field_element!("039b1a215c330c7c09e11754ec67835928e899cc67883f7f61f66dd9cce0c254"),
    },
    Affine::Point {
        x: field_element!("0127a4cadd6f44607e285e19fe8fe8955aee203b8eb42df5b22f85297590ee51"),
        y: field_element!("0362cbdc8de6a5edd6ced843ce4ae04e8ba958b054d3432a78892da2197aea94"),
    },
    Affine::Point {
        x: field_element!("073b380f64a7881c775b7922b3ed8906a0e21733216059cdd871bc002023c1fe"),
        y: field_element!("00336024dca051d29e88dec9cb75586daac63dcacf80db7feb22ff1181389119"),
    },
    Affine::Point {
        x: field_element!("07666cf671d1c1296806e678d248f226475d7ea660c575504c826ff6e5f4e112"),
        y: field_element!("03f07d41ac52208ee42bfd8cf3f6d94f1cdb3286da4a3b04604b15b2183b10c1"),
    },
    Affine::Point {
        x: field_element!("0730fd3cbbec5cd49a95d7ef612309431291ce2840863ca0f1b55352c4e3ca3c"),
        y: field_element!("03c5c0ff31efb73f7c6c1bbf3ef5e82b803a7c5032da2e61459855922eb93f4c"),
    },
    Affine::Point {
        x: field_element!("014a3f2c1caf96734052b1e6e1b623d31b6e312fefbb030f9da2defa592f49d1"),
        y: field_element!("030afca0254b5c36c9870c0749a259245d5461a5cef2147c39af838a8648bbb5"),
    },
    Affine::Point {
        x: field_element!("030688d726172033c7a90acdf1bd6db78f73001604cb864897926fe1e9701409"),
        y: field_element!("00e995bf1fd235741490f42b0935870fa00222161502bc3f437820740f5d919f"),
    },
    Affine::Point {
        x: field_element!("072310aeae39ad3d4b98ce69326ef9958178a92b0b961963adbc8d25ee70ec87"),
        y: field_element!("01193f4e981a9bcea10c8f573c9a6d3827ff7653276dbbb8f8c3f40f0607ea07"),
    },
    Affine::Point {
        x: field_element!("00bc2a355884516bd5cf1aff0c57f1781dbe015cfeb29e1f7a4d10e1a3efa778"),
        y: field_element!("01bd062cdca4456a052c372d2c224f8a6a1b651d3c31b45df5823e30652a71d9"),
    },
    Affine::Point {
        x: field_element!("0512524c09bdf5aca8ec9bc1927f183129cfc6588a7656862702ed1ecb6f162b"),
        y: field_element!("01774c66f3ee65b8c0868211bfb50c924978b9fe845aa524b7fb416babd0e5db"),
    },
    Affine::Point {
        x: field_element!("03a65a9dd00f7f75bf541e37305af1f03f54379717c3acce37a8ca51443f44e5"),
        y: field_element!("00afc5e57b6afacc4bfac384907dd62c01826bb3bd8a5baf3db2bb6cffab8e44"),
    },
    Affine::Point {
        x: field_element!("03143807c7e69f4a73a4afbc217a81601860e79568f7593f2b8605cb36248c81"),
        y: field_element!("01da6593b6acbd8db6c112cafa531d1bc095486d80d90653e725242333b001fa"),
    },
    Affine::Point {
        x: field_element!("041659b16ea2ddd0661971a198f1c17fa1a1ea7de633fe6b15ac78f477bd5419"),
        y: field_element!("02de013f5340625c71c99a5471ab5f3033e3a71c543f2758b0942e7c93539433"),
    },
    Affine::Point {
        x: field_element!("060915fdf37823cd89581857aa7402c57124adff8ce0ff1486cd7a97c8c95cef"),
        y: field_element!("0255788d130887ff3b3ab8b7f81b46f1f32be0329795e50f673b8c4e80c3a2be"),
    },
    Affine::Point {
        x: field_element!("026cbf9d690690cc406bbc03638b250f9b5d4df81731689f629fa9fa4a787216"),
        y: field_element!("03adfd4e6c8aae788fdcf163936a7c66a0d9595e0f3662558fc705bb031ec287"),
    },
    Affine::Point {
        x: field_element!("01dfaab1c31b78498c27d3f451d3f8659f388c02cecd5c89b33297be872fb9f5"),
        y: field_element!("02a55287cae8692ec02d2bda7bf3fd9490f169b34f3fb76aefa63dbe45f641ea"),
    },
    Affine::Point {
        x: field_element!("00f8c7de1d0b8aab6a8505e12e302456b5efb03d4e7850498f2e9562fc7e7e82"),
        y: field_element!("0218af27398a539ef9e265854e994828f44b957a57a211538244d9d811396774"),
    },
    Affine::Point {
        x: field_element!("0103933392126540328b995e4fca239c6dd06db8e890a32d08e224b3649eee3e"),
        y: field_element!("00633e43a2e882f4d5b73acb2b281967d5255ceeb207325b0d574c7c35c41a0f"),
    },
    Affine::Point {
        x: field_element!("049c4450e16d8ad5e683b8f463c0440255adb95c810bea824d1d639b232a14a7"),
        y: field_element!("037ad47b079eeca63f432507e64cbc8cbcd2d130f78721f9ef72ae430c38f12f"),
    },
    Affine::Point {
        x: field_element!("06f60cec11f269f33a91f85856c2b4b78a0d8d1c4d2f8177fab32fa5e855d5d3"),
        y: field_element!("003b2307f8a5481b4a8fe568c85158b06d35f574b4b5d1ffa8ecc13106cb9bc4"),
    },
    Affine::Point {
        x: field_element!("00a4f8b61a496b9e628e1c08d06352cca0cc1799307cecc52064b418bfb23f64"),
        y: field_element!("00b896ed603c94d09afeda13bc2001c0069a37e96bf31ef5c1c1a9a941427864"),
    },
    Affine::Point {
        x: field_element!("078be7deba1d4899651e8144e0fe27d9debda7ca5e2400a10b51829fb60085ce"),
        y: field_element!("0210e250e46409dbc62c58e5afc0f0225e33640a55088d6ec5ba50611e4ad360"),
    },
    Affine::Point {
        x: field_element!("0450c570dc28d999cf745718676c16bf1ea6c99dd0e4dde799665f9acbe3a4e3"),
        y: field_element!("02358eb7c9beb423c959a6acd06ceca32a8682eef047a6a40893c6bfa19c53a1"),
    },
    Affine::Point {
        x: field_element!("05319fb8f8fcb6c5f2131e1366c974c8e36b2d2f96f540b4dabde200beadc6d3"),
        y: field_element!("02478272679fd188d181071858bd495b188af772dae5c61e5569195239724856"),
    },
    Affine::Point {
        x: field_element!("02c9c205fcb6ab9297d9eb8b64a7034c87bc0855d0afa23f9b32c395123477c0"),
        y: field_element!("0351db0a3195c9848a000b1998f6f11430ee09e062cb1757ab8736061f6bfbb6"),
    },
    Affine::Point {
        x: field_element!("01c421af41c7e57f66b34f2062950cab18a6de7725fcb375995ffb0303d1ca9d"),
        y: field_element!("002d76b9628e7e853c500828c953ec190f9fcb7d17133f2c61dbd78260c76562"),
    },
    Affine::Point {
        x: field_element!("0341cb0758588ef90f7d33ad9c5fb15bd19d93049dae582d37cd9133073941c8"),
        y: field_element!("02b8a7a8836e880b0943db96f6042e40a3f319e790eead07396c8bd21b9d40a4"),
    },
    Affine::Point {
        x: field_element!("046cde37b100ae9a0237283bd3debbefb66dabda119ec6cc957679655f5a7564"),
        y: field_element!("020d13468d32300a97e06a953d99ea20d53aed9b6a4b147c82de54dd5dfa64ff"),
    },
    Affine::Point {
        x: field_element!("01248c6b1c95c9fad6052e5b9051d6a9d780e4b1776cd61c266d93f512e0f50c"),
        y: field_element!("01cdd50e7cdf6960ddaaac11c2f5ca70a9769bc93a796d58ac48a6f23e6a1f31"),
    },
    Affine::Point {
        x: field_element!("00085bade0120bfa82d7fb8c6ff49c1a2ef803a63db52763026fb00a4250f4fd"),
        y: field_element!("03a87c5af09d605472f68ac49bc0433b5d0a417923f00ddd7cab0c834b984ea4"),
    },
    Affine::Point {
        x: field_element!("0629b38f9771b9d20337824f60ffa351853b092112a4745750497a41cf798c09"),
        y: field_element!("01c249027ea66f37321e7965a73beaca60de8c94074ca45466ac068724746686"),
    },
    Affine::Point {
        x: field_element!("019a71fb7f6a4c4e7aeb088d386ec4bca80e871ff90cc7c5208e1192333a2c27"),
        y: field_element!("027fc83be8d0265706279719692fdfd78f238c969707ab17f706f8ea89e3e90a"),
    },
    Affine::Point {
        x: field_element!("013a9654e65b5884f6cb093197bfc8d235eb2c5de6b6815f362f3acf0e9e28a3"),
        y: field_element!("0058ce0948436d4a6111efcba348a2de6c2a0b9daea3ecbdd57db71be65eeed3"),
    },
    Affine::Point {
        x: field_element!("0550cdd9621f2ed5bdc2188a0a25aff90d63277de406b2d8c487ef27a281df24"),
        y: field_element!("03147ed88781a4276f96f63f39dd98079f7f4f93e8e553d78851837edce7822d"),
    },
    Affine::Point {
        x: field_element!("01fb78389d91deef89039d400eb182c1afaf7689d17f581ae06103fbdfbc97de"),
        y: field_element!("021f3a95952c314457aadae66dd169436580c8dd8ca6f9ba067e07e9644be228"),
    },
    Affine::Point {
        x: field_element!("031aa5b93c7edd5216543625f67082467cec3785b085bb5761541339e7f84827"),
        y: field_element!("028fc4810a2f801c1eb577e448190fc8a9c3f13fa9775ffc40613b36e9f1d992"),
    },
    Affine::Point {
        x: field_element!("00b3ab6b6e33571bb652c08fa82e9ad4c2d55492a34e7771eaafb7057cbcee9a"),
        y: field_element!("0271839db76c1ffc9bb53c4c83326075db73e927fde708cad3d6f175143f492b"),
    },
    Affine::Point {
        x: field_element!("07bcbfdfe79d3f695e29cb4a57869084fa6c8bc9f449a972309a44afc45ff44b"),
        y: field_element!("004f91172a2321f7475e05298358e69164a536e327be76bc07a9dfe622518e34"),
    },
    Affine::Point {
        x: field_element!("035bac8fcb38dcfc2327b5a238789880e3a0dd1aaae39ebb5046b100602ecce5"),
        y: field_element!("03383219f23ee741b38b597a79ddfdbf9349e5ba8362218bf9140784acfde934"),
    },
    Affine::Point {
        x: field_element!("0580ba87145e037f53fcd8bc1b78f3bd221e3e9ae8e5f5ff52d96d2b45c5999f"),
        y: field_element!("025d643819ec25831faf4ae524cde4601c7aeb792923be0871c6114523e3c288"),
    },
    Affine::Point {
        x: field_element!("056f1abfe8e65382dc7c8304ddcb6bc36a5619734e18049c4f33fad14620040c"),
        y: field_element!("01d491cb25dc175829f6e7f7b7593455519ba9a81e34e71179310d57fefd0d63"),
    },
    Affine::Point {
        x: field_element!("02d0a1badcde12ee5efc936b454ec563f65c38e5ed40094a09afd9bbe46d881e"),
        y: field_element!("0352288348e213120f5253971700a5556bd69503cf56844caaf9dce64cd1c3b7"),
    },
    Affine::Point {
        x: field_element!("013b90428ef3dd67aacefffa4ab8146b2a67a450ddfdc1e1198b428f584a8113"),
        y: field_element!("0005c6ff6f9ead51e640cf27f4e58c5bfaff690ff4a2fa4cb77404eca31807a9"),
    },
    Affine::Point {
        x: field_element!("03fd5f0a73c8a1eade6422f722e5d78beac65d401f752a78c952efac43223122"),
        y: field_element!("0207fbc56a142b26d20a74949a1bebb34e6bdac4af8ef6f9f870283b1b5e6e9e"),
    },
    Affine::Point {
        x: field_element!("021744e6cc98e7e3cca329f6b635a707bf0683554c50f1e55f3c667ca493ef43"),
        y: field_element!("02feacdf1ef3ca2562b9bb3bea4f564266f026862d6363e98861744b5cd1dac6"),
    },
    Affine::Point {
        x: field_element!("045b59f365381ba9dc109c3ce7ef91286512722d34ae35336454a5381329a73b"),
        y: field_element!("00f8562c02e1e53c76ff3e93a3bbc39ac7c2a2f86080065e97c1a07834476f53"),
    },
    Affine::Point {
        x: field_element!("022ac196588e0a6032ee1b8b83c3e60018e0d9562a804060623b4152ef38a973"),
        y: field_element!("036c960b27997a1753c3c05ad093a9e07f03eb6cb5ddc6f5326562e7d801d7df"),
    },
    Affine::Point {
        x: field_element!("06a66440f0653ba12478455ac3d815bb3a3a40a2fa097d02d0cec54ef630859d"),
        y: field_element!("0075ea8096d8291a5ef5d2321231099b24680e0446651f9ee23dfebce3ad1d5d"),
    },
    Affine::Point {
        x: field_element!("039a7f6cb7fc01309b484ec0ed2844235a09abf71496c6184fddf3e909d9bf45"),
        y: field_element!("02bb8794d7d753d350d1e53073f5e673fe84acfbccd3d20d44422d31ede595ac"),
    },
    Affine::Point {
        x: field_element!("031dcbe0f4746b881042d1eebaf7a5841b58d1a64a849e845fde3a4aee493032"),
        y: field_element!("02a0afffa6ab68f9524bbbdbb36af0a39fdbe6a7d964ffccf0e947a7ee63f7d7"),
    },
    Affine::Point {
        x: field_element!("03cb75eb614c135913d2338c4c53a17e4b15d6cb97a78be26e2fb60603a958f9"),
        y: field_element!("0273ebc65951e22847d31eaa25e4cde6db12669424736d3b4d44c2c1fb6b3fd9"),
    },
    Affine::Point {
        x: field_element!("00568309249c8fe83f9d3c3339535804bf7ee24578af4615959fb72c7de40247"),
        y: field_element!("02afeab112f11a5bd48d8b1bb22c68e191bf3478980895f438b5808cec84ff26"),
    },
    Affine::Point {
        x: field_element!("0590545de29ecb8082a74279a755fa973ebc14f3819cfc3c26327dccb89de723"),
        y: field_element!("00eccd156287d7e1487a00a22449f7fe38fed70467a87e4784918675029044b4"),
    },
    Affine::Point {
        x: field_element!("01a94194a59ee3414d2d0c13f4d98fab5d85abfc76d05961c8efe21187749879"),
        y: field_element!("0346ab539687db75820ccb592577e7c08b6982d9db681738981eed9fc0efbec1"),
    },
    Affine::Point {
        x: field_element!("0213608e313cffc2618025dd07d8e4e415e2b89d14ae10e051c97b77f0d0831a"),
        y: field_element!("02c5abca367b2456835a547c18071b1d8cab32fcdd3784333c72fcff9e2869a0"),
    },
    Affine::Point {
        x: field_element!("01846fdab7baa2c1bc7d3d2d9f22720548c86cb35a77ed32b564ca6bed867f7c"),
        y: field_element!("024220535ba437709c3d9764d8f29b5b9c8fb6e64f2a06141a169dc7167d6689"),
    },
    Affine::Point {
        x: field_element!("035034186b4743cd0b4afe092b6440aafd9fa083560969bebdb066bb386dd484"),
        y: field_element!("011777878bc82bd641c9a51548037ed4d420190d09dd697e5b43642bfbddef50"),
    },
    Affine::Point {
        x: field_element!("055e298f7cce440373a31910e7f8699dc96fdd3b336439550ea6d93aa9c4e524"),
        y: field_element!("0074b13aa25f834aca8820548f0e2362871b8da75c6ab582f0a09b234e3ba80e"),
    },
    Affine::Point {
        x: field_element!("06ae5047cc4598fdbf889f455911bda75a48686d8540c858494017fd5dc63500"),
        y: field_element!("03817f60bc08a03712f2f0d62e716c2558715d0519e49184754b2961e0f207ba"),
    },
    Affine::Point {
        x: field_element!("071168dc8a3505c00d577934cb8404cf74b30d85750eee5a71290a4811b215bf"),
        y: field_element!("038cc70107e91c01d1cd74f0b2334b2fb451117e39f4c59ee10f3ddb16ecefba"),
    },
    Affine::Point {
        x: field_element!("00b9195df6661eec4ad4b20ee4a8d52f081be4c348ce857cf5cb2d87928eea6c"),
        y: field_element!("002b4c1780ae4521ac7b265c816f9a1f2b727dfbd08fff4248c3431efcf46cc4"),
    },
    Affine::Point {
        x: field_element!("023593cd26865a81b13da87cfa43fd498fc7667b2ca433fffefdffd24a146b6f"),
        y: field_element!("013ef986a94c0b2c3e41c8408f13c3a142f6572dc990c17ec4c2d72aa697961b"),
    },
    Affine::Point {
        x: field_element!("027da9f5c357f283c2b8fdc3d8b77b7107a47878c9596972a981819270e95e54"),
        y: field_element!("01cd0caf236e84c40f45efe1e39c368fe7d6fa6284f6e8a671bc05b6d1410710"),
    },
    Affine::Point {
        x: field_element!("005506ee3571a3531b32c01ff7ae73600c5d2814fb6ff63e911be12845f83ff0"),
        y: field_element!("03413d4c85134e8b63f9fcb251a57302e398049d34d951bd612a905643d9c612"),
    },
    Affine::Point {
        x: field_element!("07f19ec46b6ac9001cf62e619a40dcb546b53a5603572e196d83d6a64de24f10"),
        y: field_element!("01c69eac87d3bad252d7b8fc008172ba87a4c86b6b61b45862d841cc1aba97d9"),
    },
    Affine::Point {
        x: field_element!("00ada50a0748f152cad57bd94eb5984d45943c8c4f614a4adf45585bfe23dd1e"),
        y: field_element!("0042d6d2f575c095647475442090ba3bed3da012e46ee49f6b2472efcad71ad4"),
    },
    Affine::Point {
        x: field_element!("054736c0cbb1b0bdd8036041e02bc76f84a7e21ad01f1417a3d449d015501a7d"),
        y: field_element!("0162b46dd1cfcb87ddb9c3fb3968e058d4819bc7c368ecd97cf6949db27fcd4f"),
    },
    Affine::Point {
        x: field_element!("070857e8cb7e706d1e42530141387553f613f423fc922bcec53ac6b56b871555"),
        y: field_element!("031033c16b2daefcf96985b729f7c89636c318747073e78fa3f64a4428fa875b"),
    },
    Affine::Point {
        x: field_element!("03af83fd6bb5721bf7881db4c0014a08fe4ba4f063074b8fa1663ecee515117b"),
        y: field_element!("02b8ebf5e9ca786307781b70731c9fc976edaa90cb36a0ae71f0d37432e2b40c"),
    },
    Affine::Point {
        x: field_element!("008bd447efdd38e48a86b8f7c8e0c949f45ece42375a8bbad34f5b4869a8ae59"),
        y: field_element!("011f934be306bf21dff6abf6f3d34de57560b346207c83daf97d93ad537fa02b"),
    },
    Affine::Point {
        x: field_element!("010d9556d32069530f94ba5d47c6dccd8c7fe610333736832d11aae8f0c41f72"),
        y: field_element!("005db88f419caa4944a4ded0c76dba189c04e919dfa7ad45f0b5fa6047072582"),
    },
    Affine::Point {
        x: field_element!("020df9104e7c499f57b47595331d97896290dfe0421fcb65d2910ea99df9eb19"),
        y: field_element!("00656914f3c4f5de2bfe5f8306b2c76acd1531c65f2866263291ae7d83f5c539"),
    },
    Affine::Point {
        x: field_element!("042fe8056bbf1e37ccc027fdf0f0baf59e079c603021547e59e69babc454891f"),
        y: field_element!("03f076333b26585afb77c954e7982a65caca801984449af73bfa7660344fa022"),
    },
    Affine::Point {
        x: field_element!("009adec676ba1946c07aa7cf8212f6ff72fa03ed1d7b795e5f04d2643fe8af1d"),
        y: field_element!("014fd5961123dd56aae0179803dcc03bfd6461f965f2c5800c8fd733798d46eb"),
    },
    Affine::Point {
        x: field_element!("04203df4165ed2ea934d3e7afac84563579acaea6b49ddeef7a39e8cac68f2ae"),
        y: field_element!("02a4371eb14d52a189c5b3007db95213b3a6548d297c784e70a4b22ec37d7f02"),
    },
    Affine::Point {
        x: field_element!("073a5cf5f5113c855636c90308b92d0fed0eba87898ac26d645cbaf0841e499c"),
        y: field_element!("0355a15df028eab0cc7b52fe7cbedd04b92cbea4c7897dab53f73e0b1bb8ffb6"),
    },
    Affine::Point {
        x: field_element!("0145a105ce46e537283cc81a7d4f35a0fd204da0f55f04c9c20b2a2487d33c6a"),
        y: field_element!("0115242a32fd0618983ee35c13f446edc0da4ac1a08646956756489c4c2645db"),
    },
    Affine::Point {
        x: field_element!("05867b2619036ffd683fdb76ccdc3b2823e7f4eaab846830dc7d7fad641becab"),
        y: field_element!("016902265b0b8b17b8bb792ddf84d9f2218f64fbe5b29ab579d144c388a87714"),
    },
    Affine::Point {
        x: field_element!("03f2e5d2b2004e1ef8ebe412b346f41aee37149a8a9a70fe6a7b555313a0572e"),
        y: field_element!("025e4ad2652202141be26ecf39a029c90481c90e6428bd5c34f72e4994d32bdb"),
    },
    Affine::Point {
        x: field_element!("032ea7d52cd7b375ca8bad7463122ca3d8f4ca337883562291f9352b652d946a"),
        y: field_element!("00924efc10a782741f45d78953a43404c6925042d46be35b2f9c556ab58e272e"),
    },
    Affine::Point {
        x: field_element!("0545dbc5df3151516646fb0536dbf0c1fa1297853d231c5e37c61ac735577888"),
        y: field_element!("00f813e4c6a7e41e7c99cc742e94276e2aee0161aa6cf977b8c9d7637d5a5c1b"),
    },
    Affine::Point {
        x: field_element!("01cbb022d67a70ddf17bb2fa319f243757aa31bad689d591f11c3529ce68badd"),
        y: field_element!("007a0256ea44eada33147541e0fe1670cf7ca781beec0a4a14b57a80c3c5470e"),
    },
    Affine::Point {
        x: field_element!("0497382d39f2c0cdac1265194f303667abf1c0bb8aeaa7743ed74dfb4d28f8b8"),
        y: field_element!("006b13394c5b574201011c011e40dc57f1f6d52dbf2f53ac9fe8450a3e625eb8"),
    },
    Affine::Point {
        x: field_element!("07477704fc1db3f881377fca038b023af8b6ee7c347e3da9a79c93c169f64071"),
        y: field_element!("00e1daaeef76f8fb3c6946c44eb0ebdd0212ff36b0f8cb102d41db06f220fadf"),
    },
    Affine::Point {
        x: field_element!("003f1ee0fe8c3932d2c9c6310c2c5e631433b9a0c0292c1a9363d8c86ad288b4"),
        y: field_element!("01f628453c2a9071dfac78f2a77ec4460ac6d28c7b4c899572eca76a0bf08a7d"),
    },
    Affine::Point {
        x: field_element!("03e4340e0773d92b076030e831361f6be16d051997cd07802b77b44c90673c64"),
        y: field_element!("012ab7a9bead28f300ac8b20fb39c36b995b80ce2e98b39fb9c9371631c92b0d"),
    },
    Affine::Point {
        x: field_element!("01f3aab2cb61c569c349667cf66051d158fe083fb97d292f798e0e29ab2ee850"),
        y: field_element!("0332f985be95de4a99ac7335c08d59447c12bffa09e49eac1cf146851c36d200"),
    },
    Affine::Point {
        x: field_element!("0018539daecb3e031563ca7f87381374fdf8ca02491afacc356343780565a13f"),
        y: field_element!("03c845d293df5493e4fa439903b7a122c4210360d7c135910c9e294fcbcb09c5"),
    },
    Affine::Point {
        x: field_element!("037f0e2aaa1b942720f696a34904231c77eaf8da3d36ebbd9e5a8d3f1aa62a46"),
        y: field_element!("03fb5f0da5f8e3f04eea484d915bdd522ca436a832e0345f7e3f0eb87829a753"),
    },
    Affine::Point {
        x: field_element!("0348d098093e3fcdf5e5d5b46a4d01ee64c37748bf822c23a8f3f0ff862c2048"),
        y: field_element!("01961060b57a80f3931ff9def81b1650fd7ec474ed71a4cb9814c25d2bdd8a2b"),
    },
    Affine::Point {
        x: field_element!("057bcfe37fd9822afe658f592aba136b79225f40dfa7914286bad7d243947d42"),
        y: field_element!("0329b2323491e91d4fb9f3433040572f905641487929a41403407544a3cd4924"),
    },
    Affine::Point {
        x: field_element!("01a4888b439229953110808831a5f6da862a44560115d0488ef02263c5a3e2d4"),
        y: field_element!("03db533ee388102f89e59dc32b73822ff08e995244909a2863163389642763be"),
    },
    Affine::Point {
        x: field_element!("01f14b751f7ab1f7e35ca95be3b9c682332e473749d8b736aa8f1c44450fe7c8"),
        y: field_element!("00f7006ffab9f83953ddc7e2c468a9458cb57a89f95e1b18a8d4e56afa53213e"),
    },
    Affine::Point {
        x: field_element!("01ae0d09833efc21095ab784957c6965c7c797477bb9e12eb8dd118287200b1f"),
        y: field_element!("0083b457a5cb0561f48d2bbfcb218066b79d8507102610fb00f5e4b97e6dce53"),
    },
    Affine::Point {
        x: field_element!("002a1eb679e3c07c179a050f87b6c9f99103667c80abd98d51c269d30012da76"),
        y: field_element!("0212c872b8d71bbcfa683ceca886bbadc8473d373b11254cc0ee0740bf3e37dd"),
    },
    Affine::Point {
        x: field_element!("073daa2de127bea4e2f39eea46db0fd265eb95054ff0b8a7f279736cce773a7d"),
        y: field_element!("02289c9f40428d3acbeb8e6720d8d915fbeca837536c8a41f074e0ee20352e40"),
    },
    Affine::Point {
        x: field_element!("07b9ef4b2d6cd6dfc10beb0864e8f88256b805e3a8a9be2af141d40d87998703"),
        y: field_element!("022ffb9b5a5f012048ce4c95216c66841ab30e720778d1aee143825f93de3c54"),
    },
    Affine::Point {
        x: field_element!("05ec969640bff6aa473070800a43017169545ec4df36abd17e16958b565ce05e"),
        y: field_element!("01427e3e48d02b0371bf27a1357bcefdef946239d4e143982230a06e485ad6f0"),
    },
    Affine::Point {
        x: field_element!("0007a8adb06a10de3232dfdc44b7313f8f1066a14f181d1e80a815e9804923fb"),
        y: field_element!("01eb8f09b50a1566544a22fe5b1b0b1526ec0410fc65b0df91110e3bd04b51e5"),
    },
    Affine::Point {
        x: field_element!("02e3467a6dd3995729cd2bc364cafbf99022bce224aff17e123124a4dddb1c3c"),
        y: field_element!("01463ecd9a4231721eccc7e0bfc6e5e5ad443f653e1923a2565a18ab862d55b1"),
    },
    Affine::Point {
        x: field_element!("00e8430f85e311faa85181efe941dac86fafc936bf57fb6a0790586101a1224b"),
        y: field_element!("00367146d077cf06e806d002946cdd226e18880c8bde085a97956284b055c8c5"),
    },
    Affine::Point {
        x: field_element!("02d00787799d80b66807d8a3d5c3d92c0a1b4a931bb6b25b1935f22c90d063b4"),
        y: field_element!("020f01955fa4984e21ccfbe9cb37c1a7fe8a472de1681d8a1313375482d4c19e"),
    },
    Affine::Point {
        x: field_element!("05e1b6910f988aa91f1b6f2859ffd6d742c629ae2b003fafd58f7768b9ff77b0"),
        y: field_element!("01282c219e5e5fea603f7c3b8010772c57fea147c5a7923a6ed3fb822be263ec"),
    },
    Affine::Point {
        x: field_element!("04be70267b200d1dcc5bf667b8dbb6d5379135fcb5b7de7aaaa7ed42c058ce15"),
        y: field_element!("026d8b05999ba69540dade94017fc1e6fbc0a75b586792cf7601d382b94c803c"),
    },
    Affine::Point {
        x: field_element!("005b7223584d071a6b16d3fe40d68d624fbe71ba62f5d75fe4dbc1782b4a08c2"),
        y: field_element!("006cc0f10d752d7d27477ad8a53b1adeaaa1ba8a90f641d9ac34ba741f590278"),
    },
    Affine::Point {
        x: field_element!("0500e12e3e931d94ad41e70af6bf3964dec0c4b4ca28aaf2f402eb97dd0d38f1"),
        y: field_element!("01d63eef739fd3ccf622dab93929516387c7aed4506740277b595f1d62a7b58a"),
    },
    Affine::Point {
        x: field_element!("01adec6f29dc13827b2593f100019245e2262f98f613b27cd5d655557ed2d3c5"),
        y: field_element!("01d68272866e741fc7b5b904b985b686ddb92679c41f1af6ddb9e1959840b0a7"),
    },
    Affine::Point {
        x: field_element!("07d9e8798277276de428a89255658f0412db682856229a140c9cdac83c74393d"),
        y: field_element!("01b25695e23ee5e7533dc9c6f69802be799799adb4026548c9a7e30be50f1b70"),
    },
    Affine::Point {
        x: field_element!("00fa60748663c7792ea0e046ff695114351f18041dc59d02e5c40970496272af"),
        y: field_element!("020d3b8d4548ff5ad1ee70f4f23182209ef995280bd0384f72a0f0c2a661b1cd"),
    },
    Affine::Point {
        x: field_element!("04d1eb98f30c75ce4649aae8c0ba636e99fcfbab8cfe22c9a3e522da738669ff"),
        y: field_element!("0104abdf1f0707703f8a84af84225e07241dfe6194bf341239fa3c4e225ec098"),
    },
    Affine::Point {
        x: field_element!("04622be97ebc72adae7a437f951c7cd223e33674bf84dc4c8adff8fbb4ba633d"),
        y: field_element!("007c2c4d9d8e4c6c8870dfa29fb254b729e2b13050f22ab666c2c74b8c043ac9"),
    },
    Affine::Point {
        x: field_element!("06bcf7430c00019c282276696200a9d51d04bbf594bd5ef3b102119b1722dbd1"),
        y: field_element!("005a4248eedfa549447d5f5795c49e18baa6e5a8ef66e89ab308106f142c0d3b"),
    },
    Affine::Point {
        x: field_element!("07985c6742d649eaa22fcc64baa0bd946567dc0164d7a4bed9ce51c720a2f6cb"),
        y: field_element!("03cc381671cb33456ac72ee29a1b169374bfbed013b9bdf958ed5ebf1ab40580"),
    },
    Affine::Point {
        x: field_element!("0397083357f2d245a16ec4fa88acc3f1f4a42d352a0ebca82c15ab8ffb260ee4"),
        y: field_element!("000c91a35595a6bb1b3c50613f9d92179cd9a09560ab19f8000c3a0e0231778c"),
    },
    Affine::Point {
        x: field_element!("06fdfd3a16f532e3bb06147b8412f50c92cb64696ee90df6a584487e26640ce8"),
        y: field_element!("00d9d6b6544e57f08abda4bf28f306a01fc7baea20624f69dbf86cac82640a9a"),
    },
    Affine::Point {
        x: field_element!("060fb9eb58a14178e328a27d4846143e1d20617d4c5691989c0d8385e9bcf5fd"),
        y: field_element!("01499fc6fffeabd2dd0cc32f37a087a7c23876a57a8e24a3dbf3660838cc2827"),
    },
    Affine::Point {
        x: field_element!("0010ee4ff729d78582378493481cb52c3cf5b9c9516e2eaa3b9ba5b4af58a8c5"),
        y: field_element!("037b663842055abf5ea05073b76b44e5e1e0bf6219e7ffb73f4f920ff0051e7e"),
    },
    Affine::Point {
        x: field_element!("06274dda6ff42963544b1820c9755f89696aa0501711fc00548f40036b10f6ac"),
        y: field_element!("028a1b47857c6a572d544704ab5164385c65782d3146276274ee875caf164915"),
    },
    Affine::Point {
        x: field_element!("0419b9b210c7c8c521e113b8dfa850e5ec3cf8faca12981e61d7a5fa219959ad"),
        y: field_element!("03137e62ba9ce383201c94bf091dd587098eec45a357bfd3ff4b9ffa6d207caf"),
    },
    Affine::Point {
        x: field_element!("062803675019d91788f6d700cd59e28b7ac046ac0951c88243fd14b66ab83a57"),
        y: field_element!("01333e2832e8e11244f60d21454e41e7de6d20bf822c7651f05b1b80ae22f39c"),
    },
    Affine::Point {
        x: field_element!("02802d2a3b00df159a9a431eb05d79fa0655a8867eea6c7a3e79d964f089ff29"),
        y: field_element!("0260243d4da1deffebe10a182634546487187ae8efe0ae74d01294ed78838fa4"),
    },
    Affine::Point {
        x: field_element!("006b053f5f7e7935f9fe83d5756ffe6d2de910af7c2cd61dc4811f1003073189"),
        y: field_element!("03bfa34d96274577d04e8a4c22cbe3c3660d49035f899c314742df32d94e586c"),
    },
    Affine::Point {
        x: field_element!("04f240c195474538c36e882af034393ed918758ef8d7d72d67906f1c2519b95e"),
        y: field_element!("01c698f429c248be36b3c038932f9fcd1aa26a73250d61ba3f63ae62a1eeca13"),
    },
    Affine::Point {
        x: field_element!("05c9b7161516abb8c0677c150cd80c994852ac302c7f388ba7337b79b7aaa698"),
        y: field_element!("02967285baedf5aca4eaefc6c9015ec2b11727ab5d06e01d5e51e3a438f36aa6"),
    },
    Affine::Point {
        x: field_element!("03154da379ee94e4f167454e8bab75f9e907b0f4eaf026b67092ed0912a21ff7"),
        y: field_element!("03241dddf30e4e7ca71bc7e440c25c39e5cc7aa648cbe7a49fb9d05fb82214d3"),
    },
    Affine::Point {
        x: field_element!("008aa4503d02f5d96574c07dfe47ba143576bc8d4ee58e435dff6bc1fe3a307f"),
        y: field_element!("02918ae4a051b439b79b638782a4261c06a4e972719aaffb010c6991c8d79ce4"),
    },
    Affine::Point {
        x: field_element!("02f0cf27625163b105ff3669a8bf16152e98aa21bef73241036fb7f1ec2d5f8c"),
        y: field_element!("0072b6a69aa4b6388ae3fc810084ede6b04bddb65ecbe62030899058cc778473"),
    },
    Affine::Point {
        x: field_element!("009889222c19789f2472f143282c06f8b83298591d92aa8159304c6da50b4d7b"),
        y: field_element!("029cfeeb0abbbef301ef060f19a4713f7cf469240f9b0ae9545d5185d51044e8"),
    },
    Affine::Point {
        x: field_element!("0236ef547f996b8514b51afff4ff318a83a3485b44f06f85a7f530f6c07353e8"),
        y: field_element!("0398ff05cb3f6e7b82bf1d2a8fbf19bf0707495755dc1291e74596a0b7eb54cc"),
    },
    Affine::Point {
        x: field_element!("03e48269503bf283707732dd31fed9a6f811ec6647678eb85a4239d869b8de9f"),
        y: field_element!("03628a94defc462e89b813bf4b1f271184aea22403974ee9f9f6ddea190689cf"),
    },
    Affine::Point {
        x: field_element!("0003daf40cb09c8ea803f2062c5b7445275740c41af5dc2cd4504834832f334a"),
        y: field_element!("02915d4a7a30c70a0095c58eb9632090c81ee6a58803fa0d562d5cd608a313e2"),
    },
    Affine::Point {
        x: field_element!("05193cfd984dd35231b2f93caec47622a5e61da67484b7f0165a72036b5c3587"),
        y: field_element!("0253ae3b75172c6874178b86d7565ef75846fca8e6916bd42690fe620bd62884"),
    },
    Affine::Point {
        x: field_element!("07b9ac09cc2c4db99b845ca070874049bf27b1b309b48ef6befb0bdb60d0e7b8"),
        y: field_element!("03bb9cd93d09d4f87b989dc68eb90cc2c847a5acec13462f3e1f9eb416bb3e9e"),
    },
    Affine::Point {
        x: field_element!("049860af27c534d61758bc298513d0e95eb1a33c4c43912a8f8f8de8df4c3f80"),
        y: field_element!("015e12c6499093acbec2b666be843257c73cecf55df3a33dd01e5df712a341f5"),
    },
    Affine::Point {
        x: field_element!("001f2cfd555b703defc61fcff0ee87762981c2068308a32ce49170f1efada882"),
        y: field_element!("03c102b1bdc592aaf43107fc19372d99b0fd0ad91c31596463215a187a8c60d6"),
    },
    Affine::Point {
        x: field_element!("029cad7525aff5972ba4ae899f55b580854bfc89c83ed0fdf9828459352bc450"),
        y: field_element!("02650afa537ed3ce796bd4c6ba78cfaa32a1e0f943afffc297c7a1746baf8380"),
    },
    Affine::Point {
        x: field_element!("0793f274f074dc0d0049a05bf133f927d85a5b51ff6cae9577db21d343bf2af3"),
        y: field_element!("0067552159e6ebf698a598acd21fd5d4266c18d989be0af930a7cf05e55151da"),
    },
    Affine::Point {
        x: field_element!("011790fcedb524bf397853b8b0a5ed39e4cf00c47cf165bca10108deae519ba0"),
        y: field_element!("00404405a37cf8ecce097df5b21e507db3f038c149ae9a08dbba50ac4d3993f1"),
    },
    Affine::Point {
        x: field_element!("050784a5563e9a8458d36b45f03f60fce088b71434bb6882c53b4b4009f71d7a"),
        y: field_element!("005bdf2d7f659a1fe0353ac92fbb62f6b66bd019cc56bd2b8d4779fc8d134321"),
    },
    Affine::Point {
        x: field_element!("02523d0247f5ed96f5ac86fd89210ed2a2904de1592cad5f2f9b89ec9c3456c8"),
        y: field_element!("02d0b8f9febf92372a179b5b88c5eb15d14cd343bc94f7c437b6818051beb054"),
    },
    Affine::Point {
        x: field_element!("025bee14aede135b4abe494c3eb214fa28542eec818f7834b19c63c563a1089a"),
        y: field_element!("01a11a802edbffed9cf99df8ec79dff58deba75ec1544b48fbd12d764fd95b9c"),
    },
    Affine::Point {
        x: field_element!("00d12929c7761e4aa7b8efee903b6637a67bc6d885b93ef32e1656165ce4bac8"),
        y: field_element!("02815a60012cc166ff2f2c52aa35b1baeaae9f36a99c15d5bcd5cef9433b2262"),
    },
    Affine::Point {
        x: field_element!("027191cdae17010f989fe09417fa823b062589acdbce4f5493fd9d4a5d4d5028"),
        y: field_element!("016d9047fe6a417e9acc764c507bda8db98d6c08b030d867b150c213a4578e0a"),
    },
    Affine::Point {
        x: field_element!("0614b713cfc3baa9375d01b9cd9c39966add89ef7db24c56bd26a7d5233b63b7"),
        y: field_element!("0207a5ba37fb479b951d28d6055aaf068d2968f227d294729cdd0eddd145b352"),
    },
    Affine::Point {
        x: field_element!("063c492f1a37bfb92de7fa2cccdd6b29253e0a8a3e8172ed65c998544d7575ef"),
        y: field_element!("02827ee6af3f889c1d0db2bab949387c90829fbcd3c1523dee999e30833bfa22"),
    },
    Affine::Point {
        x: field_element!("0462102edabc7fdaaafb532d0de6d01499d6d2738218c7980a7f4f3c44e7cd13"),
        y: field_element!("0114204406abf4c47bf19adeaf0246340d1cdd83e31c323ae8f4cf8c1f2a8ad5"),
    },
    Affine::Point {
        x: field_element!("05293f4e852db1e8f804b64f79f1332ed8199bbdfa2b2185a397a8b7e94c66eb"),
        y: field_element!("0251427a214f56072ce47866f01fe71b439c4ab01235cbd7df59c2988a555d95"),
    },
    Affine::Point {
        x: field_element!("05035cc9489615e70c01af3cfcac6bcbf424d70f6547eeb8771b9c424304dea0"),
        y: field_element!("00ee694e5fe786018758bc9573b8d280f21165651a01cc54b428ec9a0ddc9023"),
    },
    Affine::Point {
        x: field_element!("0291ce67245ac802ba55a2680d4dda17a14dbe427ffc59b4d01dad234a38c8f5"),
        y: field_element!("02de4539f5f3161dfb882ab742ee35576c983b0c8b3fef389c48baf1273a699b"),
    },
    Affine::Point {
        x: field_element!("03e09c31f4adb4a9aa5e3c9308db93c7c278fd24fd2dd31dfd7480eda6c6949f"),
        y: field_element!("00a5c6adbeba0f29340e40ef7dbd86727540eebd047479b08850343412df0ffb"),
    },
    Affine::Point {
        x: field_element!("00192b3df91a84c4221df56667fbac74f143086b0323b5287f83f99801f616f0"),
        y: field_element!("000ac0a4a70031d4b9f5eb7c2d044eab5bb6b2e4a296559f2a8e9b15e50ead6a"),
    },
    Affine::Point {
        x: field_element!("015cc3778ef3bccd9f624a40e54a2c129de824e4edc8dc32be533c6ec58fa1a3"),
        y: field_element!("03df3c7f18b145ec6fbc0a6ef540e39c57034537aa5335f3407f877254a5461d"),
    },
    Affine::Point {
        x: field_element!("01361a320505be375be96610ce11b83d168d381558af96121e5c19bcdab84989"),
        y: field_element!("032aada5081aae1e2d498d588d991217da17c742c50fd605fcc488ba87fe6740"),
    },
    Affine::Point {
        x: field_element!("04aef12477efc9b73c30df994d9d6ac5312b7fa06cb00c0a554b632db6f8bf9f"),
        y: field_element!("01c9686f4009eb65486b0c7b9f68cc50feb312157acff988802bad6bd5f134d8"),
    },
    Affine::Point {
        x: field_element!("0538b41594672280920c79c6d8f6159a84090b816f0ac7bad3e0612a0f296564"),
        y: field_element!("00d83d33e61e373a93742c50aebddd34076c17ad244678d5f28cdff373e2cb82"),
    },
    Affine::Point {
        x: field_element!("009c94ddab8246060e9212aa9b332faf5791a1b3d7ee2eae07cbeca416b166cf"),
        y: field_element!("028c18a4b369987195908f311e44f3a9ea06d7ed787067d7238c2f670ee2cf7f"),
    },
    Affine::Point {
        x: field_element!("044ec75843b57fb81036c6035d53ae9663d9c655e19448279d1cc20ddc2cc421"),
        y: field_element!("000503805e4a2e80a65ece6f1a129231bb96f294e9f29665de13d3b8d76f11ea"),
    },
    Affine::Point {
        x: field_element!("06b34dd128cb251a5b4bb22482f127e5a90d1c4bd07df2eb919241961c6d7b7b"),
        y: field_element!("00accce65064cd4fd4214fc2d60c5865257e9e5b32579d583dfd8560bd97724d"),
    },
    Affine::Point {
        x: field_element!("04448e0fcdf93e94944b81f7de02b28f070730ee64bd78ed365d032f76b6cdac"),
        y: field_element!("03fae31f466973dae115ac3f5f3c36e9bbbaaed3d6e27dcb1ac077dcbec91f73"),
    },
    Affine::Point {
        x: field_element!("06350a1b0b22cf89164db7a9517e4d366507430d2dff83ad75184537bbc6f5e0"),
        y: field_element!("02c07b3736397a3c31a8a95aa990c92881af3749d466f3f09d21fec26de273b8"),
    },
    Affine::Point {
        x: field_element!("00149481fb3229496c2807ec6427c27467bccc28a6881a4fa1137f1bd73272ca"),
        y: field_element!("02852590917167d7d8189405ff5c64d10fe0a4ef2a2a10336275e347add9b3bc"),
    },
    Affine::Point {
        x: field_element!("02b45b63a9c3bfe28314cead74ef372dd502845ff3046d2e9134ed94bc268f44"),
        y: field_element!("003401c067cc17f5e704690a68fc42284ec51d06aa5d85d355d3d192c369fa88"),
    },
    Affine::Point {
        x: field_element!("0125dc05dfa3de8ca810b5d7aa5d5b638598007e764b4387bfa93e5034b283fc"),
        y: field_element!("0283087e98cf9abc8b253ed05efc23dcf4fd8f6439b5d1f4dcbb47f46768c5ac"),
    },
    Affine::Point {
        x: field_element!("0781ce668cb43a010667a7a6c36d66b2ec7bc51ed57d0a9683e6c28947134027"),
        y: field_element!("023f061c691fa1c7e9fc6cd3ea8331cf9d4bd8969637c0fd84317cb8d7564b01"),
    },
    Affine::Point {
        x: field_element!("01145b72495e078da7940ba5e62fd31c87a5688e76adb3597a946e8e651fb336"),
        y: field_element!("013573ec362cf8d73c10df2d7001e8096f7efe675aab1080873de84e43a8de4e"),
    },
    Affine::Point {
        x: field_element!("00f452ecf1f79436e479c7d0b500da10c6fd23a77384197adaa8d8f6cf451d2c"),
        y: field_element!("0301b1f5f50880d6a838cb3ae71d584c10facc07722a8fad4260f9d22fdc5cfa"),
    },
    Affine::Point {
        x: field_element!("027b82d9fc22aad67ab065620f296fef3afff9e66e923fca1527211b802f7b54"),
        y: field_element!("0163b74f8767358038092ecc448bd74220a8c279bee7c6f024db051fba8d7e1e"),
    },
    Affine::Point {
        x: field_element!("01448e006bb14044015c5525ecb4f7c7e9e1e8df08fbb53df4e3d68537769497"),
        y: field_element!("0302c69f0c33676bdd50572aebd732542c880697db942217bb0f3973f54b906d"),
    },
    Affine::Point {
        x: field_element!("04c848645f9f1b5649463153e7720ca6a05ad20c6a1738c7a5326fda716c6fc8"),
        y: field_element!("029b87e3b73f5d2feec68f8774854e9c7a5aec5d3bb7b9535d93308e9293ca43"),
    },
    Affine::Point {
        x: field_element!("047f9d92000b9da0c01ac5d49d04b141ee6388962b5ab150d4b36c72a10dc2fa"),
        y: field_element!("0087175c90cd082e6976d7ab1a7b48224d2145e0771ab1f0a85b6f0cc8c19b95"),
    },
    Affine::Point {
        x: field_element!("0776257cbc540364e6c63a02da0fb982dd38444a5bae99dda68a71ec1d3ddada"),
        y: field_element!("03db95b45f1429b1996492188132e2630ba3101de0fd4df0a409dfc27231a181"),
    },
    Affine::Point {
        x: field_element!("03c17d5f64800ca2d5e9d7f6982299ddb3983fba5937dc3f8800c39f3e2c1a9c"),
        y: field_element!("0238a2919469cb05e68bad75343fa54fbfbc3e9eea7f8fac624d444d7c4145c2"),
    },
    Affine::Point {
        x: field_element!("06b3acf71111be0af766e47df3a8d46ac1b23397d8d822d7fba4efc2c0e4ccbf"),
        y: field_element!("03c4c168a99c04f8896329aded4c5152d8a68e06121e25f471f4f55f2208053d"),
    },
    Affine::Point {
        x: field_element!("0550d9e53e64598503e4a9c6f7111cba6af0754776dde52c12da55366cb61eb9"),
        y: field_element!("02b5030dcd4d686f27418f612757a709e4d0362aec81098d0dea353e97491c00"),
    },
    Affine::Point {
        x: field_element!("02a3f1b5d9b4368ad496bd91bf16de37fa34e52e7a0ca6b058ec30136e20ec54"),
        y: field_element!("01e87d6c6268a9cef609a889ada25de4226255bc691d63a56da36f469b158a23"),
    },
    Affine::Point {
        x: field_element!("007a6e8833e1a59cc49d6e62a9775bcd00246a9a14cf0924936d5128cb0ce40b"),
        y: field_element!("03ffae4943350372bca147924263768062753493dbc7d664513624c02ae7830e"),
    },
    Affine::Point {
        x: field_element!("0110478ea673fdffe4164bf4f5517335aa185e8653c50bbf2c50d3dbb17c1a96"),
        y: field_element!("00790677f005d29e2a82376bd08b641ba0964e3e7fba1d7c0d0cbe6f693618eb"),
    },
    Affine::Point {
        x: field_element!("02c773be6130b6aa8a72a95e3afe0252b572c979b4196611a2f9b52dfd779712"),
        y: field_element!("02ce027f08252315a3bbb4c1eafd6c9d361d6527955a194c2947e6f5dc2145d3"),
    },
    Affine::Point {
        x: field_element!("0209f1b79de6749e4e54906386d6384ba9c59a63cba270b7ce98c6ba080df3d6"),
        y: field_element!("01d55b0e0f998c6c885710a1c1c514812c25b110484939e0f0c734433fef9ff6"),
    },
    Affine::Point {
        x: field_element!("017bf05b850d6f7c3c6e8ea6f616c465423d54949c63f8bd688a16843a975945"),
        y: field_element!("0008ac3dcc1932ff76adb73cf7cd5c6fdf0e0e2186dd59e3bafd7194095bb004"),
    },
    Affine::Point {
        x: field_element!("06ae579ae56984305c61dddc3b86d3fb4a1180b8ea7da84522fedffb7c7279bb"),
        y: field_element!("015250c2dfce1be441ca09d33b50f068d5857f9ddd5213ce460fc395181bbd5d"),
    },
    Affine::Point {
        x: field_element!("05a18cb39b3766783fd3c619982a59080153bb7be74196668d0ee832f79c96e1"),
        y: field_element!("03393108391001e6ae9621c7953b28c48c832b12c132df70db4aa289cdea9fa7"),
    },
    Affine::Point {
        x: field_element!("05bab61b160c623a4ef4b2f20dadad33bc0f084a27690863e89f63137c548d0e"),
        y: field_element!("01a48720ccba65cbc50e628abf79b941458ef3a67e9b8cb8b26cb185a33344c9"),
    },
    Affine::Point {
        x: field_element!("060e95c353d75d8e116245f367f33f32b2e5bf7d00061745006e7e714e68809b"),
        y: field_element!("014ee6c313ceb11ed48ee677af7065a87ecf3d602512f6c9318cb52568e23af1"),
    },
    Affine::Point {
        x: field_element!("05f6e9816b504504ffee341098d06b488e1659beb2d982cf475a1e01b6ae8c95"),
        y: field_element!("0011ed7380da1ffe07d3e6b9e7206e3c66b3fbffdc5435a39160513d2b6a38f3"),
    },
    Affine::Point {
        x: field_element!("02735aa72e7b1a12633a911eeb5625ece89573d0322c5a875dcfa6d06c9a1961"),
        y: field_element!("039793f624051fcba61f6d76d27c46e376aacbca645a11f1d2f97a95ac44cd53"),
    },
    Affine::Point {
        x: field_element!("017d265332e1742ec194bdcda202432ca7462b022290538400634d40d980b7e4"),
        y: field_element!("00413a04660b681abc80f3b43f543cebb3f74214927679aae2531292650d4ce3"),
    },
    Affine::Point {
        x: field_element!("078d95fe08a202717929cc6dcb1025b21444f44948d35eee0a7c35bdf4b4cbdf"),
        y: field_element!("0040d27c27d25a3700b60a042fad2806cb3b77a72844fe16f9ba5ead3641773a"),
    },
    Affine::Point {
        x: field_element!("05a96593daad6104a0d427de3f913dcb265cd6796ff5c0d6e824456c8fe665b2"),
        y: field_element!("03e4dfca1c075a53dff8cc0a81c9402bcea687e33c1cd4478531914c7a6c537c"),
    },
    Affine::Point {
        x: field_element!("066d1749c7b1f99d1938294b1bad1a4719853bbfb9398c6a0c074bb4c0ac735f"),
        y: field_element!("02329b8aedd068face979b935d9f4bd2d86818752f07892e20a6a8bbc12f4145"),
    },
    Affine::Point {
        x: field_element!("010b15a785935283218d70415c8c684e4e4be6f7c9e3e560f154e4ace7e9fc0b"),
        y: field_element!("0154af20ff6493424e880ab39e97c8173bf6efbd3f01a8ee7f2b7b7e4c52ff5d"),
    },
    Affine::Point {
        x: field_element!("075bf97accaf63e1b7aa077214a8e6f050ab5bcd79e8554592c2591c3591ed4e"),
        y: field_element!("036e56083def993b06b237b7bd11aaa3dfc9144740c97721dedd708a26bb8e2c"),
    },
    Affine::Point {
        x: field_element!("04391b43aeda01cbb8067a4c9c3007c9086c247eb20f605dec20b7a3759a5d1a"),
        y: field_element!("027c424a167cbe5e8e7c23a7a30dd6f8a8c686b9a15e30d462a4f2d8612ae15c"),
    },
    Affine::Point {
        x: field_element!("039e7b14c4fe5b1220bd6de02597f90ed345eae3560f5f06eb7ce59e1c29b55a"),
        y: field_element!("033fbbbedab86a254f21c8cdcb9852efe20e809aef83c9432d60d4e36b0c1498"),
    },
    Affine::Point {
        x: field_element!("023505446e0fcb59e85bcd9cb24aa17e275d7e4e6c3f5af2c2984977a3633adb"),
        y: field_element!("0132b83aa4e08c969315238fe1e89250932cc64d15402a5cd78135c0db3b746c"),
    },
    Affine::Point {
        x: field_element!("0528a41355f610f31a9cdac5513626cf487065304443a2e11f3840beea8b0ad2"),
        y: field_element!("00d4288e7a5090a0417d1923081ee45030d1f2cb0986cedf534632cdb5323f7a"),
    },
    Affine::Point {
        x: field_element!("07927b4a0e8b3f6033f939a6085fcca27c4f5fab2ed3913b0ebe244c824d43cb"),
        y: field_element!("03b7d649061a04c3ceaf828ae1a2bdce0de3aad1dd00c8c63b7341cb3b3601d5"),
    },
    Affine::Point {
        x: field_element!("013131321e798d54bfc7b0eec5d2d9b6af7c66e8953adc650271714e4875b030"),
        y: field_element!("0225c946d8c0389260743bccb0243ba80228c87efc839d89b77fe6e9eea81d28"),
    },
    Affine::Point {
        x: field_element!("0335182199b61fdb519c9bfbd69726deef3711dc6fecc88c3fd2fd27c4881353"),
        y: field_element!("030afa6721f968d7ff3c3a9d1dae8282567f245d2c2515c34d031848ca70bd38"),
    },
    Affine::Point {
        x: field_element!("0221fb22ebb0ef617429aa90e11814ea9dcec7b82c3346b390548580908b3256"),
        y: field_element!("00aa50cab812b7745a5de06ae392353c99962794c1106abc923942fabe21d6c4"),
    },
    Affine::Point {
        x: field_element!("03f58a6513d52201d72075863d0521b414cecf63bc284226b9f8dd4fac1b0eb5"),
        y: field_element!("03d26e1c5fa187ae19f6a0d663481bf68bed6c760a20f71597180f95b856c7b4"),
    },
    Affine::Point {
        x: field_element!("07d69660e6bd468a73e5491fadb6d64b31ba5d640ce9410161d0fb4f393ba245"),
        y: field_element!("02f3ca4332ba9910ff4d5c7fdca290ad0191d5044e68f2c7d6169fe72316dd2c"),
    },
    Affine::Point {
        x: field_element!("04764bb058399a5151e332227af814783a1f9d11aa490da11ac8b7c55b470f23"),
        y: field_element!("020066aabb3d3361932f4df5d57014dacbdf369eef4cd6752e439a5548f7a421"),
    },
    Affine::Point {
        x: field_element!("06803fe8c27d8e096fe21540d8cb6f31aa79f66b9f0c7bab83d0bff54ae3c479"),
        y: field_element!("00b2cbfeb430b26d950a50c9999028aeecd1ecc6316a2517ea28b95f68c8dbe6"),
    },
    Affine::Point {
        x: field_element!("0432f2c2caac4907ab4d8c0395c4b913476affdb0dbaa2707ccb5616c51c9691"),
        y: field_element!("0140c73d83ccbc48be24f9cb0957bb2a704c25c8fcf822851b0656ced1d51e35"),
    },
    Affine::Point {
        x: field_element!("012920ee31174cd4af2d59497a23c972a82753acfc14e4e5e4e6bc6fdf334b86"),
        y: field_element!("02931cd90c2f479bf6e0a0ef7c04023580e2f65a6a6971d58e636fed47c34b69"),
    },
    Affine::Point {
        x: field_element!("03b54d9a831f8e0eb8f0d4bdb504513f1798ccab2eccd023c0c850846f448a9a"),
        y: field_element!("012236c98e0c9f11e3a72be8fde9336f14760288731feace737831d5da148883"),
    },
    Affine::Point {
        x: field_element!("04794fed1bd049b723f2cf733461ab79a378cac3e3f364e2683b22dd68055838"),
        y: field_element!("03c53bd2566eb206a72ad813a5e9a9b82c77dc6d477bcabdfa6cf378eeab65dc"),
    },
    Affine::Point {
        x: field_element!("0163f53947766190060280f8bb7988598e38c8fbfe65438f529165e10c30fcfe"),
        y: field_element!("01e127dfe62a2166f79aedec567a4f41e51e5f6da99d8863de18c437cee23adf"),
    },
    Affine::Point {
        x: field_element!("0251ca65faf1146010f5cebde7c1afeeca8b60102fc4dcdf2b80c1350ae282da"),
        y: field_element!("03ec5880b1654c5ac7211ef577e2ad376f6dba92fe6b283a748058721fd53033"),
    },
    Affine::Point {
        x: field_element!("00ac99dd28f3178481092059a453aee2279b875d5e95a507fa4c389f172edced"),
        y: field_element!("00359911f230df0b7d218a984dece148e72f9e3fa17c8f52272533278b5e26d2"),
    },
    Affine::Point {
        x: field_element!("035076f08953e4264ced3132094b439ed134db1e22cb40a99a1dfc0c6fa43bf8"),
        y: field_element!("03e407a8870e1b960881930299848bbcc50ae085a5a18511334d15ea3bf718b1"),
    },
    Affine::Point {
        x: field_element!("0195de272f70276bf303186c31a9248d63ab0928bd6f24702e3d5a96eda114ea"),
        y: field_element!("01f18808620644b9adfe27835ea99af6e79425e5fbd7accee73d3d4e95628f6d"),
    },
    Affine::Point {
        x: field_element!("061d40a10d88a0631c06db40bd073345e7f53698756847049fd2af3f44025db7"),
        y: field_element!("03b278351d6fc66d42fee17b7731ff9e3df3d5b62607ca33ddf89f37eaebf24f"),
    },
    Affine::Point {
        x: field_element!("07fd45a0247144d8a82260df884ed2d700c004755512dff068d1dbae818193d0"),
        y: field_element!("02d0be47eb107c7e13458bb1f774c02193519bc05188eb79a303db3d1df8f966"),
    },
    Affine::Point {
        x: field_element!("0473d2893ca9d7473941366644ae28e0fe0f9a2aaa850844888220d9c7262cdb"),
        y: field_element!("0177a77f99fea4d2bdc09af5d997532fe99811416ae817b47414c315f1fb313e"),
    },
    Affine::Point {
        x: field_element!("02c1b7ac5bb9e1716dd0e3bfdb7dbc8d8c93ac35008fd9525226286ab1d6d1bd"),
        y: field_element!("01bb341ae09a25ddf74ca4a0b0da6b894991f672cea955bd70e513ab93333c1d"),
    },
    Affine::Point {
        x: field_element!("07d2a728ca628fd159ea36200cd1d26e778ba6f0da9b3b93b52fd7aa84d2eec5"),
        y: field_element!("01bc3932bb955da301e6a9f1850ccd3aa7243302c0ba5d6ed50addd04fca8c14"),
    },
    Affine::Point {
        x: field_element!("07516bcc579c2618426086d4e8bc7150c6d6e50fc10f2d4b0baf5acb9d18d193"),
        y: field_element!("00e1ccf73b79b333bdca74145e5ad7b7016e55bae838214a7c6fd0b3373d65a8"),
    },
    Affine::Point {
        x: field_element!("040443d8dc6178ec48ed611294b2eada8b7d89e310db7fd4fc995c4569826cf4"),
        y: field_element!("034de07fea32b5f151d64ef16c52b1cc5e37f0369494cd3f9a3862a92eb65fb8"),
    },
    Affine::Point {
        x: field_element!("00dc2969658bd20ba541cd85bf00c82c63374ef35860c73e27ede8b8f78bc386"),
        y: field_element!("01e46692c1f443779b67581cf322a032f0adc5501203a8632ad21d73cead5f79"),
    },
    Affine::Point {
        x: field_element!("06240eb2a837f465a300958d036b054c95f10f84c316821db3bdddf4e99c8046"),
        y: field_element!("01105e2f7b9193fd9d140b44b7261d4ca215d239996a98ef32a910ca166aab76"),
    },
    Affine::Point {
        x: field_element!("0342a92b67440d835b2c1bc50defb95d24f9dc30497715560d12991c84dc586b"),
        y: field_element!("03454cf0298a69ceea2a7f9cd142d8a7aac47f8c86f6cb4ac7e4746f90dd740e"),
    },
    Affine::Point {
        x: field_element!("01da63a6d48bea68dc9d7f488c7d840da9565ce0e4238406591058eb54e176bb"),
        y: field_element!("0240c3958e1453cdfd8a75a953338c509ec1f5e5d5a90d441805a14348015cfd"),
    },
    Affine::Point {
        x: field_element!("00e8315c356f040df0287f0f40d09728b9277869d3e53175cce8832350ede93c"),
        y: field_element!("018943002dc5d0fe80ac5f522c5373992bbabb5bb6aed742b0dd8d9df6b894f1"),
    },
];


pub(crate) fn hash(x: &FieldElement, y: &FieldElement) -> FieldElement {
    let mut a = U256::from(x);



    match
}
