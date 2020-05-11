import {BuidlerConfig, usePlugin} from '@nomiclabs/buidler/config';

usePlugin('@nomiclabs/buidler-waffle');
usePlugin('buidler-typechain');

const config: BuidlerConfig = {
    solc: {
        version: '0.6.6',
        optimizer: {
            enabled: true,
            runs: 100000000,
        },
    },
    typechain: {
        outDir: 'typechain',
        target: 'ethers',
    },
    networks: {
        buidlerevm: {
            blockGasLimit: 100000000,
        },
    },
};

// tslint:disable-next-line:no-default-export
export default config;
