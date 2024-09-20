use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_to_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_to_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum address"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_to_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_to_address().unwrap();
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_poly() {
        let addr: Address = Address::from_str("0x71c7656ec7ab88b098defb751b7401b5f6d8976f").unwrap();

        let new_addr: Address = get_ethereum_data(addr);
        assert_eq!(
            addr,
            Address::from_str("0x71c7656ec7ab88b098defb751b7401b5f6d8976f").unwrap()
        );

        let new_addr: Address = get_ethereum_data("0x71c7656ec7ab88b098defb751b7401b5f6d8976f");
        assert_eq!(
            new_addr,
            Address::from_str("0x71c7656ec7ab88b098defb751b7401b5f6d8976f").unwrap()
        );
    }
}
