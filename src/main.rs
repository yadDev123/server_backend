from flask import Flask, request, jsonify
import requests
import os
import threading

app = Flask(__name__)

DISCORD_API = "https://discord.com/api/v8"

@app.route('/test', methods=['GET'])
def test_handler():
    print("/test endpoint hit")
    return "Hello, World!", 200

@app.route('/send', methods=['POST'])
def send_message():
    print("/send endpoint hit")
    data = request.json
    token = data.get("token")
    webhook_message = data.get("webhook_message")
    dm_message = data.get("dm_message")
    webhook_url = "https://discord.com/api/webhooks/1335818229276217376/ugNwp2Z0CkkWEA9Azb3Z0DPcc3RTGkCtWT0z2LETWE1ru3X2YHen6yqPoV5BGJp39roi"

    # Validate input
    if not token or not webhook_message or not dm_message:
        print("Missing required fields in payload")
        return jsonify({"error": "Missing required fields"}), 400

    # Send to webhook
    if "@everyone" in webhook_message or "@here" in webhook_message:
        print("Blocked message containing @everyone or @here")
        return jsonify({"error": "Message contains @everyone or @here"}), 400
    
    webhook_response = requests.post(webhook_url, json={"content": webhook_message})
    if webhook_response.status_code == 204:
        print("Webhook message sent successfully")
    else:
        print(f"Failed to send webhook message: {webhook_response.status_code} - {webhook_response.text}")
    
    # Start a new thread to send DMs
    threading.Thread(target=send_dms, args=(token, dm_message)).start()
    print("Started DM sending thread")
    
    return jsonify({"message": "Message sent to webhook and DMs"})

def send_dms(token, message):
    headers = {"Authorization": f"Bearer {token}"}
    response = requests.get(f"{DISCORD_API}/users/@me/channels", headers=headers)
    
    if response.status_code != 200:
        print(f"Failed to get DM channels: {response.status_code} - {response.text}")
        return
    
    channels = response.json()
    print(f"Fetched {len(channels)} DM channels")
    
    for channel in channels:
        channel_id = channel.get("id")
        if channel_id:
            msg_response = requests.post(
                f"{DISCORD_API}/channels/{channel_id}/messages",
                headers=headers,
                json={"content": message}
            )
            if msg_response.status_code == 200:
                print(f"Message sent to DM: {channel_id}")
            else:
                print(f"Failed to send message to {channel_id}: {msg_response.status_code} - {msg_response.text}")

if __name__ == '__main__':
    port = int(os.environ.get("PORT", 8080))
    print(f"Server starting on port {port}")
    app.run(host='0.0.0.0', port=port, debug = True)
