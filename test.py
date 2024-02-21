from flask import Flask

app = Flask(__name__)

@app.route('/')
def hello_world():
    return 'Hello, World!'

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=25565)


""" import socket

app = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
app.bind(('127.0.0.1', 25565))

while True:
    data, addr = app.recvfrom(1024)
    print(f"Received UDP packet from {addr}: {data.decode('utf-8')}")
 """