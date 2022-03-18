use sbor::*;
use scrypto::prelude::*;

#[derive(Debug, TypeId, Encode, Decode, Describe)]
pub struct Order {
    /// Order number (starting at 1)
    pub number: i64,
    /// True if this is a buy order, false if it is a sell order
    pub buy: bool,
    /// Kind of token that is being bought or sold
    pub token: ResourceDef,
    /// Price (in market's currency) the buyer is willing to bid or seller is asking
    pub price: Decimal,
    /// Amount of the purchased (or to be sold) tokens
    pub purse: Decimal,
    /// Amount of market currency held by this order for any purchases or sales which can be withdrawn
    pub payment: Decimal
}

#[allow(dead_code)]
impl Order {
    pub fn token_symbol(&self) -> String {
        self.token.metadata()["symbol"].clone()
    }

    pub fn is_filled(&self) -> bool {
        if self.buy {
            self.payment == 0.into() || self.payment < self.price
        } else {
            self.purse == 0.into()
        }
    }

    pub fn is_market_order(&self) -> bool {
        self.price == 0.into()
    }

    pub fn is_buy_order(&self) -> bool {
        self.buy
    }

    pub fn is_sell_order(&self) -> bool {
        !self.is_buy_order()
    }
}

#[derive(NonFungibleData)]
pub struct OrderTicket {
  pub order_number: i64,
  pub order_token_address: String,
  pub order_currency: String
}

#[derive(Debug, TypeId, Encode, Decode, Describe)]
pub struct MarketPrices {
    asset_prices: HashMap<String, Decimal>
}

#[allow(dead_code)]
impl MarketPrices {
    pub fn new() -> MarketPrices {
        MarketPrices { asset_prices: HashMap::new() }
    }

    pub fn assets(&self) -> Vec<String> {
        self.asset_prices.keys().cloned().collect()
    }

    pub fn get(&self, asset_symbol: String) -> Option<Decimal> {
        self.asset_prices.get(&asset_symbol).cloned()
    }

    pub fn for_address(&self, address: Address) -> Option<Decimal> {
        let resource_def = ResourceDef::from(address);
        let asset_symbol = resource_def.metadata()["symbol"].clone();

        self.get(asset_symbol)
    }

    pub fn update(&mut self, asset_symbol: String, price: Decimal) {
        self.asset_prices.insert(asset_symbol, price);
    }
}

#[derive(Debug, TypeId, Encode, Decode, Describe)]
pub struct TokenVaults {
    vaults: HashMap<Address, Vault>
}

#[allow(dead_code)]
impl TokenVaults {
    pub fn new() -> TokenVaults {
        TokenVaults { vaults: HashMap::new() }
    }

    pub fn get(&mut self, asset: &Address) -> &mut Vault {
        if !self.vaults.contains_key(asset) {
            let vault = Vault::new(*asset);

            self.vaults.insert(*asset, vault);
        }

        self.vaults.get_mut(asset).unwrap()
    }
}
