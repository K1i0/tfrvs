use std::collections::{BTreeMap, HashMap};

use crate::entries::rectangle::Rectangle;

// неограниченный сверху "стакан блять"
#[derive(Debug)]
pub struct ScheduleFFDH {
    pub actual_hight: u64,
    pub max_wigth: u64,
    pub count_lines: u64,

    // OrderedMap, вектор тасок (t, r) набор укрупненных контейнеров в одной полосе, суммарная длина по рангам
    pub lines: BTreeMap<u64, (Vec<Rectangle>, u64)>, // номер полосы -> вектор контейнеров + их суммарная длина

    pub lines_hight: HashMap<u64, u64>,
}

// Методы (типа методы структуры)
impl ScheduleFFDH {
    // Конструктор (типа) ScheduleFFDH === Self
    pub fn new(wigth: u64) -> Self {
        Self {
            actual_hight: 0,
            count_lines: 0,
            max_wigth: wigth,
            lines: BTreeMap::new(),
            lines_hight: HashMap::new(),
        }
    }

    // Добавить контейнер в стакан в полосу (текущую или новую, если срабатывает ограничение по r)
    fn add_rectangle(&mut self, mut rectangle: Rectangle) {
        for (k, (vec, line_size)) in self.lines.iter_mut() {
            // Добавить в текущую полосу контейнер
            if self.max_wigth - * line_size >= rectangle.width {
                rectangle.offset_width = *line_size;
                rectangle.offset_hight = *self.lines_hight.get(k).unwrap();

                vec.push(rectangle.clone());
                *line_size += rectangle.width;
                return;
            }
        }
        // если не нашли места для контейнера - добавляем новую полосу
        self.count_lines += 1;
        self.lines_hight.insert(self.count_lines, self.actual_hight);

        rectangle.offset_hight = self.actual_hight;
        rectangle.offset_width = 0;

        // Запихать контейнер в новую полосу
        self.lines
            .insert(self.count_lines, (vec![rectangle.clone()], rectangle.width));

        // Увеличить тетту стакана
        self.actual_hight += rectangle.hight
    }

    // На вход вектор укрупненных контейнеров
    pub fn containing_rectangels(&mut self, mut vec_rec: Vec<Rectangle>) {
        // Сортировка по времени (неуб)
        vec_rec.sort_by_key(|t| t.hight);
        vec_rec.reverse();

        // Добавляем задачи в СТАКАН
        for rect in vec_rec {
            self.add_rectangle(rect)
        }
    }

    // Генерация векторов, в котором каждый элемент представлен id (таска), набор рангов, набор t (попугаев в течение которых выполняется таска)
    // Результат по вложенным таскам
    pub fn gen_result(&self) -> Vec<(u64, String, Vec<u64>)> {
        let mut result_work_vec: Vec<(u64, String, Vec<u64>)> = Vec::new();

        // Итерация по полосам (?)
        for line in self.lines.iter() {
            // Итерация по БИГ контейнерам
            for rect in line.1.clone().0 {
                let mut rangs = format!("");
                // ранги 
                for i in rect.offset_width..(rect.offset_width + rect.width) {
                    rangs = format!("{rangs} {i}");
                }
                // временные попугаи
                let mut vec_times = Vec::new();
                for i in rect.offset_hight + 1..=rect.offset_hight + rect.hight {
                    vec_times.push(i)
                }
                // таска внутри БИГ контейнера
                for task in rect.vec_tasks {
                    let times_task = vec_times[0..(task.hight) as usize].to_vec();
                    vec_times.drain(0..(task.hight) as usize);

                    result_work_vec.push((task.id, rangs.clone(), times_task));
                    // s = format!("{s} {}:{:#?}", task.id, times_task);
                }
            }
        }
        // возвращаем набор мелких тасков, со временем выполнения и рангом каждого
        return result_work_vec;
    }
}
