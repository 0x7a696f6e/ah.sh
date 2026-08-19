use std::cmp::max;
use std::collections::HashMap;
use std::io::{IsTerminal, Write, stdin, stdout};

use comfy_table::{Attribute, Cell, Color, ContentLineStyle, Table, TableStyle, presets::NOTHING};
use crossterm::style::Stylize;

use crate::provider::{ProviderType, get_provider};
use crate::session::Session;

struct LanguageGroup {
    range: String,
    languages: Vec<String>,
}

pub fn print_warning<S: ToString>(msg: S) {
    println!("{}", msg.to_string().yellow());
}

pub fn print_success<S: ToString>(msg: S) {
    println!("{}", msg.to_string().green());
}

pub fn print_error<S: ToString>(msg: S) {
    println!("{}", msg.to_string().red());
}

pub fn print_bold<S: ToString>(msg: S) {
    println!("{}", msg.to_string().bold());
}

pub fn is_interactive() -> bool {
    stdin().is_terminal() && stdout().is_terminal()
}

pub fn ask_confirmation(prompt: &str) -> bool {
    print!("{}", prompt.yellow());
    if stdout().flush().is_err() {
        return false;
    }
    let mut input = String::new();
    if stdin().read_line(&mut input).is_err() {
        return false;
    }
    matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

pub fn print_sessions_list(sessions: &[Session]) {
    fn format_age(time: std::time::SystemTime) -> String {
        let now = std::time::SystemTime::now();
        let secs = now.duration_since(time).unwrap_or_default().as_secs();
        if secs < 60 {
            format!("{secs}s")
        } else if secs < 3600 {
            format!("{}m", secs / 60)
        } else if secs < 86400 {
            format!("{}h", secs / 3600)
        } else if secs < 86400 * 30 {
            format!("{}d", secs / 86400)
        } else if secs < 86400 * 365 {
            format!("{}m", secs / (86400 * 30))
        } else {
            format!("{}y", secs / (86400 * 365))
        }
    }

    let default_headers = ["Index", "ID", "Age", "Provider", "Languages"];
    let mut rows: Vec<Vec<String>> = Vec::with_capacity(sessions.len());

    for (i, s) in sessions.iter().enumerate() {
        let langs = s.languages.join(", ");
        let last_time = max(s.last_used_at, s.last_updated_at);
        let age = format_age(last_time);

        rows.push(vec![
            format!("#{}", i + 1),
            s.id.clone(),
            age,
            s.provider.to_string(),
            langs,
        ]);
    }
    print_sessions_table(&default_headers, &rows);
}

fn print_sessions_table(headers: &[&str], rows: &[Vec<String>]) {
    const SESSION_STYLE: TableStyle = TableStyle::new()
        .header_lines(ContentLineStyle::none().junction('│'))
        .content_lines(ContentLineStyle::none().junction('│'));

    let mut table = Table::new();
    table
        .load_style(SESSION_STYLE)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    let header_cells: Vec<Cell> = headers
        .iter()
        .map(|h| Cell::new(*h).add_attribute(Attribute::Bold).fg(Color::Blue))
        .collect();
    table.set_header(header_cells);

    for row in rows {
        let row_cells: Vec<Cell> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                if i == 1 {
                    Cell::new(cell.as_str()).fg(Color::Green)
                } else {
                    Cell::new(cell.as_str())
                }
            })
            .collect();
        table.add_row(row_cells);
    }

    println!("{table}");
}

pub fn print_provider_list(providers: &[ProviderType]) {
    let mut provider_info: Vec<(String, usize)> = Vec::with_capacity(providers.len());

    for p in providers {
        let langs = get_provider(*p).get_supported_languages();
        provider_info.push((p.to_string(), langs.len()));
    }
    print_provider_table(&provider_info);
}

fn print_provider_table(providers: &[(String, usize)]) {
    let mut table = Table::new();
    table
        .load_style(NOTHING)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Provider")
            .add_attribute(Attribute::Bold)
            .fg(Color::Blue),
        Cell::new("Languages")
            .add_attribute(Attribute::Bold)
            .fg(Color::Blue),
    ]);

    for (name, lang_count) in providers.iter() {
        table.add_row(vec![Cell::new(name), Cell::new(*lang_count)]);
    }

    println!("{table}");
}

pub fn print_provider_show(providers: &[ProviderType]) {
    for (i, provider_type) in providers.iter().enumerate() {
        if i > 0 {
            println!();
        }
        write_provider_languages(*provider_type);
    }
}

pub fn print_session_history(sessions: &[Session]) {
    println!();
    print_sessions_list(sessions);
    println!();
    print!("Enter session number to restore, or press Enter to skip: ");
    let _ = stdout().flush();
}

fn write_provider_languages(pt: ProviderType) {
    let provider = get_provider(pt);
    let supported_languages = provider.get_supported_languages();
    let language_to_aliases = provider.get_language_to_aliases();

    let mut aliases: Vec<(String, String)> = Vec::new();
    for language in supported_languages {
        if let Some(lang_aliases) = language_to_aliases.get(language) {
            for alias in lang_aliases {
                aliases.push((language.clone(), alias.clone()));
            }
        }
    }

    print_language_groups(&pt.to_string(), supported_languages, &aliases);
}

fn group_languages_by_alphabet(languages: &[String]) -> Vec<LanguageGroup> {
    let ranges: [(char, char, &str); 5] = [
        ('A', 'E', "A-E"),
        ('F', 'J', "F-J"),
        ('K', 'O', "K-O"),
        ('P', 'T', "P-T"),
        ('U', 'Z', "U-Z"),
    ];

    let mut buckets: [Vec<String>; 5] = [vec![], vec![], vec![], vec![], vec![]];
    for lang in languages {
        let first_char = lang.chars().next().unwrap_or('A').to_ascii_uppercase();
        let idx = ranges
            .iter()
            .position(|(s, e, _)| first_char >= *s && first_char <= *e)
            .unwrap_or(0);
        buckets[idx].push(lang.clone());
    }

    ranges
        .into_iter()
        .zip(buckets)
        .filter(|(_, langs)| !langs.is_empty())
        .map(|((_, _, name), languages)| LanguageGroup {
            range: name.to_string(),
            languages,
        })
        .collect()
}

fn print_language_groups(provider: &str, languages: &[String], aliases: &[(String, String)]) {
    let mut lang_to_aliases: HashMap<String, Vec<String>> = HashMap::new();
    for (lang, alias) in aliases {
        lang_to_aliases
            .entry(lang.clone())
            .or_default()
            .push(alias.clone());
    }

    let groups = group_languages_by_alphabet(languages);

    println!(
        "{} ─────────────────────────────────────────────────",
        format!("Provider: {}", provider).blue().bold()
    );
    print_bold(format!("{} languages:", languages.len()));
    println!();

    for (i, group) in groups.iter().enumerate() {
        if i > 0 {
            println!();
        }

        print_bold(&group.range);

        let mut line_parts: Vec<String> = Vec::new();
        for lang in &group.languages {
            if let Some(alias_list) = lang_to_aliases.get(lang) {
                line_parts.push(format!("{} ({})", lang, alias_list.join(", ")));
            } else {
                line_parts.push(lang.clone());
            }
        }

        println!("{}", line_parts.join(", "));
    }
}
