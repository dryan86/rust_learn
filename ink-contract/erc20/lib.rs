#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_suply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowance: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BalanceTooLow,
        AllowanceTooLow,
    }

    type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_suply: u128) -> Self {
            let mut balances = Mapping::new();

            balances.insert(Self::env().caller(), &total_suply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(Self::env().caller()),
                value: total_suply,
            });

            Self {
                total_suply,
                balances,
                ..Default::default()
            }
        }

        #[ink(message)]
        pub fn total_suply(&self) -> Balance {
            self.total_suply
        }

        #[ink(message)]
        pub fn allowance_of(&self, from: AccountId, to: AccountId) -> Balance {
            self.allowance.get(&(from, to)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            return self.transfer_helper(&sender, &to, value);
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let sender = self.env().caller();
            let allowance = self.allowance.get(&(from, sender)).unwrap_or_default();

            if allowance < value {
                return Err(Error::AllowanceTooLow);
            }

            self.allowance.insert(&(from, sender), &(allowance - value));

            return self.transfer_helper(&sender, &to, value);
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            self.allowance.insert(&(sender, spender), &value);
            self.env().emit_event(Approval {
                from: sender,
                to: spender,
                value,
            });

            Ok(())
        }

        pub fn transfer_helper(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);

            if balance_from < value {
                return Err(Error::BalanceTooLow);
            }

            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn constructor_works() {
            let erc20 = Erc20::new(100001);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(erc20.total_suply, 100001);
            assert_eq!(erc20.balance_of(accounts.alice), 100001);

            let emitted_event = ink::env::test::recorded_events().collect::<Vec<>>();
            let event = emitted_event[0];
            let decoded = <Event as scale::Decode>::decode(&mut &event.data[..]).expect("decoded error");
            match decoded {
                Event::Transfer(Transfer { from, to, value }) => {
                    assert_eq!(value, 100001);
                    assert!(from.is_none(), "mint form error");
                    assert_eq!(to, Some(accounts.alice));
                },
                _ => panic!("unexpected event"),
            }
        }

        #[ink::test]
        fn it_works() {
            let mut erc20 = Erc20::new(false);
            assert_eq!(erc20.get(), false);
            erc20.flip();
            assert_eq!(erc20.get(), true);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        use ink_e2e::{build_message, subxt::client};

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let total_supply = 1234;
            let constructor = Erc20Ref::new(total_supply);

            let contract_account_id = client
                .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let alice_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let transfer_msg = build_message::<Erc20Ref>(contract_account_id.clone()).call(|erc20|erc20.transfer(bob_acc.clone(), 100));
            let res = client.call(&alice_acc, transfer_msg, 0, None).await;
            assert!(res.is_ok());
            let balance_of_msg = build_message::<Erc20Ref>(contract_account_id.clone()).call(|erc20|erc20.balance_of(alice_acc.clone));
            let balance_of_alice = client.call_dry_run(&alice_acc, &balance_of_msg, 0, None).await;

            assert_eq!(balance_of_alice.return_value(), 1134);
            Ok(())
        }
    }
}
