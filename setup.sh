# setup.sh

#!/bin/bash

# Create a virtual environment (optional)
python3 -m venv env

# Activate the virtual environment (Unix/macOS)
source env/bin/activate

# Activate the virtual environment (Windows)
# .\env\Scripts\activate

# Upgrade pip
pip install --upgrade pip

# Install required packages
pip install -r requirements.txt

echo "Setup complete. Virtual environment created and packages installed."
