use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Output format (kebab, snake, constant, camel, pascal, normal)", default_value = "normal")]
    format: String,
    #[arg(short = 'n', long, help = "Number of names to generate", default_value_t = 1)]
    count: u32,
    #[arg(short, long, help = "Copy first name to clipboard")]
    copy: bool,
    #[arg(short = 's', long, help = "Random seed for reproducible names")]
    seed: Option<u64>,
    #[arg(short = 'p', long, help = "Prefix to add to each name")]
    prefix: Option<String>,
    #[arg(short = 'u', long, help = "Suffix to add to each name")]
    suffix: Option<String>,
    #[arg(long, help = "Show memorability scores")]
    score: bool,
}

const MAX_COUNT: u32 = 1_000_000;

const VERBS: &[&str] = &[
    "searching", "talking", "walking", "eating", "running", "sleeping", "dancing", "reading",
    "writing", "swimming", "jumping", "singing", "cooking", "driving", "painting", "playing",
    "watching", "listening", "thinking", "building", "flying", "laughing", "crying", "hugging",
    "kissing", "fighting", "winning", "losing", "growing", "shrinking", "breathing", "climbing",
    "drawing", "exploring", "fishing", "gardening", "hiking", "inventing", "jogging", "knitting",
    "learning", "meditating", "navigating", "observing", "photographing", "questioning", "racing",
    "sailing", "teaching", "understanding", "vacationing", "whispering", "yawning", "zooming",
    "admiring", "baking", "chasing", "dreaming", "enjoying", "forgiving", "gathering", "hoping",
    "ignoring", "joking", "kicking", "loving", "moving", "noticing", "opening", "praying",
    "quitting", "resting", "smiling", "traveling", "unlocking", "viewing", "waving", "yearning",
    "acting", "bathing", "cleaning", "digging", "editing", "feeding", "grilling", "hunting",
    "investing", "juggling", "lifting", "mixing", "napping", "organizing", "picking", "quarreling",
    "riding", "shopping", "typing", "unwinding", "visiting", "washing", "yodeling", "zapping",
    "camping", "drinking", "falling", "glowing", "healing", "joining", "keeping", "leaving",
    "marching", "nodding", "owning", "packing", "quoting", "rolling", "sneaking", "tasting",
    "voting", "waiting", "yelling", "zoning", "arranging", "borrowing", "escaping", "framing",
    "glancing", "holding", "judging", "kneeling", "launching", "measuring", "nesting", "offering",
    "pacing", "quizzing", "reaching", "serving", "taming", "wishing", "answering", "blocking",
    "calling", "drilling", "exchanging", "glimpsing", "hiding", "jamming", "licking", "naming",
    "peering", "recording", "shining", "timing", "aiming", "blinking", "chewing", "dropping",
    "folding", "gazing", "helping", "igniting", "loading", "meeting",
];

