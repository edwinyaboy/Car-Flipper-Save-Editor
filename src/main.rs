use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

mod save;
use save::*;

const USAGE: &str = "\
cf-save - Car Flipper save editor

USAGE:
  cf-save                          Interactive editor (picks a save)
  cf-save list                     List saves
  cf-save info   <save>            Show a summary of the save
  cf-save dump   <save> [out]      Dump every property (path, type, value, offset)
  cf-save find   <save> <text>     Find properties whose path contains <text>
  cf-save get    <save> <path>     Print one property
  cf-save set    <save> <path> <v> Set one property and write the file
  cf-save boards <save>            Show the Curbside/Dougslist spawn schedule
  cf-save spawn  <save> <i> <0|1> <hour>
                                   Set spawn slot i to a board (0=Curbside, 1=Dougslist)
  cf-save respawn <save> <i|all>   Mark spawn slot i (or all slots) as unused
  cf-save reseed <save> <n>        Reroll the whole spawn schedule
  cf-save cash   <save> <amount>   Shortcut for set <save> Phone.Cash <amount>
  cf-save level  <save> <amount>   Shortcut for set <save> Phone.Level <amount>
  cf-save xp     <save> <amount>   Shortcut for set <save> Phone.Xp <amount>
  cf-save sync-index <save>        Copy Cash/Level from <save> into CF_Index.sav
  cf-save help

PATHS:
  Properties are addressed by dotted paths.  Array elements use [n]:
    Phone.Cash
    Phone.Level
    Phone.Xp
    Phone.Day
    Phone.SpentToday
    Phone.BestDrift
    Phone.Lifetime.CarsSold
    Phone.SoldCars[0].Listing.AskingPrice
    Phone.XpPending[0].BuyXp
    Phone.BoardFeed.CareerSeed
    Phone.BoardFeed.Slots[0].Hour
    Phone.BoardFeed.Slots[0].Board      (0=Curbside, 1=Dougslist)
    Phone.BoardFeed.Slots[0].bDone
    Phone.BoardFeed.Slots[0].Seed
    Phone.BoardFeed.Slots[0].bCheap

  Use 'find' or 'dump' to discover paths.

NOTES:
  * Quit the game before editing; the game rewrites saves on exit.
  * Changes are written immediately and a .bak is created next to the save.
  * String edits must keep the exact same length (the file is patched in place).
";





fn need_arg<'a>(args: &'a [String], i: usize, what: &str) -> Res<&'a str> {
    args.get(i)
        .map(|s| s.as_str())
        .ok_or_else(|| format!("missing {}", what))
}

fn cli_set(path: &str, prop: &str, value: &str, sync: bool) -> Res<()> {
    let p = PathBuf::from(path);
    let mut ed = Editor::load(&p)?;
    let new = ed.set(prop, value)?;
    let bak = ed.save()?;
    println!("{} = {}", prop, new);
    println!("backup: {}", bak.display());
    if sync {
        if let Some(msg) = sync_index_for(&ed.path, &ed.data, &ed.nodes)? {
            println!("{}", msg);
        }
    }
    Ok(())
}

