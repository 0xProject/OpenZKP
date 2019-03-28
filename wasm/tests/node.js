var starkcrypto = require('../pkg/starkcrypto_wasm');

console.log('Testing Hash:');

for(var i = 0; i < 10; i++) {
    var result = starkcrypto.pedersen_hash(
        '3d937c035c878245caf64531a5756109c53068da139362728feb561405371cb',
        '208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a');
    if (result != '2d895bd76790645fb867eaf57037e4aa8af1bbb139a84d01e311a7c53f3111b') {
        console.error('Test failed!');
    }
}
