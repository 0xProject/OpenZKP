var starkcrypto = require("../pkg/starkcrypto_wasm");

console.log("StarkCrypto WebAssembly support.");

// Test nop
{
    var result = starkcrypto.nop(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );
    if (
        result ==
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    ) {
        console.log("Nop test succeed.");
    } else {
        console.error("Nop test failed!");
    }
}

// Test hash
{
    var result = starkcrypto.pedersen_hash(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );
    if (
        result ==
        "004cd9415015d53d3d71f13e865a52a70457c60fa534fe0efffe34d2f6af6744"
    ) {
        console.log("Pedersen hash test succeed.");
    } else {
        console.error("Pedersen hash test failed!");
    }
}

// Test public key
{
    var result = starkcrypto.public_key(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    );
    if (
        result.x ==
            "02511bef9567504bdf55d49657f8e20b2b4a5d59b32c8983de33f53d3ecc330e" &&
        result.y ==
            "069df6eb1781680d36aa740c1f4ce0ff9f960a005362c136960ed48ed373a250"
    ) {
        console.log("Public key test succeed.");
    } else {
        console.error("Public key test failed!");
    }
}

// Test sign
{
    var result = starkcrypto.sign(
        "01e542e2da71b3f5d7b4e9d329b4d30ac0b5d6f266ebef7364bf61c39aac35d0",
        "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc"
    );
    if (
        result.r ==
            "0010eaece1a727f8c64faf2f236943c2691ba8ca34e1da77880586f5c20fcf63" &&
        result.w ==
            "077e670848f61ff0a6d7f4f04a4740f8d50dcf8db8e7a4522dc05ef8c2d3ad89"
    ) {
        console.log("Sign test succeed.");
    } else {
        console.error("Sign test failed!");
    }
}

// Test verify
{
    var correct = starkcrypto.verify(
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        {
            r:
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca",
            w:
                "020709125651d6d1147c4f45e72ecd4848432fa86b3b867c9e7f61b47bcb907c"
        },
        {
            x:
                "02511bef9567504bdf55d49657f8e20b2b4a5d59b32c8983de33f53d3ecc330e",
            y:
                "069df6eb1781680d36aa740c1f4ce0ff9f960a005362c136960ed48ed373a250"
        }
    );
    var incorrect = starkcrypto.verify(
        "0218a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        {
            r:
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca",
            w:
                "020709125651d6d1147c4f45e72ecd4848432fa86b3b867c9e7f61b47bcb907c"
        },
        {
            x:
                "02511bef9567504bdf55d49657f8e20b2b4a5d59b32c8983de33f53d3ecc330e",
            y:
                "069df6eb1781680d36aa740c1f4ce0ff9f960a005362c136960ed48ed373a250"
        }
    );
    if (correct == true && incorrect == false) {
        console.log("Verify test succeed.");
    } else {
        console.error("Verify test failed!");
    }
}

// Test maker hash
{
    var result = starkcrypto.maker_hash({
        vault_a: 21,
        vault_b: 27,
        amount_a: 6873058723796400,
        amount_b: 852209057714036,
        token_a:
            "005fa3383597691ea9d827a79e1a4f0f7989c35ced18ca9619de8ab97e661020",
        token_b:
            "00774961c824a3b0fb3d2965f01471c9c7734bf8dbde659e0c08dca2ef18d56a",
        trade_id: 0
    });
    if (
        result ==
        "035d22e6b67d9dbe893149ede8ae5efb82d1a3f97734689f5189031cc45eebbd"
    ) {
        console.log("Maker hash test succeed.");
    } else {
        console.error("Maker hash test failed!");
    }
}

// Test taker hash
{
    var result = starkcrypto.taker_hash(
        "01c280f77aa5859027c67411b6859584143d49970528bcbd8db131d39ecf7eb1",
        2,
        31
    );
    if (
        result ==
        "024e516a8e5f3a523f7725108516bbded20cb290c21925c95836fd66af4c0ec1"
    ) {
        console.log("Taker hash test succeed.");
    } else {
        console.error("Taker hash test failed!");
    }
}

