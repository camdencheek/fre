use anyhow::{Context, Result};
use fre::*;
use std::str::FromStr;

fn main() -> Result<()> {
    let matches = args::get_app().get_matches();

    // Construct the path to the store file
    let store_file = args::get_store_path(&matches)?;

    // Attempt to read and unmarshal the store file
    let mut usage = store::read_store(&store_file)
        .with_context(|| format!("failed to read store file {:?}", &store_file))?;

    // If a new half life is defined, parse and set it
    if let Some(h) = matches.value_of("halflife") {
        let half_life = h.parse::<f32>().context("parsing halflife")?;
        usage.set_half_life(half_life);
    }

    // TODO write a test for this
    if usage.half_lives_passed() > 5.0 {
        usage.reset_time()
    }

    // Determine the sorting method. Defaults to frecent if unspecified
    let sort_method = match matches.value_of("sort_method") {
        Some("recent") => SortMethod::Recent,
        Some("frequent") => SortMethod::Frequent,
        Some("frecent") => SortMethod::Frecent,
        None => SortMethod::Frecent,
        Some(_) => unreachable!(), // enforced by clap
    };

    // Print the directories if --sorted or --stat are specified
    if matches.is_present("sorted") || matches.is_present("stat") {
        let limit = matches
            .value_of("limit")
            .map(usize::from_str)
            .transpose()
            .context("parse limit")?;

        usage.print_sorted(sort_method, matches.is_present("stat"), limit);
    }

    // Increment a directory
    if matches.is_present("add") {
        // This unwrap is okay because clap should catch a missing directory before this
        let item = matches.value_of("item").unwrap();

        usage.add(item);
    }

    // Handle increasing or decreasing a directory's score by a given weight
    if matches.is_present("increase") || matches.is_present("decrease") {
        // Get a positive weight if increase, negative if decrease
        let weight = match (matches.value_of("increase"), matches.value_of("decrease")) {
            (Some(i), None) => f32::from_str(i).context("parsing increase")?,
            (None, Some(d)) => -1.0 * f32::from_str(d).context("parsing decrease")?,
            _ => unreachable!(), // enforced by clap and block guard
        };

        // Get the item to increase/decrease
        let item = matches.value_of("item").unwrap(); // enforced by clap

        usage.adjust(item, weight);
    }

    // Delete a directory
    if matches.is_present("delete") {
        let item = matches.value_of("item").unwrap();
        usage.delete(item);
    }

    // Truncate store to top N directories
    if let Some(n) = matches.value_of("truncate") {
        let keep_num = n.parse::<usize>().context("parsing truncate")?;
        usage.truncate(keep_num, sort_method);
    }

    // Write the updated store file
    store::write_store(usage, &store_file).context("writing store")?;

    Ok(())
}
