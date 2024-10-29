from tkinter import *
import lib

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

    # 0 is null, 1~8 is number, 9 is unclicked, @ is marked
    button_states = {}
    buttons = {}
    judge_list = [2, 2]
    map = [[0 for i in range(c)] for i in range(c)]

    # draw the map
    for i in range(c):
        for j in range(c):
            button_states[(i, j)] = 9
            btn = Button(frame, width = 1, height = 1)
            btn.grid(row = i, column = j)

            btn.bind("<Button-1>", lambda event, x=i, y=j: left_click(event, x, y))
            btn.bind("<Button-3>", lambda event, x=i, y=j: right_click(event, x, y))

            buttons[(i, j)] = btn

    def left_click(event, row, col):
        button = event.widget
        send("~click" + str(row) + "," + str(col) + "$")
        s = recv()
        map = lib.unpack_message(s, c, judge_list)

        # win
        if judge_list[1] == 1:
            sign_page("you win!!!")
        # lose
        if judge_list[1] == 0:
            sign_page("you lose...")             
        # update the current map
        for i in range(c):
            for j in range(c):
                button_states[(i, j)] = map[i][j]

    def right_click(event, r, c):
        button = event.widget
        send("~mark" + str(r) + "," + str(c) + "$")
        s = recv()
        map = lib.unpack_message(s, c, judge_list)
        # update the current map
        for i in range(c):
            for j in range(c):
                button_states[(i, j)] = map[i][j]


    def send(s):
        print(s)
    
    def recv():
        return "~1$"

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

l = Label(root, width = 20, height= 1, text = "choose the level", font= ("Arial", 40))
l.place(x = 500, y = 150, anchor = "center")

choice = IntVar()
choice.set(9)
r1 = Radiobutton(root, text = "easy", variable = choice, value = 9, font= ("Arial", 30))
r2 = Radiobutton(root, text = "medium", variable = choice, value = 16, font= ("Arial", 30))
r3 = Radiobutton(root, text = "hard", variable = choice, value = 22, font= ("Arial", 30))
r1.place(x = 400, y = 225)
r2.place(x = 400, y = 325)
r3.place(x = 400, y = 425)

b = Button(root, text = "start", font= ("Arial", 30), width = 10, height = 1, command = lambda: open_game_page(choice.get()))
b.place(x = 375, y = 550)

mainloop()