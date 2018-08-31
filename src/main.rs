#[macro_use]
extern crate serde_derive;

use clap::{App,Arg};
use std::collections::HashMap;
use directories::ProjectDirs;
use serde_json;
use rayon::prelude::*;

use std::fs::{File,self};
use std::time::{SystemTime,UNIX_EPOCH};
use std::f64;
use std::process::exit;
use std::path::Path;
use std::default::Default;
use std::cmp::Ordering;
use std::io::prelude::*;
use std::io::{BufReader,BufWriter};

fn is_int(s: String) -> Result<(),String> {
    match s.parse::<i64>() {
        Ok(_) => Ok(()),
        Err(_e) => Err(format!("invalid integer {}", s))
    }
}

fn main() {

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("store")
             .short("S")
             .long("store")
             .value_name("FILE")
             .help("Use a custom storage file for the directory weights")
             .takes_value(true))
        .arg(Arg::with_name("purge")
             .short("P")
             .long("purge")
             .help("Purge directories that no longer exist from the database")
             .takes_value(false))
        .arg(Arg::with_name("promote")
             .short("p")
             .long("promote")
             .help("Promote a directory by AMOUNT visits")
             .value_name("AMOUNT")
             .takes_value(true))
        .arg(Arg::with_name("demote")
             .short("d")
             .long("demote")
             .help("Demote a directory by AMOUNT visits")
             .value_name("AMOUNT")
             .takes_value(true))
        .arg(Arg::with_name("truncate")
             .short("T")
             .long("truncate")
             .help("Truncate the stored directories to only the top N")
             .value_name("N")
             .validator(is_int)
             .takes_value(true))
        .arg(Arg::with_name("sort_method")
             .short("s")
             .long("sort_method")
             .help("The method to sort by most used")
             .takes_value(true)
             .possible_values(&["frecent","frequent","recent"])
             .default_value("frecent"))
        .arg(Arg::with_name("top")
             .short("t")
             .long("top")
             .help("Print the top N directories")
             .takes_value(true)
             .default_value("0")
             .validator(is_int))
        .get_matches();


    usage.truncate(10, &SortMethod::Frecent);

	fs::create_dir_all(&store_dir).unwrap();
	let file = File::create(&store_file).unwrap();
    let writer = BufWriter::new(file);
	serde_json::to_writer(writer, &usage).unwrap();

}

#[derive(Serialize, Deserialize, Debug)]
struct DirectoryUsage {
    time_created: SystemTime,
	half_life_secs: u64,
    directories: HashMap<String, DirectoryStats>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DirectoryStats {
	score: f64,
	last_accessed: u64,
	num_accesses: u64,
}


impl Default for DirectoryStats {
	fn default() -> DirectoryStats {
		DirectoryStats{
			last_accessed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
			num_accesses: 0,
			score: 0.0, // TODO actually calculate the starting score
		}
	}
}

enum SortMethod {
	Recent,
	Frequent,
	Frecent,
}

impl DirectoryUsage {
	fn purge(&mut self) {
		self.directories.retain(|dir,_| Path::new(&dir).exists());
	}

	fn max(&self, sort_method: &SortMethod) -> String {	
		self.directories.iter()
			.max_by(|(dir1,val1),(dir2,val2)| val1.cmp(val2, sort_method))	
			.unwrap().0
            .to_string()
	}

	fn sorted(&self, sort_method: &SortMethod) -> Vec<String> {
        let mut unsorted_vector: Vec<_> = self.directories
            .iter()
            .collect();


        unsorted_vector.par_sort_by(|(_,val1),(_,val2)| val1.cmp(val2, sort_method).reverse());
        unsorted_vector.iter()
            .map(|&(dir,val)| dir.clone())
            .collect()

	}

	fn truncate(&mut self, keep_num: usize, sort_method: &SortMethod) {
        let sorted = self.sorted(sort_method);
        for dir in sorted.iter().skip(keep_num) {
            self.directories.remove(dir) ;
        }
	}

	fn reset_time(time: SystemTime) {
		unimplemented!();
	}
	
	fn access_dir(&mut self, path: String) {
		self.promote_dir(path, 1.0);
	}

	fn promote_dir(&mut self, path: String, amount: f64) {
		let mut stats = self.directories
			.entry(path)
			.or_insert(DirectoryStats::default());

		stats.num_accesses += amount as u64;
        stats.last_accessed = self.time_created
            .elapsed()
            .expect("Time went backward")
            .as_secs();
		stats.score += amount * 2.0f64.powf(
            stats.last_accessed as f64 / 
			self.half_life_secs as f64
		);
	}

	fn demote_dir(&mut self, path: String, amount: f64) {
		let mut stats = self.directories
			.entry(path)
			.or_insert(DirectoryStats::default());

		stats.num_accesses += amount as u64;
		stats.score -= amount * 2.0f64.powf(
            stats.last_accessed as f64 / 
			self.half_life_secs as f64
		);
	}	
}

impl DirectoryStats {
	fn cmp_frequent(&self, other: &DirectoryStats) -> Ordering {
		self.num_accesses.cmp(&other.num_accesses)
	}

	fn cmp_recent(&self, other: &DirectoryStats) -> Ordering {
		self.last_accessed.cmp(&other.last_accessed)
	}

	fn cmp_frecent(&self, other: &DirectoryStats) -> Ordering {
		self.score.partial_cmp(&other.score).unwrap_or(Ordering::Less)
	}

	fn cmp(&self, other: &DirectoryStats, method: &SortMethod) -> Ordering {
		match method {
			SortMethod::Frequent => self.cmp_frequent(other),
			SortMethod::Recent => self.cmp_recent(other),
			SortMethod::Frecent => self.cmp_frecent(other)
		}
	}
}

fn retrieve_stored_results() {
    let project_dirs = ProjectDirs::from("","",env!("CARGO_PKG_NAME"))
        .expect("Cannot find project directory");

    let store_dir = project_dirs
		.data_dir()
		.to_path_buf();
	
	let mut store_file = store_dir.clone();
	store_file.push(format!("{}.json", env!("CARGO_PKG_NAME")));

    let mut usage: DirectoryUsage = if store_file.is_file() {
        let file = File::open(&store_file)
                .expect(&format!("Cannot open file {}", &store_file.to_str().unwrap()));
        let mut reader = BufReader::new(file);
		serde_json::from_reader(reader)
            .expect("Cannot unmarshal json from storage file")
    } else {
		DirectoryUsage{
			time_created: SystemTime::now(),
			half_life_secs: 60 * 60 * 24 * 7 * 2, // two week half life
			directories: HashMap::new(),
		}	
	};
}
