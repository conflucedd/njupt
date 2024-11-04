from tkinter import *
import lib

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

    l1 = Label(rp, font = ("Arial", 30), width = 30, height = 1, text = "level 1: no record yet")
    l2 = Label(rp, font = ("Arial", 30), width = 30, height = 1, text = "level 2: no record yet")
    l3 = Label(rp, font = ("Arial", 30), width = 30, height = 1, text = "level 3: no record yet")

    with open("/home/carter/njupt/front/record.txt", "r") as file:
        content = file.read()
        parsed_strings = content.split(",")
        # print(parsed_strings[0])
        # print(parsed_strings[1])
        # print(parsed_strings[2])

        l1.config(text = parsed_strings[0])
        l2.config(text = parsed_strings[1])
        l3.config(text = parsed_strings[2])

    l1.config(text = lib.get_record(1))
    l2.config(text = lib.get_record(2))
    l3.config(text = lib.get_record(3))
    l1.place(x = 100, y = 100)
    l2.place(x = 100, y = 250)
    l3.place(x = 100, y = 400)

    strings = [l1.cget("text"), l2.cget("text"), l3.cget("text")]

    def on_close():
        with open("/home/carter/njupt/front/record.txt", "w") as file:
            file.write(",".join(strings))

        rp.destroy()
        root.deiconify()

    rp.protocol("WM_DELETE_WINDOW", on_close)