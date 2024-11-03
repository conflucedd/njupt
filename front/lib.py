from tkinter import *
from functools import partial
import time
import os

is_changed = False

def send(s):
    while (os.path.exists("/tmp/send")):     
        time.sleep(0.1)
    a = open("/tmp/send", 'w')
    a.write(s)

def recv():
    while (os.path.exists("/tmp/recv") == False):     
        time.sleep(0.1)
    a = open("/tmp/recv", 'r')
    s = a.read()
    os.remove("/tmp/recv")
    return s

def draw_map(frame, r_state, timer, map):
    global is_changed
    is_changed = False

    row = map[0]
    col = map[1]
    mine_num = map[2]

    buttons = {}
    states = {}

    #start
    send("~start" + str(row) + "," + str(col) + "," + str(mine_num) + "$")
    recv()

    for i in range(row):
        for j in range(col):
            btn = Button(frame, width = 1, height = 1, font= ("Arial", 20))
            btn.config(text = ' ')
            btn.grid(row = i, column = j)

            btn.bind("<Button-1>", lambda event, x=i, y=j: left_click(event, x, y, row, col, buttons, states, r_state, timer))
            btn.bind("<Button-3>", lambda event, x=i, y=j: right_click(event, x, y, row, col, buttons, states, r_state, timer))

            buttons[(i, j)] = btn

    r_state.config(text = str(mine_num) + " mines remaining")

def left_click(event, x, y, row, col, buttons, states, r_state, timer):
    send("~click" + str(x) + "," + str(y) + "$")
    s = recv()

    timer.start_timer()

    # game finish
    if s == "~win$" or s == "~lost$":
        timer.stop_timer()

        disable_buttons(row, col, buttons)

        if s == "~win$":
            r_state.config(text = "you win !!!")
        else:
            r_state.config(text = "you lose ...")
        send("~answer$")
        s = recv()

    a = 1
    for i in range(row):
         for j in range(col):
            states[(i, j)] = s[a]
            a += 1
    update_button_state(buttons, states, row, col, timer)

def right_click(event, x, y, row, col, buttons, states, r_state, timer):
    send("~mark" + str(x) + "," + str(y) + "$")
    s = recv()

    a = 1
    for i in range(row):
        for j in range(col):
            states[(i, j)] = s[a]
            a += 1
    update_button_state(buttons, states, row, col, timer)

    # get the number of the remaining mines
    send("~left$")
    num = recv()
    r_state.config(text = str(num) + " mines remaining")

def disable_buttons(row, col, buttons):
    for i in range(row):
        for j in range(col):
            buttons[(i, j)].unbind("<Button-1>")
            buttons[(i, j)].unbind("<Button-3>")
            buttons[(i, j)].config(state = "disabled")

def update_button_state(buttons, states, row, col, timer):
    global is_changed
    is_changed = True

    for i in range(row):
        for j in range(col):
            if states[(i, j)] == 'r':
                buttons[(i, j)].config(text = '#', bg = 'green')
            elif states[(i, j)] == 't':
                buttons[(i, j)].config(text = 'T', bg = 'orange')
            elif states[(i, j)] == '0':
                buttons[(i, j)].config(text = ' ', bg = 'yellow')
            elif states[(i, j)] == '@':
                buttons[(i, j)].config(text = '#')
            elif states[(i, j)] == 'b':
                buttons[(i, j)].config(text = '!', bg = 'red')
            elif states[(i, j)] == '9':
                buttons[(i, j)].config(text = ' ')
            else:
                buttons[(i, j)].config(text = states[(i, j)], bg = 'yellow')

def return_to_start(root, gp, timer):
    global is_changed
    if is_changed == False:
        gp.destroy()
        root.deiconify()
    else:
        sign_page(root, gp, timer)

def sign_page(root, gp, timer):
    sp = Toplevel(gp)
    sp.title("")
    window_width = 800
    window_height = 200
    screen_width = sp.winfo_screenwidth()
    screen_height = sp.winfo_screenheight()
    position_top = int(screen_height/2 - window_height/2)
    position_left = int(screen_width/2 - window_width/2)
    sp.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")

    l = Label(sp, text = "Do you want to leave ?", width = 400, height = 1, font = ("Arial", 30))
    l.pack(padx = 20, pady = 20, anchor = "n")

    b1 = Button(sp, text = "yes", font= ("Arial", 30), width = 10, height = 1, command = partial(go_back, root, gp, sp, timer))
    b1.place(x = 200, y = 150, anchor = "center")

    b2 = Button(sp, text = "no", font= ("Arial", 30), width = 10, height = 1, command = partial(continue_game, sp))
    b2.place(x = 600, y = 150, anchor = "center")

def continue_game(sp):
    sp.destroy()

def go_back(root, gp, sp, timer):
    timer.reset_timer()
    sp.destroy()
    gp.destroy()
    root.deiconify()

class MineSweeperTimer:
    def __init__(self, gp):
        self.root = gp
        self.timer_label = Label(gp, width = 15, height = 1, text = "time used: 0 secs", font = ("Arial", 30))
        self.timer_label.place(x = 300, y = 100, anchor = "center")

        self.time_elapsed = 0
        self.timer_running = False
        self.timer_id = None

    def start_timer(self):
        if not self.timer_running:
            self.timer_running = True
            self.update_timer()

    def update_timer(self):
        if self.timer_running:
            self.timer_label.config(text = f"time used: {self.time_elapsed} secs")
            self.timer_id = self.root.after(1000, self.update_timer)
            self.time_elapsed += 1

    def stop_timer(self):
            self.timer_running = False

    def reset_timer(self):
        self.time_elapsed = 0
        self.timer_label.config(text = "time used: 0 secs")