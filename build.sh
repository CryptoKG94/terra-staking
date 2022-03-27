#!/bin/bash

#Build Flag
PARAM=$1
####################################    Constants    ##################################################

#depends on mainnet or testnet
# NODE="--node https://lcd.terra.dev:443"
# CHAIN_ID=juno-1
# DENOM="ujuno"
# #REWARD TOKEN is BLOCK
# REWARD_TOKEN_ADDRESS="juno1w5e6gqd9s4z70h6jraulhnuezry0xl78yltp5gtp54h84nlgq30qta23ne"
# #STAKE TOKEN is LP TOKEN for BLOCK-JUNO pool
# STAKE_TOKEN_ADDRESS="juno1cmmpty2dgs9h36vtrwxk53pmkwe3fgn5833wpay4ap0unm6svgks7aajke"

##########################################################################################

NODE="--node http://167.99.25.150:26657"
#NODE="--node https://rpc.uni.junomint.com:443"
CHAIN_ID=Bombay-12
DENOM="uluna"
REWARD_TOKEN_ADDRESS="juno1yqmcu5uw27mzkacputegtg46cx55ylwgcnatjy3mejxqdjsx3kmq5a280s"
STAKE_TOKEN_ADDRESS="juno18hh4dflvfdcuklc9q4ghlr83fy5k4sdx6rgfzzwhdfqznsj4xjzqdsn5cc"

##########################################################################################
#not depends
NODECHAIN=" $NODE --chain-id $CHAIN_ID"
TXFLAG=" $NODECHAIN --gas-prices 0.025$DENOM --gas auto --gas-adjustment 1.3"
WALLET="--from kg"

WASMFILE="anchor_staking.wasm"

FILE_UPLOADHASH="uploadtx.txt"
FILE_CONTRACT_ADDR="contractaddr.txt"
FILE_CODE_ID="code.txt"

ADDR_KG=""

CreateEnv() {

    # install rust
    # curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    # source $HOME/.cargo/env

    rustup default stable
    rustup target list --installed
    rustup target add wasm32-unknown-unknown

    export PATH="~/.cargo/bin:$PATH"
}

InstallTerrad() {
    wget https://github.com/terra-money/core/releases/download/v0.5.17/terra_0.5.17_Linux_x86_64.tar.gz
    tar xzvf terra_0.5.17_Linux_x86_64.tar.gz
    cp terrad /usr/bin/
}

RustBuild() {

    echo "================================================="
    echo "Rust Optimize Build Start"
    cd contracts/staking

    RUSTFLAGS='-C link-arg=-s' cargo wasm

    cd ../../
    # mkdir artifacts
    cp target/wasm32-unknown-unknown/release/$WASMFILE artifacts/$WASMFILE
}

#Writing to FILE_UPLOADHASH
Upload() {
    #secretcli tx compute store artifacts/$WASMFILE $WALLET $TXFLAG
    echo "================================================="
    cd artifacts
    echo "Upload $WASMFILE"
    UPLOADTX=$(terrad tx wasm store $WASMFILE $WALLET $TXFLAG --output json -y | jq -r '.txhash')
    echo "Upload txHash:"$UPLOADTX
    
    #save to FILE_UPLOADHASH
    cd ..
    echo $UPLOADTX > $FILE_UPLOADHASH
    echo "wrote last transaction hash to $FILE_UPLOADHASH"
}

#Read code from FILE_UPLOADHASH
GetCode() {
    echo "================================================="
    echo "Get code from transaction hash written on $FILE_UPLOADHASH"
    
    #read from FILE_UPLOADHASH
    TXHASH=$(cat $FILE_UPLOADHASH)
    echo "read last transaction hash from $FILE_UPLOADHASH"
    echo $TXHASH
    
    QUERYTX="terrad query tx $TXHASH $NODECHAIN --output json"
	CODE_ID=$(terrad query tx $TXHASH $NODECHAIN --output json | jq -r '.logs[0].events[-1].attributes[0].value')
	echo "Contract Code_id:"$CODE_ID

    #save to FILE_CODE_ID
    echo $CODE_ID > $FILE_CODE_ID
}

if [[ $PARAM == "" ]]; then
    RustBuild
    Upload
sleep 7
    GetCode
sleep 7
    Instantiate
sleep 7
    GetContractAddress
sleep 5
   BuyTicket
sleep 7
    NewRound
sleep 7
    PrintState
sleep 1
    PrintBalance
else
    $PARAM
fi