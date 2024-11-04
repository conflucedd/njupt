from tkinter import *

def record_page(root):
    # hide the start page
    root.withdraw()

    # initialize the record page
    rp = Toplevel(root)
    rp.title("record")
    rp.resizable(False, False)
    window_width = 800
    window_height = 600
    screen_width = rp.winfo_screenwidth()
    screen_height = rp.winfo_screenheight()
    position_top = int(screen_height/2 - window_height/2)
    position_left = int(screen_width/2 - window_width/2)
    rp.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")

    l1 = Label(rp, font = ("Arial", 30), width = 30, height = 1)
    l2 = Label(rp, font = ("Arial", 30), width = 30, height = 1)
    l3 = Label(rp, font = ("Arial", 30), width = 30, height = 1)

    with open("/home/carter/njupt/front/record.txt", "r") as file:
        content = file.read()
        parsed_strings = content.split(",")

        l1.config(text = parsed_strings[0])
        l2.config(text = parsed_strings[1])
        l3.config(text = parsed_strings[2])

    l1.place(x = 100, y = 100)
    l2.place(x = 100, y = 250)
    l3.place(x = 100, y = 400)

    gb = Button(rp, text = "go back", font= ("Arial", 30), width = 10, height = 1, command = lambda: go_back(root, rp))
    gb.place(x = 660, y = 50, anchor = "center")

    def on_close():
        rp.destroy()
        root.deiconify()

    rp.protocol("WM_DELETE_WINDOW", on_close)
    
def go_back(root, rp):
    rp.destroy()
    root.deiconify()