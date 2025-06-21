import React from "react";
import Header from "../header/header.jsx";
import { Box, Center, Textarea, Text, Button } from "@chakra-ui/react";
import "./verifyProof.css";

export default function VerifyProof() {

const [textInput , setTextInput] = React.useState(null);

const handleTextAreaChange = (event)=> {
    setTextInput(event.target.value);
}
  return (
    <>
      <Header />
      <Center>
        <Box
          background="gray.50"
          shadow="12px 12px 2px 1px rgba(193, 193, 199, 0.1)"
          className="verify-container"
        >
          <Box>
            <Text>Enter the signed signature below to verify</Text>
          </Box>
          <Box className="proof-text-holder">
            <Textarea
              variant="subtle"
              size={"xl"}
              width={"800px"}
              height={"200px"}
              overflow={'scroll'}
              placeholder="paste the signature to verify ..."
              onChange={(event)=>handleTextAreaChange(event)}
            ></Textarea>
          </Box>

          <Box className="proof-holder">
                          <Button
                            variant={"subtle"}
                            background={"green.600"}
                            disabled={textInput == null}
                            color={"white"}
                            size={"lg"}
                            onClick={()=>{}}
                          >
                            Verify
                          </Button>
                        </Box>
        </Box>
      </Center>
    </>
  );
}
