

import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  CLValueBuilder,
  CLPublicKey,
  DeployUtil,
  Signer,
  Keys


} from "casper-js-sdk";

import { readKeys, readWasm } from "./Utils";

enum Network {
  // MAINNET = "mainnet",
  TESTNET = "testnet",
  PRIVATE = "private"
}

const currentNetwork = Network.PRIVATE

function setupEnv(network: Network): [string, Keys.AsymmetricKey, string] {
  switch (network.valueOf()) {
    case Network.PRIVATE:
      return [
        "casper-net-1",
        readKeys("../nctl-docker/users/user-10"),
        "http://localhost:11101/rpc"
      ]

    case Network.TESTNET:
      return [
        "casper-test",
        readKeys("../../../test-1-ed25519-keys"),
        "http://94.130.10.55:7777/rpc"
      ]

    default:
      throw new Error("Unknown network: " + currentNetwork)
  }
}

const [network, keys, nodeRpc] = setupEnv(currentNetwork)

const client = new CasperClient(nodeRpc);
const contract = new Contracts.Contract(client);
const wasmPath = "/home/mike/casper-project/test-dapp/project-1/client/wasm/contract.wasm"

async function runScenario() {

  console.log({ accountHex: keys.accountHex() })

  let r = await client.nodeClient.getStatus()
  console.log({ nodeStatus: r })

  const wasm = readWasm(wasmPath)

  let deploy = contract.install(
    wasm,
    RuntimeArgs.fromMap({}),
    "21334128500",
    keys.publicKey,
    network,
    [keys]
  )

  const res = await client.putDeploy(deploy)
  console.log({ installHex: res })
  return res
}

runScenario().then(res => {
  console.log("--- Result ---")
  console.log(res)
}
).catch(e => console.log("Error calling scenario: " + e))