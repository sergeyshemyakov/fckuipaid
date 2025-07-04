import * as React from "react";
import { useConnect } from "wagmi";
import { Button , Image} from "@chakra-ui/react"
import WalletLogo from "./assets/wallet.svg"

export function WalletOptions() {
  const { connectors, connect } = useConnect();

  return connectors.map((connector) => (
    <WalletOption
      key={connector.uid}
      connector={connector}
      onClick={() => connect({ connector })}
    />
  ));
}

function WalletOption({ connector, onClick }) {
  const [ready, setReady] = React.useState(false);

  React.useEffect(() => {
    (async () => {
      const provider = await connector.getProvider();
      setReady(!!provider);
    })();
  }, [connector]);

  return (
    <Button disabled={!ready} onClick={onClick} colorPalette="teal" variant="subtle">
      <Image src={WalletLogo} alt="wallet-logo" /> {connector.name}
    </Button>
  );
}
