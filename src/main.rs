use anyhow::{Context, Result};
use clap::Parser;
use fre::{args::Cli, *};

fn main() -> Result<()> {
    let args = Cli::try_parse()?;

    // Construct the path to the store file
    let store_file = args::get_store_path(&args)?;

    // Attempt to read and unmarshal the store file
    let mut usage = store::read_store(&store_file)
        .with_context(|| format!("failed to read store file {:?}", &store_file))?;

    // If a new half life is defined, parse and set it
    if let Some(h) = args.janitor.halflife {
        usage.set_half_life(h);
    }

    // TODO write a test for this
    if usage.half_lives_passed() > 5.0 {
        usage.reset_time()
    }

    // Print the directories if --sorted or --stat are specified
    if args.stats.sorted || args.stats.stat {
        usage.print_sorted(args.sort_method, args.stats.stat, args.stats.limit);
    }

    // Increment a directory
    if args.updates.add {
        usage.add(args.item.as_ref().expect("add requires an item"));
    }

    // Handle increasing or decreasing a directory's score by a given weight
    if args.updates.increase.is_some() || args.updates.decrease.is_some() {
        // Get a positive weight if increase, negative if decrease
        let weight = match (args.updates.increase, args.updates.decrease) {
            (Some(i), None) => i,
            (None, Some(d)) => -d,
            _ => panic!("increase and decrease cannot both be set"), // enforced by clap and block guard
        };

        usage.adjust(
            args.item
                .as_ref()
                .expect("item is required for increase or decrease"),
            weight,
        );
    }

    // Delete a directory
    if args.updates.delete {
        usage.delete(&args.item.expect("delete requires an item"));
    }

    // Truncate store to top N directories
    if let Some(n) = args.janitor.truncate {
        usage.truncate(n, args.sort_method);
    }

    // Write the updated store file
    store::write_store(usage, &store_file).context("writing store")?;

    Ok(())
}
