import funcall as fc
import tkinter as tk
from tkinter import *
from tkinter import scrolledtext
from tkinter import filedialog
from tkinter import messagebox
from tkinter import ttk

from functools import partial

def run_condition(entries=[]):
  ready_to_run = True
  for entry in entries:
    if not entry.get():
      ready_to_run = False
  if ready_to_run:
    run_lab_3()
  else:
    show_pop()

def show_pop():
  messagebox.showinfo('Внимание!', 'Для запуска программы необходимо указать все аргументы: n, c1, c2, c3, eps!') 

def run_lab_3():
  fc.call_main_loop(int(n_1.get()), int(c1_1.get()), int(c2_1.get()), int(c3_1.get()), float(eps_1.get()))

textInfo_lab1 = ""
with open("doc/lab1.txt", "r", encoding="utf-8") as ifile:
  textInfo_lab1 = ifile.read()

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

# Создание вкладки для запуска программ
inner_notebook_1 = ttk.Notebook(frame1)
inner_notebook_1.pack()

frame1_1 = tk.Frame(inner_notebook_1, width=700, height=600)
inner_notebook_1.add(frame1_1, text="Лабораторная работа №2")

frame1_2 = tk.Frame(inner_notebook_1, width=700, height=600)
inner_notebook_1.add(frame1_2, text="Лабораторная работа  №3")

# Создание вкладки для теории по лабораторным работам
frame2 = tk.Frame(notebook)
notebook.add(frame2, text="Теория")

inner_notebook = ttk.Notebook(frame2)
inner_notebook.pack()

frame2_0 = tk.Frame(inner_notebook)
inner_notebook.add(frame2_0, text="Теория лабораторной работы №1")

frame2_1 = tk.Frame(inner_notebook)
inner_notebook.add(frame2_1, text="Теория лабораторной работы №2")

frame2_2 = tk.Frame(inner_notebook)
inner_notebook.add(frame2_2, text="Теория лабораторной работы №3")

# ----------------------------------------------------

txt_lab1 = tk.scrolledtext.ScrolledText(frame2_0, width=80, height=34)
txt_lab1.pack()
txt_lab1.insert(tk.END, textInfo_lab1)

txt_lab2 = tk.scrolledtext.ScrolledText(frame2_1, width=80, height=34)
txt_lab2.pack()
txt_lab2.insert(tk.END, textInfo_lab2)

txt_lab3 = tk.scrolledtext.ScrolledText(frame2_2, width=80, height=34)
txt_lab3.pack()
txt_lab3.insert(tk.END, textInfo_lab3)

# ----------------------------------------------------

entries_lab2 = []

# Данные для запуска программ лабораторных
n_1_lbl = ttk.Label(frame1_2, text="Введите количество ЭМ", width=35)
n_1_lbl.grid(column=0, row=0)
n_1 = ttk.Entry(frame1_2, width=30)
n_1.grid(column=1, row=0)
entries_lab2.append(n_1)

c1_1_lbl = ttk.Label(frame1_2, text="Введите параметр генерации c1", width=35)
c1_1_lbl.grid(column=0, row=1)
c1_1 = ttk.Entry(frame1_2, width=30)
c1_1.grid(column=1, row=1)
entries_lab2.append(c1_1)

c2_1_lbl = ttk.Label(frame1_2, text="Введите параметр генерации c2", width=35)
c2_1_lbl.grid(column=0, row=2)
c2_1 = ttk.Entry(frame1_2, width=30)
c2_1.grid(column=1, row=2)
entries_lab2.append(c2_1)

c3_1_lbl = ttk.Label(frame1_2, text="Введите параметр генерации c3", justify='left', width=35)
c3_1_lbl.grid(column=0, row=3)
c3_1 = ttk.Entry(frame1_2, width=30)
c3_1.grid(column=1, row=3)
entries_lab2.append(c3_1)

eps_1_lbl = ttk.Label(frame1_2, text="Введите допустимую погрешность e", justify='left', width=35)
eps_1_lbl.grid(column=0, row=4)
eps_1 = ttk.Entry(frame1_2, width=30)
eps_1.grid(column=1, row=4)
entries_lab2.append(eps_1)

run_btn = ttk.Button(frame1_2, text="Запуск программы", command=partial(run_condition, entries_lab2), width=65)
run_btn.grid(column=0, row=5, columnspan=2, rowspan=2)

# importFileButton = ttk.Button(text="Set Import File", width=110, command=openFile)
# importFileButton.pack()
# exportFileButton = ttk.Button(text="Clear Text Window", width=110, command=clearTextWindow)
# exportFileButton.pack()

root.mainloop()