## escrow

http server containing an escrow wallet. executes arbitrary calls on providers
of choice. built using `axum` and `ethers`


### sample request

Body: 
```
{
	"from": "0x0d828dae4c1dfa77bdc54d43fb421e60abb7eb83",
	"to": "0x0000000000000000000000000000000000000000",
	"gas": "0x383",
	"gas_price": "",
	"value": "0x2",
	"data": "",
	"nonce": "0x4"
}
```
