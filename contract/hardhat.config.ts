import { task } from "hardhat/config";
import "@nomiclabs/hardhat-waffle";

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
export default {
  solidity: "0.7.3",
  networks: {
    private: {
      url: "http://52.179.136.216:8545",
      chainId: 1337,
      accounts: ['0x911f12fb0246378dc32cc675b0535b3056d8c166b156d08bc5f29c968e4585b6']
    }
  },
};
//0xeA4c45d4e0f2517016774f7ce45B03606B753aE8