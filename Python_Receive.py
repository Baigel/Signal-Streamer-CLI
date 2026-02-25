import socket

import time

import numpy as np
import matplotlib.pyplot as plt

'''
for i in range(10):
    y = np.random.random()
    plt.scatter(i, y)
    plt.pause(0.05)
'''
#plt.show()




UDP_IP = "127.0.0.1"
UDP_PORT = 12345

sock = socket.socket(socket.AF_INET, # Internet
                     socket.SOCK_DGRAM) # UDP
sock.bind((UDP_IP, UDP_PORT))

sum_time = 0
data_arr = []
while True:
	
    data, addr = sock.recvfrom(10) # buffer size is 1024 bytes
    
    data_arr.insert(0, float(data))
    print(float(data))
    plt.cla()
    plt.plot(data_arr[::-1], color='blue', linestyle='-', marker='o')
    plt.pause(0.00001)
    
    if len(data_arr) > 100:
    	del data_arr[1000:]

plt.show()
