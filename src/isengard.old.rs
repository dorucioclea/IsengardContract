#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Isengard {

    #[init]
    fn init(
        &self,
        #[var_args] opt_token_id: OptionalArg<TokenIdentifier>,
    ) -> SCResult<()> {
        let token_id = opt_token_id
            .into_option()
            .unwrap_or_else(TokenIdentifier::egld);
        require!(
            token_id.is_egld() || token_id.is_valid_esdt_identifier(),
            "Invalid token provided"
        );

        let my_address: Address = self.blockchain().get_caller();
        self.set_owner(&my_address);

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

    // endpoints
    #[payable("EGLD")]
    #[endpoint]
    fn fund(
        &self,
        #[payment_token] payment_token: TokenIdentifier,
        #[payment_amount] payment_amount: Self::BigUint,
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
    fn fund_nft(
        &self,
        #[payment_token] _payment_token: TokenIdentifier,
    ) -> SCResult<()> {
        let token_type = self.call_value().esdt_token_type();
        require!(
            token_type == EsdtTokenType::NonFungible,
            "Invalid payment token"
        );

        let _caller = self.blockchain().get_caller(); // get the user that sent this request

        self.add_transaction(); 
        Ok(())
    }

    #[endpoint]
    fn retrieve_nft(
        &self,
    ) -> SCResult<()> {
        let token_type = self.call_value().esdt_token_type();
        require!(
            token_type == EsdtTokenType::NonFungible,
            "Invalid payment token"
        );

        let _caller = self.blockchain().get_caller(); // get the user that sent this request

        self.add_transaction(); 
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

    #[endpoint]
    fn add(&self) -> SCResult<()> {
        let counter = self.get_counter();
        let new = counter + 1;
        self.set_counter(&new);

        self.add_transaction(); 
        Ok(())
    }

    #[endpoint]
    fn substract(&self) -> SCResult<()> {
        let counter = self.get_counter();
        let new = counter - 1;
        self.set_counter(&new);
        
        self.add_transaction(); 
        Ok(())
    }

    #[endpoint]
    fn test(&self) -> SCResult<()> {
        Ok(())
    }

    // private

    fn add_transaction(&self){
        let transaction_counter = self.get_transaction_counter();
        let counter = transaction_counter + 1;
        self.set_transaction_counter(&counter);
    }

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> Address;
    
    #[view(getAcceptedPaymentToken)]
    #[storage_mapper("acceptedPaymentTokenId")]
    fn accepted_payment_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &Address) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getCounter)]
    #[storage_get("counter")]
    fn get_counter(&self) -> i64;

    #[storage_set("counter")]
    fn set_counter(&self, sum: &i64);

    #[view(getTransactionCount)]
    #[storage_get("transactionCount")]
    fn get_transaction_counter(&self) -> u64;

    #[storage_set("transactionCount")]
    fn set_transaction_counter(&self, sum: &u64);
    
    // storage
    #[storage_set("owner")]
    fn set_owner(&self, address: &Address);
    
}


