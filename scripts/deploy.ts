import { execSync } from "child_process";
import {
  getDeploymentCommand,
  getStoreCommand,
  optimizeCommand,
  sendContractToContainer,
} from "./commands";

const log = console.log;

const contract_path = "artifacts/";

const contracts_wasm = {
    "paga.wasm": {
        name: "paga",
        instanceProps: (data: any) => {
          return JSON.stringify({});
        },
        address: "",
      },
  "electors.wasm": {
    name: "electors",
    instanceProps: (data: any) => {
      return JSON.stringify({
        owner: "neutron1x8y240crs906dcs6l8hzqnwapy0ns05n7utcyq",
        paga_contract: data["paga.wasm"].address,
      });
    },
    address: "",
  },
  "politicians.wasm": {
    name: "politicians",
    instanceProps: (data: any) => {
      return JSON.stringify({
        owner: "neutron1x8y240crs906dcs6l8hzqnwapy0ns05n7utcyq",
        paga_contract: data["paga.wasm"].address,
      });
    },
    address: "",
  },

};

log("compiling contracts...");
execSync("cargo build --release --target wasm32-unknown-unknown", {
  stdio: "ignore",
});
log("Compilation completed successfully!");

log("optimizing contracts...");
execSync(optimizeCommand, { stdio: "ignore" });
log("Optimization completed successfully!");

const storeContract = async (contractName: string) => {
  const storeCommand = getStoreCommand(contractName);
  const storeRes = execSync(storeCommand, { encoding: "utf8" });
  log(storeRes);

  const tx = storeRes.split("txhash: ")[1].split("\n")[0];
  log(`${tx}`);
  await new Promise((r) => setTimeout(r, 2000));
  let codeId: string =
    execSync(
      `docker exec neutron neutrond query wasm list-code --home /opt/neutron/data/test-1`,
      { encoding: "utf8" }
    )
      .split("code_id: ")
      .at(-1)
      ?.split("\n")[0]
      .trim()
      .replace(/"/g, "") ?? "";

  codeId = parseInt(codeId).toString();
  log(`codeId: ${codeId}`);
  return codeId;
};

log("deploying contracts...");
(async () => {
  for (const contract in contracts_wasm) {
    const contractName =
      contracts_wasm[contract as keyof typeof contracts_wasm].name;

    sendContractToContainer(contract_path, contract);
    const codeId = await storeContract(contractName);

    console.log("codeId: ", codeId);

    const instanceProps =
      contracts_wasm[contract as keyof typeof contracts_wasm].instanceProps(
        contracts_wasm
      );
    const deployRes = execSync(
      getDeploymentCommand(contractName, codeId, instanceProps),
      { encoding: "utf8" }
    );
    const deployTx = deployRes.split("txhash: ")[1].split("\n")[0];
    log(`deploy tx : ${deployTx}`);
    await new Promise((r) => setTimeout(r, 2000));
    const contractAddress = execSync(
      `docker exec neutron neutrond query wasm list-contract-by-code ${codeId} --home /opt/neutron/data/test-1`,
      { encoding: "utf8" }
    )
      .split("- ")[1]
      .split("\n")[0]
      .trim();

    contracts_wasm[contract as keyof typeof contracts_wasm].address =
      contractAddress;
  }

  log("Contracts deployed successfully!");
  log("Contracts addresses:");
  for (const contract in contracts_wasm) {
    const contractName =
      contracts_wasm[contract as keyof typeof contracts_wasm].name;
    const contractAddress =
      contracts_wasm[contract as keyof typeof contracts_wasm].address;
    log(`${contractName}: ${contractAddress}`);
  }
})();