// Test order lifecycle test
{
    var maker_private =
        "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc";
    var taker_private =
        "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc";
    var maker_public = starkcrypto.public_key(maker_private);
    var taker_public = starkcrypto.public_key(taker_private);
    let order = {
        vault_a: 21,
        vault_b: 27,
        amount_a: 2154686749748910716,
        amount_b: 1470242115489520459,
        token_a:
            "005fa3383597691ea9d827a79e1a4f0f7989c35ced18ca9619de8ab97e661020",
        token_b:
            "00774961c824a3b0fb3d2965f01471c9c7734bf8dbde659e0c08dca2ef18d56a",
        trade_id: 0
    };
    var maker_sig = starkcrypto.maker_sign(order, maker_private);
    var taker_sig = starkcrypto.taker_sign(order, 2, 31, taker_private);
    var maker_valid = starkcrypto.maker_verify(order, maker_sig, maker_public);
    var taker_valid = starkcrypto.taker_verify(
        order,
        2,
        31,
        taker_sig,
        taker_public
    );
    if (maker_valid == true && taker_valid == true) {
        console.log("Order lifecycle test test succeed.");
    } else {
        console.error("Order lifecycle test test failed!");
    }
}

// Benchmark nop
console.time("Benchmark 100k nop");
for (var i = 0; i < 100000; i++) {
    var result = starkcrypto.nop(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );
}
console.timeEnd("Benchmark 100k nop");

// Benchmark hash
console.time("Benchmark 10k hash");
for (var i = 0; i < 10000; i++) {
    var result = starkcrypto.pedersen_hash(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );
}
console.timeEnd("Benchmark 10k hash");

// Benchmark public key
console.time("Benchmark 10k public key");
for (var i = 0; i < 10000; i++) {
    var result = starkcrypto.public_key(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    );
}
console.timeEnd("Benchmark 10k public key");

// Benchmark sign
console.time("Benchmark 10k sign");
for (var i = 0; i < 10000; i++) {
    var result = starkcrypto.sign(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    );
}
console.timeEnd("Benchmark 10k sign");

// Benchmark verify
console.time("Benchmark 10k verify");
for (var i = 0; i < 1000; i++) {
    var result = starkcrypto.verify(
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        {
            r:
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca",
            w:
                "020709125651d6d1147c4f45e72ecd4848432fa86b3b867c9e7f61b47bcb907c"
        },
        {
            x:
                "02511bef9567504bdf55d49657f8e20b2b4a5d59b32c8983de33f53d3ecc330e",
            y:
                "069df6eb1781680d36aa740c1f4ce0ff9f960a005362c136960ed48ed373a250"
        }
    );
}
console.timeEnd("Benchmark 10k verify");

// Benchmark verify
console.time("Benchmark 1k maker_hash");
for (var i = 0; i < 1000; i++) {
    var result = starkcrypto.maker_hash({
        vault_a: 21,
        vault_b: 27,
        amount_a: 2154686749748910716,
        amount_b: 1470242115489520459,
        token_a:
            "005fa3383597691ea9d827a79e1a4f0f7989c35ced18ca9619de8ab97e661020",
        token_b:
            "00774961c824a3b0fb3d2965f01471c9c7734bf8dbde659e0c08dca2ef18d56a",
        trade_id: 0
    });
}
console.timeEnd("Benchmark 1k maker_hash");

// Benchmark verify
console.time("Benchmark 1k maker taker sign");
for (var i = 0; i < 1000; i++) {
    var maker_private =
        "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc";
    var taker_private =
        "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc";
    let order = {
        vault_a: 21,
        vault_b: 27,
        amount_a: 2154686749748910716,
        amount_b: 1470242115489520459,
        token_a:
            "005fa3383597691ea9d827a79e1a4f0f7989c35ced18ca9619de8ab97e661020",
        token_b:
            "00774961c824a3b0fb3d2965f01471c9c7734bf8dbde659e0c08dca2ef18d56a",
        trade_id: 0
    };
    var maker_sig = starkcrypto.maker_sign(order, maker_private);
    var taker_sig = starkcrypto.taker_sign(order, 2, 31, taker_private);
}
console.timeEnd("Benchmark 1k maker taker sign");
