var starkcrypto = require("../pkg/starkcrypto_wasm");

// Deep comparisson for simple objects.
function deepEqual(x, y) {
    const ok = Object.keys,
        tx = typeof x,
        ty = typeof y;
    return x && y && tx === "object" && tx === ty
        ? ok(x).length === ok(y).length &&
              ok(x).every(key => deepEqual(x[key], y[key]))
        : x === y;
}

console.log("StarkCrypto WebAssembly support.");
starkcrypto.init();

// Test nop
// (No python equivalent)
{
    const result = starkcrypto.nop(
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        "3201502841558479033173442359765767139800663714742290109531927391993829786728"
    );
    if (
        result ==
        "3201502841558479033173442359765767139800663714742290109531927391993829786728"
    ) {
        console.log("Nop test succeed.");
    } else {
        console.error("Nop test failed!");
    }
}

// Test hash
// python3 ./signature_cli.py hash --msg \
//   1740729136829561885683894917751815192814966525555656371386868611731128807883 \
//   919869093895560023824014392670608914007817594969197822578496829435657368346
{
    const result = starkcrypto.pedersen_hash(
        "1740729136829561885683894917751815192814966525555656371386868611731128807883",
        "919869093895560023824014392670608914007817594969197822578496829435657368346"
    );
    const expected = {
        msg_hash:
            "135779816710909020805564054333139590933910151167913961258433275464447387460"
    };
    if (deepEqual(result, expected)) {
        console.log("Pedersen hash test succeed.");
    } else {
        console.error("Pedersen hash test failed!");
    }
}

// Test public key
// python3 ./signature_cli.py priv_to_pub --priv_key \
//   1740729136829561885683894917751815192814966525555656371386868611731128807883
{
    const result = starkcrypto.public_key(
        "1740729136829561885683894917751815192814966525555656371386868611731128807883"
    );
    const expected = {
        x:
            "1047933115726230933936444839278495959863389921309454379825626157597302993678",
        y:
            "2992976248333999363926335380274708371827571188820497486743080013169132020304"
    };
    if (deepEqual(result, expected)) {
        console.log("Public key test succeed.");
    } else {
        console.error("Public key test failed!");
    }
}

// Test sign
// (No python equivalent)
{
    const result = starkcrypto.sign(
        "857382457629205625681214596003761548740910932106684834463954993440159970768",
        "1699550429262868952957733065396688802326540225623380427551300052767936406476"
    );
    const expected = {
        r:
            "29890947993046261611645984470574583259836861919677343317860415270909103971",
        w:
            "3389523773481968765502141586752314378581761033301891571958931787092130508169"
    };
    if (deepEqual(result, expected)) {
        console.log("Sign test succeed.");
    } else {
        console.error("Sign test failed!");
    }
}

// Test verify
// python3 ./signature_cli.py verify \
// --msg_hash 919869093895560023824014392670608914007817594969197822578496829435657368346\
// --r 874739451078007766457464989774322083649278607533249481151382481072868806602\
// --w 917056236706666338779046290895782911901388693926022961665551824457713815676\
// --pub_key_x 1047933115726230933936444839278495959863389921309454379825626157597302993678 \
// --pub_key_y 2992976248333999363926335380274708371827571188820497486743080013169132020304
{
    const result = starkcrypto.verify(
        "919869093895560023824014392670608914007817594969197822578496829435657368346",
        {
            r:
                "874739451078007766457464989774322083649278607533249481151382481072868806602",
            w:
                "917056236706666338779046290895782911901388693926022961665551824457713815676"
        },
        {
            x:
                "1047933115726230933936444839278495959863389921309454379825626157597302993678",
            y:
                "2992976248333999363926335380274708371827571188820497486743080013169132020304"
        }
    );
    const expected = { is_valid: true };
    if (deepEqual(result, expected)) {
        console.log("Verify test succeed.");
    } else {
        console.error("Verify test failed!");
    }
}

// Test maker sign
// python3 ./signature_cli.py sign_maker --vault_a 21 --vault_b 27
// --amount_a 6873058723796400 --amount_b 852209057714036
// --token_a 168976971209324910088270776698114429107164817795147365551345596466024812576\
// --token_b 210761264384301076547763280170970365712477797880932555831340857495337358698\
// --trade_id 0 --priv_key 1699550429262868952957733065396688802326540225623380427551300052767936406476
//
// NOTE: this requires replacing signature.py line 79 with
//     k = 0x689bc54bedc93f9acbe9315b4538489c8654a9160d46319f5157b226b8f41ad
{
    const result = starkcrypto.maker_sign(
        {
            vault_a: 21,
            vault_b: 27,
            amount_a: "6873058723796400",
            amount_b: "852209057714036",
            token_a:
                "168976971209324910088270776698114429107164817795147365551345596466024812576",
            token_b:
                "210761264384301076547763280170970365712477797880932555831340857495337358698",
            trade_id: 0
        },
        "1699550429262868952957733065396688802326540225623380427551300052767936406476"
    );
    const expected = {
        maker_msg:
            "710471947606888870830612264939157805860689127401500845152678489563118455148",
        r:
            "2082956756539342940653267974841073721753715489508559753023107475569635554744",
        w:
            "1720329235658045098345293225536236153800035436641504589044650073399453764796"
    };
    if (deepEqual(result, expected)) {
        console.log("Maker sign test succeed.");
    } else {
        console.error("Maker sign test failed!");
    }
}