const NOUNS: &[&str] = &[
    "laptop", "bathroom", "window", "banana", "phone", "kitchen", "book", "car", "tree", "mirror",
    "house", "door", "table", "chair", "computer", "garden", "river", "mountain", "city", "street",
    "beach", "ocean", "forest", "desert", "bridge", "tower", "castle", "ship", "plane", "train",
    "apple", "bicycle", "cloud", "doorway", "elephant", "flower", "guitar", "hat", "island",
    "jacket", "kite", "lamp", "moon", "notebook", "piano", "quilt", "rose", "sun", "turtle",
    "umbrella", "violin", "whale", "xylophone", "yacht", "zebra", "airplane", "balloon", "camera",
    "diamond", "engine", "fountain", "globe", "helmet", "iceberg", "jungle", "key", "lantern",
    "museum", "nest", "orchard", "pyramid", "quarry", "rainbow", "statue", "temple", "universe",
    "volcano", "waterfall", "yogurt", "zoo", "anchor", "butterfly", "candle", "drum", "envelope",
    "feather", "glove", "honey", "ink", "jewel", "kettle", "leaf", "map", "needle", "owl", "pearl",
    "queen", "ring", "sword", "throne", "unicorn", "vase", "wagon", "yarn", "zodiac", "album",
    "brush", "cactus", "desk", "eraser", "folder", "gym", "houseplant", "journal", "keyboard",
    "lunchbox", "magnet", "organizer", "pencil", "ruler", "stapler", "textbook", "wallet",
    "briefcase", "dresser", "eyebrow", "fingernail", "goggles", "handkerchief", "icecube",
    "keychain", "lips", "mustache", "necklace", "overcoat", "poncho", "scarf", "tiara", "uniform",
    "vneck", "watch", "zircon", "apartment", "bakery", "cafeteria", "diner", "elevator",
    "farmhouse", "garage", "hospital", "inn", "jail", "laundry", "motel", "nursery", "office",
    "pharmacy", "restaurant", "school", "tavern", "university", "villa", "warehouse", "youth",
    "aquarium", "canal", "dam", "fjord", "glacier", "harbor", "kayak", "lagoon", "marsh",
    "peninsula", "river", "swamp", "tundra", "valley", "yard", "zenith", "asteroid", "barnacle",
    "coral", "dune", "ecosystem", "fen", "gorge", "hill", "isthmus", "knoll", "lake", "meadow",
    "oasis", "plateau", "quagmire", "ridge", "savanna", "trail", "upland", "verge", "wasteland",
    "xeric", "zone",
];

fn main() {
    let args = Args::parse();

    if args.count > MAX_COUNT {
        eprintln!("Error: count cannot exceed {}", MAX_COUNT);
        std::process::exit(1);
    }

    if let Some(seed) = args.seed {
        fastrand::seed(seed);
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let generated_names = if args.count > 0 {
        generate_and_print(&args, &mut handle)
    } else {
        Vec::new()
    };

    if args.copy {
        if !generated_names.is_empty() {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let text_to_copy = if generated_names.len() == 1 {
                    generated_names[0].clone()
                } else {
                    generated_names.join("\n")
                };
                
                if clipboard.set_text(&text_to_copy).is_ok() {
                    let count = generated_names.len();
                    let item = if count == 1 { "item" } else { "items" };
                    eprintln!("\nCopied {} {} to clipboard", count, item);
                }
            }
        }
    }
}

