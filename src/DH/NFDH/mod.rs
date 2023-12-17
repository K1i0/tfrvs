use crate::entries::rectangle::Rectangle;
use anyhow::{bail, Result};
use std::{collections::HashMap, u64};

// Структура стакана заполняемого алгоритмом NFDH
#[derive(Debug)]
pub struct ScheduleNFDH {
    pub actual_hight: u64,
    pub actual_wigth: u64,
    pub max_wigth: u64,

    pub actual_line: u64,

    pub lines: HashMap<u64, Vec<Rectangle>>,
    pub lines_hight: HashMap<u64, u64>,
}

impl ScheduleNFDH {
    // Добавить таску в полосу, если есть место
    fn add_rectangle(&mut self, rect: &mut Rectangle) -> Result<()> {
        // Нет места в полосе
        if rect.width > self.max_wigth - self.actual_wigth {
            bail!("ряд закончился");
        }

        rect.offset_hight = self.actual_hight;
        rect.offset_width = self.actual_wigth;

        self.actual_wigth += rect.width;
        if !self.lines.contains_key(&self.actual_line) {
            self.lines.insert(self.actual_line, Vec::new());
        }

        self.lines
            .get_mut(&self.actual_line)
            .unwrap()
            .push(rect.clone());

        rect.usage = true;
        Ok(())
    }

    // Добавить полоску в обжээкт
    fn add_line(&mut self) {
        // println!("wtf {self:#?}");

        self.lines_hight.insert(self.actual_line, self.actual_hight);

        self.actual_hight += self.lines.get(&self.actual_line).unwrap()[0].hight;

        self.actual_wigth = 0;
        self.actual_line += 1;
    }

    // Конструхтор
    pub fn new(size: u64) -> Self {
        Self {
            actual_hight: 0,
            actual_wigth: 0,
            max_wigth: size,

            actual_line: 1,
            lines: HashMap::new(),
            lines_hight: HashMap::new(),
        }
    }

    // 
    pub fn containing_rectangels(&mut self, mut vec_rec: Vec<Rectangle>) {
        // Сортировка БИГ тасков по времени
        vec_rec.sort_by_key(|t| t.hight);
        vec_rec.reverse();

       // println!("{:#?}", vec_rec);

        // Пока можем извлекать биг таски из очереди (пока не закончились)
        while !vec_rec.is_empty() {
            for task in vec_rec.iter_mut() {
                // Если таска не добавлена (флажок юююзззззэджж)
                if !task.usage {
                    let _ = self.add_rectangle(task);
                }
                // Если нет места в полосе
                if self.max_wigth - self.actual_wigth == 0 {
                    break;
                }
            }

            // Убрать поюзанные контейнеры
            vec_rec.retain(|t| !t.usage);
            // Добавить полоску
            self.add_line()
        }
    }

    // Генерация результата (вектор тасок)
    pub fn gen_result(&self) -> Vec<(u64, String, Vec<u64>)> {
        let mut result_work_vec: Vec<(u64, String, Vec<u64>)> = Vec::new();

        for line in self.lines.iter() {
            for rect in line.1.clone() {
                let mut rangs = format!("");

                for i in rect.offset_width..(rect.offset_width + rect.width) {
                    rangs = format!("{rangs} {i}");
                }

                let mut vec_times = Vec::new();
                for i in rect.offset_hight + 1..=rect.offset_hight + rect.hight {
                    vec_times.push(i)
                }

                for task in rect.vec_tasks {
                    let times_task = vec_times[0..(task.hight) as usize].to_vec();
                    vec_times.drain(0..(task.hight) as usize);

                    result_work_vec.push((task.id, rangs.clone(), times_task));
                    // s = format!("{s} {}:{:#?}", task.id, times_task);
                }
            }
        }

        return result_work_vec;
    }
}
