from tkinter import *
from functools import partial
import game
import record

# main menu
root = Tk()
root.title("Mine Sweeper")
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

# custom input
e1 = Entry(root, font= ("Arial", 20), width = 5)
e2 = Entry(root, font= ("Arial", 20), width = 5)
e3 = Entry(root, font= ("Arial", 20), width = 5)

l1 = Label(root, text = "x", width = 1, font= ("Arial", 20))
l2 = Label(root, text = "y",width = 1, font= ("Arial", 20))
l3 = Label(root, text = "mines",width = 5, font= ("Arial", 20))

def place_custom():
    e1.place(x = 470, y = 500)
    e2.place(x = 595, y = 500)
    e3.place(x = 770, y = 500)

    l1.place(x = 445, y = 500)
    l2.place(x = 570, y = 500)
    l3.place(x = 690, y = 500)

def place_forget():
    e1.place_forget()
    e2.place_forget()
    e3.place_forget()

    l1.place_forget()
    l2.place_forget()
    l3.place_forget()

# level select
level = IntVar()
level.set(1)
r1 = Radiobutton(root, text = "level 1", variable = level, value = 1, height = 1, font= ("Arial", 30), command = place_forget)
r2 = Radiobutton(root, text = "level 2", variable = level, value = 2, height = 1, font= ("Arial", 30), command = place_forget)
r3 = Radiobutton(root, text = "level 3", variable = level, value = 3, height = 1, font= ("Arial", 30), command = place_forget)
r4 = Radiobutton(root, text = "custom", variable = level, value = 4, height = 1, font= ("Arial", 30), command = place_custom)
r1.place(x = 375, y = 200)
r2.place(x = 375, y = 275)
r3.place(x = 375, y = 350)
r4.place(x = 375, y = 425)

def get_custom():
    if level.get() == 1:
        a1 = 9
        a2 = 9
        a3 = 10
    elif level.get() == 2:
        a1 = 16
        a2 = 16
        a3 = 40
    elif level.get() == 3:
        a1 = 16
        a2 = 30
        a3 = 99
    else:
        a1 = int(e1.get())
        a2 = int(e2.get())
        a3 = int(e3.get())

    return (a1, a2, a3)

b = Button(root, text = "start", font= ("Arial", 30), width = 10, height = 1, command = lambda: game.game_page(root, get_custom()))
b.place(x = 375, y = 575)

record = Button(root, text = "record", font= ("Arial", 30), width = 10, height = 1, command = partial(record.record_page, root))
record.place(x = 375, y = 675)

def quit_game():
    root.destroy()

qt = Button(root, text = "quit", font= ("Arial", 30), width = 10, height = 1, command = quit_game)
qt.place(x = 860, y = 50, anchor = "center")

mainloop()