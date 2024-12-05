use std::collections::HashMap;

pub fn solve(input: String) {
    let mut follows: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut lines = input.lines().peekable();
    while !lines.peek().unwrap().is_empty() {
        let mut pages = lines
            .next()
            .unwrap()
            .split("|")
            .map(|s| s.parse::<i32>().unwrap());

        follows
            .entry(pages.next().unwrap())
            .or_default()
            .push(pages.next().unwrap());
    }

    _ = lines.next();

    let part1: i32 = lines
        .clone()
        .map(|line| {
            let pages: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

            for (index, page) in pages.iter().enumerate() {
                for page_which_must_follow in follows.get(page).unwrap_or(&Vec::new()) {
                    if pages[0..index].contains(page_which_must_follow) {
                        return 0;
                    }
                }
            }

            pages[pages.len() / 2]
        })
        .sum();

    println!("Day 5 part 1: {}", part1);

    let wrongly_ordered_page_lists: Vec<Vec<i32>> = lines
        .filter_map(|line| {
            let pages: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

            for (index, page) in pages.iter().enumerate() {
                for page_which_must_follow in follows.get(page).unwrap_or(&Vec::new()) {
                    if pages[0..index].contains(page_which_must_follow) {
                        return Some(pages);
                    }
                }
            }
            None
        })
        .collect();

    fn reorder(mut pages: Vec<i32>, follows: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        for (index, page) in pages.iter().enumerate() {
            for page_which_must_follow in follows.get(page).unwrap_or(&Vec::new()) {
                if pages[0..index].contains(page_which_must_follow) {
                    let index_of_page_which_must_follow = pages[0..index]
                        .iter()
                        .position(|page| page == page_which_must_follow)
                        .unwrap();
                    pages.swap(index, index_of_page_which_must_follow);
                    return reorder(pages, follows);
                }
            }
        }
        pages
    }

    let part2: i32 = wrongly_ordered_page_lists
        .iter()
        .map(|list| {
            let reordered = reorder(list.to_vec(), &follows);
            reordered[reordered.len() / 2]
        })
        .sum();

    println!("Day 5 part 2: {}", part2);
}
