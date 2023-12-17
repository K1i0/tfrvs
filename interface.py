import funcall as fc
import tkinter as tk
from tkinter import *
from tkinter import scrolledtext
from tkinter import filedialog
from tkinter import messagebox
from tkinter import ttk

from functools import partial

def lab3_run_condition(entries=[]):
  ready_to_run = True
  for entry in entries:
    if not entry.get():
      ready_to_run = False
  if ready_to_run:
    run_lab_3()
  else:
    lab3_show_pop()

def lab3_show_pop():
  messagebox.showinfo('Внимание!', 'Для запуска программы необходимо указать все аргументы: n, c1, c2, c3, eps!') 

def run_lab_3():
  fc.call_main_loop(int(n_1.get()), int(c1_1.get()), int(c2_1.get()), int(c3_1.get()), float(eps_1.get()))

def lab2_run_condition(entries=[]):
  ready_to_run = True
  for entry in entries:
    if not entry.get():
      ready_to_run = False
  if ready_to_run:
    run_lab_2()
  else:
    lab2_show_pop()

def lab2_show_pop():
  if choice.get() == "Распарсить файл логов":
    messagebox.showinfo('Внимание!', 'Для запуска парсера необходимо указать все аргументы: path_to_log, log_cnt!')
  else:
    messagebox.showinfo('Внимание!', 'Для запуска программы необходимо указать все аргументы: filePath, tetta, rank!') 

def run_lab_2():
  if choice.get() == "Распарсить файл логов":
    print("Start parsing!")
    fc.call_parse_logs(log_path.get(), int(log_count.get()))
  else:
    if (choosed_algorithm == "NFDH"):
      fc.call_main_loop_lab2(data_path.get(), int(tetta.get()), int(maxr.get()), 1)
    else:
      fc.call_main_loop_lab2(data_path.get(), int(tetta.get()), int(maxr.get()), 2)

def toggle_visibility(entries_parser, entries_runner):
    if choice.get() == "Распарсить файл логов":
      print("Show Parser Info")
      for entry in entries_parser:
          entry.grid(pady=2)
      for entry in entries_lab2_runner:
        entry.grid_remove()
    else:
      print("Show Runner Info")
      for entry in entries_parser:
        entry.grid_remove()
      for entry in entries_runner:
        entry.grid(pady=2)

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
inner_notebook_1 = ttk.Notebook(frame1, width=700)
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
# 
entries_lab2_parser = []
entries_lab2_parser_conditions = []

entries_lab2_runner = []
entries_lab2_runner_conditions = []

entries_lab3 = []

# Данные для запуска программ лабораторных
# ------------Лаб2-------------------------------------

action_lbl = ttk.Label(frame1_1, text="Введите действие", width=50)
action_lbl.grid(column=0, row=0)

choice = StringVar()

parse_rbtn = ttk.Radiobutton(frame1_1, text="Распарсить файл логов", value="Распарсить файл логов", variable=choice, width=50, command=partial(toggle_visibility, entries_lab2_parser, entries_lab2_runner))
parse_rbtn.grid(column=0, row=1)

run_rbtn = ttk.Radiobutton(frame1_1, text="Запустить алгоритм NF/FFDH", value="Запустить алгоритм NF/FFDH", variable=choice, width=50, command=partial(toggle_visibility, entries_lab2_parser, entries_lab2_runner))
run_rbtn.grid(column=1, row=1)

# Элементы, заполняемые при парсинге
log_path_lbl = ttk.Label(frame1_1, text="Введите относительный путь к логам", width=50)
entries_lab2_parser.append(log_path_lbl)
log_path = ttk.Entry(frame1_1, width=50)
entries_lab2_parser.append(log_path)
entries_lab2_parser_conditions.append(log_path)

log_count_lbl = ttk.Label(frame1_1, text="Введите количество строк для парсинга", width=50)
entries_lab2_parser.append(log_count_lbl)
log_count = ttk.Entry(frame1_1, width=50)
entries_lab2_parser.append(log_count)
entries_lab2_parser_conditions.append(log_count)

parse_btn = ttk.Button(frame1_1, text="Запуск парсера", command=partial(lab2_run_condition, entries_lab2_parser_conditions), width=50)
entries_lab2_parser.append(parse_btn)

# Элементы, заполняемые при запуске алгоритма
data_path_lbl = ttk.Label(frame1_1, text="Введите относительный путь к данным", width=50)
entries_lab2_runner.append(data_path_lbl)
data_path = ttk.Entry(frame1_1, width=50)
entries_lab2_runner.append(data_path)
entries_lab2_runner_conditions.append(data_path)

tetta_lbl = ttk.Label(frame1_1, text="Введите максимальное тетта контейнера", width=50)
entries_lab2_runner.append(tetta_lbl)
tetta = ttk.Entry(frame1_1, width=50)
entries_lab2_runner.append(tetta)
entries_lab2_runner_conditions.append(tetta)

maxr_lbl = ttk.Label(frame1_1, text="Введите максимальный ранг задачи (контейнера)", width=50)
entries_lab2_runner.append(maxr_lbl)
maxr = ttk.Entry(frame1_1, width=50)
entries_lab2_runner.append(maxr)
entries_lab2_runner_conditions.append(maxr)

choosed_algorithm = ""

algorithms = ["NFDH", "FFDH"]
algorithms_comb = ttk.Combobox(frame1_1, textvariable=choosed_algorithm,values=algorithms, state="readonly", width=48)
entries_lab2_runner.append(algorithms_comb)
entries_lab2_runner_conditions.append(algorithms_comb)

runner_btn = ttk.Button(frame1_1, text="Запуск алгоритма", command=partial(lab2_run_condition, entries_lab2_runner_conditions), width=50)
entries_lab2_runner.append(runner_btn)

# ------------Лаб3-------------------------------------
n_1_lbl = ttk.Label(frame1_2, text="Введите количество ЭМ", width=35)
n_1_lbl.grid(column=0, row=0)
n_1 = ttk.Entry(frame1_2, width=30)
n_1.grid(column=1, row=0)
entries_lab3.append(n_1)

c1_1_lbl = ttk.Label(frame1_2, text="Введите параметр генерации c1", width=35)
c1_1_lbl.grid(column=0, row=1)
c1_1 = ttk.Entry(frame1_2, width=30)
c1_1.grid(column=1, row=1)
entries_lab3.append(c1_1)

c2_1_lbl = ttk.Label(frame1_2, text="Введите параметр генерации c2", width=35)
c2_1_lbl.grid(column=0, row=2)
c2_1 = ttk.Entry(frame1_2, width=30)
c2_1.grid(column=1, row=2)
entries_lab3.append(c2_1)

c3_1_lbl = ttk.Label(frame1_2, text="Введите параметр генерации c3", justify='left', width=35)
c3_1_lbl.grid(column=0, row=3)
c3_1 = ttk.Entry(frame1_2, width=30)
c3_1.grid(column=1, row=3)
entries_lab3.append(c3_1)

eps_1_lbl = ttk.Label(frame1_2, text="Введите допустимую погрешность e", justify='left', width=35)
eps_1_lbl.grid(column=0, row=4)
eps_1 = ttk.Entry(frame1_2, width=30)
eps_1.grid(column=1, row=4)
entries_lab3.append(eps_1)

run_btn = ttk.Button(frame1_2, text="Запуск программы", command=partial(lab3_run_condition, entries_lab3), width=65)
run_btn.grid(column=0, row=5, columnspan=2, rowspan=2)

root.mainloop()