use std::fmt::Display;

fn insert_range<>(vec: &mut Vec<(u64, u64)>, min: u64, max: u64){
    let len = vec.len();

    // println!("Inserting: {min} {max}");
    let mut did_do_something = false;
    let mut was_between = false;
    let mut inserted_at = 0;

    for i in 0..len{
        inserted_at = i;
        let (min_r, max_r) = vec[i];

        // We want to check the min values first
        // If our min is less than the min_r
        let min_smaller_than_min = min < min_r;
        let min_smaller_than_max = min < max_r;

        let max_smaller_than_min = max < min_r;
        let max_smaller_than_max = max < max_r;
        if (max_smaller_than_min){
            /*
                      <-----> min_r - max_r
            min - max
             */
            // This is a new range
            // println!("\tInsert At {i}");
            vec.insert(i, (min, max));
            did_do_something = true;
            break;
        }else if (min_smaller_than_min && (max_smaller_than_max || max == max_r)) {
            /*
                <-----> min_r    -   max_r
                              ^^^^^^^^^^^^
            min    -               max
             */
            // Then we can just move the min of this range
            // println!("\tJust adjust the min");
            vec[i] = (min, max_r);
            did_do_something = true;
            break;
        }else if (min_smaller_than_min && !max_smaller_than_max){
            /*
                <-----> min_r - max_r <----->
            min                -               max
             */
            vec[i] = (min, max);
            did_do_something = true;
            break;
        }else if (!min_smaller_than_min && max_smaller_than_max){
            /*
            min_r    -         max_r
                 ^^^^^^^^^^^^
                  min - max
             */
            // Then we can ignore this
            // println!("\tIts inbetween");
            was_between = true;
            break;
        }else if ((min_smaller_than_max || min == max_r) && !max_smaller_than_max){
            /*
            min_r    -   max_r
                   ^^^^^^^^^^^^^
                    min          - max
             */
            // Then lets just adjust the max
            // println!("\tJust the max");
            did_do_something = true;
            vec[i] = (min_r, max);
            break;
        }
        // Else move on to the next value
        /*
        min_r - max_r <----->
                              min - max
        */
    }

    if (!did_do_something && !was_between){
        // println!("\tAdd it to the end");
        vec.push((min, max));
    }else if (!was_between){
        // We might need to consolidate
        let (mut min, mut max) = vec[inserted_at];

        let mut did_modify_prev = false;
        let mut did_modify_next = false;

        if (inserted_at > 0){
            // Lets compare
            let (min_p, max_p) = vec[inserted_at - 1];
            if (min <= max_p){
                if (min <= min_p){
                    // Our current one has the lowest min
                }else{
                    // Previous min is lower
                    min = min_p;
                }

                did_modify_prev = true;
            }
        }

        let (min_n, max_n) = vec[inserted_at + 1];
        if (max >= min_n){
            if (max >= max_n){
                // Our max is the greater one
            }else{
                max = max_n;
            }

            did_modify_next= true;
        }


        vec[inserted_at] = (min, max);
        if (did_modify_next){
            vec.remove(inserted_at + 1);
        }

        if (did_modify_prev){
            vec.remove(inserted_at -1);
        }



    }
}

#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    let mut ranges = vec![];
    let mut is_parsing_ranges = true;
    let mut fresh = 0;

    input.lines().for_each(|line| {
        if (line.is_empty()){
            is_parsing_ranges = false;
            return;
        }

        if (is_parsing_ranges){
            let (min_range, max_range) = line.split_once('-').unwrap();
            insert_range(&mut ranges, min_range.parse::<u64>().unwrap(), max_range.parse::<u64>().unwrap());
        }else {
            let id = line.parse::<u64>().unwrap();
            for (min, max) in ranges.iter(){
                if (id < *min) {
                    break;
                }

                if (id >= *min && id <= *max){
                    fresh += 1;
                    break;
                }
            }
        }
    });

    fresh
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    let mut ranges = vec![];
    let mut fresh = 0;


    for line in input.lines(){
        if (line.is_empty()){
            break;
        }

        let (min_range, max_range) = line.split_once('-').unwrap();
        insert_range(&mut ranges, min_range.parse::<u64>().unwrap(), max_range.parse::<u64>().unwrap());
    }

    ranges.iter().fold(0, |acc, &(min, max)| acc + ((max+1)-min))
}
