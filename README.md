
# Functionality:
- User must be able to send an NFT to the contract for sale -> contract must know that this is sale and not auction and must save owner.   - DONE
- User must be able to buy the NFT if found on the sale list   - DONE
- User must be able to retrieve there NFT (pong) if the NFT is on the sale list and the user is the Owner. - DONE
- User must be able to send an NFT to the contract for auction -> contract must know that this is auction and not sale and must save owner. - DONE
- User must be able to bid on the NFT if found on the auction list - DONE (but auction list must exist in backend)
- User must be able to retrieve their NFT (pong) if the NFT wasn't sold in the auction - DONE ( If there were no bidders, otherwise it's sold. )
- User must be able to retrieve their bid ammount if the auction was won by someone else. - DONE ( the bid amount is sent automatically)
- Users can request a view of the status of the bids.  - Not done yet.
- Users can request a view of the time left to bid. - Not done yet.
- User must be able to add NFTs to a drop. - Not done yet. Must clarify drops.
- Users must be able to join a drop for a fixed price. - Not done yet. Must clarify drops. 
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
`erdpy --verbose contract call $CONTRACT_ADDRESS --pem="../wallet/wallet.pem" --gas-limit=9000000 --function="test" --proxy="https://testnet-gateway.elrond.com" --recall-nonce --send`

`erdpy contract query $CONTRACT_ADDRESS --function="get" --proxy="https://testnet-gateway.elrond.com"`


## Interact with transactions (also using some variables from above)
Docs at: https://docs.elrond.com/sdk-and-tools/erdpy/sending-bulk-transactions/

The following transaction will fund the contract with 0.01 EGLD.
`MYWALLET="erd1q0pqc9g2uv98r9uc9c5l8zt2rtvaz72rpp72eauhw39tlzdfngtsjp4xln"`
`PEM_FILE="../wallet/wallet.pem"`
`PROXY="https://testnet-gateway.elrond.com"`
`NONCE=$(erdpy account get --nonce --address=$MYWALLET --proxy=$PROXY)`
`DENOMINATION="0000000000000000"`
`GAS_LIMIT=5000000`
`erdpy --verbose tx new --send --outfile="output/bon-mission-tx-$NONCE.json" --pem=$PEM_FILE --nonce=$NONCE --receiver=$CONTRACT_ADDRESS --value="1$DENOMINATION" --gas-limit=$GAS_LIMIT --proxy=$PROXY --data="fund"`

In order to retrieve all the funds you sent to the contract use the following code:
`erdpy --verbose tx new --send --outfile="output/bon-mission-tx-$NONCE.json" --pem=$PEM_FILE --nonce=$NONCE --receiver=$CONTRACT_ADDRESS --value="0" --gas-limit=$GAS_LIMIT --proxy=$PROXY --data="retrieve"`
Note: Don't forget to update the NONCE variable after each transaction.


### Sending an NFT to the Contract from the wallet
`ESDTNFTTransfer@34535449434b2d643438396430@06@01@000000000000000005003ed42802d3f3205dbb3729a3e0ba3e3e16cc33f39a17@66756e645f6e6674@34535449434b2d643438396430@06`

`ESDTNFTTransfer@<token identifier in hexadecimal encoding>@<the nonce after the NFT creation in hexadecimal encoding>@<quantity to transfer in hexadecimal encoding> @<destination address in hexadecimal encoding>@<name of method to call in hexadecimal encoding> @<first argument of the method in hexadecimal encoding> @<second argument of the method in hexadecimal encoding>`


### Retrieve an NFT from the contract to the wallet.
data="retrieve_nft@34535449434b2d643438396430@06"


### Add NFT for Sale
`add_nft_for_sale`
`ESDTNFTTransfer@34535449434b2d643438396430@0d@01@000000000000000005000eb0eeced4ac0be4258d35847986c8c5a734f3739a17@6164645f6e66745f666f725f73616c65@2386F26FC10000`
This NFT will be bought with 0.01 EGLD

### Buy NFT on sale
Use GAS -> 29000000
`buy_nft_from_sale`
`erd1qqqqqqqqqqqqqpgq002q4p9k4jllln9w882wagrk5s22ga30ngts649jv5`
`buy_nft_from_sale@34535449434b2d643438396430@0d`

### Add NFT on Auction
Use GAS -> 29000000
`add_nft_for_auction`
`add_nft_for_auction@collection@nonce@starting@ending@deadline` 
`ESDTNFTTransfer@34535449434b2d643438396430@0f@01@000000000000000005005a3e2b3486045c78b9dd34421ce92514caf202e69a17@6164645f6e66745f666f725f61756374696f6e@2386F26FC10000@6A94D74F430000@1633456503`

### Bid on Auction
Use GAS -> 20000000
`bid`
`erd1qqqqqqqqqqqqqpgqtglzkdyxq3w83wwax3ppe6f9zn90yqhxngtsr2qy9c`
`bid@34535449434b2d643438396430@0f`

### End auction
`end_auction`
`erd1qqqqqqqqqqqqqpgqtglzkdyxq3w83wwax3ppe6f9zn90yqhxngtsr2qy9c`
`endAuction@34535449434b2d643438396430@0f`

# Tests to run:
    1. Make sure that if you add a NFT sale only you can cancel the sale, nobody else should be able to cancel the sale.
    2. Auction should fail to end if the deadline is not met or if the highest bid is smaller than the final price.
    3. The auction must be ended only by the auction owner or the highest bidder( highest bidder on only if final price touched).
    4. Make sure that if there are not bidders to an auction, when the auction is ended, the owner gets his NFT back.
    5. A user can't bid on his own auction.
    6. If the bid is equal or higher than the final price, end the auction and do the transfers without requiring an endAuction.

# Future improvements 
    1. Add x-time to auctions. (ask panica)
    2. Add a refferal system. (refferal parameter on all calls ? )