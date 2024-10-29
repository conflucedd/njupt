from tkinter import *
import lib

is_changed = False

# game page
def game_page(c):
    # update is_changed to False
    global is_changed
    is_changed = False

    # hide the start page
    root.withdraw()

    # initialize the game page
    gp = Toplevel(root)
    gp.title("boom")
    window_width = 1920
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

    # the frame to put the control buttons
    frame2 = Frame(gp)
    frame2.place(x = 1920, y = 400, anchor = "e")

    # tell you win or lose
    r_state = Label(gp, width = 20, height= 1, text = " ", font= ("Arial", 50))
    r_state.place(x = 960, y = 80, anchor = "center")

    #go back to the start page
    def return_to_start():
        global is_changed
        if is_changed == False:
            gp.destroy()
            root.deiconify()
        else:
            sign_page()

    b = Button(frame2, text = "go back", font= ("Arial", 30), width = 10, height = 1, command = return_to_start)
    b.pack(padx = 20, pady = 20, anchor = "n")

    # tell you the game is not finished
    def sign_page():
        sp = Toplevel(gp)
        sp.title("")
        window_width = 800
        window_height = 200
        screen_width = sp.winfo_screenwidth()
        screen_height = sp.winfo_screenheight()
        position_top = int(screen_height/2 - window_height/2)
        position_left = int(screen_width/2 - window_width/2)
        sp.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")

        def continue_game():
            sp.destroy()

        def go_back():
            sp.destroy()
            gp.destroy()
            root.deiconify()

        l = Label(sp, text = "Do you want to leave ?", width = 400, height = 1, font = ("Arial", 30))
        l.pack(padx = 20, pady = 20, anchor = "n")

        b1 = Button(sp, text = "yes", font= ("Arial", 30), width = 10, height = 1, command = go_back)
        b1.place(x = 200, y = 150, anchor = "center")

        b2 = Button(sp, text = "no", font= ("Arial", 30), width = 10, height = 1, command = continue_game)
        b2.place(x = 600, y = 150, anchor = "center")

    row = 0
    col = 0
    if c == 1:
        row = 9
        col = 9
    elif c == 2:
        row = 16
        col = 16
    elif c == 3:
        row = 16
        col = 30

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
        global is_changed
        is_changed = True

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
        lib.send("~click" + str(x) + "," + str(y) + "$")
        s = lib.recv()

        # you win
        if s == "~win$":
            disable_buttons()
            r_state.config(text = "you win !!!")
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

        # you lose
        elif s == "~lost$":
            disable_buttons()
            r_state.config(text = "you lose ...")
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

b = Button(root, text = "start", font= ("Arial", 30), width = 10, height = 1, command = lambda: game_page(choice.get()))
b.place(x = 375, y = 550)

mainloop()