import { utils } from 'ethers';

// Note - This proof corresponds to the small fib example in rust.
// It is not full of accurate data yet.

export const testing_proof: any = {
    public_inputs: utils.arrayify(utils.defaultAbiCoder.encode(['bytes32[]'], [[
        '0x03E8000000000000000000000000000000000000000000000000000000000000',
        '0x0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f']
    ])),
    trace_root: '0x018dc61f748b1a6c440827876f30f63cb6c4c188000000000000000000000000',
    constraint_root: '0xe276ce1357d4030a4c84cdfdb4dd77845d3f80e9000000000000000000000000',
    trace_oods_values: [
        '0x00c2266f7bd8ac00173e9ed9e1a895b5edde25463310e7c7d08b5b74e58f0b9c',
        '0x048c47b867722c68ccfefc4cbddaaa37a2d0a86034e084f0be438cb8a8e3d950',
        '0x0426b72fcbd3771b5e279e929ad1ce0a19684efbdb39b0dcf332d172731f3766',
        '0x00027d92a46c5ff35d8159d142c3ac886486dbadd86a8933ea5f1ff0fd525f67',
    ],
    constraint_oods_values: ['0x01e94b626dcff9d77c33c75b33d8457ba91534da30442d41d717a06e3f65211d'],
    pow_nonce: '0x0000000000000860',
    trace_values: ['0x017542df7e3f39cbc54c7cb16cbc0841e347194abd21415fb6dec07f597dd598'],
    constraint_values: ['0x017542df7e3f39cbc54c7cb16cbc0841e347194abd21415fb6dec07f597dd598'],
    trace_decommitment: ['0x017542df7e3f39cbc54c7cb16cbc0841e347194abd21415fb6dec07f597dd598'],
    constraint_decommitment: ['0x017542df7e3f39cbc54c7cb16cbc0841e347194abd21415fb6dec07f597dd598'],
    fri_values: [['0x017542df7e3f39cbc54c7cb16cbc0841e347194abd21415fb6dec07f597dd598']],
    fri_roots: [
        '0x620a934880b6c7d893acf17a21cc9c10058a7add000000000000000000000000',
        '0x07d0e56d711bb2f95d1c6670d0ae0c4d86b8ca72000000000000000000000000',
        '0x5832e517d6d978e7b580ecd9694a407dde8f684b000000000000000000000000',
    ],
    fri_decommitments: [['0x017542df7e3f39cbc54c7cb16cbc0841e347194abd21415fb6dec07f597dd598']],
    last_layer_coeffiencts: [
        '0x02d7c636c2c38ae6161cfa8d5541ce6cd1cb4e8845f4484b6412ae3de821406e',
        '0x00fd1d59f94bb3642f1f6e90a88250b784c14ef514c5c344b05ae4df03c5d146',
        '0x05f8a83f99ce1a3ca896ac2ed65c96bb6f919f51c15dd2c8e37500d7d72f7a39',
        '0x017ff18ed0486993c548a4d1a1e744e116a66f487180b2ac714f9ee35ecfb53a',
    ],
};
