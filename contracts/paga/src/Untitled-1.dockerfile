neutrond tx wasm store /paga.wasm \
  --from mywallet \
  --keyring-backend test \
  --home ./data/test-1 \
  --gas auto --gas-adjustment 1.3 \
  --chain-id test-1 \
  --broadcast-mode async \
  --yes


neutrond tx wasm instantiate <code_id> \
  '{"owner":"neutron1x8y240crs906dcs6l8hzqnwapy0ns05n7utcyq"}' \
  --from mywallet \
  --label "paga-contract" \
  --keyring-backend test \
  --home ./data/test-1 \
  --gas auto --gas-adjustment 1.3 \
  --chain-id test-1 \
  --broadcast-mode block \
  --yes
