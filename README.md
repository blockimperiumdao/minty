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

Minty expects the CSV to conform to the following format and as of version 0.2.0 expects the delimiter to be a '|' to allow passing JSON into the extra column as ',' would cause failures. Each should be a column:

- title : string - the title of the artwork. If more than one copy, paras will append the copy number to this
- description: string - description of the series/content.
- media : string that is the IPFS ID for the artwork
- media_hash: string - if using a URL for media, this is the base64 hash that represents it
- copies: number - the number of them you want published
- extra: string - json representing extra data (untested)
- price: number - number of near this costs
- royalty_account: string - the account that will be paid royalties when this asset is sold
- royalty_pct: number - the pct royalty to be paid (keep this a whole number - no decimals)

Sample CSV Snippet (yes, the header is necessary - no you can't change the order of the fields)
```
title|description|media|media_hash|copies|extra|price|royalty_account|royalty_pct
AfroRick Publish Run|This is a test of a full publish from the tool|bafybeid7ztbmhjx3266jm6fyoaft7xvwup2ex2da2odjvfl4s4pvxvgjni||22||10000|afrorick.testnet|7
```
