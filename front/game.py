from tkinter import *
from functools import partial
import lib

def game_page(root, map):
    # hide the start page
    root.withdraw()

    # initialize the game page
    gp = Toplevel(root)
    gp.title("boom")
    gp.resizable(False, False)
    window_width = 1920
    window_height = 1080
    screen_width = gp.winfo_screenwidth()
    screen_height = gp.winfo_screenheight()
    position_top = int(screen_height/2 - window_height/2)
    position_left = int(screen_width/2 - window_width/2)
    gp.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")
    gp.grid_rowconfigure(0, weight = 1)
    gp.grid_columnconfigure(0, weight = 1)

    # scroll the page when the it's not big enough
    canvas = Canvas(gp)
    scbx = Scrollbar(gp, width = 20, orient = "horizontal", command = canvas.xview)
    scby = Scrollbar(gp, width = 20, orient = "vertical", command = canvas.yview)

    canvas.configure(xscrollcommand = scbx.set, yscrollcommand = scby.set)

    frame = Frame(canvas)

    canvas.create_window((960, 540), window = frame, anchor = "center")

    def update(event):
        canvas.configure(scrollregion = canvas.bbox("all"))

    frame.bind("<Configure>", update)

    canvas.grid(row = 0, column = 0, sticky = "nsew")
    scbx.grid(row = 1, column = 0, sticky = "ew")
    scby.grid(row = 0, column = 1, sticky = "ns")

    # tell the number of the unmarked mines, win or lose
    r_state = Label(gp, width = 20, height= 1, text = " ", font= ("Arial", 30))
    r_state.place(relx = 0.5, rely = 0.02, anchor = "n")

    # tell the time
    timer = lib.MineSweeperTimer(gp)

    # the button to go back to the start page
    gb = Button(gp, text = "go back", font= ("Arial", 30), width = 10, height = 1, command = partial(lib.return_to_start, root, gp, timer))
    gb.place(relx = 0.98, rely = 0.02, anchor = "ne")

    lib.draw_map(frame, r_state, timer, map)

    def on_close():
        gp.destroy()
        root.deiconify()

    gp.protocol("WM_DELETE_WINDOW", on_close)