# mcp-server: An MCP server using Anthropic crate rmcp

## Description
An MCP server providing the following tools to an agentic system.

* ETH tools: Some standard EVM tools to check the balance of an address, and send ETH.
* Brave: A web search tool.
* 0xProtocol: An api for querying swap prices of ERC20 tokens.
* Uniswap tools: Provides contract calls to the Uniswap v2 router

## Requirements
The following environment variables are required to run the server.

* `MCP_SERVER_ADDRESS`
* `MCP_SERVER_PORT`
* `ETH_RPC`
* `BRAVE_API_KEY`
* `ZERO_X_API_KEY`

## Starting Server

```
make start
```
