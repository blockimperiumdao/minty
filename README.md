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
