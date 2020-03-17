const Coin = artifacts.require("PublicCoinTesting");

// Vanilla Mocha test. Increased compatibility with tools that integrate Mocha.
describe("Greeter contract", function() {
  let accounts;
  let coin_contract;
  let init_hex = "0x0123456789abcded"

  before(async function() {
    accounts = await web3.eth.getAccounts();
    coin_contract = await Coin.new();
  });

  describe("Reading and Writing", function() {
    it("Should read the correct data from multiple reads", async function() {        
        let read_data = await coin_contract.init_and_read.call(init_hex, 3);

        assert.equal(read_data[0], "0x7d84f75ca3e9328b92123c1790834ee0084e02c09b379c6f95c5d2ae8739b9c8");
        assert.equal(read_data[1], "0x4ed5f0fd8cffa8dec69beebab09ee881e7369d6d084b90208a079eedc67d2d45");
        assert.equal(read_data[2], "0x2389a47fe0e1e5f9c05d8dcb27b069b67b1c7ec61a5c0a3f54d81aea83d2c8f0");
    });

    it("Should have the correct digest after a write", async function() {
        let digest = await coin_contract.init_and_write.call(init_hex, "0x7d84f75ca3e9328b92123c1790834ee0084e02c09b379c6f95c5d2ae8739b9c8");
        assert.equal(digest, "0x3174a00d031bc8deff799e24a78ee347b303295a6cb61986a49873d9b6f13a0d");
    });
  });
});


