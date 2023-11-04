Three main operations:
(1) add O(1)
(2) cancel O(1)
(3) execute O(1)

"What are the best bid and offer?"
"How much volume is there between prices A and B?"
"What is order X's current position in the book?"

Volume: the number of shares
Sparsity: average distance in cents between limits that have volume
Slippage: the difference between the expected price of a trade and the actual executed price.

HFT system depends on high volume to work efficiently. Market-making. What's market-making? 


Goal: match participants that are willing to sell an asset with participants that are willing to buy it

Two types of orders: market orders and limit orders. 

Market orders: willing to either buy or sell the asset immediately, preferably at the best available price
Limit orders: show the interest

Limit orders make liquidity





we use red-black tree because we put emphasis on insertions and deletions instead of searching (we are using the hash table to do O(1) search)



Thoughts:
Can we make it a distributed orderbook using Kademlia?