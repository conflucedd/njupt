from tkinter import *

# game page
def open_game_page():
    root.withdraw()
    gp = Toplevel(root)
    gp.title("boom")
    window_width = 1000
    window_height = 800
    screen_width = root.winfo_screenwidth()
    screen_height = root.winfo_screenheight()
    position_top = int(screen_height/2 - window_height/2)
    position_left = int(screen_width/2 - window_width/2)
    gp.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")
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

s = StringVar()
r1 = Radiobutton(root, text = "easy", variable = s, value = "1", font= ("Arial", 30))
r2 = Radiobutton(root, text = "medium", variable = s, value = "2", font= ("Arial", 30))
r3 = Radiobutton(root, text = "hard", variable = s, value = "3", font= ("Arial", 30))
r1.place(x = 400, y = 225)
r2.place(x = 400, y = 325)
r3.place(x = 400, y = 425)

b = Button(root, text = "start", font= ("Arial", 30), width = 10, height = 1, command = open_game_page)
b.place(x = 375, y = 550)

mainloop()