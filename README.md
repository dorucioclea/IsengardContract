
# Functionality:
- User must be able to send an NFT to the contract for auction -> contract must know that this is auction and not auction and must save owner.
- User must be able to send an NFT to the contract for sale -> contract must know that this is sale and not auction and must save owner.
- User must be able to bid on the NFT if found on the auction list
- User must be able to buy the NFT if found on the sale list
- User must be able to retrieve their NFT (pong) if the NFT wasn't sold in the auction
- User must be able to retrieve there NFT (pong) if the NFT is on the sale list and the user is the Owner.
- User must be able to retrieve their bid ammount if the auction was won by someone else.
- Users can request a view of the status of the bids?
- Users can request a view of the time left to bid.
- User must be able to add NFTs to a drop.
- Users must be able to join a drop for a fixed price.
- User must receive their refferal reward is there is one to be offered. (without claim?)

- Users 
- Fields
- Contract must have a contract manager set -> Our address that can send instructions


# Backend

- If the user clicks on "Create new wallet" -> Redirect to Elrond to create a wallet and then redirect back to our site with QueryParam=?NewAccount and create a new account. Ask for name, email and a ?password?. 
- NFT's must have categories -> saved in elk or any backend. Preferably elk for speed.
- Users must be able to reffer other users.


# Frontend

- User must be able to Sign in with their wallet.
    - if user already exists in our db, show their profile
    - else ask them to create a new account

- User must be able to Create a new profile ( and a new wallet )

- User must be able to view his/hers NFTs

- User can post an NFT to sale.

# Commands

# Interaction
## Basic interactions
`erdpy contract build`
`erdpy contract deploy`
`export CONTRACT_ADDRESS=$(python3 -c "import json; data = json.load(open('deploy-testnet.interaction.json')); print(data['emitted_tx']['address'])")`
`erdpy --verbose contract call $CONTRACT_ADDRESS --pem="../wallet/wallet.pem" --gas-limit=9000000 --function="test" --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send`

`erdpy contract query $CONTRACT_ADDRESS --function="get" --proxy="https://devnet-gateway.elrond.com"`


## Interact with transactions (also using some variables from above)
Docs at: https://docs.elrond.com/sdk-and-tools/erdpy/sending-bulk-transactions/

The following transaction will fund the contract with 0.01 EGLD.
`MYWALLET="erd17e4uuvhhnncye6mxxzffmgfhtyz8tpf4ug25he23z99j6yg8lwfqus4n28"`
`PEM_FILE="../wallet/wallet.pem"`
`PROXY="https://devnet-gateway.elrond.com"`
`NONCE=$(erdpy account get --nonce --address=$MYWALLET --proxy=$PROXY)`
`DENOMINATION="000000000000000000"`
`erdpy --verbose tx new --send --outfile="bon-mission-tx-$NONCE.json" --pem=$PEM_FILE --nonce=$NONCE --receiver=$CONTRACT_ADDRESS --value="10000000000000000$DENOMINATION" --gas-limit=9000000 --proxy=$PROXY --data="fund"`

In order to retrieve all the funds you sent to the contract use the following code:
`erdpy --verbose tx new --send --outfile="bon-mission-tx-$NONCE.json" --pem=$PEM_FILE --nonce=$NONCE --receiver=$CONTRACT_ADDRESS --value="0$DENOMINATION" --gas-limit=50000000 --proxy=$PROXY --data="retrieve"`
Note: Don't forget to update the NONCE variable after each transaction.


### Sending an NFT to the Contract from the wallet
`ESDTNFTTransfer@34535449434b2d666533313938@04@01@0000000000000000050090c561af3472f25db43fe7bc41f73261b45d3c85fb92@66756e645f6e6674`

`ESDTNFTTransfer@<token identifier in hexadecimal encoding>@<the nonce after the NFT creation in hexadecimal encoding>@<quantity to transfer in hexadecimal encoding> @<destination address in hexadecimal encoding>@<name of method to call in hexadecimal encoding> @<first argument of the method in hexadecimal encoding> @<second argument of the method in hexadecimal encoding>`