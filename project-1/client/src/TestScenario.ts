

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
import { ContractSDK } from "./ContractSDK";
import { deployFromJson } from "casper-js-sdk/dist/lib/DeployUtil";

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

const contactSdk = new ContractSDK(nodeRpc, network)

const wasmPath = "/home/mike/casper-project/test-dapp/project-1/client/wasm/contract.wasm"

// can be found from regression cost test in contract repo
const contractInstallCost = "21334128500"

async function runScenario() {

  console.log({ accountHex: keys.accountHex() })

  let r = await contactSdk.contractClient.casperClient?.nodeClient.getStatus()
  console.log({ nodeStatus: r })

  const wasm = readWasm(wasmPath)
  const [installDeploy, deployHash] = await contactSdk.installOnChain(
    wasm,
    contractInstallCost, 
    keys.publicKey,
    [keys]
  )

  console.log({deployHash: deployHash })

  console.log("Awaiting deploy ready...")
  const installDeployResult = await contactSdk.awaitDeployed(installDeploy)
  
  if (!ContractSDK.isDeploySuccesfull(installDeployResult)) {
    console.log({installDeployResult: installDeployResult.execution_results[0].result})
    throw new Error("Install deploy failed")
  }
  console.log("Contract installed")
}

runScenario().then(res => {
  console.log("--- Result ---")
  console.log(res)
}
).catch(e => console.log("Error calling scenario: " + e))
