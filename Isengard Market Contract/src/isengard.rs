#![no_std]

elrond_wasm::imports!();

pub mod auction;
//pub mod sale;

use auction::*; // use all in auction?
//use sale::*; // use all in sale?

#[elrond_wasm::contract]
pub trait Isengard {

    #[init]
    fn init(
        &self
    ) -> SCResult<()> {
        let owner_address: ManagedAddress = self.blockchain().get_caller();
        self.set_owner(&owner_address);
        self.version().set(&1);
        //self.set_counter(&0);
        // Set prices for auctions, sales, etc.

    
        Ok(())
    }

    // owner endpoints
    #[only_owner]
    #[endpoint]
    fn empty_wallet(&self) -> SCResult<()>{
        let token_id = self.accepted_payment_token_id().get();
        let balance = self.blockchain().get_sc_balance(&token_id,0);
        let address = self.blockchain().get_owner_address();

        self.send()
            .direct(&address,&token_id, 0, &balance, b"retrieve successful");

        self.add_transaction();    
        Ok(())
    }
    
    // Owner endpoint to modify fixed prices
    #[only_owner]
    #[endpoint]
    fn update_prices(&self) -> SCResult<()>{   
        Ok(())
    }

    // views
    #[view(isUpForAuction)]
    fn is_up_for_sale(&self, token_id: &TokenIdentifier,nonce: &u64) -> bool {
        !self.sale_wrapper(&token_id, &nonce).is_empty()
    }

    // endpoints
    #[payable("EGLD")]
    #[endpoint]
    fn fund(
        &self,
        #[payment_token] payment_token: TokenIdentifier,
        #[payment_amount] payment_amount: BigUint,
    ) -> SCResult<()> {
        require!(
            payment_token == self.accepted_payment_token_id().get(),
            "Invalid payment token"
        );

        let caller = self.blockchain().get_caller(); // get the user that sent this request
        self.deposit(&caller)
            .update(|deposit| *deposit += payment_amount);
        
        self.add_transaction(); 
        Ok(())
    }

    // This function must be able to receive an NFT and a sum of EGLD.
    // The NFTs sender will be saved in storage as the NFT owner so he can retrieve it at any time.
    // The sum of EGLD is a tax we perceive so we can handle gas fees if the user adds and retrieves the NFT from the contract.
    #[payable("*")]
    #[endpoint]
    fn donate_nft(
        &self
    ) -> SCResult<()> {
        let token_type = self.call_value().esdt_token_type();
        require!(
            token_type == EsdtTokenType::NonFungible,
            "Invalid token donation"
        );

        self.add_transaction(); 
        Ok(())
    }

    // Maybe add a small fee for cancelling a sale.
    #[endpoint]
    fn cancel_sale(
        &self,
        token_id: TokenIdentifier,
        nonce: u64
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller(); // get the user that sent this request
        let amount = BigUint::from(1u64); // Create a BigUint with value of 1.

        
        let sale_w = self.sale_wrapper(&token_id, &nonce).get();
        // Make sure the one who calls this is the one who added the nft.
        require!(
           caller == sale_w.sale.unwrap().nft_owner,
           "You can't cancel a sale of an NFT you don't own."
        );

        self.send().direct(&caller, &token_id, nonce, &amount , b"retrieve successful");
        
        self.sale_wrapper(&token_id, &nonce).clear();
        self.add_transaction(); 
        Ok(())
    }

    // When the user adds an NFT for sale, add our fixed price to the price set by the user.
    #[payable("*")]
    #[allow(clippy::too_many_arguments)]
    #[endpoint]
    fn add_nft_for_sale(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
         price: BigUint,
    ) -> SCResult<()> {
        let token_type = self.call_value().esdt_token_type();
        require!(
            token_type == EsdtTokenType::NonFungible,
            "Invalid payment token"
        );

        let nft_owner = self.blockchain().get_caller(); // get the user that sent this request
        
        //let _token_data = self.blockchain().get_esdt_token_data(&caller, &token_id, nonce);
        let sale = Sale::new(
            &nft_owner,
            &price
        );

        // let state = NftStates::new(
        //     &nft_owner,
        //     NftState::Sale
        // );

        let wrapper = SaleWrapper::new_sale(sale, NftState::Sale);
        self.sale_wrapper(&token_id, &nonce).set(&wrapper);

        //self.nft_states(&token_id, &nonce).set(&state);
        //self.sale(&token_id, &nonce).set(&sale);
        //self.add();

        self.add_transaction(); 
        Ok(())
    }

