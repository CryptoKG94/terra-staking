docker run --rm -v "/kg/terra-staking/terra-staking/contracts/staking/src/contract.rs":/code \
  --mount type=volume,source="$(basename "/kg/terra-staking/terra-staking/contracts/staking/src/contract.rs")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.11.4