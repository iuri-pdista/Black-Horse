import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((socket.gethostname(),1234))

msg = s.recv(1024)
print(msg.decode("utf-8"))
count = 0
file = open("test.txt", "rb")
content = file.read(50000)
s.send(content)
content = file.read(50000)
count += 1    