# minty

Minty is a system for allowing NFT authors targeting multiple NFT marketplaces to create an Excel spreadsheet with the details and have those published to the networks. 
The first network supported is the NEAR Paras network.

Minty relies on the NEAR CLI (command line interface) being installed to do some of the heavy lifting. 

https://github.com/near/near-cli

Once the NEAR CLI is installed, you can invoke the command for publishing by creating an Excel or Google Sheet and exporting it to CSV. Save this CSV to your machine
and invoke the command as

> minty {path to CSV file} {account to publish under}

```
minty foo.csv afrorick.testnet
```

## CSV Format

Minty expects the CSV to conform to the following format. Each should be a column:

- title : string - the title of the artwork. If more than one copy, paras will append the copy number to this
- media : string that is the IPFS ID for the artwork
- copies: number - the number of them you want published
- price: number - number of near this costs
- royal_act: string - the account that will be paid royalties when this asset is sold
- royalty_pct: number - the pct royalty to be paid (keep this a whole number - no decimals)
