

near call dev-1675845053607-10248492376981 new '' --gas 242794783120800 --accountId bot4.testnet

near call dev-1675845053607-10248492376981 create_vote '{"nft_contract_id":"nft.testnet", "name":"test", "description":"description", "answers":["bob", "alisa"], "min_votes_to_win":0, "min_participations":0 }' --gas 242794783120800 --accountId bot4.testnet

near call dev-1675845053607-10248492376981 vote '{"contract_id":"nft.testnet", "index":0, "answer":0, "nft_token_id":"12" }' --gas 242794783120800 --accountId bot4.testnet


near view dev-1675845053607-10248492376981 get_vote '{"contract_id":"nft.testnet", "index":0 }' 
near view dev-1675845053607-10248492376981 get_votes_by_contract '{"contract_id":"nft.testnet", "limit":10, "offset":0 }'

NEAR_ENV=mainnet near call nft-vote.near create_vote '{"nft_contract_id":"nft.testnet", "name":"test", "description":"description", "answers":["bob", "alisa"], "min_votes_to_win":0, "meta":"" }' --gas 242794783120800 --accountId mydev.near  --depositYocto 10 

NEAR_ENV=mainnet near call nft-vote.near vote '{"contract_id":"nft.testnet", "index":0, "answer":0, "nft_token_id":"12" }' --gas 242794783120800 --accountId mydev.near  --depositYocto 10 

