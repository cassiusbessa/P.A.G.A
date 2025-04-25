import { execSync } from "child_process";

export const optimizeCommand = `docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0`;

export const getStoreCommand = (contractName: string) => {
  return `docker exec neutron neutrond tx wasm store /contracts/${contractName}.wasm \
          --fees 200000untrn --from mywallet \
          --gas auto  --gas-adjustment 1.3 \
          --keyring-backend test \
          --home /opt/neutron/data/test-1 \
          --broadcast-mode sync \
          --chain-id test-1  \
          --yes`;
};

export const getDeploymentCommand = (
  contractName: string,
  code_id: string,
  instanceProps: any
) => {
    return `
          docker exec neutron neutrond tx wasm instantiate ${code_id} \
          '${instanceProps}' \
            --label "${contractName}" \
            --from mywallet \
            --fees 10000untrn \
            --gas auto --gas-adjustment 1.3 \
            --keyring-backend test \
            --home /opt/neutron/data/test-1 \
            --broadcast-mode sync \
            --chain-id test-1 \
            --yes \
            --no-admin \
`;
};

export const sendContractToContainer = (
  contract_path: string,
  contractName: string
) => {
  console.log(`sending ${contractName} to the container`);
  const contractPath = `${contract_path}${contractName}`;
  execSync(
    `
          docker exec neutron rm -rf /contracts && \
          docker exec neutron mkdir -p /contracts && \
          docker cp ${contractPath} neutron:/contracts/${contractName}`,
    { stdio: "ignore" }
  );
  console.log(`contract ${contractName} sent successfully!`);
};
