Aslaks Varmtvannsbereder App
============================

Kontrollerer Aslaks varmtvannsbereder basert på strømpris fra Nordpool.

## Flow

### App start-up
1. What time is it?
	- If time is before 13:00:
		- Get data for today
		- Start scheduler to get new data @ ~13:00
	- If time is after 13:00:
		- Get data for today
		- Get data for tomorrow
		- Start scheduler to get new data @ ~13:00

### App main loop
1. Repeating scheduler @ ~13:00 every day:
	- Get data
	- Mark lowest priced hours
2. Repeating scheduler @ every hour:
	- Check if this hour is a low price hour
	- Turn on heater if low price
	- Turn off heater if high price


## Links

- [Nordpool API](https://developers.nordpoolgroup.com/)
- [TuyAPI](https://github.com/EmilSodergren/rust-tuyapi)
