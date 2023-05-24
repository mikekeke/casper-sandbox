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
import axios from 'axios';

const CasperWalletProvider = window.CasperWalletProvider;

const provider = CasperWalletProvider();

const casperClient = new CasperClient("http://94.130.10.55:7777/rpc");
const contract = new Contracts.Contract();

function DeployIncrement(props) {
  return <button onClick={() => deployIncrement(props)}>Deploy Increment</button>;
}

function deployIncrement(props) {
  provider.isConnected()
    .then(connected => {
      if (connected) {
        deploy()
      } else {
        alert("Wallet not connected. Connect first.")
      }
    })
    .catch(err => alert("Could not check wallet connection: " + err.message));
}

async function deploy() {
  contract.setContractHash("hash-2131266e1784f2f1bd9061f579f4a55ef04b40d1e11ad7dc12dd9597d3870f05");

  provider.getActivePublicKey()
    .then(pubKeyHex => {
      console.log("PK hex: " + pubKeyHex);
      const pubKey = CLPublicKey.fromHex(pubKeyHex);
      const deploy = contract.callEntrypoint(
        "counter_inc",
        RuntimeArgs.fromMap({}),
        pubKey,
        "casper-test",
        "160000000"
      );
      const deployJson = DeployUtil.deployToJson(deploy);
      console.log(deploy);
      return provider.sign(JSON.stringify(deployJson), pubKeyHex)
        .then(sig => { return [sig, deploy, pubKey] });
    })
    .then(res => {
      const [signature, deploy, pubKey] = res
      if (signature.cancelled) {
        throw new Error("Sign cancelled");
      } else {
        console.log("here-1");

        const signedDeploy = DeployUtil.setSignature(
          deploy,
          signature.signature,
          pubKey
        );
        return signedDeploy;
      }
    })
    .then(signedDeploy => {
      console.log('Sign successful:')
      console.log(signedDeploy);
      sendDeploy(signedDeploy);
    })
    .then(deployResponse => {
      console.log("Depl resp: " + deployResponse);
    })
    .catch(err => {
      alert('Deploy error: ' + err);
    })
    ;
}

async function sendDeploy(signedDeploy) {
  console.log("--------------");
  console.log(JSON.stringify(signedDeploy));
  console.log(DeployUtil.deployToJson(signedDeploy));
  console.log(JSON.stringify(DeployUtil.deployToJson(signedDeploy)));
  const toSend = JSON.stringify(DeployUtil.deployToJson(signedDeploy));
  try {
    const response = await axios.post(
      "http://127.0.0.1:8080/accept_deploy",
      toSend,
      { headers: { 'Content-Type': 'application/json' } });
    console.log(response);
    alert(response.data);
  } catch (error) {
    alert(error.message);
  }
}

async function sendDeployCl(signedDeploy) {
  return casperClient.putDeploy(signedDeploy);
}

export default DeployIncrement;