use std::fs::read_to_string;

// Задачки
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub hight: u64, // высота (время)
    pub width: u64, // ширина (ранг)
}

impl Task {
    // Читать из файла массив задач
    pub fn read_tasks_from_file(file_path: &str, wigth_schedule: u64) -> Vec<Task> {
        let mut result = Vec::new();

        for line in read_to_string(file_path).unwrap().lines() {
            let line = line.to_string();
            let tasl_str = line.split(';').collect::<Vec<_>>();

            assert_eq!(tasl_str.len(), 3);

            //  проверка на размер задач в ширину
            let width = tasl_str[1].to_string().parse().unwrap();

            assert_eq!(width <= wigth_schedule, true);

            result.push(Task {
                id: tasl_str[0].to_string().parse().unwrap(),
                hight: tasl_str[2].to_string().parse().unwrap(),
                width,
            })
        }

        return result;
    }
}
