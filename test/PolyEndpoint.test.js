const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("PolyEndpoint", function () {
  let polyEndpoint;
  let owner;
  let admin1;
  let admin2;
  let user;

  beforeEach(async function () {
    [owner, admin1, admin2, user] = await ethers.getSigners();
    
    const PolyEndpoint = await ethers.getContractFactory("PolyEndpoint");
    polyEndpoint = await PolyEndpoint.deploy();
    await polyEndpoint.waitForDeployment();
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await polyEndpoint.owner()).to.equal(owner.address);
    });

    it("Should have zero endpoints initially", async function () {
      expect(await polyEndpoint.getEndpointCount()).to.equal(0);
    });
  });

  describe("Admin Management", function () {
    it("Owner should be able to add admin", async function () {
      await expect(polyEndpoint.addAdmin(admin1.address))
        .to.emit(polyEndpoint, "AdminAdded")
        .withArgs(admin1.address);
      
      expect(await polyEndpoint.admins(admin1.address)).to.equal(true);
    });

    it("Owner should be able to remove admin", async function () {
      await polyEndpoint.addAdmin(admin1.address);
      expect(await polyEndpoint.admins(admin1.address)).to.equal(true);

      await expect(polyEndpoint.removeAdmin(admin1.address))
        .to.emit(polyEndpoint, "AdminRemoved")
        .withArgs(admin1.address);
      
      expect(await polyEndpoint.admins(admin1.address)).to.equal(false);
    });

    it("Non-owner should not be able to add admin", async function () {
      await expect(
        polyEndpoint.connect(user).addAdmin(admin1.address)
      ).to.be.revertedWith("PolyEndpoint: caller is not the owner");
    });

    it("Cannot add zero address as admin", async function () {
      await expect(
        polyEndpoint.addAdmin(ethers.ZeroAddress)
      ).to.be.revertedWith("PolyEndpoint: admin address cannot be zero");
    });

    it("Cannot add duplicate admin", async function () {
      await polyEndpoint.addAdmin(admin1.address);
      await expect(
        polyEndpoint.addAdmin(admin1.address)
      ).to.be.revertedWith("PolyEndpoint: address is already an admin");
    });
  });

  describe("Endpoint Management", function () {
    beforeEach(async function () {
      await polyEndpoint.addAdmin(admin1.address);
    });

    it("Owner should be able to add endpoint", async function () {
      await expect(polyEndpoint.addEndpoint("https://api.example.com"))
        .to.emit(polyEndpoint, "EndpointAdded")
        .withArgs("https://api.example.com");
      
      expect(await polyEndpoint.getEndpointCount()).to.equal(1);
      expect(await polyEndpoint.hasEndpoint("https://api.example.com")).to.equal(true);
    });

    it("Admin should be able to add endpoint", async function () {
      await expect(polyEndpoint.connect(admin1).addEndpoint("https://api.admin.com"))
        .to.emit(polyEndpoint, "EndpointAdded");
      
      expect(await polyEndpoint.getEndpointCount()).to.equal(1);
    });

    it("Should be able to add multiple endpoints", async function () {
      await polyEndpoint.addEndpoint("https://api1.example.com");
      await polyEndpoint.addEndpoint("https://api2.example.com");
      await polyEndpoint.addEndpoint("https://api3.example.com");

      expect(await polyEndpoint.getEndpointCount()).to.equal(3);
    });

    it("Should be able to remove endpoint", async function () {
      await polyEndpoint.addEndpoint("https://api.example.com");
      await polyEndpoint.addEndpoint("https://api2.example.com");

      await expect(polyEndpoint.removeEndpoint("https://api.example.com"))
        .to.emit(polyEndpoint, "EndpointRemoved")
        .withArgs("https://api.example.com");
      
      expect(await polyEndpoint.getEndpointCount()).to.equal(1);
      expect(await polyEndpoint.hasEndpoint("https://api.example.com")).to.equal(false);
      expect(await polyEndpoint.hasEndpoint("https://api2.example.com")).to.equal(true);
    });

    it("Cannot add empty endpoint", async function () {
      await expect(
        polyEndpoint.addEndpoint("")
      ).to.be.revertedWith("PolyEndpoint: endpoint cannot be empty");
    });

    it("Cannot add duplicate endpoint", async function () {
      await polyEndpoint.addEndpoint("https://api.example.com");
      await expect(
        polyEndpoint.addEndpoint("https://api.example.com")
      ).to.be.revertedWith("PolyEndpoint: endpoint already exists");
    });

    it("Cannot remove non-existent endpoint", async function () {
      await expect(
        polyEndpoint.removeEndpoint("https://api.example.com")
      ).to.be.revertedWith("PolyEndpoint: endpoint does not exist");
    });

    it("Non-admin should not be able to add endpoint", async function () {
      await expect(
        polyEndpoint.connect(user).addEndpoint("https://api.example.com")
      ).to.be.revertedWith("PolyEndpoint: caller is not an admin or owner");
    });

    it("Non-admin should not be able to remove endpoint", async function () {
      await polyEndpoint.addEndpoint("https://api.example.com");
      await expect(
        polyEndpoint.connect(user).removeEndpoint("https://api.example.com")
      ).to.be.revertedWith("PolyEndpoint: caller is not an admin or owner");
    });
  });

  describe("View Functions", function () {
    beforeEach(async function () {
      await polyEndpoint.addAdmin(admin1.address);
      await polyEndpoint.addEndpoint("https://api1.example.com");
      await polyEndpoint.addEndpoint("https://api2.example.com");
      await polyEndpoint.addEndpoint("https://api3.example.com");
    });

    it("Should get all endpoints", async function () {
      const endpoints = await polyEndpoint.getAllEndpoints();
      expect(endpoints.length).to.equal(3);
      expect(endpoints[0]).to.equal("https://api1.example.com");
      expect(endpoints[1]).to.equal("https://api2.example.com");
      expect(endpoints[2]).to.equal("https://api3.example.com");
    });

    it("Should get endpoint by index", async function () {
      const endpoint = await polyEndpoint.getEndpoint(1);
      expect(endpoint).to.equal("https://api2.example.com");
    });

    it("Should revert when index out of bounds", async function () {
      await expect(
        polyEndpoint.getEndpoint(10)
      ).to.be.revertedWith("PolyEndpoint: index out of bounds");
    });

    it("Should check endpoint existence", async function () {
      expect(await polyEndpoint.hasEndpoint("https://api1.example.com")).to.equal(true);
      expect(await polyEndpoint.hasEndpoint("https://api4.example.com")).to.equal(false);
    });
  });

  describe("Ownership Management", function () {
    it("Should transfer ownership", async function () {
      await expect(polyEndpoint.transferOwnership(admin1.address))
        .to.emit(polyEndpoint, "OwnershipTransferred")
        .withArgs(owner.address, admin1.address);
      
      expect(await polyEndpoint.owner()).to.equal(admin1.address);
    });

    it("Only owner should be able to transfer ownership", async function () {
      await expect(
        polyEndpoint.connect(user).transferOwnership(admin1.address)
      ).to.be.revertedWith("PolyEndpoint: caller is not the owner");
    });

    it("Cannot transfer to zero address", async function () {
      await expect(
        polyEndpoint.transferOwnership(ethers.ZeroAddress)
      ).to.be.revertedWith("PolyEndpoint: new owner cannot be zero address");
    });
  });
});