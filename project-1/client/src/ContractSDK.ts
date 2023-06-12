import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  CLValueBuilder,
  CLPublicKey,
  DeployUtil,
  Signer,
  Keys,
  GetDeployResult


} from "casper-js-sdk";

export class ContractSDK {
  private casperClient: CasperClient;

  public contractClient: Contracts.Contract;

  constructor(public nodeAddress: string, public chainName: string) {
    this.casperClient = new CasperClient(nodeAddress);
    this.contractClient = new Contracts.Contract(this.casperClient);
  }

  public mkInstallDeploy(
    wasm: Uint8Array,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[],
  ): DeployUtil.Deploy {
    return this.contractClient.install(
      wasm,
      RuntimeArgs.fromMap({}),
      paymentAmount,
      deploySender,
      this.chainName,
      keys
    )
  }

  public async installOnChain(
    wasm: Uint8Array,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[]
  ): Promise<[DeployUtil.Deploy, string]> {
    const installDeploy = this.mkInstallDeploy(wasm, paymentAmount, deploySender, keys)
    return this.casperClient.putDeploy(installDeploy)
      .then(deployHash => { return [installDeploy, deployHash] })
  }

  public async awaitDeployed(
    deploy: DeployUtil.Deploy,
    timeout?: number
  ): Promise<GetDeployResult> {
    return this.casperClient.nodeClient.waitForDeploy(deploy, timeout)
  }

  public static isDeploySuccesfull(deployResult: GetDeployResult): boolean {
    if (deployResult.execution_results[0].result.Success) {
      return true
    } else {
      return false
    }
  }
}