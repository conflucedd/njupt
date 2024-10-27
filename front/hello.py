from tkinter import *

root = Tk()
root.title("boom")
root.iconbitmap()
root.geometry("1200x800")
root.resizable(False, False)

def open_game_page():
    gp = Toplevel(root)
    gp.title("boom")
    gp.geometry("800x800")

l = Label(root, width = 20, height= 1, text = "choose the level", font= ("Arial", 40))
l.pack(pady = 100)

s = StringVar()
r1 = Radiobutton(root, text = "easy", variable = s, value = "1", font= ("Arial", 30))
r2 = Radiobutton(root, text = "medium", variable = s, value = "2", font= ("Arial", 30))
r3 = Radiobutton(root, text = "hard", variable = s, value = "3", font= ("Arial", 30))
r1.pack(anchor = "w", padx = 450)
r2.pack(anchor = "w", padx = 450)
r3.pack(anchor = "w", padx = 450)

b = Button(root, text = "start", font= ("Arial", 30), width = 10, height = 1, command = open_game_page)
b.place(x = 450, y = 500)

mainloop()