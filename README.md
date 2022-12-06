# SubRPC

SubRPC helps maintaining and managing a local list of RPC Endpoints. Using the `subrpc`, the user can aggregate data from several public or private registries of RPC endpoints and maintain their local database fresh.

While the `subrpc` cli allows using the data in a terminal, the `subrpc-core` crate helps any third party app fetching and maintainging a list of RPC endpoints.

This helps your apps no longer having to provide a flag such as `your-cli --url wss://rpc.polkadot.io` but instead use `your-cli --chain polkadot` and be sure to connect to a quality RPC endpoint.
