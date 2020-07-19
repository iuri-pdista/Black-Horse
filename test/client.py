import socket
from os import system
import sys
from sys import platform

filePath = sys.argv
filePath.pop(0)
numberOfFiles = len(filePath)
print(filePath[0], numberOfFiles)
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((socket.gethostname(),6660))

welcomeMsg = s.recv(1024)
print(welcomeMsg.decode("utf-8"))
numberOfFilesStr = str(numberOfFiles)
s.send(bytes(numberOfFilesStr, "utf-8"))
count = 0
while (count < numberOfFiles):
    file = open(filePath[count], "rb")
    content = file.read(50000)
    s.send(content)
    print("Sending file: " + filePath[count])
    count += 1