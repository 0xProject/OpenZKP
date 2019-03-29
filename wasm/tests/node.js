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
        "02d895bd76790645fb867eaf57037e4aa8af1bbb139a84d01e311a7c53f3111b"
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
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    );
    if (
        result.r ==
            "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca" &&
        result.w ==
            "020709125651d6d1147c4f45e72ecd4848432fa86b3b867c9e7f61b47bcb907c"
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
        amount_a: 2154686749748910716,
        amount_b: 1470242115489520459,
        token_a:
            "005fa3383597691ea9d827a79e1a4f0f7989c35ced18ca9619de8ab97e661020",
        token_b:
            "00774961c824a3b0fb3d2965f01471c9c7734bf8dbde659e0c08dca2ef18d56a",
        trade_id: 0
    });
    if (
        result ==
        "00b196f2b56d05c32cccdc7de5c39f161f9766b93c7796ae3655fd8279325626"
    ) {
        console.log("Maker hash test succeed.");
    } else {
        console.error("Maker hash test failed!");
    }
}

// Test taker hash
{
    var result = starkcrypto.taker_hash(
        "00b196f2b56d05c32cccdc7de5c39f161f9766b93c7796ae3655fd8279325626",
        2,
        31
    );
    if (
        result ==
        "01fc24ac854878ce580cae987e1fb13ba099f39567662e486aaae7c1f892a77d"
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
    var order_hash = starkcrypto.maker_hash(order);
    var taker_sig = starkcrypto.taker_sign(order_hash, 2, 31, taker_private);
    var maker_valid = starkcrypto.maker_verify(order, maker_sig, maker_public);
    var taker_valid = starkcrypto.taker_verify(
        order_hash,
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
console.time("Benchmark 1000x nop");
for (var i = 0; i < 1000; i++) {
    var result = starkcrypto.nop(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );
}
console.timeEnd("Benchmark 1000x nop");

// Benchmark hash
console.time("Benchmark 100x hash");
for (var i = 0; i < 100; i++) {
    var result = starkcrypto.pedersen_hash(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );
}
console.timeEnd("Benchmark 100x hash");

// Benchmark public key
console.time("Benchmark 100x public key");
for (var i = 0; i < 100; i++) {
    var result = starkcrypto.public_key(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    );
}
console.timeEnd("Benchmark 100x public key");

// Benchmark sign
console.time("Benchmark 1000x sign");
for (var i = 0; i < 1000; i++) {
    var result = starkcrypto.sign(
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb"
    );
}
console.timeEnd("Benchmark 1000x sign");

// Benchmark verify
console.time("Benchmark 10x verify");
for (var i = 0; i < 10; i++) {
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
console.timeEnd("Benchmark 10x verify");

// Benchmark verify
console.time("Benchmark 10x maker_hash");
for (var i = 0; i < 10; i++) {
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
console.timeEnd("Benchmark 10x maker_hash");

// Benchmark verify
console.time("Benchmark 10x maker taker sign");
for (var i = 0; i < 10; i++) {
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
    var order_hash = starkcrypto.maker_hash(order);
    var taker_sig = starkcrypto.taker_sign(order_hash, 2, 31, taker_private);
}
console.timeEnd("Benchmark 10x maker taker sign");
