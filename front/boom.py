from tkinter import *
import lib
import numpy
# game page
def open_game_page(c):
    # hide the start page
    root.withdraw()

    # initialize the game page
    gp = Toplevel(root)
    gp.title("boom")
    window_width = 1080
    window_height = 1080
    screen_width = gp.winfo_screenwidth()
    screen_height = gp.winfo_screenheight()
    position_top = int(screen_height/2 - window_height/2)
    position_left = int(screen_width/2 - window_width/2)
    gp.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")

    # put the map into a frame
    frame = Frame(gp)
    frame.grid(row = 0, column = 0, padx = 20, pady = 20)
    gp.grid_rowconfigure(0, weight = 1)
    gp.grid_columnconfigure(0, weight = 1)

    # tell you win or lose
    
    def sign_page(s):
        sp = Toplevel(gp)
        sp.title("")
        window_width = 800
        window_height = 200
        screen_width = sp.winfo_screenwidth()
        screen_height = sp.winfo_screenheight()
        position_top = int(screen_height/2 - window_height/2)
        position_left = int(screen_width/2 - window_width/2)
        sp.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")

        l = Label(sp, text = s, width = 400, height = 1, font = ("Arial", 30))
        l.pack(padx = 20, pady = 20, anchor = "center")

        #go back to the start page
        def return_to_start():
            sp.destroy()
            gp.destroy()
            root.deiconify()

        b = Button(sp, text = "continue", font= ("Arial", 20), width = 10, height = 1, command = return_to_start)
        b.pack(padx = 20, pady = 20, anchor = "center")
    

    row = 0
    col = 0
    if c == 1:
        row = 9
        col = 9
    elif c == 2:
        row = 16
        col = 16
    elif c == 3:
        row = 30
        col = 16

    button_states = {}
    buttons = {}
    map = [[0 for x in range(col)] for y in range(row)]

    #start
    lib.send("~start" + str(row) + "," + str(col) + "$")
    lib.recv()

    # draw the map
    for i in range(row):
        for j in range(col):
            button_states[(i, j)] = ' '
            btn = Button(frame, width = 1, height = 1, font= ("Arial", 20))
            btn.config(text = ' ')
            btn.grid(row = i, column = j)

            btn.bind("<Button-1>", lambda event, x=i, y=j: left_click(event, x, y))
            btn.bind("<Button-3>", lambda event, x=i, y=j: right_click(event, x, y))

            buttons[(i, j)] = btn

    # update the map
    def update_button_state():
        for i in range(row):
            for j in range(col):
                if button_states[(i, j)] == '9':
                    buttons[(i, j)].config(text = ' ')
                elif button_states[(i, j)] == '0':
                    buttons[(i, j)].config(text = ' ', bg = 'yellow')
                elif button_states[(i, j)] == '@':
                    buttons[(i, j)].config(text = '@')
                else:
                    buttons[(i, j)].config(text = button_states[(i, j)], bg = 'yellow')

    # when the game is finished, disable all the buttons
    def disable_buttons():
        for i in range(row):
            for j in range(col):
                buttons[(i, j)].unbind("<Button-1>")
                buttons[(i, j)].unbind("<Button-3>")
                buttons[(i, j)].config(state = "disabled")

    def left_click(event, x, y):
        button = event.widget
        lib.send("~click" + str(x) + "," + str(y) + "$")
        s = lib.recv()

        # you win
        if s == "~win$":
            disable_buttons()
            lib.send("~answer$")
            s = lib.recv()

            a = 1
            for i in range(row):
                for j in range(col):
                    map[i][j] = s[a]
                    a += 1
            
            for i in range(row):
                for j in range(col):
                    button_states[(i, j)] = map[i][j]
            update_button_state()
            sign_page("you win!!!")

        # you lose
        elif s == "~lost$":
            disable_buttons()
            lib.send("~answer$")
            s = lib.recv()

            a = 1
            for i in range(row):
                for j in range(col):
                    map[i][j] = s[a]
                    a += 1
            
            for i in range(row):
                for j in range(col):
                    button_states[(i, j)] = map[i][j]
            update_button_state()
            sign_page("you lose...")

        else:
            a = 1
            for i in range(row):
                for j in range(col):
                    map[i][j] = s[a]
                    a += 1
           
        # update the current map
        for i in range(row):
            for j in range(col):
                button_states[(i, j)] = map[i][j]
        update_button_state()

    def right_click(event, x, y):
        button = event.widget
        lib.send("~mark" + str(x) + "," + str(y) + "$")
        s = lib.recv()
        a = 1
        for i in range(row):
            for j in range(col):
                map[i][j] = s[a]
                a += 1
                
        # update the current map
        for i in range(row):
            for j in range(col):
                button_states[(i, j)] = map[i][j]
        update_button_state()

    # when close the game page, open the start page
    def on_close():
        gp.destroy()
        root.deiconify()
    gp.protocol("WM_DELETE_WINDOW", on_close)

# start page
root = Tk()
root.title("boom")
root.resizable(False, False)
window_width = 1000
window_height = 800
screen_width = root.winfo_screenwidth()
screen_height = root.winfo_screenheight()
position_top = int(screen_height/2 - window_height/2)
position_left = int(screen_width/2 - window_width/2)
root.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")

l = Label(root, width = 20, height= 1, text = "Mine Sweeper", font= ("Arial", 40))
l.place(x = 500, y = 150, anchor = "center")

choice = IntVar()
choice.set(1)
r1 = Radiobutton(root, text = "easy", variable = choice, value = 1, font= ("Arial", 30))
r2 = Radiobutton(root, text = "medium", variable = choice, value = 2, font= ("Arial", 30))
r3 = Radiobutton(root, text = "hard", variable = choice, value = 3, font= ("Arial", 30))
r1.place(x = 400, y = 225)
r2.place(x = 400, y = 325)
r3.place(x = 400, y = 425)

b = Button(root, text = "start", font= ("Arial", 30), width = 10, height = 1, command = lambda: open_game_page(choice.get()))
b.place(x = 375, y = 550)

mainloop()