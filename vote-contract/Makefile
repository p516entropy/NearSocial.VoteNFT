DEV_CONTRACT=

build:
	cd contract && cargo build --target wasm32-unknown-unknown --release && cd ../ && \
	cp contract/target/wasm32-unknown-unknown/release/vote_nft.wasm ./out/main.wasm


deploy-dev:
	make build && \
	near dev-deploy

deploy-prod:
	make build && \
	NEAR_ENV=mainnet near deploy nft-vote.near


dev-init:
	near call $(DEV_CONTRACT) new '{"owner_id":"me.testnet"}' --accountId me.testnet  --gas 242794783120800    


init:
	NEAR_ENV=mainnet near call nft-vote.near new '' --accountId nft-vote.near  --gas 242794783120800    
