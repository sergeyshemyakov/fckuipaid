# fckuipaid

Fckuipaid project from zkHack Berlin 2025: Anonymized login lib for paywalled web2 services

# Problem

Imagine you want to create a web2 website vitalikfeetpicsmonthly.com that provides quality content for a modest fee. Your users would be happy to pay, but they would not want to be tracked in any way, even correlating different login sessions. 

fckuipaid.xyz is a website and a login flow that could be integrated into a website allowing you exactly that. Users pay monthly fee to a smart contract on Arbitrum and log in with a zk proof of payment, so web2 service knows that they paid.

# How fckuipaid works

## Flow
- User approves spending to the payment contract 
- User calls pay(month) function on the payment contract that pulls the required fee and registers payment for a given month
- User uses Boundless Steel coprocessor to generate a proof of payment. For this user signs a Metamask message, the following is proved: signature is valid (i.e. user controls the privKey), hasPaid function returns true for the given month and user (checked against the current state root). The proof could be generated locally in theory (not in our impl)
-  User copies the zk proof together with the public inputs (current block hash, month number) and pastes it to the website. The proof is verified against public inputs, on success user is logged in.

## Components

- *fckuipaid.xyz*: Website that provides a convenient UI for paying the monthly fee for a chosen service and requests the backend to generate a valid proof of payment.
- *Boundless*: Boundless SDK is used on the backend to generate a zk proof of payment agains the current state root. We make a request to boundless prover market.
- *vitalikfeetpics.com*: web2 website that verifies the zk proof of payment. It also checks that the payment month from the public inputs is the current month.

# What we have done

There are following subdirectories:

## fckuipaid

Foundry project that contains smart contracts for test ERC-20 payment token and PaymentReceiver that registers user payments for a given month. It also contains a deployment script for these smart contracts to deploy and verify them on Arbitrum Sepolia. Note that PaymentReceiver needs to be deployed separately for each new web2 service.

## frontend

Vite frontend implementations, has two pages: fckuipaid and vitalikfeetpics (accessible at /verify).

fckuipaid page correctly processes payment and prepares data for generating zk proof on the backend. However it does not process the response from the backend since we didn't manage to make it work.

vitalikfeetpics does not implement any verification logic because we didn't manage to generate a proof. However it should just decode ABI encoded journal, seal and imageID bytes, make a simple RPC call to Risc0 verifier and check the month from the journal.

## boundless

Contains a host and guest programs for generating the correct proof locally (basically erc20 example + erc20-counter example glued together). The main function of this folder is to generate ELF guest program, which we have done and uploade the binary to IPFS here: https://ipfs.io/ipfs/bafybeibrj22d3fthskfsokat6gklu77pjwynqn4aocgl7mwxq3ak2u3eje.

## backend

Contains a rust server to interact with the frontend. It implements a request to boundless market to generate a proof for particular valid values (so the proof could be actually produced). The market is called offchain (we have provided a deposit on Ethereum Sepolia), the program is submitted via IPFS, the input is submitted directly with the request. For the reasons that we have not completely understood, we haven't managed to get a successful response (also no staked funds were spent).

# What still needs to be done

1. Fix backed communication to boundless market, debug proof generation
2. ABI encode seal, journal and imageID on the backend into a hex string so it could be displayed on fckuipaid frontend and be copied to vitalikfeetpics.
3. Add ABI decoding logic to vitalikfeetpics frontend and make a call to Risk0 verifier to check the proof.