    // When the user adds an NFT for sale, add our fixed price to the price set by the user.
    #[payable("*")]
    #[endpoint]
    fn add_nft_for_auction(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_nonce] nonce: u64,
            starting_price: BigUint,
            final_price: BigUint,
            start_time : u64,
            deadline : u64
    ) -> SCResult<()> {
        let token_type = self.call_value().esdt_token_type();
        require!(
            token_type == EsdtTokenType::NonFungible,
            "Invalid payment token"
        );

        let nft_owner = self.blockchain().get_caller(); // get the user that sent this request
        
        //let _token_data = self.blockchain().get_esdt_token_data(&caller, &token_id, nonce);
        let auction = Auction::new(
            &nft_owner,
            &starting_price,
            &final_price,
            deadline,
            start_time,
        );

        // let state = NftStates::new(
        //     &nft_owner,
        //     NftState::Auction
        // );

        //self.nft_states(&token_id, &nonce).set(&state);
        // self.auction(&token_id, &nonce).set(&auction);
        //self.add();
        let wrapper = SaleWrapper::new_auction(auction, NftState::Auction);
        self.sale_wrapper(&token_id, &nonce).set(&wrapper);

        self.add_transaction(); 
        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint]
    fn bid(&self,
        token_id: TokenIdentifier,
        nonce: u64,
        #[payment] bid_amount: BigUint
    ) -> SCResult<()> {
        require!(
            self.is_up_for_sale(&token_id, &nonce),
            "nft is not up for a type of sale!"
        );

        let  mut wrapper = self.sale_wrapper(&token_id, &nonce).get();

        //TODO MAKE SURE THIS AUCTION EXISTS SO THIS DOESN'T PANIC
        let  mut auction = wrapper.auction.unwrap();

        require!(
            self.blockchain().get_block_timestamp() < auction.deadline,
            "auction ended already!"
        );

        let caller = self.blockchain().get_caller(); // get the user that sent this request
        
        require!(
            caller != auction.nft_owner ,
            "you can not bid on your own auction"
        );

        require!(
           caller != self.blockchain().get_sc_address(),
            "can't transfer to this contract!"
        );
        require!(
            bid_amount >= auction.starting_price,
            "bid amount must be higher than or equal to starting price!"
        );
        require!(
            bid_amount > auction.current_bid,
            "the amount of EGLD must be higher than the current bid."
        );
        require!(
            bid_amount <= auction.final_price,
            "bid amount must be less than or equal to ending price!"
        );  
 
        // Refund losing bid
        if !auction.current_winner.is_zero() {
            self.send()
                .direct_egld(&auction.current_winner, &auction.current_bid, b"bid refund");
        }

        if auction.final_price <= bid_amount {

            self.send().direct_egld(&auction.nft_owner, &bid_amount, b"EGLD sent successfully");
            self.sale_wrapper(&token_id, &nonce).clear();

            self.add_transaction(); 

            //let amount = BigUint::from(1u64); // Create a BigUint with value of 1.
            //self.send().direct(&caller, &token_id, nonce, &amount , b"retrieve successful");

            //Ok(())
            Ok(self.transfer_to(caller, token_id, nonce))
        }else{
            auction.current_bid = bid_amount;
            auction.current_winner = caller;

            wrapper.auction = Some(auction);
            self.sale_wrapper(&token_id, &nonce).set(&wrapper);

            self.add_transaction(); 

            Ok(())
        }
    }

    #[endpoint]
    fn end_auction(&self, 
        token_id: TokenIdentifier,
        nonce: u64
    ) -> SCResult<()> {
        require!(
            self.is_up_for_sale(&token_id, &nonce),
            "nft is not up for auction!"
        );


        let wrapper = self.sale_wrapper(&token_id, &nonce).get();

        //TODO MAKE SURE THIS AUCTION EXISTS SO THIS DOESN'T PANIC
        let auction = wrapper.auction.unwrap();

        require!(
            self.blockchain().get_block_timestamp() > auction.deadline
                || auction.current_bid == auction.final_price,
            "auction has not ended yet!"
        );

        if !auction.current_winner.is_zero() {
            // send nft to the auction winner
            self.send().direct_egld(&auction.nft_owner, &auction.current_bid, b"EGLD sent successfully");
            self.add_transaction(); 
            //self.nft_states(&token_id, &nonce).clear();
            self.sale_wrapper(&token_id, &nonce).clear();

            let amount = BigUint::from(1u64); // Create a BigUint with value of 1.
            self.send().direct(&auction.current_winner, &token_id, nonce, &amount , b"retrieve successful");

            Ok(self.transfer_to(auction.current_winner, token_id, nonce))
            
        } else {
            // return nft to its owner
            self.add_transaction(); 
            //self.nft_states(&token_id, &nonce).clear();
            self.sale_wrapper(&token_id, &nonce).clear();

            Ok(self.transfer_to(auction.nft_owner, token_id, nonce))
        }
    }

    #[payable("EGLD")]
    #[endpoint]
    fn buy_nft_from_sale(&self,
        token_id: TokenIdentifier,
        nonce: u64,
        #[payment] payment: BigUint
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller(); // get the user that sent this request
        let nft_count = BigUint::from(1u64);
        // let sale = self.sale(&token_id, &nonce).get();
        
        let sale_w = self.sale_wrapper(&token_id, &nonce).get();
        let sale = sale_w.sale.unwrap();
        // Make sure the one who calls this is the one who added the nft.
        require!(
           caller != sale.nft_owner,
           "You can't buy an nft that you put up for sale"
        );

        require!(
            !caller.is_zero(),
            "Can't transfer to default address 0x0!"
        );
        require!(
           caller != self.blockchain().get_sc_address(),
            "Can't transfer to this contract!"
        );
        require!(
            sale.price == payment,
            "The amount of EGLD doesn't match the price {sale.price} {amount}"
        );
 
        // Send the NFT to the new owner
        self.send()
            .direct(&caller, &token_id, nonce, &nft_count, b"nft sent successfully");

        // Send the EGLD to the old owner.
        self.send()
            .direct_egld(&sale.nft_owner, &payment, b"EGLD sent successfully");

        self.sale_wrapper(&token_id, &nonce).clear();

        self.add_transaction(); 
        // self.nft_states(&token_id, &nonce).clear();

        Ok(())
    }

    #[endpoint]
    fn retrieve(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller(); // get the user that sent this request
        let token_id = self.accepted_payment_token_id().get(); // id of the ESDT Token (EGLD in this case)
        let deposit = self.deposit(&caller).get(); // retrieve the amount this address has deposited.

        self.deposit(&caller).clear(); // clear this deposit from our storage?

        self.send()
            .direct(&caller, &token_id, 0, &deposit, b"retrieve successful");

        self.add_transaction(); 
        Ok(())
    }

    // #[endpoint]
    // fn add(&self){
    //     let counter = self.get_counter();
    //     let new = counter + 1;
    //     self.set_counter(&new);
    // }

    // #[endpoint]
    // fn substract(&self){
    //     let counter = self.get_counter();
    //     let new = counter - 1;
    //     self.set_counter(&new);
    // }

    // private
    fn transfer_to(&self, 
        address: ManagedAddress, 
        token_id: TokenIdentifier,
        nonce: u64
    ){
        let amount = BigUint::from(1u64); // Create a BigUint with value of 1.
        self.send().direct(&address, &token_id, nonce, &amount , b"retrieve successful");
    }

    fn add_transaction(&self){
        let transaction_counter = self.get_transaction_counter();
        let counter = transaction_counter + 1;
        self.set_transaction_counter(&counter);
    }

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> ManagedAddress;
    
    // storage
    #[view(getAcceptedPaymentToken)]
    #[storage_mapper("acceptedPaymentTokenId")]
    fn accepted_payment_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // #[storage_mapper("nftDeposit")]
    // fn nft_deposit(&self, nft_id: u32) -> SingleValueMapper<Self::Storage, Auction<Self::Api>>;
    


    // #[view(getActiveSalesCounter)]
    // #[storage_get("activeSalesCounter")]
    // fn get_counter(&self) -> i64;

    // #[storage_set("activeSalesCounter")]
    // fn set_counter(&self, sum: &i64);

    #[view(getTransactionCount)]
    #[storage_get("transactionCount")]
    fn get_transaction_counter(&self) -> u64;
    //base64 to binary then binary to decimal to see this.

    #[storage_set("transactionCount")]
    fn set_transaction_counter(&self, sum: &u64);

    #[storage_set("owner")]
    fn set_owner(&self, address: &ManagedAddress);   

    #[view(getVersion)]
    #[storage_mapper("version")]
    fn version(&self) -> SingleValueMapper<u64>;

    // This might not be needed any more after the new method is implemented.
    #[view(getNftState)]
    #[storage_mapper("nftStates")]
    fn nft_states(&self, nft_id: &TokenIdentifier, nonce:&u64) -> SingleValueMapper<NftStates<Self::Api>>;

    #[storage_mapper("sale")]
    fn sale(&self, nft_id: &TokenIdentifier, nonce:&u64) -> SingleValueMapper<Sale<Self::Api>>;

    #[storage_mapper("auction")]
    fn auction(&self, nft_id: &TokenIdentifier,nonce:&u64) -> SingleValueMapper<Auction<Self::Api>>;

    #[view(getWrapper)]
    #[storage_mapper("saleWrapper")]
    fn sale_wrapper(&self, nft_id: &TokenIdentifier,nonce:&u64) -> SingleValueMapper<SaleWrapper<Self::Api>>;

    #[view(getSale)]
    #[storage_get("sale")]
    fn get_sale(&self, nft_id: &TokenIdentifier, nonce:&u64) -> Sale<Self::Api>;
    
}


