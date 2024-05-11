# Importing the Ethereum account handling library
import eth_account
# Importing the LocalAccount class for managing Ethereum accounts locally
from eth_account.signers.local import LocalAccount
# Importing the json module to handle JSON files
import json
# Importing the os module to interact with the operating system
import os

# Importing Exchange class from a custom module presumably for handling exchange operations
from hyperliquid.exchange import Exchange
# Importing Info class from a custom module presumably for fetching information about the exchange or user
from hyperliquid.info import Info

# Defining a function setup with optional parameters for base URL and WebSocket usage
def setup(base_url=None, skip_ws=False):
    # Constructing the file path for the configuration file
    config_path = os.path.join(os.path.dirname(__file__), "config.json")
    # Opening and loading the JSON configuration file
    with open(config_path) as f:
        config = json.load(f)
    # Creating a LocalAccount instance using a secret key from the config
    account: LocalAccount = eth_account.Account.from_key(config["secret_key"])
    # Fetching the account address from the config, or defaulting to the derived address
    address = config["account_address"]
    if address == "":
        address = account.address
    # Printing the account address being used
    print("Running with account address:", address)
    # Conditional check for discrepancy in addresses
    if address != account.address:
        print("Running with agent address:", account.address)
    # Creating an instance of Info to fetch user state
    info = Info(base_url, skip_ws)
    # Fetching the user state for the specified address
    user_state = info.user_state(address)
    # Extracting the margin summary from the user state
    margin_summary = user_state["marginSummary"]
    # Checking if the account value is zero and raising an exception if true
    if float(margin_summary["accountValue"]) == 0:
        print("Not running the example because the provided account has no equity.")
        # Extracting the domain from the base URL
        url = info.base_url.split(".", 1)[1]
        # Forming the error string for clarity on the issue
        error_string = f"No accountValue:\nIf you think this is a mistake, make sure that {address} has a balance on {url}.\nIf address shown is your API wallet address, update the config to specify the address of your account, not the address of the API wallet."
        # Raising an exception with the formed error message
        raise Exception(error_string)
    # Creating an instance of Exchange to handle trading operations
    exchange = Exchange(account, base_url, account_address=address)
    # Returning the address, info object, and exchange object
    return address, info, exchange
