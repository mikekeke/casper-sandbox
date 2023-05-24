const CasperWalletProvider = window.CasperWalletProvider;

const provider = CasperWalletProvider();

function ConnectWallet(props) {
  return <button onClick={() => connectWallet(props)}>Connect Wallet</button>;
}

function connectWallet(props) {
  provider.requestConnection()
    .then(connected => {
      if (connected) {
        provider.getActivePublicKey()
          .then(pk => props.setPublicKey(pk));
      } else {
        alert("Wallet not conencted");
      }
    })
    .catch(err => alert("Could not connect to wallet: " + err));
}


export default ConnectWallet;