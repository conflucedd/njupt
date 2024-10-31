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

# level select
level = IntVar()
level.set(1)
r1 = Radiobutton(root, text = "easy", variable = level, value = 1, font= ("Arial", 30))
r2 = Radiobutton(root, text = "medium", variable = level, value = 2, font= ("Arial", 30))
r3 = Radiobutton(root, text = "hard", variable = level, value = 3, font= ("Arial", 30))
r1.place(x = 400, y = 225)
r2.place(x = 400, y = 325)
r3.place(x = 400, y = 425)

b = Button(root, text = "start", font= ("Arial", 30), width = 10, height = 1, command = lambda: game.game_page(root, level.get()))
b.place(x = 375, y = 550)

mainloop()