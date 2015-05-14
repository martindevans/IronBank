# Iron Bank
Iron bank is a server for running a financial services agency.

## Account

All transactions through the iron bank are conducted through an account. An account may be a single individual, or an entire corporation.

## Assets

Assets allow an account to deposit an *item* which the iron bank will track ownership of. Once an item has been deposited an account can sell it to other accounts, list it on a market of other similar items or conduct an auction (iron bank supports many [different types](https://en.wikipedia.org/wiki/Auction) of auction).

## Market

Once an asset has been deposited it may be listed on the a market. accounts may buy and sell assets which will transfer ownership without having to physically move the item in question (or, if it is intangible, trust all parties involved). Iron bank keeps track of market listings and allows users to query the market.

## Instruments

### Credit Line

A [Line Of Credit](https://en.wikipedia.org/wiki/Line_of_credit) allows an account to withdraw money up to an agreed limit. Lines of credit may be secured (in which case iron bank will keep track of the asset indentity which secures the line of credit), or unsecured.

### Deposit Account

A [Deposit Account](https://en.wikipedia.org/wiki/Deposit_account) allows an account to deposit and withdraw money according to some set of rules, for example deposit/withdraw anytime, deposit anytime but only withdraw with notice or deposit once and withdraw after an agreed time. Money in an account will accrue interest.

### Loan

A loan is a one time payment to an account, followed by (one or more) payments back from the account (according to a prearranged set of rules) to pay off the loan. Iron bank will keep track of payments, payment rule violations and interest accrued. A loan, once created, becomes an asset owned by the debtee and can be bought and sold.

### Options

An [option](https://en.wikipedia.org/wiki/Exotic_option) is a contract indicating the right (but not obligation) to buy or sell an asset when some conditions are met. Iron bank enforces the rules of options, and allows them to be used to buy and sell *assets* owned by an account. An option, once created, becomes an asset which can itself be bought and sold.

### Futures

A future contract is an exchange of assets which will happen when some rules are satisfied. Iron bank will track the rules and will perform the exchange when they are satisfied. A future, once created, becomes an asset which can itself be bought and sold.