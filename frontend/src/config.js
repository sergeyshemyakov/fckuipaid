import { http, createConfig } from 'wagmi'
import { arbitrumSepolia } from 'wagmi/chains'
import { metaMask } from 'wagmi/connectors'

export const config = createConfig({
  autoConnect: true,
  chains: [arbitrumSepolia],
  transports: {
    [arbitrumSepolia.id]: http(),
  },
  // publicClient,
  connectors: [metaMask()],
});     