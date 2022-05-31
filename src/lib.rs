//! This contract implements simple leaflink backed by storage on blockchain.
//!
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, require, PanicOnDefault, Timestamp};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Education {
    pub school: String,
    pub college: Option<String>,
    pub major: Option<String>,
    pub started_at: Option<Timestamp>, // education start time
    pub ended_at: Option<Timestamp>,   // education end time
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
    pub company: String,
    pub position: Option<String>,
    pub desc: Option<String>,
    pub started_at: Option<Timestamp>, // job start time
    pub ended_at: Option<Timestamp>,   // job end time
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Comment {
    pub commentator: AccountId,
    pub detail: String,
    pub update_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTInfo {
    pub contract_id: AccountId,
    pub token_id: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Profile {
    pub owner_id: AccountId,
    pub avatar: Option<String>,
    pub nfts: UnorderedSet<NFTInfo>,
    pub tags: UnorderedSet<String>,
    pub educations: UnorderedSet<Education>,
    pub jobs: UnorderedSet<Job>,
    pub poaps: UnorderedSet<NFTInfo>,
    pub comments: UnorderedSet<Comment>,
    pub following: UnorderedSet<AccountId>,
    pub follow_by: UnorderedSet<AccountId>,
    pub last_update_at: Option<Timestamp>,
}

#[near_bindgen]
impl Profile {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            avatar: None,
            nfts: UnorderedSet::new(b"a".to_vec()),
            tags: UnorderedSet::new(b"b".to_vec()),
            educations: UnorderedSet::new(b"c".to_vec()),
            jobs: UnorderedSet::new(b"d".to_vec()),
            poaps: UnorderedSet::new(b"e".to_vec()),
            comments: UnorderedSet::new(b"f".to_vec()),
            following: UnorderedSet::new(b"g".to_vec()),
            follow_by: UnorderedSet::new(b"h".to_vec()),
            last_update_at: None,
        }
    }

    pub fn get_owner_id(&self) -> AccountId {
        return self.owner_id.clone();
    }

    pub fn add_avatar(&mut self, url: String) {
        require!(
            self.owner_id == env::predecessor_account_id(),
            "Owner's method"
        );
        self.avatar = Some(url);
    }

    pub fn get_avatar(&self) -> Option<String> {
        return self.avatar.clone();
    }

    pub fn add_nft(&mut self, nft: NFTInfo) {
        require!(
            self.owner_id == env::predecessor_account_id(),
            "Owner's method"
        );
        self.nfts.insert(&nft);
    }

    pub fn get_nfts(&self) -> Vec<NFTInfo> {
        return self.nfts.to_vec();
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(&tag);
    }

    pub fn get_tags(&self) -> Vec<String> {
        return self.tags.to_vec();
    }

    pub fn add_education(&mut self, edu: Education){
        require!(
            self.owner_id == env::predecessor_account_id(),
            "Owner's method"
        );
        self.educations.insert(&edu);
    }

    pub fn get_educations(&self) -> Vec<Education>{
        return self.educations.to_vec();
    }
    
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_create_profile() {
        let context = get_context(false);
        testing_env!(context.build());
        // Init contract
        let contract = Profile::new(accounts(0));
        let owner_id = contract.get_owner_id();
        assert_eq!(accounts(0), owner_id);
    }

    #[test]
    fn test_avatar() {
        let context = get_context(false);
        testing_env!(context.build());
        // Init contract
        let mut contract = Profile::new(accounts(0));
        let url = String::from("http://test.com/xxx.jpg");
        let url2 = url.clone();
        contract.add_avatar(url);
        let query_url_option = contract.get_avatar();
        if let Some(query_url) = query_url_option {
            assert_eq!(url2, query_url);
        }
    }

    #[test]
    #[should_panic(expected = "Owner's method")]
    fn test_nfts() {
        let context = get_context(false);
        testing_env!(context.build());

        // Init contract
        let mut contract = Profile::new(accounts(0));
        let nft = NFTInfo {
            contract_id: accounts(1),
            token_id: "0".to_string(),
        };
        let nfts = contract.get_nfts();
        assert_eq!(nfts.len(), 0);
        contract.add_nft(nft);
        let nfts = contract.get_nfts();
        assert_eq!(nfts.len(), 1);

        let nft = NFTInfo {
            contract_id: accounts(1),
            token_id: "0".to_string(),
        };
        let mut contract2 = Profile::new(accounts(1));
        contract2.add_nft(nft);
    }

    #[test]
    fn test_tags() {
        let context = get_context(false);
        testing_env!(context.build());

        // Init contract
        let mut contract = Profile::new(accounts(0));
        let tag = String::from("tag_name");
        let tags = contract.get_tags();
        assert_eq!(tags.len(), 0);
        contract.add_tag(tag);
        let tags = contract.get_tags();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "tag_name");
    }
}
