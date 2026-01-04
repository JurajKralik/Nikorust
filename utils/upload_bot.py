import requests
import os
from pathlib import Path

# Navigate to repo root
script_dir = Path(__file__).parent
repo_root = script_dir.parent
os.chdir(repo_root)

# Load .env file manually
env_file = repo_root / ".env"
if env_file.exists():
    with open(env_file) as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                key, value = line.split("=", 1)
                os.environ[key] = value

bot_id = 939
token = os.environ.get("AIARENA_API_KEY", "")
bot_zip_file = "./RustyNikolaj.zip"

if not token:
    raise SystemExit("Error: AIARENA_API_KEY not set in environment")

if not Path(bot_zip_file).exists():
    raise SystemExit(f"Error: {bot_zip_file} not found")

auth = {"Authorization": f"Token {token}"}
with open(bot_zip_file, "rb") as bot_zip:
    response = requests.patch(
        f"https://aiarena.net/api/bots/{bot_id}/",
        headers=auth,
        data={
            "bot_zip_publicly_downloadable": True,
            "bot_data_publicly_downloadable": False,
        },
        files={
            "bot_zip": bot_zip,
        },
    )
    result = response.json()
    print(result)
    if response.status_code == 202 or response.status_code == 200:
        print("Upload successful! 🚀")
    else:
        print(f"Upload failed with status {response.status_code}")