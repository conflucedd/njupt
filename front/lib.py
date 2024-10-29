import time
import os

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