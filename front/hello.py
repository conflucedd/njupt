from tkinter import *

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

    # draw the map
    for i in range(c):
        for j in range(c):
            btn = Button(frame, width = 1, height = 1)
            btn.grid(row = i, column = j)
            
            def left_click(event):
                button = event.widget
            def right_click(event):
                button = event.widget

            btn.bind("<Button-1>", left_click)
            btn.bind("<Button-3>", right_click)

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