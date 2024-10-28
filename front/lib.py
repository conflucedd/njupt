import time
import os
import numpy

# unpack the message we get
'''
def unpack_message(s, n, list):
    if s == "~OK$":
        # continue
        return

    elif s == "~win$":
        # you win
        list[1] = 1
    
    elif s == "~lost$":
        # you lose
        list[1] = 0
    
    else:
        array = numpy.zeros(n, n)
        x = 1
        for i in range(n):
            for j in range(n):
                array[i][j] = s[x]
                x = x + 1
        
        print(array)
        return array
'''
def send(s):
    while (os.path.exists("/tmp/send")):     
        time.sleep(0.1)
    print(s)
    a = open("/tmp/send", 'w')
    a.write(s)

def recv():
    while (os.path.exists("/tmp/recv") == False):     
        time.sleep(0.1)
    a = open("/tmp/recv", 'r')
    s = a.read()
    print(s)
    os.remove("/tmp/recv")
    return s