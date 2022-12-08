# SubRPC

## Intro

SubRPC helps maintaining and managing a local list of RPC Endpoints. Using the `subrpc`, the user can aggregate data from several public or private registries of RPC endpoints and maintain their local database fresh.

While the `subrpc` cli allows using the data in a terminal, the `subrpc-core` crate helps any third party app fetching and maintainging a list of RPC endpoints.

This helps your apps no longer having to provide a flag such as `your-cli --url wss://rpc.polkadot.io` but instead use `your-cli --chain polkadot` and be sure to connect to a quality RPC endpoint.

## Quick start

    cargo install subrpc
    subrpc reg add https://raw.githubusercontent.com/chevdor/subrpc/master/registry/subrpc.json
    subrpc reg add https://paritytech.github.io/polkadot_network_directory/registry.json
    subrpc reg up
    subrpc reg ls
    subrpc ep ls

## Registries

Registries are mainly a list of RPC endpoints, stored into a json file and available via a web server (public or not).

You can find below a list of public registries:

-   [Polkadot Network Directory Registry](https://paritytech.github.io/polkadot_network_directory/registry.json): The [Polkadot Network Directory](https://paritytech.github.io/polkadot_network_directory) project is aware of many chains and projects and provides a registry that is automatically updated and based on the data available to the directory

-   [SubRPC Registry](https://raw.githubusercontent.com/chevdor/subrpc/master/registry/subrpc.json): A small basic registry to get started and show an example

-   Add your registry with [Pull Request](https://github.com/chevdor/subrpc/pulls)

## Usage

## Help

    `subrpc` allows managing a set of registry providing rpc nodes

    Usage: subrpc <COMMAND>

    Commands:
      registry   Manage your registries
      system     System
      endpoints  Endpoints
      config     Config
      help       Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help information
      -V, --version  Print version information

## Config

    Config

    Usage: subrpc config <COMMAND>

    Commands:
      list  Config list
      edit  Config edit
      help  Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help information
      -V, --version  Print version information

## Registry

    Manage your registries

    Usage: subrpc registry <COMMAND>

    Commands:
      list    List currently known registries
      show    Show the list of registries and some of the content
      add     Add a new registry. It will be enabled by default
      update  Fetch the latest data from the registries and update the list of endpoints
      help    Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help information
      -V, --version  Print version information

## Endpoints

    Endpoints

    Usage: subrpc endpoints <COMMAND>

    Commands:
      list  Show the list of all endpoints
      get   Get one or some endpoints
      ping  Ping endpoints
      help  Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help information
      -V, --version  Print version information

## System

    System

    Usage: subrpc system <COMMAND>

    Commands:
      info  Show general system information such as the location of relevant files
      init  Reset your local database
      help  Print this message or the help of the given subcommand(s)

    Options:
      -h, --help     Print help information
      -V, --version  Print version information

## FAQ

> Is SubRPC a registry ?

No. While SubRPC provides a very basic default list of RPC endpoints, this list will remain small and rather unmaintained. It will contain only a few of the core RPC Enpoints of the network and will surely miss many important ones.

> Can I use a private registry ?

Sure! And this is the idea behind SubRPC. You can tap into both public and private registry to build up your very own database of RPC Endpoints.

> How can I be sure that my list is up to date ?

The `subrpc` can help you refresh/update your endpoints in 2 ways: first it will check the registries for new endpoints and add them to your list, but it will also test the endpoints in order to ensure that your local endpoints list remains of quality.

Using the `subrpc-core` crate also gather (locally) some information about the RPC Endpoints you are connecting to. This is done to allow you skipping poor quality Endpoints over time, while keeping a list of quality RPC Endpoints up to date. This is all local and 100% opinionated: an endpoint you may see as bad may work great for someone else, moreover, the config (will) allow customizing the list of endpoints.

> Can I host my own registry ?

Absolutely and that would be great. If you make a public one, please open a PR in this repo so we can document the list of public registries.

> How does SubRPC decides whether an RPC Endpoint is good or bad ?

Eveytime you connect (or ask for a refresh), SubRPC checks whether an endpoint is reachable and if it is, gathers a few metrics about it. This allows maintaining a list of the best endpoints **for you**. This list is personal and remains on your machine.
