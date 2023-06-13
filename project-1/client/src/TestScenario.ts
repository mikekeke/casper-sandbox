

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

  const nodeStatus = await contactSdk.contractClient.casperClient?.nodeClient.getStatus()
  console.log({ nodeStatus: nodeStatus })

  const contractHash = contactSdk.findContractHash(keys.publicKey)

  if (!contractHash) {
    await installContract()
  } else {
    console.log("Contract already installed. Procceding to endpoints calls.")
  }

  console.log("Setting contract hash to client")
  await contactSdk.setAccoutHash(keys.publicKey)

  console.log("Calling register")
  let [regDeploy, regDeployHash] = await contactSdk.register(
    "312402510",
    keys.publicKey,
    [keys]
  )
  console.log("Awaiting reg deploy ready...")
  const regDeployResult = await contactSdk.awaitDeploy(regDeploy)
  console.log(JSON.stringify(regDeployResult.execution_results[0].result))
}

async function installContract() {
  const wasm = readWasm(wasmPath)
  const [installDeploy, deployHash] = await contactSdk.installOnChain(
    wasm,
    contractInstallCost,
    keys.publicKey,
    [keys]
  )

  console.log({ deployHash: deployHash })

  console.log("Awaiting install deploy ready...")
  const installDeployResult = await contactSdk.awaitDeploy(installDeploy)

  if (!ContractSDK.isDeploySuccesfull(installDeployResult)) {
    console.log({ installDeployResult: installDeployResult.execution_results[0].result })
    const cause = installDeployResult.execution_results[0].result.Failure?.error_message
    throw new Error("Install deploy failed: " + cause)
  }
  console.log("Contract installed")
}


runScenario().then(res => {
  console.log("--- Result ---")
  console.log(res)
}
).catch(e => console.log("Error calling scenario: " + e))