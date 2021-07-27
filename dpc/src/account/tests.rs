// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod testnet1 {
    use crate::{testnet1::parameters::Testnet1Parameters, Account, AccountScheme, Address, PrivateKey, ViewKey};

    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use std::{convert::TryInto, str::FromStr};

    const ALEO_TESTNET1_PRIVATE_KEY: &str = "APrivateKey1xittpFSFfHsTqgAVujN2PXsyNz7Gvt3fK9hEB9RQkeHUHM4";
    const ALEO_TESTNET1_VIEW_KEY: &str = "AViewKey1cW7MLCbtLzz7esFi7VrKXzd52BukX5xB6x8kCQoReuQv";
    const ALEO_TESTNET1_ADDRESS: &str = "aleo18wlwzpp04dvtra4fe9yk5xr2aq3kwcrwdepx4s3q2vvrp3tg6yqqxhjevz";

    const ITERATIONS: usize = 25;

    #[test]
    fn test_account_new() {
        let mut rng = ChaChaRng::seed_from_u64(1231275789u64);

        // Check the seeded derivation matches the hardcoded value, as a sanity check.
        let account = Account::<Testnet1Parameters>::new(&mut rng).unwrap();
        assert_eq!(ALEO_TESTNET1_PRIVATE_KEY, account.private_key.to_string());
        assert_eq!(ALEO_TESTNET1_VIEW_KEY, account.view_key.to_string());
        assert_eq!(ALEO_TESTNET1_ADDRESS, account.address.to_string());

        // Attempt to sample for a new account ITERATIONS times.
        for _ in 0..ITERATIONS {
            assert!(Account::<Testnet1Parameters>::new(&mut rng).is_ok());
        }
    }

    #[test]
    fn test_account_derivation() {
        let private_key = PrivateKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_PRIVATE_KEY).unwrap();
        let view_key = ViewKey::<Testnet1Parameters>::from_private_key(&private_key).unwrap();
        let address = Address::<Testnet1Parameters>::from_private_key(&private_key).unwrap();

        assert_eq!(ALEO_TESTNET1_PRIVATE_KEY, private_key.to_string());
        assert_eq!(ALEO_TESTNET1_VIEW_KEY, view_key.to_string());
        assert_eq!(ALEO_TESTNET1_ADDRESS, address.to_string());
    }

    #[test]
    fn test_private_key_from_str() {
        let private_key = PrivateKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_PRIVATE_KEY).unwrap();
        assert_eq!(ALEO_TESTNET1_PRIVATE_KEY, private_key.to_string());
    }

    #[test]
    fn test_private_key_from_invalid_str() {
        assert!(PrivateKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_VIEW_KEY).is_err());
        assert!(PrivateKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_ADDRESS).is_err());
        assert!(PrivateKey::<Testnet1Parameters>::from_str("APrivateKey1abcdefghijklmnopqrstuvwxyz").is_err());
        assert!(PrivateKey::<Testnet1Parameters>::from_str("APrivateKey1").is_err());
        assert!(PrivateKey::<Testnet1Parameters>::from_str("").is_err());
    }

    #[test]
    fn test_private_key_into_view_key() {
        let private_key = PrivateKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_PRIVATE_KEY).unwrap();
        let view_key: ViewKey<_> = private_key.try_into().unwrap();
        assert_eq!(ALEO_TESTNET1_VIEW_KEY, view_key.to_string());
    }

    #[test]
    fn test_view_key_from_str() {
        let view_key = ViewKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_VIEW_KEY).unwrap();
        assert_eq!(ALEO_TESTNET1_VIEW_KEY, view_key.to_string());
    }

    #[test]
    fn test_view_key_from_invalid_str() {
        assert!(ViewKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_PRIVATE_KEY).is_err());
        assert!(ViewKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_ADDRESS).is_err());
        assert!(ViewKey::<Testnet1Parameters>::from_str("AViewKey1abcdefghijklmnopqrstuvwxyz").is_err());
        assert!(ViewKey::<Testnet1Parameters>::from_str("AViewKey1").is_err());
        assert!(ViewKey::<Testnet1Parameters>::from_str("").is_err());
    }

    #[test]
    fn test_private_key_into_address() {
        let private_key = PrivateKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_PRIVATE_KEY).unwrap();
        let address: Address<_> = private_key.try_into().unwrap();
        assert_eq!(ALEO_TESTNET1_ADDRESS, address.to_string());
    }

    #[test]
    fn test_view_key_into_address() {
        let view_key = ViewKey::<Testnet1Parameters>::from_str(ALEO_TESTNET1_VIEW_KEY).unwrap();
        let address: Address<_> = view_key.try_into().unwrap();
        assert_eq!(ALEO_TESTNET1_ADDRESS, address.to_string());
    }

    #[test]
    fn test_address_from_str() {
        let address = Address::<Testnet1Parameters>::from_str(ALEO_TESTNET1_ADDRESS).unwrap();
        assert_eq!(ALEO_TESTNET1_ADDRESS, address.to_string());
    }

    #[test]
    fn test_address_from_invalid_str() {
        assert!(Address::<Testnet1Parameters>::from_str(ALEO_TESTNET1_PRIVATE_KEY).is_err());
        assert!(Address::<Testnet1Parameters>::from_str(ALEO_TESTNET1_VIEW_KEY).is_err());
        assert!(Address::<Testnet1Parameters>::from_str("aleo1").is_err());
        assert!(Address::<Testnet1Parameters>::from_str("").is_err());
    }
}

