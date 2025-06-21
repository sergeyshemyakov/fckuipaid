import { Box, Text, Image, Button } from "@chakra-ui/react";
import React from "react";
import { WalletOptions } from "../../walletOption";
import PaymentLogo from "../../assets/bank.svg";
import "./header.css";
import { useAccount, useDisconnect } from "wagmi";

export default function Header() {
  const { isConnected, address } = useAccount();
  const { disconnect } = useDisconnect();

  return (
    <>
      <Box className="header-container">
        <Box className="company-name-container">
          <Image src={PaymentLogo} height={"3rem"} alt="payment-logo" />
          <Text className="header-title" data-testid="header-title-testid">
            fcukipaid
          </Text>
        </Box>
        {!isConnected && (
          <Box className="search-block">
            <WalletOptions />
          </Box>
        )}
        {isConnected && (
          <Box className="search-block">
            <Text>{address}</Text>
            <Button
              colorPalette="teal"
              variant="subtle"
              onClick={() => disconnect()}
            >
              Disconnect
            </Button>
          </Box>
        )}
      </Box>
      <hr className="hearder-underline"></hr>
    </>
  );
}
