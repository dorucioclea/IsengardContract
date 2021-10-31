#![no_std]

elrond_wasm::imports!();

pub mod presale;
//pub mod sale;

use presale::*; // use all in auction?
//use sale::*; // use all in sale?

#[elrond_wasm::contract]
pub trait TokenSale {

    #[init]
    fn init(
        &self,
        start_time1: u64,
        end_time1: u64,
        start_time2: u64,
        end_time2: u64,
        start_time3: u64,
        end_time3: u64,
        egld_price1: BigUint,
        egld_price2: BigUint,
        egld_price3: BigUint,
        max_tokens1: BigUint,
        max_tokens2: BigUint,
        max_tokens3: BigUint,
        token_type : TokenIdentifier
    ) -> SCResult<()> {
        let owner_address: ManagedAddress = self.blockchain().get_caller();
        self.set_owner(&owner_address);

        let presale1 = Presale::new(
            &max_tokens1,
            &BigUint::zero(),
            start_time1,
            end_time1,
            egld_price1,
        );

        let presale2 = Presale::new(
            &max_tokens2,
            &BigUint::zero(),
            start_time2,
            end_time2,
            egld_price2,
        );

        let presale3 = Presale::new(
            &max_tokens3,
            &BigUint::zero(),
            start_time3,
            end_time3,
            egld_price3,
        );

        self.presale(1).set(&presale1);
        self.presale(2).set(&presale2);
        self.presale(3).set(&presale3);

        self.isengard_token().set(&token_type);
        // Maybe also create Token here

        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn fund(
        &self,
        #[payment] _payment: BigUint,
        #[payment_token] token_id: TokenIdentifier,
    ) -> SCResult<()> {
        
        require!(
            token_id == self.isengard_token().get(),
            "Invalid payment token"
        );

        Ok(())
    }
    

    #[only_owner]
    #[endpoint]
    fn set_presale3_price(&self, price: BigUint, ) -> SCResult<()> { 
        let mut presale = self.presale(3).get();
        presale.price = price;
        self.presale(3).set(&presale);

        
        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint]
    fn buy_presale1(&self,
        #[payment] payment: BigUint
    ) -> SCResult<()> {
        let caller: ManagedAddress = self.blockchain().get_caller();
        let multiplier = BigUint::from(1000000 as u64);

        require!(
            !caller.is_zero(),
            "Can't transfer to default address 0x0!"
        );

        require!(
            payment > BigUint::zero(),
            "Can't transfer to default address 0x0!"
        );

        let mut presale = self.presale(1).get();

        require!(
            self.blockchain().get_block_timestamp() > presale.start_time
                && self.blockchain().get_block_timestamp() < presale.end_time,
            "This presale is not ongoing."
        );

        let bought = &payment / &presale.price;

        require!(
            &presale.max_tokens - &presale.sold_tokens > bought,
                "Not enough tokens left in this presale to honor this order."
        );

        // Calculate 30% of that amount.
        let amount_to_send = (&payment / &presale.price) * (BigUint::from(30 as u64) * &multiplier) / (BigUint::from(100 as u64) * &multiplier);

        // Freeze rest of 70%.
        let to_freeze = (&payment / &presale.price) * (BigUint::from(70 as u64) * &multiplier) / (BigUint::from(100 as u64) * &multiplier);
        
        let frozen_tokens = UserTokens::new(
            &to_freeze,
            self.blockchain().get_block_timestamp()
        );
        self.frozen_presale3(&caller).set(&frozen_tokens);

        presale.sold_tokens+= (payment / &presale.price);
        self.presale(1).set(&presale);

        // Send 30% of the tokens to the user.
        Ok(self.transfer_to(caller, amount_to_send))
    }

    #[payable("EGLD")]
    #[endpoint]
    fn buy_presale2(&self,
        #[payment] payment: BigUint
    ) -> SCResult<()> {
        let caller: ManagedAddress = self.blockchain().get_caller();

        require!(
            !caller.is_zero(),
            "Can't transfer to default address 0x0!"
        );

        require!(
            payment > BigUint::zero() ,
            "Can't transfer to default address 0x0!"
        );

        let presale = self.presale(2).get();

        require!(
            self.blockchain().get_block_timestamp() > presale.start_time
                && self.blockchain().get_block_timestamp() < presale.end_time,
            "This presale is not ongoing."
        );

        Ok(())
    }


    //ESDTTransfer@495345542d636161313064@01c6bf52634000@66756e64
    #[payable("EGLD")]
    #[endpoint]
    fn buy_presale3(&self,
        #[payment] payment: BigUint
    ) -> SCResult<()> {
        let caller: ManagedAddress = self.blockchain().get_caller();

        require!(
            !caller.is_zero(),
            "Can't transfer to default address 0x0!"
        );

        require!(
            payment > BigUint::zero() ,
            "Can't transfer to default address 0x0!"
        );

        let presale = self.presale(3).get();

        require!(
            self.blockchain().get_block_timestamp() > presale.start_time
                && self.blockchain().get_block_timestamp() < presale.end_time,
            "This presale is not ongoing."
        );
        
        // Calculate amount of tokens bought.
        let bought = payment / presale.price;
        // Calculate 30% of that amount.
        let amount_to_send = (BigUint::from(30 as u64) / BigUint::from(100 as u64)) * &bought;
        // Freeze rest of 70%.
        let to_freeze = &bought - &amount_to_send;
        
        let frozen_tokens = UserTokens::new(
            &to_freeze,
            self.blockchain().get_block_timestamp()
        );
        self.frozen_presale3(&caller).set(&frozen_tokens);

        // Send 30% of the tokens to the user.
        Ok(self.transfer_to(caller, amount_to_send))
    }

    // TODO: Upgrade to set claimable tokens. 
    #[endpoint]
    fn claim_tokens(&self,
        
    )-> SCResult<()> {
        let caller: ManagedAddress = self.blockchain().get_caller();

        require!(
            !caller.is_zero(),
            "Can't transfer to default address 0x0!"
        );

        let amount = BigUint::zero();

        // Check if the user has these 
        // let presale1_tokens = self.frozen_presale1(&caller).get(); // this might panic?!
        // let presale2_tokens = self.frozen_presale2(&caller).get(); // this might panic?!
        // let presale3_tokens = self.frozen_presale3(&caller).get(); // this might panic?!

        // Calculate how many tokens user can claim and add them to tokens.

        Ok(self.transfer_to(caller, amount))
        // Send tokens to the user
    }

    
    // private
    fn transfer_to(&self, 
        address: ManagedAddress, 
        amount: BigUint
    ){
        let token_id = self.isengard_token().get();
        self.send().direct(&address, &token_id, 0, &amount , b"retrieve successful");
    }

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> ManagedAddress;

    #[storage_set("owner")]
    fn set_owner(&self, address: &ManagedAddress);   

    #[view(getPresale)]
    #[storage_mapper("presale")]
    fn presale(&self, presale_no: i32) -> SingleValueMapper<Presale<Self::Api>>;

    // #[storage_mapper("frozen_tokens")]
    // fn frozen_tokens(&self, address : &ManagedAddress) -> VecMapper<UserTokens<Self::Api>>;

    #[storage_mapper("frozen_presale1")]
    fn frozen_presale1(&self, address : &ManagedAddress) -> SingleValueMapper<UserTokens<Self::Api>>;

    #[storage_mapper("frozen_presale2")]
    fn frozen_presale2(&self, address : &ManagedAddress) -> SingleValueMapper<UserTokens<Self::Api>>;
    
    #[storage_mapper("frozen_presale3")]
    fn frozen_presale3(&self, address : &ManagedAddress) -> SingleValueMapper<UserTokens<Self::Api>>;

    #[storage_mapper("isengard_token")]
    fn isengard_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("isengard_team")]
    fn team_members(&self) -> VecMapper<ManagedAddress>;
}