#[cfg(test)]
mod testnet2 {
    use crate::{testnet2::parameters::Testnet2Parameters, Account, AccountScheme, Address, PrivateKey, ViewKey};

    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use std::{convert::TryInto, str::FromStr};

    const ALEO_TESTNET2_PRIVATE_KEY: &str = "APrivateKey1xittpFSFfHsTqgAVujN2PXsyNz7Gvt3fK9hEB9RQkeHUHM4";
    const ALEO_TESTNET2_VIEW_KEY: &str = "AViewKey1cW7MLCbtLzz7esFi7VrKXzd52BukX5xB6x8kCQoReuQv";
    const ALEO_TESTNET2_ADDRESS: &str = "aleo18wlwzpp04dvtra4fe9yk5xr2aq3kwcrwdepx4s3q2vvrp3tg6yqqxhjevz";

    const ITERATIONS: usize = 25;

    #[test]
    fn test_account_new() {
        let mut rng = ChaChaRng::seed_from_u64(1231275789u64);

        // Check the seeded derivation matches the hardcoded value, as a sanity check.
        let account = Account::<Testnet2Parameters>::new(&mut rng).unwrap();
        assert_eq!(ALEO_TESTNET2_PRIVATE_KEY, account.private_key.to_string());
        assert_eq!(ALEO_TESTNET2_VIEW_KEY, account.view_key.to_string());
        assert_eq!(ALEO_TESTNET2_ADDRESS, account.address.to_string());

        // Attempt to sample for a new account ITERATIONS times.
        for _ in 0..ITERATIONS {
            assert!(Account::<Testnet2Parameters>::new(&mut rng).is_ok());
        }
    }

    #[test]
    fn test_account_derivation() {
        let private_key = PrivateKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_PRIVATE_KEY).unwrap();
        let view_key = ViewKey::<Testnet2Parameters>::from_private_key(&private_key).unwrap();
        let address = Address::<Testnet2Parameters>::from_private_key(&private_key).unwrap();

        assert_eq!(ALEO_TESTNET2_PRIVATE_KEY, private_key.to_string());
        assert_eq!(ALEO_TESTNET2_VIEW_KEY, view_key.to_string());
        assert_eq!(ALEO_TESTNET2_ADDRESS, address.to_string());
    }

    #[test]
    fn test_private_key_from_str() {
        let private_key = PrivateKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_PRIVATE_KEY).unwrap();
        assert_eq!(ALEO_TESTNET2_PRIVATE_KEY, private_key.to_string());
    }

    #[test]
    fn test_private_key_from_invalid_str() {
        assert!(PrivateKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_VIEW_KEY).is_err());
        assert!(PrivateKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_ADDRESS).is_err());
        assert!(PrivateKey::<Testnet2Parameters>::from_str("APrivateKey1abcdefghijklmnopqrstuvwxyz").is_err());
        assert!(PrivateKey::<Testnet2Parameters>::from_str("APrivateKey1").is_err());
        assert!(PrivateKey::<Testnet2Parameters>::from_str("").is_err());
    }

    #[test]
    fn test_private_key_into_view_key() {
        let private_key = PrivateKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_PRIVATE_KEY).unwrap();
        let view_key: ViewKey<_> = private_key.try_into().unwrap();
        assert_eq!(ALEO_TESTNET2_VIEW_KEY, view_key.to_string());
    }

    #[test]
    fn test_view_key_from_str() {
        let view_key = ViewKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_VIEW_KEY).unwrap();
        assert_eq!(ALEO_TESTNET2_VIEW_KEY, view_key.to_string());
    }

    #[test]
    fn test_view_key_from_invalid_str() {
        assert!(ViewKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_PRIVATE_KEY).is_err());
        assert!(ViewKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_ADDRESS).is_err());
        assert!(ViewKey::<Testnet2Parameters>::from_str("AViewKey1abcdefghijklmnopqrstuvwxyz").is_err());
        assert!(ViewKey::<Testnet2Parameters>::from_str("AViewKey1").is_err());
        assert!(ViewKey::<Testnet2Parameters>::from_str("").is_err());
    }

    #[test]
    fn test_private_key_into_address() {
        let private_key = PrivateKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_PRIVATE_KEY).unwrap();
        let address: Address<_> = private_key.try_into().unwrap();
        assert_eq!(ALEO_TESTNET2_ADDRESS, address.to_string());
    }

    #[test]
    fn test_view_key_into_address() {
        let view_key = ViewKey::<Testnet2Parameters>::from_str(ALEO_TESTNET2_VIEW_KEY).unwrap();
        let address: Address<_> = view_key.try_into().unwrap();
        assert_eq!(ALEO_TESTNET2_ADDRESS, address.to_string());
    }

    #[test]
    fn test_address_from_str() {
        let address = Address::<Testnet2Parameters>::from_str(ALEO_TESTNET2_ADDRESS).unwrap();
        assert_eq!(ALEO_TESTNET2_ADDRESS, address.to_string());
    }

    #[test]
    fn test_address_from_invalid_str() {
        assert!(Address::<Testnet2Parameters>::from_str(ALEO_TESTNET2_PRIVATE_KEY).is_err());
        assert!(Address::<Testnet2Parameters>::from_str(ALEO_TESTNET2_VIEW_KEY).is_err());
        assert!(Address::<Testnet2Parameters>::from_str("aleo1abcdefghijklmnopqrstuvwxyz").is_err());
        assert!(Address::<Testnet2Parameters>::from_str("aleo1").is_err());
        assert!(Address::<Testnet2Parameters>::from_str("").is_err());
    }
}
