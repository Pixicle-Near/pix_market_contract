use near_sdk::json_types::U64;

use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        id: U64,
        token_id: TokenId,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) {
        let initial_storage_usage = env::storage_usage();

        // Get the series and how many tokens currently exist (edition number = cur_len + 1)
        let mut series = self.series_by_id.get(&id.0).expect("Not a series");

        //Change this when listing
        let mut royalty = HashMap::new();

        if let Some(perpetual_royalties) = perpetual_royalties {
            assert!(
                perpetual_royalties.len() < 7,
                "Cannot add more than 6 perpetual royalty amounts"
            );
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
            }
        }
        // finish up

        let token = Token {
            series_id: id.0,
            owner_id: receiver_id,
            approved_account_ids: Default::default(),
            next_approval_id: 0,
            royalty: royalty, //also this
        };

        assert!(
            self.token_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );
        series.tokens.insert(&token_id);

        self.token_metadata_by_id.insert(&token_id, &metadata);

        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }
}