// Test taker sign
// python3 ./signature_cli.py sign_maker --vault_a 21 --vault_b 27
// --amount_a 6873058723796400 --amount_b 852209057714036
// --token_a 168976971209324910088270776698114429107164817795147365551345596466024812576\
// --token_b 210761264384301076547763280170970365712477797880932555831340857495337358698\
// --trade_id 0 --taker_vault_a 2 --taker_vault_b 31 \
// --priv_key 1043001682421203376945321761381453872933181159161894427363884596240717713089
//
// NOTE: this requires replacing signature.py line 79 with
//     k = 0x1ed2c3d93af13b064784b734f05bfb9838a19eea53ee0b502bf2b20b1661a05
{
    const result = starkcrypto.taker_sign(
        {
            vault_a: 21,
            vault_b: 27,
            amount_a: "6873058723796400",
            amount_b: "852209057714036",
            token_a:
                "168976971209324910088270776698114429107164817795147365551345596466024812576",
            token_b:
                "210761264384301076547763280170970365712477797880932555831340857495337358698",
            trade_id: 0
        },
        2,
        31,
        "1043001682421203376945321761381453872933181159161894427363884596240717713089"
    );
    const expected = {
        maker_msg:
            "710471947606888870830612264939157805860689127401500845152678489563118455148",
        taker_msg:
            "3352318676647792916544761969851899543842156867533342014273537377500814333988",
        r:
            "1946337478642587999776598742031228742273738226158101006612665042806856730080",
        w:
            "2833477950891576348646763500424705069561372684358745311108148957048765586550"
    };
    if (deepEqual(result, expected)) {
        console.log("Taker sign test succeed.");
    } else {
        console.error("Taker sign test failed!");
    }
}

// Test order lifecycle test
{
    var maker_private =
        "1699550429262868952957733065396688802326540225623380427551300052767936406476";
    var taker_private =
        "2082956756539342940653267974841073721753715489508559753023107475569635554744";
    var maker_public = starkcrypto.public_key(maker_private);
    var taker_public = starkcrypto.public_key(taker_private);
    let order = {
        vault_a: 21,
        vault_b: 27,
        amount_a: "6873058723796400",
        amount_b: "852209057714036",
        token_a:
            "168976971209324910088270776698114429107164817795147365551345596466024812576",
        token_b:
            "210761264384301076547763280170970365712477797880932555831340857495337358698",
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
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        "3201502841558479033173442359765767139800663714742290109531927391993829786728"
    );
}
console.timeEnd("Benchmark 100k nop");

// Benchmark hash
console.time("Benchmark 10k hash");
for (var i = 0; i < 10000; i++) {
    var result = starkcrypto.pedersen_hash(
        "1740729136829561885683894917751815192814966525555656371386868611731128807883",
        "919869093895560023824014392670608914007817594969197822578496829435657368346"
    );
}
console.timeEnd("Benchmark 10k hash");

// Benchmark public key
console.time("Benchmark 10k public key");
for (var i = 0; i < 10000; i++) {
    var result = starkcrypto.public_key(
        "1740729136829561885683894917751815192814966525555656371386868611731128807883"
    );
}
console.timeEnd("Benchmark 10k public key");

// Benchmark sign
console.time("Benchmark 10k sign");
for (var i = 0; i < 10000; i++) {
    var result = starkcrypto.sign(
        "857382457629205625681214596003761548740910932106684834463954993440159970768",
        "1699550429262868952957733065396688802326540225623380427551300052767936406476"
    );
}
console.timeEnd("Benchmark 10k sign");

// Benchmark verify
console.time("Benchmark 10k verify");
for (var i = 0; i < 1000; i++) {
    var result = starkcrypto.verify(
        "919869093895560023824014392670608914007817594969197822578496829435657368346",
        {
            r:
                "874739451078007766457464989774322083649278607533249481151382481072868806602",
            w:
                "917056236706666338779046290895782911901388693926022961665551824457713815676"
        },
        {
            x:
                "1047933115726230933936444839278495959863389921309454379825626157597302993678",
            y:
                "2992976248333999363926335380274708371827571188820497486743080013169132020304"
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
        amount_a: "6873058723796400",
        amount_b: "852209057714036",
        token_a:
            "168976971209324910088270776698114429107164817795147365551345596466024812576",
        token_b:
            "210761264384301076547763280170970365712477797880932555831340857495337358698",
        trade_id: 0
    });
}
console.timeEnd("Benchmark 1k maker_hash");

// Benchmark verify
console.time("Benchmark 1k maker taker sign");
for (var i = 0; i < 1000; i++) {
    var maker_private =
        "168976971209324910088270776698114429107164817795147365551345596466024812576";
    var taker_private =
        "210761264384301076547763280170970365712477797880932555831340857495337358698";
    let order = {
        vault_a: 21,
        vault_b: 27,
        amount_a: "6873058723796400",
        amount_b: "852209057714036",
        token_a:
            "168976971209324910088270776698114429107164817795147365551345596466024812576",
        token_b:
            "210761264384301076547763280170970365712477797880932555831340857495337358698",
        trade_id: 0
    };
    var maker_sig = starkcrypto.maker_sign(order, maker_private);
    var taker_sig = starkcrypto.taker_sign(order, 2, 31, taker_private);
}
console.timeEnd("Benchmark 1k maker taker sign");
