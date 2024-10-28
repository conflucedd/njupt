import os
if os.path.exists("/tmp/recv"):
    os.remove("/tmp/recv")
if os.path.exists("/tmp/send"):
    os.remove("/tmp/send")