import {
  WagmiProvider,
  useAccount,
  useDisconnect,
} from "wagmi";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { config } from "./config";
import { WalletOptions } from "./walletOption"
const queryClient = new QueryClient();




export function Profile() {
  const { address } = useAccount();
  const { disconnect } = useDisconnect();
  return (<>
  <button onClick={() => disconnect()}>Disconnect</button>
  <div>{address}</div>
  </>)
  
}


function ConnectWallet() {
  const { isConnected } = useAccount()
  if (isConnected) return <Profile />
  return <WalletOptions />
}

function App() {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <ConnectWallet />
      </QueryClientProvider>
    </WagmiProvider>
  );
}

export default App;
