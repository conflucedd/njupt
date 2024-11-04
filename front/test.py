from tkinter import *

root = Tk()
window_width = 1000
window_height = 800
screen_width = root.winfo_screenwidth()
screen_height = root.winfo_screenheight()
position_top = int(screen_height/2 - window_height/2)
position_left = int(screen_width/2 - window_width/2)
root.geometry(f"{window_width}x{window_height}+{position_left}+{position_top}")

bomb_image = PhotoImage(file = '/home/carter/njupt/front/bomb.png')

for i in range(10):
        for j in range(10):
            btn = Button(root, width = 256, height = 256, font= ("Arial", 20))
            btn.config(text = ' ')
            btn.grid(row = i, column = j)

            btn.config(image = bomb_image)

            # btn.bind("<Button-1>", lambda event, x=i, y=j: left_click(event, x, y, row, col, buttons, states, r_state, timer))
            # btn.bind("<Button-3>", lambda event, x=i, y=j: right_click(event, x, y, row, col, buttons, states, r_state, timer))

            # buttons[(i, j)] = btn

mainloop()