fn generate_and_print(args: &Args, handle: &mut impl std::io::Write) -> Vec<String> {
    let mut generated_names = if args.copy {
        Vec::with_capacity(args.count as usize)
    } else {
        Vec::new()
    };

    let format_lower = args.format.to_lowercase();
    let format_str = format_lower.as_str();
    let prefix = args.prefix.as_deref().unwrap_or("");
    let suffix = args.suffix.as_deref().unwrap_or("");
    let uppercase_prefix = args.prefix.as_ref().map(|p| p.to_uppercase());
    let uppercase_suffix = args.suffix.as_ref().map(|s| s.to_uppercase());

    let mut buffer = String::with_capacity(256);
    let buffer_size = (args.count as usize).min(1_000_000) * 30;
    let mut output_buffer = if args.copy || args.score {
        String::new()
    } else {
        String::with_capacity(buffer_size)
    };

    for _ in 0..args.count {
        buffer.clear();
        let verb = VERBS[fastrand::usize(..VERBS.len())];
        let noun = NOUNS[fastrand::usize(..NOUNS.len())];

        let name = match format_str {
            "kebab" => {
                if !prefix.is_empty() {
                    buffer.push_str(prefix);
                    buffer.push('-');
                }
                buffer.push_str(verb);
                buffer.push('-');
                buffer.push_str(noun);
                if !suffix.is_empty() {
                    buffer.push('-');
                    buffer.push_str(suffix);
                }
                buffer.trim_matches('-').to_string()
            },
            "snake" => {
                if !prefix.is_empty() {
                    buffer.push_str(prefix);
                    buffer.push('_');
                }
                buffer.push_str(verb);
                buffer.push('_');
                buffer.push_str(noun);
                if !suffix.is_empty() {
                    buffer.push('_');
                    buffer.push_str(suffix);
                }
                buffer.replace("__", "_")
            },
            "constant" => {
                if let Some(ref p) = uppercase_prefix {
                    buffer.push_str(p);
                    buffer.push('_');
                }
                buffer.push_str(&verb.to_uppercase());
                buffer.push('_');
                buffer.push_str(&noun.to_uppercase());
                if let Some(ref s) = uppercase_suffix {
                    buffer.push('_');
                    buffer.push_str(s);
                }
                buffer.clone()
            },
            "camel" => {
                buffer.push_str(prefix);
                buffer.push_str(verb);
                capitalize_to(&noun, &mut buffer);
                if !suffix.is_empty() {
                    capitalize_to(suffix, &mut buffer);
                }
                buffer.clone()
            },
            "pascal" => {
                capitalize_to(prefix, &mut buffer);
                capitalize_to(verb, &mut buffer);
                capitalize_to(noun, &mut buffer);
                if !suffix.is_empty() {
                    capitalize_to(suffix, &mut buffer);
                }
                buffer.clone()
            },
            _ => {
                if !prefix.is_empty() {
                    buffer.push_str(prefix);
                    buffer.push(' ');
                }
                buffer.push_str(verb);
                buffer.push(' ');
                if !suffix.is_empty() {
                    buffer.push_str(suffix);
                    buffer.push(' ');
                }
                buffer.push_str(noun);
                buffer.replace("  ", " ")
            }
        };

        if args.copy {
            generated_names.push(name.clone());
        }

        if args.score {
            let score = calculate_memorability_score(&name);
            writeln!(handle, "{} (score: {:.1})", name, score).ok();
        } else if args.copy {
            output_buffer.push_str(&name);
            output_buffer.push('\n');
        } else {
            output_buffer.push_str(&name);
            output_buffer.push('\n');
        }
    }

    if !output_buffer.is_empty() {
        handle.write_all(output_buffer.as_bytes()).ok();
    }

    generated_names
}

fn capitalize_to(s: &str, buffer: &mut String) {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        buffer.extend(first.to_uppercase());
        buffer.extend(chars);
    }
}

fn calculate_memorability_score(name: &str) -> f64 {
    let cleaned = name.replace(['_', '-'], " ");
    let clean_name = cleaned.trim();
    let words: Vec<&str> = clean_name.split_whitespace().collect();
    
    let mut score = 50.0;
    
    // Length penalty/bonus
    let length = clean_name.len();
    if length <= 15 {
        score += 10.0;
    } else if length > 30 {
        score -= 10.0;
    }
    
    // Word count bonus
    match words.len() {
        2 => score += 15.0,
        3 => score += 10.0,
        1 => score -= 5.0,
        _ => score -= 5.0,
    }
    
    // Phonetic simplicity bonus
    let simple_phonetics = ["a", "e", "i", "o", "u", "k", "p", "t", "m", "n", "l", "r"];
    let simple_count = clean_name.chars().filter(|c| simple_phonetics.contains(&c.to_string().as_str())).count();
    let phonetic_ratio = simple_count as f64 / clean_name.len() as f64;
    score += phonetic_ratio * 20.0;
    
    // Repetition penalty
    let mut char_counts = std::collections::HashMap::new();
    for c in clean_name.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    let max_repetition = char_counts.values().max().unwrap_or(&1);
    if *max_repetition > 3 {
        score -= 5.0;
    }
    
    // Common words bonus
    let common_words = ["app", "run", "go", "do", "make", "get", "set", "new", "old", "big", "small"];
    let common_count = words.iter().filter(|w| common_words.contains(w)).count() as f64;
    score += common_count * 5.0;
    
    // Alliteration bonus
    if words.len() >= 2 {
        let first_chars: Vec<char> = words.iter().filter_map(|w| w.chars().next()).collect();
        let alliterative_count = first_chars.windows(2).filter(|w| w[0] == w[1]).count();
        score += (alliterative_count as f64) * 8.0;
    }
    
    score.max(0.0).min(100.0)
}


