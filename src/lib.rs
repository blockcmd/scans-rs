use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanSingleAddressEtherBalanceApiResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanMultipleAddressesEtherBalancesApiResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<AccountsBalances>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanListNormalTransactionsByAddressApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanNormalTransaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanNormalTransaction {
	pub blockNumber: String,
	pub timeStamp: String,
	pub hash: String,
	pub nonce: String,
	pub blockHash: String,
	pub transactionIndex: String,
	pub from: String,
	pub to: String,
	pub value: String,
	pub gas: String,
	pub gasPrice: String,
	pub isError: String,
	pub txreceipt_status: String,
	pub input: String,
	pub contractAddress: String,
	pub cumulativeGasUsed: String,
	pub gasUsed: String,
	pub confirmations: String,
	pub methodId: String,
	pub functionName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanListInternalTransactionsByAddressApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanInternalTransaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanInternalTransaction {
	pub blockNumber: String,
	pub timeStamp: String,
	pub hash: String,
	pub from: String,
	pub to: String,
	pub value: String,
	pub contractAddress: String,
	pub input: String,
	#[serde(rename = "type")]
	pub type_: String,
	pub gas: String,
	pub gasUsed: String,
	pub traceId: String,
	pub isError: String,
	pub errCode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanListInternalTransactionsByTransactionHashApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanInternalTransaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanInternalTransactionWithoutHash {
	pub blockNumber: String,
	pub timeStamp: String,
	pub from: String,
	pub to: String,
	pub value: String,
	pub contractAddress: String,
	pub input: String,
	#[serde(rename = "type")]
	pub type_: String,
	pub gas: String,
	pub gasUsed: String,
	pub isError: String,
	pub errCode: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountsBalances {
    pub account: String,
    pub balance: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct SolanaFmApiRequest {
    pub accountHashes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolanaFmApiResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<SolanaFmApiResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct SolanaFmApiResult {
    pub accountHash: String,
    pub onchain: SolanaFmDataOnchain,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct SolanaFmDataOnchain {
    pub lamports: u128,
    pub data: Vec<String>,
    pub owner: String,
    pub executable: bool,
    pub rentEpoch: u128,
}

pub struct Scans {
    api_keys: String,
}

#[allow(dead_code)]
impl Scans {
    const BASE_ETHERSCAN_API_URL: &'static str = "https://api.etherscan.io/api";
    const BASE_ARBISCAN_API_URL: &'static str = "https://api.arbiscan.io/api";
    const BASE_OPTIMISM_API_URL: &'static str = "https://api-optimistic.etherscan.io/api";
    const BASE_ZKSYNCSCAN_API_URL: &'static str = "https://api-era.zksync.network/api";
    const BASE_SCROLLSCAN_API_URL: &'static str = "https://api.scrollscan.com/api";
    const BASE_SOLANA_FM_API_URL: &'static str = "https://api.solana.fm/";
    const BASE_COINGECKO_API_URL: &'static str = "https://api.coingecko.com/api/v3";

    // Constructor function
    pub fn new(api_keys: String) -> Self {
        Scans { api_keys }
    }

    fn select_chain(chain_id: &str) -> &str {
        match chain_id {
            "1" => Self::BASE_ETHERSCAN_API_URL,
            "10" => Self::BASE_OPTIMISM_API_URL,
            "42161" => Self::BASE_ARBISCAN_API_URL,
            "324" => Self::BASE_ZKSYNCSCAN_API_URL,
            "534352" => Self::BASE_SCROLLSCAN_API_URL,
            _ => Self::BASE_ETHERSCAN_API_URL, // Default to BASE_ETHERSCAN_API_URL
        }
    }

    fn select_explorer(explorer_id: &str) -> &str {
        match explorer_id {
            "etherscan" => Self::BASE_ETHERSCAN_API_URL,
            "arbiscan" => Self::BASE_ARBISCAN_API_URL,
            "optimism" => Self::BASE_OPTIMISM_API_URL,
            "zksync" => Self::BASE_ZKSYNCSCAN_API_URL,
            "scroll" => Self::BASE_SCROLLSCAN_API_URL,
            "solana-fm" => Self::BASE_SOLANA_FM_API_URL,
            _ => Self::BASE_ETHERSCAN_API_URL, // Default to BASE_ETHERSCAN_API_URL
        }
    }

    // ETHERSCAN - ACCOUNTS MODULE
    pub async fn get_account_balance(
        &self,
        chain_id: &str,
        address: &str,
    ) -> Result<EtherscanSingleAddressEtherBalanceApiResponse, reqwest::Error> {
        let url: String = format!(
            "{}?module=account&action=balance&address={}&tag=latest&apikey={}",
            Self::select_chain(chain_id),
            address,
            self.api_keys
        );
        let res = reqwest::get(&url)
            .await?
            .json::<EtherscanSingleAddressEtherBalanceApiResponse>()
            .await?;
        Ok(res.clone())
    }

    pub async fn get_multiple_accounts_balances(
        &self,
        chain_id: &str,
        addresses: &str,
    ) -> Result<EtherscanMultipleAddressesEtherBalancesApiResponse, reqwest::Error> {
        let url: String = format!(
            "{}?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
            Self::select_chain(chain_id),
            addresses,
            self.api_keys
        );
        let res = reqwest::get(&url).await?.json::<EtherscanMultipleAddressesEtherBalancesApiResponse>().await?;
        Ok(res.clone())
    }

	pub async fn get_normal_transactions_by_address(
		&self,
		chain_id: &str,
		address: &str,
		startblock: i128,
		endblock: i128,
		page: i128,
		offset: i128,
		sort: &str,
	) -> Result<EtherscanListNormalTransactionsByAddressApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=txlist&address={}&startblock={}&endblock={}&page={}&offset={}&sort={}&apikey={}",
			Self::select_chain(chain_id),
			address,
			startblock,
			endblock,
			page,
			offset,
			sort,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListNormalTransactionsByAddressApiResponse>().await?;
		Ok(res.clone())
	}

	pub async fn get_internal_transactions_by_address(
		&self,
		chain_id: &str,
		address: &str,
		startblock: i128,
		endblock: i128,
		page: i128,
		offset: i128,
		sort: &str,
	) -> Result<EtherscanListInternalTransactionsByAddressApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&page={}&offset={}&sort={}&apikey={}",
			Self::select_chain(chain_id),
			address,
			startblock,
			endblock,
			page,
			offset,
			sort,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListInternalTransactionsByAddressApiResponse>().await?;
		Ok(res.clone())
	}

	pub async fn get_internal_transactions_by_tx_hash(
		&self,
		chain_id: &str,
		tx_hash: &str,
	) -> Result<EtherscanListInternalTransactionsByTransactionHashApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=txlistinternal&txhash={}&apikey={}",
			Self::select_chain(chain_id),
			tx_hash,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListInternalTransactionsByTransactionHashApiResponse>().await?;
		Ok(res.clone())
	}

	pub async fn get_internal_transactions_by_block_range(
		&self,
		chain_id: &str,
		tx_hash: &str,
	) -> Result<EtherscanListInternalTransactionsByTransactionHashApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=txlistinternal&txhash={}&apikey={}",
			Self::select_chain(chain_id),
			tx_hash,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListInternalTransactionsByTransactionHashApiResponse>().await?;
		Ok(res.clone())
	}

	// ETHERSCAN - TOKENS MODULE
    pub async fn get_token_balance(
        &self,
        chain_id: &str,
        address: &str,
        token_address: &str,
    ) -> Result<String, reqwest::Error> {
        let url: String = format!("{}?module=account&action=tokenbalance&contractaddress={}&address={}&tag=latest&apikey={}", Self::select_chain(chain_id), token_address, address, self.api_keys);
        let res = reqwest::get(&url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        Ok(res.get("result").unwrap().clone())
    }

    // fetch the current price of a cryptocurrency in USD from the Coingecko API
    pub async fn fetch_price(&self, pair: &str) -> Result<f64, reqwest::Error> {
        let url = format!(
            "{}/simple/price?ids={}&vs_currencies=usd",
            Self::BASE_COINGECKO_API_URL, pair
        );
        let res = reqwest::get(&url)
            .await?
            .json::<HashMap<String, HashMap<String, f64>>>()
            .await?;
        Ok(res.get(pair).unwrap().get("usd").unwrap().clone())
    }

    pub async fn get_solana_multiple_accounts(
        &self,
        addresses: Vec<String>,
    ) -> Result<SolanaFmApiResponse, reqwest::Error> {
        let url: String = format!("{}v0/accounts", Self::BASE_SOLANA_FM_API_URL);
        let data: SolanaFmApiRequest = SolanaFmApiRequest {
            accountHashes: addresses,
        };
        let res = reqwest::Client::new()
            .post(&url)
            .json(&data)
            .send()
            .await?
            .json::<SolanaFmApiResponse>()
            .await?;
        Ok(res.clone())
    }
}