use ethers::prelude::abigen;

abigen!(
    Erc20Contract,
    "$CARGO_MANIFEST_DIR/src/contract/erc20-abi.json"
);
abigen!(
    Erc721Contract,
    "$CARGO_MANIFEST_DIR/src/contract/erc721-abi.json"
);
abigen!(
    Erc1155Contract,
    "$CARGO_MANIFEST_DIR/src/contract/erc1155-abi.json"
);
