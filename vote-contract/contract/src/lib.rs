use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;

use near_contract_standards::non_fungible_token::Token;
use near_sdk::ext_contract;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::Gas;
use near_sdk::Promise;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseResult};
use std::collections::HashMap;
use std::convert::TryFrom;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Vote {
    nft_contract_id: AccountId,
    owner_id: AccountId,
    name: String,
    description: String,
    meta: String,
    status: u8,
    answers: Vector<String>,
    votes: HashMap<String, u8>,
}

#[derive(Serialize, Deserialize)]
pub struct VoteOutModel {
    nft_contract_id: AccountId,
    owner_id: AccountId,
    name: String,
    description: String,
    meta: String,
    status: u8,
    answers: Vec<String>,
    votes: HashMap<String, u8>,
}

#[ext_contract(ext_nft)]
pub trait NftContract {
    fn nft_token(&self, token_id: String);
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn update_total_balance_callback(&self);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    votes: HashMap<AccountId, Vector<Vote>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            votes: HashMap::new(),
        }
    }

    // The function is annotated with the #[payable] attribute, which means that it
    // can receive a payment along with the function call. In this case, the function
    // creates a new vote by accepting a deposit of at least 1 yoctoNEAR token.

    // The function takes several parameters, including the nft_contract_id which
    // is an Account ID representing the NFT contract where the vote is associated with.
    // The name, description, and meta parameters are strings that provide additional details
    // about the vote. The answers parameter is a vector of strings that contains the available answers for the vote.

    // The function creates a new Vote struct with the provided details and saves
    // it in a hashmap. The hashmap is indexed by the nft_contract_id to allow for
    // multiple votes for different NFT contracts. The votes field of the contract
    // contains a vector of votes for each NFT contract.

    // The function then returns a Promise that sends the attached deposit to the
    // p516entropy.near (creator) account. The Promise is a way to perform an action on the
    // blockchain asynchronously. In this case, the function is transferring
    // the attached deposit to the specified account.

    // Overall, this function allows for the creation of a vote pool for a specific
    // NFT contract, with the ability to specify multiple answers for the vote.
    // The payment required to create the vote pool ensures that only serious users
    // can create a new vote pool. Additionally, the use of a hashmap to store the
    // votes allows for efficient retrieval of the votes for a specific NFT contract.

    #[payable]
    pub fn create_vote(
        &mut self,
        nft_contract_id: AccountId,
        name: String,
        description: String,
        meta: String,
        answers: Vec<String>,
    ) -> Promise {
        let mut key = b"ans".to_vec();
        key.extend(self.votes.len().to_ne_bytes().to_vec());
        assert!(
            env::attached_deposit() > 1,
            "Need attach 1 NEAR to create vote pool"
        );

        let mut vote = Vote {
            owner_id: env::predecessor_account_id(),
            nft_contract_id: nft_contract_id.clone(),
            name: name,
            description: description,
            meta: meta,
            status: 0,
            answers: Vector::new(key),
            votes: HashMap::new(),
        };

        for a in answers {
            vote.answers.push(&a);
        }
        if self.votes.contains_key(&nft_contract_id) {
            self.votes.get_mut(&nft_contract_id).unwrap().push(&vote);
        } else {
            let mut contract_votes = Vector::new(nft_contract_id.as_bytes());
            contract_votes.push(&vote);
            self.votes.insert(nft_contract_id, contract_votes);
        }
        Promise::new(AccountId::try_from("p516entropy.near".to_string()).unwrap())
            .transfer(env::attached_deposit())
    }

    // get Vote json state

    pub fn get_vote(&self, contract_id: AccountId, index: u64) -> VoteOutModel {
        let vote = self.votes.get(&contract_id).unwrap().get(index).unwrap();
        let mut answers = Vec::new();
        for ans in vote.answers.iter() {
            answers.push(ans);
        }

        VoteOutModel {
            nft_contract_id: vote.nft_contract_id,
            owner_id: vote.owner_id,
            name: vote.name,
            description: vote.description,
            answers: answers,
            status: vote.status,
            votes: vote.votes,
            meta: vote.meta,
        }
    }

    // get list of Votes json state for contarct

    pub fn get_votes_by_contract(
        &self,
        contract_id: AccountId,
        limit: u64,
        offset: u64,
    ) -> Vec<VoteOutModel> {
        if self.votes.get(&contract_id).is_none() {
            return vec![];
        }
        let contract_votes = self.votes.get(&contract_id).unwrap();

        let mut result = Vec::new();
        for index in offset..offset + limit {
            if contract_votes.get(index).is_none() {
                break;
            }
            let vote = contract_votes.get(index).unwrap();
            let mut answers = Vec::new();
            for ans in vote.answers.iter() {
                answers.push(ans);
            }
            result.push(VoteOutModel {
                nft_contract_id: vote.nft_contract_id,
                owner_id: vote.owner_id,
                name: vote.name,
                description: vote.description,
                answers: answers,
                votes: vote.votes,
                status: vote.status,
                meta: vote.meta,
            });
        }
        result
    }

    // The function first creates a cross-contract call using the Promise::new()
    // method to call the nft_token function in the provided contract_id contract.
    // The nft_token function is passed a JSON object containing the token_id parameter
    // to check if the user owns the specific NFT token required to vote.
    // The function_call() method is called on the promise object to initiate
    // the cross-contract call. The then() method is used to chain another
    // promise after the first one completes.

    // If the first promise succeeds, the second promise calls the complete_vote
    // function in the current contract by using Promise::new() and function_call()
    // methods. The complete_vote function takes several parameters including account_id
    // (predecessor account), nft_token_id, answer, index, and contract_id.

    // The function is marked with the #[payable] attribute indicating that the
    // function can receive payments in NEAR tokens. However, no condition is given
    // regarding the required attached deposit to execute the function.

    // Overall, this function allows a user to vote on a specific poll by using
    // a specific NFT token. By using cross-contract calls, the function ensures
    // that the user actually owns the required NFT token before allowing them to vote.

    #[payable]
    pub fn vote(
        &mut self,
        contract_id: AccountId,
        index: u64,
        answer: u8,
        nft_token_id: String,
    ) -> Promise {
        // We use a cross-contract call to check if the voter has NFT
        Promise::new(contract_id.clone())
            .function_call(
                "nft_token".to_string(),
                json!({ "token_id": nft_token_id })
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                0,
                Gas(40_000_000_000_000),
            )
            .then(
                Promise::new(env::current_account_id()).function_call(
                    "complete_vote".to_string(),
                    json!({
                        "account_id": env::predecessor_account_id(),
                        "nft_token_id": nft_token_id,
                        "answer": answer,
                        "index": index,
                        "contract_id": contract_id
                    })
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                    0,
                    Gas(15_000_000_000_000),
                ),
            )
    }

    // The function first retrieves the vote by getting the Vector of votes associated
    // with the given contract_id and then the vote at the specified index. The retrieved
    // vote is then checked to ensure that the user executing the function is the owner
    // of the vote by comparing the owner account ID with the current predecessor_account_id().
    // If the comparison fails, an error message is returned.

    // If the comparison passes, the status of the retrieved vote is set to 1,
    // indicating that the vote has been closed. The vote is then replaced in the
    // vector of votes associated with the contract_id. Finally, the function logs a
    // message indicating that the vote has been closed.

    #[payable]
    pub fn close_vote(&mut self, contract_id: AccountId, index: u64) {
        let mut vote = self.votes.get(&contract_id).unwrap().get(index).unwrap();
        assert!(
            vote.owner_id == env::predecessor_account_id(),
            "User dont have access to this vote"
        );
        vote.status = 1;
        self.votes
            .get_mut(&contract_id)
            .unwrap()
            .replace(index, &vote);

        env::log_str(format!("Vote closed").as_str());
    }

    // The function first checks if there is exactly one promise result available,
    // which is expected since this function is called as a callback. If there are
    // multiple results, it will panic. If the promise failed, it will also panic.
    // If the promise is successful, the function will deserialize the result as a
    // Token object using near_sdk::serde_json::from_slice. The Token object contains
    // information about the NFT, including its owner.

    // The function then retrieves the vote specified by the contract_id and index
    // from the contract's internal state. It checks if the NFT specified by nft_token_id
    // has already been used to vote in this particular vote, and if it has, it will panic.
    // It then checks if the answer is within the bounds of the available answers for this vote.

    // If both checks pass, the function will update the vote by adding the NFT and the chosen
    // answer to the votes map of the vote. It then replaces the original vote with the updated
    // one in the internal state of the contract.

    #[private]
    pub fn complete_vote(
        &mut self,
        contract_id: AccountId,
        index: u64,
        answer: u8,
        nft_token_id: String,
        account_id: AccountId,
    ) {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        // handle the result from the cross contract call
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic_str("oops!"),
            PromiseResult::Successful(result) => {
                let token = near_sdk::serde_json::from_slice::<Token>(&result).unwrap();
                assert!(token.owner_id == account_id);

                let mut vote = self.votes.get(&contract_id).unwrap().get(index).unwrap();
                let mut votes = vote.votes;
                assert!(
                    !votes.contains_key(&nft_token_id),
                    "Vote already done for this nft"
                );
                assert!(answer < vote.answers.len() as u8, "Too big answer index");

                votes.insert(nft_token_id, answer);
                vote.votes = votes;
                self.votes
                    .get_mut(&contract_id)
                    .unwrap()
                    .replace(index, &vote);

                env::log_str(format!("Vote {} counted", answer).as_str());
            }
        }
    }
}
