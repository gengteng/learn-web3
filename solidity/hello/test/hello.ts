import "@nomicfoundation/hardhat-ethers";
import {ethers} from "hardhat";
import {expect} from "chai";

describe("hello", function () {
    it("should return 'Hello, world!'", async function () {
        const Hello = await ethers.getContractFactory("Hello");
        const hello = await Hello.deploy();
        await hello.waitForDeployment();

        expect(await hello.hello()).to.equal("Hello, World!");
    });
});