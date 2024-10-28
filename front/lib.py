import time
import os

# unpack the message we get
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
        m = [[0 for i in range(n)] for i in range(n)]
        x = 1
        list2 = s.split()
        for i in range(n):
            for j in range(n):
                m[i][j] = s[x]
                x = x + 1

def send(s):
    print("H")
    while (os.path.exists("/tmp/send")):     
        time.sleep(0.1)
    print(s)
    a = open("/tmp/send", 'w')
    a.write(s)

def recv():
    while (os.path.exists("/tmp/recv") == False):     
        time.sleep(0.1)
    a = open("/tmp/recv", 'rw')
    s = a.read()
    print(s)
    os.remove(a)
    return s