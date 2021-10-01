
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

/// If the user clicks on "Create new wallet" -> Redirect to Elrond to create a wallet and then redirect back to our site with QueryParam=?NewAccount 
/// and create a new account. Ask for name, email and a ?password?. 

/// NFT's must have categories -> saved in elk or any backend. Preferably elk for speed.

/// Users must be able to reffer other users.


# Frontend

/// User must be able to Sign in with their wallet.
    /// if user already exists in our db, show their profile
    /// else ask them to create a new account

/// User must be able to Create a new profile ( and a new wallet )

/// User must be able to view his/hers NFTs

/// User can post an NFT to sale.


