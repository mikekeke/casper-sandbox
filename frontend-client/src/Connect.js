import { Signer } from "casper-js-sdk";

function Connect(props) {
  return <button onClick={() => connectSigner(props)}>Connect Signer</button>;
}

function connectSigner(props) {
  Signer.isConnected()
    .then((s) => {
      if (s === false) {
        Signer.sendConnectionRequest();
      } else {
        Signer.getActivePublicKey()
          .then((pubKey) => {
            props.setPublicKey(pubKey);
          })
          .catch((error) => {
            alert(error.message);
          });
      }
    })
    .catch((error) => {
      alert(error.message);
    });
}

export default Connect;