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
            gas: 100000000000,
            gasPrice: 1,
            blockGasLimit: 100000000000,
            allowUnlimitedContractSize: true,
        },
    },
};

// tslint:disable-next-line:no-default-export
export default config;
