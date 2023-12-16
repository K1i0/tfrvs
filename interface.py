import funcall as fc
import tkinter as tk
from tkinter import scrolledtext
from tkinter import filedialog
from tkinter import ttk

textInfo_lab2 = ""
with open("doc/lab2.txt", "r", encoding="utf-8") as ifile:
  textInfo_lab2 = ifile.read()

textInfo_lab3 = ""
with open("doc/lab3.txt", "r", encoding="utf-8") as ifile:
  textInfo_lab3 = ifile.read()

root = tk.Tk()
root.title("Т-я ф-я рас вы-х сис!")
root.geometry("700x600")
root.resizable(False, False)

notebook = ttk.Notebook(root)
notebook.pack()

frame1 = tk.Frame(notebook)
notebook.add(frame1, text="Запуск")

# Создание вкладки для теории по лабораторным работам
frame2 = tk.Frame(notebook)
notebook.add(frame2, text="Теория")

inner_notebook = ttk.Notebook(frame2)
inner_notebook.pack()

frame2_1 = tk.Frame(inner_notebook)
inner_notebook.add(frame2_1, text="Теория лабораторной работы №2")

frame2_2 = tk.Frame(inner_notebook)
inner_notebook.add(frame2_2, text="Теория лабораторной работы №3")

# ----------------------------------------------------

# label1 = tk.Label(frame1, text="Страница 1")
# label1.pack()

# label2 = tk.Label(frame2, text="Страница 2")
# label2.pack()

txt_lab2 = tk.scrolledtext.ScrolledText(frame2_1, width=80, height=34)
txt_lab2.pack()
txt_lab2.insert(tk.END, textInfo_lab2)

txt_lab3 = tk.scrolledtext.ScrolledText(frame2_2, width=80, height=34)
txt_lab3.pack()
txt_lab3.insert(tk.END, textInfo_lab3)

# importFileButton = ttk.Button(text="Set Import File", width=110, command=openFile)
# importFileButton.pack()
# exportFileButton = ttk.Button(text="Clear Text Window", width=110, command=clearTextWindow)
# exportFileButton.pack()

root.mainloop()