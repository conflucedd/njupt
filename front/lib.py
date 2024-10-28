# unpack the message we get and update the state of the map
def update_map(s, n):
    if s == "~OK$":
        return

    elif s == "~win$":
        # you win
        return
    
    elif s == "~lost$":
        # you lose
        return
    
    else:
        i = 0, j = 0
        for a in s:
            if a == "~":
                continue
            if a.isnumeric() == True:
                