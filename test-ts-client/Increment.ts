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



// npx tsc && node .build/HelloWorld.js 
async function getKeyPair() {
    return Keys.Ed25519.parseKeyFiles(
        "../../test-1-ed25519-keys/public_key.pem",
        "../../test-1-ed25519-keys/secret_key.pem")
}

async function getPubKeyHex(): Promise<string> {
    return getKeyPair().then(k => k.accountHex())
}

const casperClient = new CasperClient("http://94.130.10.55:7777/rpc");
const contract = new Contracts.Contract();

async function runIncrement() {
    contract.setContractHash("hash-2131266e1784f2f1bd9061f579f4a55ef04b40d1e11ad7dc12dd9597d3870f05");
    const pubKeyHex = await getPubKeyHex();
    console.log("PK hex: " + pubKeyHex);
    const deploy = contract.callEntrypoint(
        "counter_inc",
        RuntimeArgs.fromMap({}),
        CLPublicKey.fromHex(pubKeyHex),
        "casper-test",
        "160000000"
    );

    const jsonDeploy = DeployUtil.deployToJson(deploy);

    const keyPair = await getKeyPair();
    console.log(DeployUtil.deployToJson(deploy));
    const signed = DeployUtil.signDeploy(deploy, keyPair);
    console.log(DeployUtil.deployToJson(signed));
    return casperClient.putDeploy(signed);
}

runIncrement().then(res => {
    console.log("Res:")
    console.log(res)
}
).catch(e => console.log("Error calling inc: " + e))