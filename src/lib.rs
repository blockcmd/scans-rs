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
	pub result: Vec<EtherscanInternalTransactionWithoutHash>,
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
#[allow(non_snake_case)]
pub struct EtherscanInternalTransactionByBlockRangeApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanInternalTransactionInBlockRange>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanInternalTransactionInBlockRange {
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
	pub errCode: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanListErc20TokenTransfersByAddressApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanErc20TokenTransferEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanErc20TokenTransferEvents {
	pub blockNumber: String,
	pub timeStamp: String,
	pub hash: String,
	pub nonce: String,
	pub blockHash: String,
	pub from: String,
	pub contractAddress: String,
	pub to: String,
	pub value: String,
	pub tokenName: String,
	pub tokenSymbol: String,
	pub tokenDecimal: String,
	pub transactionIndex: String,
	pub gas: String,
	pub gasPrice: String,
	pub gasUsed: String,
	pub cumulativeGasUsed: String,
	pub input: String,
	pub confirmations: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanListErc721TokenTransfersByAddressApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanErc721TokenTransferEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanErc721TokenTransferEvents {
	pub blockNumber: String,
	pub timeStamp: String,
	pub hash: String,
	pub nonce: String,
	pub blockHash: String,
	pub from: String,
	pub contractAddress: String,
	pub to: String,
	pub tokenID: String,
	pub tokenName: String,
	pub tokenSymbol: String,
	pub tokenDecimal: String,
	pub transactionIndex: String,
	pub gas: String,
	pub gasPrice: String,
	pub gasUsed: String,
	pub cumulativeGasUsed: String,
	pub input: String,
	pub confirmations: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanListErc1155TokenTransfersByAddressApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanErc1155TokenTransferEvents>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanErc1155TokenTransferEvents {
	pub blockNumber: String,
	pub timeStamp: String,
	pub hash: String,
	pub nonce: String,
	pub blockHash: String,
	pub transactionIndex: String,
	pub gas: String,
	pub gasPrice: String,
	pub gasUsed: String,
	pub cumulativeGasUsed: String,
	pub input: String,
	pub contractAddress: String,
	pub from: String,
	pub to: String,
	pub tokenID: String,
	pub tokenValue: String,
	pub tokenName: String,
	pub tokenSymbol: String,
	pub confirmations: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EtherscanListBlocksValidatedByAddressApiResponse {
	pub status: String,
	pub message: String,
	pub result: Vec<EtherscanBlockValidated>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct EtherscanBlockValidated {
	pub blockNumber: String,
	pub timeStamp: String,
	pub blockReward: String,
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

    /// ETHERSCAN - ACCOUNTS MODULE
	/// Get Ether Balance for a Single Address
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

	/// Get Ether Balance for Multiple Addresses in a Single Call
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

	/// Get a list of 'Normal' Transactions By Address
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

	/// Get a list of 'Internal' Transactions by Address
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

	/// Get 'Internal Transactions' by Transaction Hash
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

	/// Get "Internal Transactions" by Block Range
	pub async fn get_internal_transactions_by_block_range(
		&self,
		chain_id: &str,
		page: i128,
		offset: i128,
		startblock: i128,
		endblock: i128,
		sort: &str,
	) -> Result<EtherscanInternalTransactionByBlockRangeApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=txlistinternal&startblock={}&endblock={}&page={}&offset={}&sort={}&apikey={}",
			Self::select_chain(chain_id),
			page,
			offset,
			startblock,
			endblock,
			sort,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanInternalTransactionByBlockRangeApiResponse>().await?;
		Ok(res.clone())
	}

	/// Get a list of 'ERC20 - Token Transfer Events' by Address
	pub async fn get_erc20_transfer_events_by_address(
		&self,
		chain_id: &str,
		address: &str,
		contractaddress: &str,
		page: i128,
		offset: i128,
		startblock: i128,
		endblock: i128,
		sort: &str,
	) -> Result<EtherscanListErc20TokenTransfersByAddressApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=tokentx&contractaddress={}&address={}&page={}&offset={}&startblock{}&endblock={}&sort={}&apikey={}",
			Self::select_chain(chain_id),
			contractaddress,
			address,
			page,
			offset,
			startblock,
			endblock,
			sort,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListErc20TokenTransfersByAddressApiResponse>().await?;
		Ok(res.clone())
	}

	/// Get a list of 'ERC721 - Token Transfer Events' by Address
	pub async fn get_erc721_transfer_events_by_address(
		&self,
		chain_id: &str,
		address: &str,
		contractaddress: &str,
		page: i128,
		offset: i128,
		startblock: i128,
		endblock: i128,
		sort: &str,
	) -> Result<EtherscanListErc721TokenTransfersByAddressApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=tokennfttx&contractaddress={}&address={}&page={}&offset={}&startblock{}&endblock={}&sort={}&apikey={}",
			Self::select_chain(chain_id),
			contractaddress,
			address,
			page,
			offset,
			startblock,
			endblock,
			sort,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListErc721TokenTransfersByAddressApiResponse>().await?;
		Ok(res.clone())
	}

	/// Get a list of 'ERC1155 - Token Transfer Events' by Address
	pub async fn get_erc1155_transfer_events_by_address(
		&self,
		chain_id: &str,
		address: &str,
		contractaddress: &str,
		page: i128,
		offset: i128,
		startblock: i128,
		endblock: i128,
		sort: &str,
	) -> Result<EtherscanListErc721TokenTransfersByAddressApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=token1155tx&contractaddress={}&address={}&page={}&offset={}&startblock{}&endblock={}&sort={}&apikey={}",
			Self::select_chain(chain_id),
			contractaddress,
			address,
			page,
			offset,
			startblock,
			endblock,
			sort,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListErc721TokenTransfersByAddressApiResponse>().await?;
		Ok(res.clone())
	}

	/// Get list of Blocks Validated by Address
	pub async fn get_blocks_validated_by_address(
		&self,
		chain_id: &str,
		address: &str,
		blocktype: &str,
		page: i128,
		offset: i128,
		sort: &str,
	) -> Result<EtherscanListBlocksValidatedByAddressApiResponse, reqwest::Error> {

		let url: String = format!(
			"{}?module=account&action=getminedblocks&address={}&blocktype={}&page={}&offset={}&sort={}&apikey={}",
			Self::select_chain(chain_id),
			address,
			blocktype,
			page,
			offset,
			sort,
			self.api_keys
		);
		let res = reqwest::get(&url).await?.json::<EtherscanListBlocksValidatedByAddressApiResponse>().await?;
		Ok(res.clone())
	}


	/// ETHERSCAN - TOKENS MODULE
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


	/// SOLANA FM - ACCOUNTS MODULE
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

	// COINGECKO
	pub async fn cg_ping(&self) -> Result<HashMap<String, String>, reqwest::Error> {
		let url = format!(
			"{}/ping",
			Self::BASE_COINGECKO_API_URL
		);
		let res = reqwest::get(&url)
			.await?
			.json::<HashMap<String, String>>()
			.await?;
		Ok(res.clone())
	}

	pub async fn fetch_price(&self, pair: &str) -> Result<HashMap<String, HashMap<String, f64>>, reqwest::Error> {
		let url = format!(
			"{}/simple/price?ids={}&vs_currencies=usd",
			Self::BASE_COINGECKO_API_URL, pair
		);
		let res = reqwest::get(&url)
			.await?
			.json::<HashMap<String, HashMap<String, f64>>>()
			.await?;
		Ok(res.clone())
	}
}