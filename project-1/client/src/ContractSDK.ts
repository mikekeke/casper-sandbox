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
    this.contractClient.setContractHash
    return this.contractClient.install(
      wasm,
      RuntimeArgs.fromMap({}),
      paymentAmount,
      deploySender,
      this.chainName,
      keys
    )
  }

  public async findContractHash(publicKey: CLPublicKey): Promise<string | undefined> {
    const rootHash = await this.casperClient.nodeClient.getStateRootHash()
    const accountHash = publicKey.toAccountHashStr()
    const state = await this.casperClient.nodeClient
      .getBlockState(rootHash, accountHash, [])
      return state
      .Account
      ?.namedKeys
      .find(key => key.name === "add-with-registry-contract-key")
      ?.key
  }

  public async setAccoutHash(publicKey: CLPublicKey): Promise<void> {
    const contractHash = await this.findContractHash(publicKey)
    if (!contractHash) {
      throw new Error("Contract hash not found under expected key. Is contract deployed?")
    }
    console.log({ contractHash: contractHash })
    this.contractClient.setContractHash(contractHash)

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

  public async awaitDeploy(
    deploy: DeployUtil.Deploy,
    timeout?: number
  ): Promise<GetDeployResult> {
    return this.casperClient.nodeClient.waitForDeploy(deploy, timeout)
  }

  public async register(
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[],): Promise<[DeployUtil.Deploy, string]> {
    const deploy = this.contractClient.callEntrypoint(
      "register_user_key",
      RuntimeArgs.fromMap({}),
      deploySender,
      this.chainName,
      paymentAmount,
      keys
    )
    return this.casperClient.putDeploy(deploy)
      .then(deployHash => { return [deploy, deployHash] })
  }

  public static isDeploySuccesfull(deployResult: GetDeployResult): boolean {
    if (deployResult.execution_results[0].result.Success) {
      return true
    } else {
      return false
    }
  }
}