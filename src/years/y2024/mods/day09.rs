const INPUT_FILE_PATH: &str = "src/years/y2024/inputs/day09.txt";

fn input_to_string() -> String {
    std::fs::read_to_string(INPUT_FILE_PATH).expect(&format!("File not found at {INPUT_FILE_PATH}"))
}

#[allow(dead_code)]
struct File {
    id: u64,
    location: usize,
    size: usize,
}

#[derive(Clone)]
struct Disk {
    files: Vec<Option<u64>>,
}

impl Disk {
    #[allow(dead_code)]
    fn new(input: String) -> Self {
        let mut file_id = 0;
        input
            .chars()
            .enumerate()
            .fold(Disk { files: Vec::new() }, |mut output, (i, char)| {
                let size = char.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    // file
                    output.files.extend(vec![
                        Some(file_id.to_string().parse::<u64>().unwrap());
                        size
                    ]);
                    file_id += 1;
                } else {
                    // empty space
                    output.files.extend(vec![None; size])
                }
                output
            })
    }

    fn compact_disk_via_fragmenting_files(self) -> Self {
        let mut new_disk = self.clone();

        while let (Some(free_i), Some(content_i)) = (
            new_disk.files.iter().position(|element| *element == None),
            Some(
                new_disk.files.len()
                    - new_disk
                        .files
                        .iter()
                        .rev()
                        .position(|element| element.is_some())
                        .unwrap()
                    - 1,
            ),
        ) {
            new_disk.files[free_i] = new_disk.files[content_i]; // place content into free space
            new_disk.files.remove(content_i); // remove content
        }
        new_disk
    }

    #[allow(dead_code)]
    fn find_file_from_right(&self, start_at: &usize) -> Option<(u64, usize, usize)> {
        // Find the rightmost file up to start_at position
        let position = self.files[..=*start_at]
            .iter()
            .rposition(|element| element.is_some())?;

        // Get the file ID
        let file_id = self.files[position].unwrap();

        // Find the start of this file by scanning left until we find a different value
        let start = (0..=position)
            .rev()
            .find(|&i| self.files[i] != Some(file_id))
            .map_or(0, |i| i + 1);

        // Calculate the size by scanning right until we find a different value
        let size = self.files[start..]
            .iter()
            .take_while(|&&x| x == Some(file_id))
            .count();

        Some((file_id, start, size))
    }

    #[allow(dead_code)]
    fn find_free_space_of_size(&self, size: usize, up_to_position: usize) -> Option<usize> {
        self.files[..up_to_position]
            .windows(size)
            .position(|window| window.iter().all(|element| element.is_none()))
    }

    #[allow(dead_code)]
    fn compact_disk_via_migrating_files(self) -> Self {
        let mut start_at = self.files.len() - 1;
        let mut new_disk: Disk = self.clone();

        // println!("starting with files...\n{}", Disk::files_to_string(&new_disk.files)); // debug

        while let Some((file_id, file_location, file_size)) = self.find_file_from_right(&start_at) {
            // println!("found file (id: {}, lc: {}, sz: {})", file_id, file_location, file_size); // debug
            // let file_print = new_disk.files[file_location..file_location + file_size].to_vec(); // debug
            // println!("showing file: {}", Disk::files_to_string(&file_print)); // debug

            start_at = match file_location.checked_sub(1) {
                Some(x) => x,
                None => break,
            };

            // Only look for free space up to the current file's location
            if let Some(free_location) = new_disk.find_free_space_of_size(file_size, file_location)
            {
                // println!("found free space (lc: {})", free_location); // debug

                for i in free_location..free_location + file_size {
                    new_disk.files[i] = Some(file_id);
                }

                // println!("cloning file to free location...\n{}", Disk::files_to_string(&new_disk.files)); // debug

                for i in file_location..file_location + file_size {
                    new_disk.files[i] = None;
                }
                // println!("deleting file at original location...\n{}", Disk::files_to_string(&new_disk.files)); // debug
                // println!("==="); // debug
            }
        }
        Disk {
            files: new_disk.files,
        }
    }

    #[allow(dead_code)]
    fn files_to_string(files: &Vec<Option<u64>>) -> String {
        files
            .iter()
            .map(|element| {
                if element.is_none() {
                    '.'.to_string()
                } else {
                    element.unwrap().to_string()
                }
            })
            .collect::<String>()
    }

    #[allow(dead_code)]
    fn calculate_checksum(self) -> u64 {
        self.files
            .iter()
            .enumerate()
            .fold(0, |mut total, (i, element)| {
                if let Some(num) = element {
                    total += num * i as u64;
                }
                total
            })
    }
}

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        Disk::calculate_checksum(Disk::compact_disk_via_fragmenting_files(Disk::new(
            input_to_string()
        )))
    )
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{}",
        Disk::calculate_checksum(Disk::compact_disk_via_migrating_files(Disk::new(
            input_to_string()
        )))
    )
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    fn str_to_files(input: &str) -> Vec<Option<u64>> {
        input
            .chars()
            .map(|char| match char {
                '.' => None,
                _ => Some(char.to_digit(10).unwrap() as u64),
            })
            .collect::<Vec<Option<u64>>>()
    }

    use super::*;

    #[ignore]
    #[allow(dead_code)]
    fn test_decompress_disk() {
        let test: Disk = Disk::new("12345".to_string());
        let expected: Vec<Option<u64>> = str_to_files("0..111....22222");

        assert_eq!(test.files, expected)
    }

    #[ignore]
    #[allow(dead_code)]
    fn compact_disk_via_fragmenting_files() {
        let test: Disk = Disk::compact_disk_via_fragmenting_files(Disk::new("12345".to_string()));
        let expected = str_to_files("022111222");

        assert_eq!(test.files, expected)
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_calculate_checksum() {
        let test: u64 = Disk::calculate_checksum(Disk::compact_disk_via_fragmenting_files(
            Disk::new("12345".to_string()),
        ));
        let expected: u64 = 60;

        assert_eq!(test, expected)
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_find_free_space_of_size() {
        let test_disk = Disk::new("12345".to_string());
        let test = Disk::find_free_space_of_size(&test_disk, 2, test_disk.files.len());
        let expected = Some(1);
        assert_eq!(test, expected)
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_find_file_from_right() {
        let test_disk = Disk::new("12345".to_string());
        let start_at = test_disk.files.len() - 1;
        let test = Disk::find_file_from_right(&test_disk, &start_at);
        let expected = Some((2, 10, 5));
        assert_eq!(test, expected)
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_compact_disk_via_migrating_files() {
        let test =
            Disk::compact_disk_via_migrating_files(Disk::new("2333133121414131402".to_string()))
                .files;
        let expected = str_to_files("00992111777.44.333....5555.6666.....8888..");
        assert_eq!(test, expected)
    }
}
