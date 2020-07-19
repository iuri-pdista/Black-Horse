import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind((socket.gethostname(),6660))
s.listen(5)
i = 0

while True:
    clientsocket, address = s.accept()
    print(f"Connection from {address} has been established!")
    clientsocket.send(bytes("Welcome to the server!", "utf-8"))
    numberOfFiles = clientsocket.recv(4)
    print(numberOfFiles.decode("utf-8"))
    numberOfFilesInt = int(numberOfFiles)
    while i < numberOfFilesInt:
        try:
            print("Receiving file...")
            l = clientsocket.recv(50000)
            file = open('file_'+ str(i)+".txt",'wb')
            file.write(l)
            i = i + 1 
            file.close()
        except Exception as error:
            error = "Could not receive file"
            print(error)
    clientsocket.close()

s.close()

