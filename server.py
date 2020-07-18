import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind((socket.gethostname(),1234))
s.listen(5)
i = 0 

while True:
    clientsocket, address = s.accept()
    print(f"Connection from {address} has been established!")
    clientsocket.send(bytes("Welcome to the server!", "utf-8"))
    l = clientsocket.recv(50000)
    f = open('file_'+ str(i)+".txt",'wb')
    f.write(l)
    i = i + 1
    f.close()
    clientsocket.close()

s.close()

