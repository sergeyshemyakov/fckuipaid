import React from "react";
import HeaderVTK from "../headerVTK/headervtk.jsx";
import { Box, Center, Textarea, Text, Button, Image } from "@chakra-ui/react";
//import { useWriteContract } from "wagmi";
import { VERIFYCONTRACTADDRESS, VERIFYCONTRACTABI } from "./verifyContract.js";
import "./verifyProof.css";
import VitalikPic from "../../assets/vtkfoot.jpg";

export default function VerifyProof() {
  const [textInput, setTextInput] = React.useState(null);
  const [loading, setLoading] = React.useState(false);

  const [verified, setVefied] = React.useState(false);

  const ontextAreaChange = (event) => {
    setTextInput(event.target.value);
  };

  const handleTextAreaSubmission = () => {
    setLoading(true);
    setTimeout(() => {
      setLoading(false);
      setVefied(true);
    }, 4000);
  };
  return (
    <>
      <HeaderVTK />
      {loading && (
        <>
          <Center>
            <Text>Redirecting to content page .....</Text>
          </Center>
        </>
      )}
      {!verified && !loading && (
        <Center>
          <Box
            background="gray.50"
            shadow="12px 12px 2px 1px rgba(193, 193, 199, 0.1)"
            className="verify-container"
          >
            <Box>
              <Text>Enter the ZK Proof to login </Text>
            </Box>
            <Box className="proof-text-holder">
              <Textarea
                variant="subtle"
                size={"xl"}
                width={"800px"}
                height={"200px"}
                overflow={"scroll"}
                placeholder="paste the ZK Proof to verify ..."
                onChange={(event) => ontextAreaChange(event)}
              ></Textarea>
            </Box>

            <Box className="proof-holder">
              <Button
                variant={"subtle"}
                background={"green.600"}
                disabled={textInput == null}
                color={"white"}
                size={"lg"}
                onClick={() => {
                  handleTextAreaSubmission();
                }}
              >
                Verify to login
              </Button>
            </Box>
          </Box>
        </Center>
      )}
      {verified && (
        <Center>
          <Box className="loggedin-page">
            <Text>Welcome to Vitalik FootPics , unknown user !!!</Text>
            <Image
              src={VitalikPic}
              width="300px"
              height="620px"
              alt="vitalik pic"
            />
          </Box>
        </Center>
      )}
    </>
  );
}
