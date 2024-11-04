from tkinter import *
from functools import partial
import lib

def game_page(root, map):
    # hide the start page
    root.withdraw()

    # initialize the game page
    gp = Toplevel(root)
    gp.title("boom")
    root.resizable(False, False)
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

    # tell the number of the unmarked mines, win or lose
    r_state = Label(gp, width = 20, height= 1, text = " ", font= ("Arial", 30))
    r_state.place(x = 960, y = 50, anchor = "center")

    # tell the time
    timer = lib.MineSweeperTimer(gp)

    # the button to go back to the start page
    gb = Button(gp, text = "go back", font= ("Arial", 30), width = 10, height = 1, command = partial(lib.return_to_start, root, gp, timer))
    gb.place(x = 1780, y = 50, anchor = "center")

    lib.draw_map(frame, r_state, timer, map)

    def on_close():
        gp.destroy()
        root.deiconify()

    gp.protocol("WM_DELETE_WINDOW", on_close)