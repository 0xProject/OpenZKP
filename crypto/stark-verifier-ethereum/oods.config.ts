import {BuidlerConfig, usePlugin} from '@nomiclabs/buidler/config';

usePlugin('@nomiclabs/buidler-waffle');
usePlugin('buidler-typechain');

const config: BuidlerConfig = {
    solc: {
        version: '0.5.11',
        optimizer: {
            enabled: false,
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
    paths: {sources: './oods_contracts'},
};

// tslint:disable-next-line:no-default-export
export default config;
