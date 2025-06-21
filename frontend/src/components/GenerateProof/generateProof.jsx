import React, { useState } from "react";
import {
  Center,
  Box,
  Select,
  Avatar,
  useSelectContext,
  HStack,
  Text,
  Portal,
  Button,
  Textarea,
} from "@chakra-ui/react";
import Header from "../header/header.jsx";
import "./generateProof.css";
import { subscriptions, months, years } from "./subscriptions.js";
import ZketFlix from "../../assets/zketFlix.svg";
import Vfp from "../../assets/vfp.svg";
import {
  useAccount,
  useWriteContract,
  useSignMessage,
} from "wagmi";
import {
  ERC20ABI,
  SMARTCONTRACTABI,
  ERC20ADDRESS,
  SMARTCONTRACTADDRESS,
  zkKey
} from "./contract.js";
import { parseUnits } from "viem";

export default function GenerateProof() {
  const { isConnected } = useAccount();
  const [step, setStep] = React.useState("idle"); //'idle' | 'approving' | 'paying'
  const [approveTxHash, setApproveTxHash] = React.useState(null); // `0x${string}` | null
  const [payTxHash, setPayTxHash] = React.useState(null);

  const [zkProofKey , setZkProofKey] = React.useState(null);
  

  const [monthSelected, setMonthSelected] = React.useState(6);
  const [yearSelected, setYearSelected] = React.useState(2025);

  const { writeContractAsync: writeApprove } = useWriteContract();
  const { writeContractAsync: writePay } = useWriteContract();

  const { signMessageAsync} = useSignMessage()


  const handleSign = async() => {
    const signData = await signMessageAsync({ message: 'This message proofs your ownership of your address in zk proof' })
    console.log("signature",signData);
    setZkProofKey(zkKey);
  }



  const handleApprove = async () => {
    setStep("approving");
    console.log("calling approval");
    const approve = await writeApprove({
      abi: ERC20ABI,
      address: ERC20ADDRESS,
      functionName: "approve",
      args: [SMARTCONTRACTADDRESS, parseUnits("10", 18)],
      onSuccess: (data) => {
        console.log(" approval success");
        setApproveTxHash(data.hash);
      },
      onError: (error) => {
         console.log(" approval Failed");
        console.error("Pay TX failed:", error);
      },
    });
    console.log("approve : ", approve);
    setApproveTxHash(approve);
  };

  const handlePay = async () => {
    await handleApprove();
    console.log("calling pay");
      setStep("paying");
      const pay = await writePay({
        abi: SMARTCONTRACTABI,
        address: SMARTCONTRACTADDRESS,
        functionName: "pay",
        args: [BigInt(monthSelected.toString() + yearSelected.toString())], //monthandYear
        onSuccess: (data) => {
          console.log("calling pay success");
          setPayTxHash(data.hash);
        },
        onError: (error) => {
          console.error("Pay TX failed:", error);
        },
      });

      console.log("pay: ", pay);
      setPayTxHash(pay);
  };

  const SelectValue = () => {
    const select = useSelectContext();
    const items = select.selectedItems;
    const { name } = items[0];
    return (
      <Select.ValueText placeholder="Select subscription">
        <HStack>
          <Avatar.Root shape="rounded" size="sm">
            <Avatar.Image src={Vfp} alt={name} />
            <Avatar.Fallback name={name} />
          </Avatar.Root>
          {name}
        </HStack>
      </Select.ValueText>
    );
  };

  const subscriptionChange = (event) => {
    console.log(event.target.value);
  };

  const monthChange = (event) => {
    console.log(event.target.value);
    setMonthSelected(event.target.value);
  };

  const yearChange = (event) => {
    console.log(event.target.value);
    setYearSelected(event.target.value);
  };

  return (
    <>
      <Header />
      <Center>
        <Box
          background="gray.50"
          className="proof-container"
          shadow="12px 12px 2px 1px rgba(193, 193, 199, 0.1)"
        >
          <Box className="year-holder">
            <Text>Buy subscriptions</Text>
          </Box>

          <Box className="subscription-holder">
            <Select.Root
              collection={subscriptions}
              size="lg"
              width="500px"
              defaultValue={["vfp_monthly"]}
              positioning={{ sameWidth: true }}
              variant={"subtle"}
              onChange={(event) => subscriptionChange(event)}
            >
              <Select.HiddenSelect />
              <Select.Label>Select Subscription</Select.Label>
              <Select.Control>
                <Select.Trigger>
                  <SelectValue />
                </Select.Trigger>
                <Select.IndicatorGroup>
                  <Select.Indicator />
                </Select.IndicatorGroup>
              </Select.Control>
              <Select.Positioner>
                <Select.Content>
                  {subscriptions.items.map((item) => (
                    <Select.Item
                      item={item}
                      key={item.id}
                      justifyContent="flex-start"
                      _disabled={item.id == "vfp_monthly" ? false : true}
                    >
                      <Avatar.Root shape="rounded" size="2xs">
                        <Avatar.Image
                          src={item.id == "vfp_monthly" ? Vfp : ZketFlix}
                          alt={item.name}
                        />
                        <Avatar.Fallback name={item.name} />
                      </Avatar.Root>
                      {item.name}
                      <Select.ItemIndicator />
                    </Select.Item>
                  ))}
                </Select.Content>
              </Select.Positioner>
            </Select.Root>
          </Box>
          <Box className="period-holder">
            <Box className="month-holder">
              <Select.Root
                collection={months}
                size="sm"
                width="400px"
                variant={"subtle"}
                onChange={(event) => monthChange(event)}
              >
                <Select.HiddenSelect />
                <Select.Label>Select month</Select.Label>
                <Select.Control>
                  <Select.Trigger>
                    <Select.ValueText placeholder="Select month" />
                  </Select.Trigger>
                  <Select.IndicatorGroup>
                    <Select.Indicator />
                  </Select.IndicatorGroup>
                </Select.Control>
                <Portal>
                  <Select.Positioner>
                    <Select.Content>
                      {months.items.map((month) => (
                        <Select.Item item={month} key={month.id}>
                          {month.name}
                          <Select.ItemIndicator />
                        </Select.Item>
                      ))}
                    </Select.Content>
                  </Select.Positioner>
                </Portal>
              </Select.Root>
            </Box>
            <Box className="year-holder">
              <Select.Root
                collection={years}
                size="sm"
                width="400px"
                variant={"subtle"}
                onChange={(event) => yearChange(event)}
              >
                <Select.HiddenSelect />
                <Select.Label>Select year</Select.Label>
                <Select.Control>
                  <Select.Trigger>
                    <Select.ValueText placeholder="Select year" />
                  </Select.Trigger>
                  <Select.IndicatorGroup>
                    <Select.Indicator />
                  </Select.IndicatorGroup>
                </Select.Control>
                <Portal>
                  <Select.Positioner>
                    <Select.Content>
                      {years.items.map((year) => (
                        <Select.Item item={year} key={year.id}>
                          {year.name}
                          <Select.ItemIndicator />
                        </Select.Item>
                      ))}
                    </Select.Content>
                  </Select.Positioner>
                </Portal>
              </Select.Root>
            </Box>
          </Box>
          <Center>
            <Box className="buttons-holder">
               {/* <Box className="pay-holder">
                <Button
                  variant={"subtle"}
                  background={"green.600"}
                  color={"white"}
                  size={"lg"}
                  disabled={!isConnected}
                  onClick={() => handleApprove()}
                >
                  Approve
                </Button>
              </Box> */}
              <Box className="pay-holder">
                <Button
                  variant={"subtle"}
                  background={"green.600"}
                  color={"white"}
                  size={"lg"}
                  disabled={!isConnected}
                  onClick={() => handlePay()}
                >
                  Pay
                </Button>
              </Box>
              <Box className="proof-holder">
                <Button
                  variant={"subtle"}
                  background={"green.600"}
                  disabled={!payTxHash}
                  color={"white"}
                  size={"lg"}
                  onClick={()=>handleSign()}
                >
                  Generate Proof
                </Button>
              </Box>
            </Box>
          </Center>
          <Center>
            <Box className="proof-text-holder">
              <Textarea
                variant="subtle"
                size={"xl"}
                width={"800px"}
                height={"200px"}
                overflow={'scroll'}
                placeholder="Proof text will be available here..."
                value={zkProofKey}
              ></Textarea>
            </Box>
          </Center>
        </Box>
      </Center>
    </>
  );
}
