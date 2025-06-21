import { Box, Text, Image, Button } from "@chakra-ui/react";
import React from "react";
import VFPLOGO from "../../assets/vfp.svg";
import "./headervtk.css";

export default function HeaderVTK() {
  return (
    <>
      <Box className="header-container">
        <Box className="company-name-container">
          <Image src={VFPLOGO} height={"2rem"} alt="payment-logo" />
          <Text className="header-title" data-testid="header-title-testid">
            Vitalik Feet Pics
          </Text>
        </Box>
      </Box>
      <hr className="hearder-underline"></hr>
    </>
  );
}
