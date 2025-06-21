import { WagmiProvider} from "wagmi";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { config } from "./config";
import { WalletOptions } from "./walletOption";
import { ChakraProvider, defaultSystem } from "@chakra-ui/react";
import Header from "./components/header/header.jsx"
const queryClient = new QueryClient();


function App() {
  return (
    <ChakraProvider value={defaultSystem}>
      <WagmiProvider config={config}>
        <QueryClientProvider client={queryClient}>
          <Header />
        </QueryClientProvider>
      </WagmiProvider>
    </ChakraProvider>
  );
}

export default App;
