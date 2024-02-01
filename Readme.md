
How to run :
1. Install dependencies 
   - `cargo build`
2. Run the server 
   -  `cargo run`


Sample APIs:

```curl -X http://127.0.0.1:8090/api/txn?signature=2iHLyJJNNZ5Dvcoy3JLQLmAsZ9bVCF22HNeKDcYtkEajd7oc3EKMLm6xb9beDumdC8zoi8Wms4YNW2pJnpUvHuts ```


Sample response:

 ```
{
    "slot": 117147758,
    "transaction": {
        "signatures": [
            "2iHLyJJNNZ5Dvcoy3JLQLmAsZ9bVCF22HNeKDcYtkEajd7oc3EKMLm6xb9beDumdC8zoi8Wms4YNW2pJnpUvHuts"
        ],
        "message": {
            "accountKeys": [
                {
                    "pubkey": "919mqjGT1nxig7qxUBhhzWSg9bX8vU6z1KbB2iiLT3xh",
                    "writable": true,
                    "signer": true,
                    "source": "transaction"
                },
                {
                    "pubkey": "D7SDvdPic7ixkfLeThPjJ1tdXmfvAZ1CKGNUjVizj5S",
                    "writable": true,
                    "signer": false,
                    "source": "transaction"
                },
                {
                    "pubkey": "5G8mtcRP1bKratf1Nxd79ZFEwSXwe5UbnA4fq9BRWVtP",
                    "writable": true,
                    "signer": false,
                    "source": "transaction"
                },
                {
                    "pubkey": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                    "writable": false,
                    "signer": false,
                    "source": "transaction"
                },
                {
                    "pubkey": "11111111111111111111111111111111",
                    "writable": false,
                    "signer": false,
                    "source": "transaction"
                },
                {
                    "pubkey": "MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8",
                    "writable": false,
                    "signer": false,
                    "source": "transaction"
                }
            ],
            "recentBlockhash": "AYybaicpdSRYfi5hRbUE6NCPrVL331vMtJrfvSs6JDTN",
            "instructions": [
                {
                    "programId": "MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8",
                    "accounts": [
                        "919mqjGT1nxig7qxUBhhzWSg9bX8vU6z1KbB2iiLT3xh",
                        "D7SDvdPic7ixkfLeThPjJ1tdXmfvAZ1CKGNUjVizj5S",
                        "5G8mtcRP1bKratf1Nxd79ZFEwSXwe5UbnA4fq9BRWVtP",
                        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                        "11111111111111111111111111111111"
                    ],
                    "data": "2RD37fomjEhKy9A5KfNd2C7G",
                    "stackHeight": null
                }
            ]
        }
    },
    "meta": {
        "err": null,
        "status": {
            "Ok": null
        },
        "fee": 5000,
        "preBalances": [
            2434757394,
            2039280,
            0,
            953185920,
            1,
            1141440
        ],
        "postBalances": [
            2433304714,
            2039280,
            1447680,
            953185920,
            1,
            1141440
        ],
        "innerInstructions": [
            {
                "index": 0,
                "instructions": [
                    {
                        "program": "system",
                        "programId": "11111111111111111111111111111111",
                        "parsed": {
                            "info": {
                                "lamports": 1447680,
                                "newAccount": "5G8mtcRP1bKratf1Nxd79ZFEwSXwe5UbnA4fq9BRWVtP",
                                "owner": "MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8",
                                "source": "919mqjGT1nxig7qxUBhhzWSg9bX8vU6z1KbB2iiLT3xh",
                                "space": 80
                            },
                            "type": "createAccount"
                        },
                        "stackHeight": null
                    },
                    {
                        "program": "spl-token",
                        "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                        "parsed": {
                            "info": {
                                "account": "D7SDvdPic7ixkfLeThPjJ1tdXmfvAZ1CKGNUjVizj5S",
                                "authority": "919mqjGT1nxig7qxUBhhzWSg9bX8vU6z1KbB2iiLT3xh",
                                "authorityType": "accountOwner",
                                "newAuthority": "GUfCR9mK6azb9vcpsxgXyj7XRPAKJd4KMHTTVvtncGgp"
                            },
                            "type": "setAuthority"
                        },
                        "stackHeight": null
                    }
                ]
            }
        ],
        "logMessages": [
            "Program MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8 invoke [1]",
            "Program 11111111111111111111111111111111 invoke [2]",
            "Program 11111111111111111111111111111111 success",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
            "Program log: Instruction: SetAuthority",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1770 of 182485 compute units",
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
            "Program MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8 consumed 20837 of 200000 compute units",
            "Program MEisE1HzehtrDpAAT8PnLHjpSSkRYakotTuJRPjTpo8 success"
        ],
        "preTokenBalances": [
            {
                "accountIndex": 1,
                "mint": "7pdEDRNAKmANipzTZR1HtK3PLYEJfmBMmMdoxyqb5ydN",
                "uiTokenAmount": {
                    "uiAmount": 1.0,
                    "decimals": 0,
                    "amount": "1",
                    "uiAmountString": "1"
                },
                "owner": "919mqjGT1nxig7qxUBhhzWSg9bX8vU6z1KbB2iiLT3xh"
            }
        ],
        "postTokenBalances": [
            {
                "accountIndex": 1,
                "mint": "7pdEDRNAKmANipzTZR1HtK3PLYEJfmBMmMdoxyqb5ydN",
                "uiTokenAmount": {
                    "uiAmount": 1.0,
                    "decimals": 0,
                    "amount": "1",
                    "uiAmountString": "1"
                },
                "owner": "GUfCR9mK6azb9vcpsxgXyj7XRPAKJd4KMHTTVvtncGgp"
            }
        ],
        "rewards": []
    },
    "blockTime": 1642727642
}

 ```