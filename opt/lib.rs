#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod opt {
    use ink::storage::Mapping;
    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the user is already opted-in.
        UserAlreadyOptedIn,
        /// Returned if the user does not already exist in the map of opted-in
        /// users
        UserDoesNotExist,
    }
    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Opt {
        /// Stores all the opted-in users
        /// If the user AccountID is not in the map it is assumed that they are not opted-in
        // TODO: Remove pub
        pub opted_in_users: Mapping<AccountId, bool>,
    }

    impl Opt {
        /// Constructor that initializes that creates an empty map for the mechanism.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                opted_in_users: Mapping::new(),
            }
        }
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn opt_in(&mut self) -> Result<()> {
            let caller = self.env().caller();
            ink::env::debug_println!( "caller is: {:#?}", caller );
            if self.opted_in_users.contains(&caller) {
                return Err(Error::UserAlreadyOptedIn);
            }
            self.opted_in_users.insert(caller, &true);
            ink::env::debug_println!( "caller opt-in successful" );
            self.env().emit_event(UserOptedIn { user: caller });
            Ok(())
        }

        #[ink(message)]
        pub fn opt_out(&mut self)  -> Result<()>{
            let caller = self.env().caller();
            if !self.opted_in_users.contains(&caller) {
                return Err(Error::UserDoesNotExist);
            };
            let _ = self.opted_in_users.remove(caller);
            self.env().emit_event(UserOptedOut { user: caller });
            Ok(())
        }

        #[ink(message)]
        pub fn is_user_opted_in(&self) -> bool {
            let caller = self.env().caller();
            self.opted_in_users.get(&caller).unwrap_or(false)
        }
        #[ink(message)]
        pub fn get(&self) -> bool {
            let caller = self.env().caller();
            self.opted_in_users.get(&caller).unwrap_or(false)
        }
    }

    #[ink(event)]
    pub struct UserOptedIn {
        #[ink(topic)]
        user: AccountId,
    }

    #[ink(event)]
    pub struct UserOptedOut {
        #[ink(topic)]
        user: AccountId,
    }
}

/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
/// module and test functions are marked with a `#[test]` attribute.
/// The below code is technically just normal Rust code.
#[cfg(test)]
mod tests {
    use crate::opt::Opt;
    /// We test if the default constructor does its job.
    #[ink::test]
    fn default_works() {
        let mut opt = Opt::default();
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        // Before opting in, should return false
        assert_eq!(opt.get(), false);
        opt.opted_in_users.insert(accounts.alice, &true); // Directly inserting for test
        assert_eq!(opt.get(), true);

        // Mimic opt-out and check again
        opt.opted_in_users.remove(accounts.alice);
        assert_eq!(opt.get(), false);
    }
}
