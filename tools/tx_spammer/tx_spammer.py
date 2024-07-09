"""
This script is used to spam transactions on the Taiko network. It reads the private key and recipient address from a .env file,
connects to the Taiko network, and sends a specified number of transactions to the recipient address.

Setup:
1. Create a virtual environment:
   python -m venv venv

2. Activate the virtual environment:
   - On Windows: venv\Scripts\activate
   - On macOS/Linux: source venv/bin/activate

3. Install the required dependencies:
   pip install -r requirements.txt

4. Create a .env file in the tools/tx_spammer directory with the following content:
   PRIVATE_KEY=<your_private_key>
   RECIPIENT_ADDRESS=<recipient_address>

5. Run the script:
   python tx_spammer.py
"""



import time
from web3 import Web3
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

private_key = os.getenv('PRIVATE_KEY')
if not private_key:
    raise Exception("Environment variable PRIVATE_KEY not set")

recipient = os.getenv('RECIPIENT_ADDRESS')
if not recipient:
    raise Exception("Environment variable RECIPIENT_ADDRESS not set")

# Connect to the Taiko network
w3 = Web3(Web3.HTTPProvider('https://RPC.helder.taiko.xyz'))

# Check if connected
if not w3.is_connected():
    raise Exception("Failed to connect to the Taiko network")

# Get the account from the private key
account = w3.eth.account.from_key(private_key)

# Define the amount to send (in Wei)
amount = w3.to_wei(0.01, 'ether')

# Function to send a transaction
def send_transaction():
    nonce = w3.eth.get_transaction_count(account.address)
    tx = {
        'nonce': nonce,
        'to': recipient,
        'value': amount,
        'gas': 21000,
        'gasPrice': w3.to_wei('10', 'gwei')
    }
    signed_tx = w3.eth.account.sign_transaction(tx)
    tx_hash = w3.eth.send_raw_transaction(signed_tx.rawTransaction)
    print(f'Transaction sent: {tx_hash.hex()}')

# Function to spam transactions
def spam_transactions(count):
    for _ in range(count):
        send_transaction()
        time.sleep(1)  # Add a delay to avoid nonce issues

# Start spamming transactions
spam_transactions(100)  # Replace 100 with the number of transactions you want to send