import { usePlugin } from "@nomiclabs/buidler/config";
import { BuidlerConfig } from "@nomiclabs/buidler/config";

usePlugin("@nomiclabs/buidler-waffle");
usePlugin("buidler-typechain");

const config: BuidlerConfig = {
  solc: {
    version: "0.6.4"
  },
  typechain: {
    outDir: "typechain",
    target: "ethers"
  }
};

export default config;
