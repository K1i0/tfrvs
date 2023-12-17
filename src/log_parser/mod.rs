use regex::Regex;
use std::{
    fs::{read_to_string, File},
    io::Write,
};

// Достать из логов индекс, t, r
pub fn parse(file_path_src: String, dst: String, count_max: i64) {
    let mut file = File::create(dst).unwrap();

    let mut counter = 0;

    for line in read_to_string(file_path_src).unwrap().lines() {
        if counter >= count_max {
            break;
        }
        counter += 1;
        let params = line.to_string();
        let params = params.split(' ').collect::<Vec<_>>();

        let re_id = Regex::new(r"JobId=").unwrap();
        let re_start_time = Regex::new(r"StartTime=").unwrap();
        let re_end_time = Regex::new(r"EndTime=").unwrap();
        let re_node_cnt = Regex::new(r"NodeCnt=").unwrap();

        let mut job_id = 0;
        let mut start_time = 0;
        let mut end_time = 0;
        let mut node_count = 0;
        let mut time = 0;

        for param in params {
            if re_start_time.is_match(param) {
                let param = param.to_string();
                let t_vec = param.split(':').collect::<Vec<_>>();

                // println!("{param}: {} {}", t_vec[1], t_vec[2])

                start_time = t_vec[1].to_string().parse::<i64>().unwrap() * 60
                    + t_vec[2].to_string().parse::<i64>().unwrap();
            }

            if re_end_time.is_match(param) {
                let param = param.to_string();
                let t_vec = param.split(':').collect::<Vec<_>>();

                // println!("{param}: {} {}", t_vec[1], t_vec[2])

                end_time = t_vec[1].to_string().parse::<i64>().unwrap() * 60
                    + t_vec[2].to_string().parse::<i64>().unwrap();
            }

            if re_end_time.is_match(param) {
                let param = param.to_string();
                let t_vec = param.split(':').collect::<Vec<_>>();

                // println!("{param}: {} {}", t_vec[1], t_vec[2])

                end_time = t_vec[1].to_string().parse::<i64>().unwrap() * 60
                    + t_vec[2].to_string().parse::<i64>().unwrap();
            }

            if re_node_cnt.is_match(param) {
                let param = param.to_string();
                let t_vec = param.split('=').collect::<Vec<_>>();

                node_count = t_vec[1].to_string().parse::<i64>().unwrap()
            }

            if re_id.is_match(param) {
                let param = param.to_string();
                let t_vec = param.split('=').collect::<Vec<_>>();

                job_id = t_vec[1].to_string().parse::<i64>().unwrap()
            }

            time = {
                if start_time < end_time {
                    end_time - start_time
                } else {
                    3600 - start_time + end_time
                }
            };
        }
        // println!("{job_id} -- {node_count} -- {time}")
        let str_pt = format!("{job_id};{node_count};{time}\n");

        let _ = file.write_all(str_pt.as_bytes());
    }
}
