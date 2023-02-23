NEAR Votes | NEAR Social
====

## Contract

1. Init contarct

 `near call CONTRACT new ''  --accountId me.testnet `

2. Create vote (0.5 NEAR cost to protect spam)
- `contract_id` - NFT contract adress
- `name` - Vote name
- `description` - Vote description
- `answers` - List[str] of answers
- `meta` - any string

`near call CONTRACT create_vote '{"nft_contract_id":"nft.testnet", "name":"test", "description":"description", "answers":["bob", "alisa"], "meta":""}' --accountId me.testnet `


3. Vote 
- `answer` - answer index
- `nft_token_id` - nft to vote
- `contract_id` - NFT contract adress
- `index` - vote's index

`near call CONTRACT vote '{"contract_id":"nft.testnet", "index":0, "answer":0, "nft_token_id":"12" }' --accountId me.testnet `


4. Close vote 
- `contract_id` - NFT contract adress
- `index` - vote's index

`near call CONTRACT close_vote '{"contract_id":"nft.testnet", "index":0}' --accountId me.testnet `


## View methods

1. Get vote
- `contract_id` - NFT contract adress
- `index` - vote's index

`near view CONTRACT get_vote '{"contract_id":"nft.testnet", "index":0 }' `

2. Get all votes for contract
- `contract_id` - NFT contract adress
- `index` - vote's index
- `limit` 
- `offset` 

`near view CONTRACT get_votes_by_contract '{"contract_id":"nft.testnet", "limit":10, "offset":0 }'`


