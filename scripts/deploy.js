const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();

  console.log("Deploying PolyEndpoint with the account:", deployer.address);

  const PolyEndpoint = await hre.ethers.getContractFactory("PolyEndpoint");
  const polyEndpoint = await PolyEndpoint.deploy();

  await polyEndpoint.waitForDeployment();

  console.log("PolyEndpoint deployed to:", await polyEndpoint.getAddress());
  console.log("Owner:", await polyEndpoint.owner());
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });