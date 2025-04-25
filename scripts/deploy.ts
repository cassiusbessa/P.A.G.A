import { execSync } from "child_process";
import {
  getContractByCodeIdCommand,
  getDeploymentCommand,
  getStoreCommand,
  sendContractToContainer,
} from "./commands/commands";
import contractSchema from "./contract_schema";
import { log } from "console";
import { contract_path, sleep } from "./utils";
import { compileProject } from "./commands/compile_command";
import { TContractName } from "./types/type";
import { listContracts, listContractsByCodeId } from "./commands/contract";

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

const instanciateContract = (contractName: string, codeId: string) => {
  const instanceProps =
    contractSchema[contractName as keyof typeof contractSchema].instanceProps(
      contractSchema
    );
  const instantiateRes = execSync(
    getDeploymentCommand(contractName, codeId, instanceProps),
    { encoding: "utf8" }
  );
  log(instantiateRes);

  const tx = instantiateRes.split("txhash: ")[1].split("\n")[0];
  log(`${tx}`);
};

log("deploying contracts...");
(async () => {
  compileProject();

  for (const contract in contractSchema) {
    const current_obj = contractSchema[contract as TContractName];
    sendContractToContainer(contract_path, contract);
    const codeId = await storeContract(current_obj.name);
    instanciateContract(contract, codeId);
    await sleep(3000);
    const contractAddress = await listContractsByCodeId(codeId);
    contractSchema[contract as TContractName].address = contractAddress;
  }
  log("Contracts deployed successfully!");
  listContracts(contractSchema);
})();
