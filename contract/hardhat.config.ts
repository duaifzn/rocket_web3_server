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
      accounts: ['0x7d1fe7133ae962a50f860468ea1351f83e417dbdaab635294efdd8e6e3eef031']
    }
  },
};
//0x096A37b3Ca5d96d3b251825291a10a92164cAcAf
//0xE0623b806252314b8638C64e5B0E010674071272