fn interactive(initial: Option<String>) -> Res<()> {
    let path = match initial {
        Some(p) => PathBuf::from(p),
        None => {
            let saves = list_save_files()?;
            if saves.is_empty() {
                return Err(format!("no .sav files found in {}", save_dir().display()));
            }
            if saves.len() == 1 {
                saves[0].clone()
            } else {
                println!("Saves in {}:", save_dir().display());
                for (i, p) in saves.iter().enumerate() {
                    let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                    let len = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
                    println!("  [{}] {} ({} bytes)", i, name, len);
                }
                print!("Select save [0]: ");
                io::stdout().flush().ok();
                let mut line = String::new();
                io::stdin()
                    .read_line(&mut line)
                    .map_err(|e| e.to_string())?;
                let t = line.trim();
                let idx = if t.is_empty() {
                    0
                } else {
                    t.parse::<usize>()
                        .map_err(|_| "please enter a number".to_string())?
                };
                saves
                    .get(idx)
                    .cloned()
                    .ok_or_else(|| "invalid selection".to_string())?
            }
        }
    };

    let mut ed = Editor::load(&path)?;
    println!();
    ed.print_info();
    println!();
    println!("Commands: show, dump, find <text>, get <path>, set <path> <value>");
    println!("          cash <n>, level <n>, xp <n>, save, quit");
    println!("Edits are kept in memory until you type 'save'.");

    loop {
        print!("\ncf-save> ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if io::stdin()
            .read_line(&mut line)
            .map_err(|e| e.to_string())?
            == 0
        {
            break;
        }
        let line = line.trim().trim_start_matches('\u{feff}');
        if line.is_empty() {
            continue;
        }

        let mut parts = line
            .splitn(3, char::is_whitespace)
            .filter(|s| !s.is_empty());
        let cmd = parts.next().unwrap_or("");
        let a = parts.next();
        let b = parts.next();

        let result: Res<()> = (|| match cmd {
            "help" | "?" => {
                println!("show                  show summary");
                println!("dump                  dump all properties");
                println!("find <text>           find property paths containing <text>");
                println!("get <path>            show a property");
                println!("set <path> <value>    change a property");
                println!("boards                show the Curbside/Dougslist spawn schedule");
                println!("spawn <i> <0|1> <hr>  set spawn slot i to board (0=Curbside, 1=Dougslist) at hour hr");
                println!("respawn <i>           mark spawn slot i as not yet used");
                println!("reseed <n>            reroll the whole board schedule");
                println!("cash <n>              set Phone.Cash");
                println!("level <n>             set Phone.Level");
                println!("xp <n>                set Phone.Xp");
                println!("save                  write changes (creates a .bak)");
                println!("quit                  exit (warns about unsaved changes)");
                Ok(())
            }
            "show" | "info" => {
                ed.print_info();
                Ok(())
            }
            "dump" => {
                print_tree(&ed.nodes, 0);
                Ok(())
            }
            "find" => {
                let needle = a.ok_or("usage: find <text>")?.to_ascii_lowercase();
                let mut flat = Vec::new();
                collect(&ed.nodes, &mut flat);
                for n in flat
                    .iter()
                    .filter(|n| n.path.to_ascii_lowercase().contains(&needle))
                {
                    println!("{} = {}", n.path, n.display);
                }
                Ok(())
            }
            "get" => {
                let p = a.ok_or("usage: get <path>")?;
                let n = ed.get(p)?;
                println!("{} = {}", n.path, n.display);
                Ok(())
            }
            "set" => {
                let p = a.ok_or("usage: set <path> <value>")?;
                let v = b.ok_or("usage: set <path> <value>")?;
                let new = ed.set(p, v)?;
                println!("{} = {}", p, new);
                Ok(())
            }
            "cash" | "money" => {
                let v = a.ok_or("usage: cash <amount>")?;
                let new = ed.set("Phone.Cash", v)?;
                println!("Phone.Cash = {}", new);
                Ok(())
            }
            "boards" | "schedule" => print_boards(&ed),
            "respawn" => {
                let i = a.ok_or("usage: respawn <slot|all>")?;
                if i.eq_ignore_ascii_case("all") {
                    let n = respawn_all(&mut ed)?;
                    println!("marked {} spawn slots as unused", n);
                    return Ok(());
                }
                let p = format!("Phone.BoardFeed.Slots[{}].bDone", i);
                let new = ed.set(&p, "false")?;
                println!("{} = {}", p, new);
                Ok(())
            }
            "spawn" => {
                let i = a.ok_or("usage: spawn <slot> <board 0|1> <hour>")?;
                let rest = b.ok_or("usage: spawn <slot> <board 0|1> <hour>")?;
                let mut it = rest.split_whitespace();
                let board = it.next().ok_or("usage: spawn <slot> <board 0|1> <hour>")?;
                let hour = it.next().ok_or("usage: spawn <slot> <board 0|1> <hour>")?;
                let pb = format!("Phone.BoardFeed.Slots[{}].Board", i);
                let ph = format!("Phone.BoardFeed.Slots[{}].Hour", i);
                let nb = ed.set(&pb, board)?;
                let nh = ed.set(&ph, hour)?;
                println!("{} = {} ({})", pb, nb, board_label(board));
                println!("{} = {}", ph, nh);
                Ok(())
            }
            "reseed" => {
                let v = a.ok_or("usage: reseed <n>")?;
                let new = ed.set("Phone.BoardFeed.CareerSeed", v)?;
                println!("Phone.BoardFeed.CareerSeed = {}", new);
                Ok(())
            }
            "level" => {
                let v = a.ok_or("usage: level <n>")?;
                let new = ed.set("Phone.Level", v)?;
                println!("Phone.Level = {}", new);
                Ok(())
            }
            "xp" => {
                let v = a.ok_or("usage: xp <amount>")?;
                let new = ed.set("Phone.Xp", v)?;
                println!("Phone.Xp = {}", new);
                Ok(())
            }
            "save" => {
                let bak = ed.save()?;
                println!("saved. backup: {}", bak.display());
                if let Some(msg) = sync_index_for(&ed.path, &ed.data, &ed.nodes)? {
                    println!("{}", msg);
                }
                Ok(())
            }
            "quit" | "exit" => {
                if ed.dirty {
                    println!("You have unsaved changes. Type 'save' first, or 'quit!' to discard.");
                    Ok(())
                } else {
                    std::process::exit(0);
                }
            }
            "quit!" => {
                std::process::exit(0);
            }
            other => Err(format!("unknown command '{}' (type 'help')", other)),
        })();

        if let Err(e) = result {
            println!("error: {}", e);
        }
    }
    Ok(())
}

fn run(args: &[String]) -> Res<()> {
    match args.first().map(|s| s.as_str()) {
        None => interactive(None),
        Some("help") | Some("-h") | Some("--help") => {
            print!("{}", USAGE);
            Ok(())
        }
        Some("list") => {
            let saves = list_save_files()?;
            println!("Saves in {}:", save_dir().display());
            for p in saves {
                let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("?");
                let len = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                println!("  {} ({} bytes)", name, len);
            }
            Ok(())
        }
        Some("info") => {
            let p = need_arg(args, 1, "<save>")?;
            let ed = Editor::load(Path::new(p))?;
            ed.print_info();
            Ok(())
        }
        Some("dump") => {
            let p = need_arg(args, 1, "<save>")?;
            let ed = Editor::load(Path::new(p))?;
            if let Some(out) = args.get(2) {
                let mut s = String::new();
                dump_to_string(&ed.nodes, 0, &mut s);
                fs::write(out, s).map_err(|e| format!("{}: {}", out, e))?;
                println!("wrote {}", out);
            } else {
                print_tree(&ed.nodes, 0);
            }
            Ok(())
        }
        Some("find") => {
            let p = need_arg(args, 1, "<save>")?;
            let needle = need_arg(args, 2, "<text>")?.to_ascii_lowercase();
            let ed = Editor::load(Path::new(p))?;
            let mut flat = Vec::new();
            collect(&ed.nodes, &mut flat);
            let mut hits = 0;
            for n in flat
                .iter()
                .filter(|n| n.path.to_ascii_lowercase().contains(&needle))
            {
                println!("{} = {}", n.path, n.display);
                hits += 1;
            }
            if hits == 0 {
                println!("no properties matched '{}'", needle);
            }
            Ok(())
        }
        Some("get") => {
            let p = need_arg(args, 1, "<save>")?;
            let path = need_arg(args, 2, "<path>")?;
            let ed = Editor::load(Path::new(p))?;
            let n = ed.get(path)?;
            println!("{} = {}", n.path, n.display);
            Ok(())
        }
        Some("boards") | Some("schedule") => {
            let p = need_arg(args, 1, "<save>")?;
            let ed = Editor::load(Path::new(p))?;
            print_boards(&ed)
        }
        Some("respawn") => {
            let p = need_arg(args, 1, "<save>")?;
            let i = need_arg(args, 2, "<slot|all>")?;
            if i.eq_ignore_ascii_case("all") {
                let mut ed = Editor::load(Path::new(p))?;
                let n = respawn_all(&mut ed)?;
                let bak = ed.save()?;
                println!("marked {} spawn slots as unused", n);
                println!("backup: {}", bak.display());
                return Ok(());
            }
            let path = format!("Phone.BoardFeed.Slots[{}].bDone", i);
            cli_set(p, &path, "false", false)
        }
        Some("spawn") => {
            let p = need_arg(args, 1, "<save>")?;
            let i = need_arg(args, 2, "<slot>")?;
            let board = need_arg(args, 3, "<board 0|1>")?;
            let hour = need_arg(args, 4, "<hour>")?;
            let bp = format!("Phone.BoardFeed.Slots[{}].Board", i);
            let hp = format!("Phone.BoardFeed.Slots[{}].Hour", i);
            cli_set(p, &bp, board, false)?;
            cli_set(p, &hp, hour, false)
        }
        Some("reseed") => {
            let p = need_arg(args, 1, "<save>")?;
            let v = need_arg(args, 2, "<n>")?;
            cli_set(p, "Phone.BoardFeed.CareerSeed", v, false)
        }
        Some("set") => {
            let p = need_arg(args, 1, "<save>")?;
            let path = need_arg(args, 2, "<path>")?;
            let value = need_arg(args, 3, "<value>")?;
            cli_set(p, path, value, true)
        }
        Some("cash") | Some("money") => {
            let p = need_arg(args, 1, "<save>")?;
            let value = need_arg(args, 2, "<amount>")?;
            cli_set(p, "Phone.Cash", value, true)
        }
        Some("level") => {
            let p = need_arg(args, 1, "<save>")?;
            let value = need_arg(args, 2, "<n>")?;
            cli_set(p, "Phone.Level", value, true)
        }
        Some("xp") => {
            let p = need_arg(args, 1, "<save>")?;
            let value = need_arg(args, 2, "<amount>")?;
            cli_set(p, "Phone.Xp", value, true)
        }
        Some("sync-index") => {
            let p = need_arg(args, 1, "<save>")?;
            let path = PathBuf::from(p);
            let ed = Editor::load(&path)?;
            match sync_index_for(&ed.path, &ed.data, &ed.nodes)? {
                Some(msg) => println!("{}", msg),
                None => println!("nothing to update (no matching slot or values already match)"),
            }
            Ok(())
        }
        Some("edit") => interactive(args.get(1).cloned()),
        Some(other)
            if other.to_ascii_lowercase().ends_with(".sav") || Path::new(other).exists() =>
        {
            interactive(Some(other.to_string()))
        }
        Some(other) => Err(format!("unknown command '{}'\n\n{}", other, USAGE)),
    }
}

fn dump_to_string(nodes: &[Node], ind: usize, out: &mut String) {
    use std::fmt::Write as _;
    for n in nodes {
        for _ in 0..ind {
            out.push_str("  ");
        }
        let _ = writeln!(
            out,
            "{} : {} = {} @0x{:X}",
            n.path, n.typ, n.display, n.val_off
        );
        if !n.children.is_empty() {
            dump_to_string(&n.children, ind + 1, out);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
