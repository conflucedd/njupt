from tkinter import *
import game

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
    e1.place(x = 450, y = 475)
    e2.place(x = 575, y = 475)
    e3.place(x = 750, y = 475)

    l1.place(x = 425, y = 475)
    l2.place(x = 550, y = 475)
    l3.place(x = 670, y = 475)

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
r1 = Radiobutton(root, text = "easy", variable = level, value = 1, font= ("Arial", 20), command = place_forget)
r2 = Radiobutton(root, text = "medium", variable = level, value = 2, font= ("Arial", 20), command = place_forget)
r3 = Radiobutton(root, text = "hard", variable = level, value = 3, font= ("Arial", 20), command = place_forget)
r4 = Radiobutton(root, text = "custom", variable = level, value = 4, font= ("Arial", 20), command = place_custom)
r1.place(x = 400, y = 200)
r2.place(x = 400, y = 275)
r3.place(x = 400, y = 350)
r4.place(x = 400, y = 425)

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

mainloop()