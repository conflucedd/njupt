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