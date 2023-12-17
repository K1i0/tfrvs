use std::collections::HashMap;

use super::miniTasks::Task;

/// Сжатые задачи. Биг контейнер
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub id: u64,
    pub usage: bool,

    pub hight: u64, // высота (время)
    pub width: u64, // ширина (ранг)\

    pub offset_hight: u64,
    pub offset_width: u64,

    pub vec_tasks: Vec<Task>,
}

impl Rectangle {
    // На вход набор тасок и макс_тетта
    // Из масиива тасок в контейнер (вернуть набор контейнеров) ранг_контейнера, набор_тасок
    pub fn converting(vec_task: &Vec<Task>, max_hight: u64) -> HashMap<u64, Vec<Rectangle>> {
        let mut result_map: HashMap<u64, Vec<Rectangle>> = HashMap::new();

        let mut id: u64 = 0;

        for task in vec_task {
            if !result_map.contains_key(&task.width) {
                result_map.insert(task.width, Vec::new());
            }

            let mut flag = false;

            for rectangle in result_map.get_mut(&task.width).unwrap() {
                // Если в контейнере есть место
                if max_hight - rectangle.hight > task.hight {
                    rectangle.vec_tasks.push(task.clone());
                    rectangle.hight += task.hight;
                    flag = true;
                    break;
                }
            }

            if flag {
                continue;
            }
            // если не получилось добавить в какой-то из контейнеров таску - создаем новый контейнер и добавляем туда нашу задачку

            id += 1;

            // Добавить новый пустой контейнер
            result_map.get_mut(&task.width).unwrap().push(Rectangle {
                id,
                usage: false,
                hight: task.hight,
                width: task.width,

                offset_hight: 0,
                offset_width: 0,
                vec_tasks: vec![task.clone()],
            })
        }
        // Вернуть мапу контейнеров
        return result_map;
    }
}
