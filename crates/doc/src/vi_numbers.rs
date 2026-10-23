//! Vietnamese number words for the CardText and OrdText field formats (`\* CardText`), as
//! documents write amounts ("một triệu hai trăm năm mươi nghìn đồng").
//!
//! A WordCraft extension: Word (Microsoft 365 for Windows, English interface, observed
//! 2026-10-23) keeps English words for `vi-VN` text. Written from standard Vietnamese
//! orthography; where usage varies, the common written form is chosen:
//! - 1 after "mươi" is "mốt" (21 = hai mươi mốt), but "một" after "mười" and "linh" (11, 101);
//! - 5 after "mười"/"mươi" is "lăm" (15 = mười lăm, 25 = hai mươi lăm), "năm" elsewhere;
//! - 4 after "mươi" is "tư" (24 = hai mươi tư), "bốn" elsewhere (14 = mười bốn, 104 = một trăm
//!   linh bốn);
//! - a zero tens digit before a unit is "linh" (105 = một trăm linh năm; "lẻ" is the southern
//!   variant);
//! - a group of three digits after a larger one is read in full, "không trăm" included
//!   (1 005 = một nghìn không trăm linh năm); groups of zeros are skipped (1 000 000 = một triệu);
//! - thousands are "nghìn" ("ngàn" is the southern variant), then "triệu" and "tỷ"; past a
//!   thousand billion "tỷ" repeats (10^12 = một nghìn tỷ, 10^18 = một tỷ tỷ).
//!
//! Words are lower case, as running text writes them; the field's `\* Caps` and `\* FirstCap`
//! switches capitalise them.

const DIGITS: [&str; 10] = ["không", "một", "hai", "ba", "bốn", "năm", "sáu", "bảy", "tám", "chín"];

const BILLION: u64 = 1_000_000_000;

fn digit(d: u64) -> &'static str {
    usize::try_from(d).ok().and_then(|d| DIGITS.get(d)).copied().unwrap_or("")
}

/// Vietnamese cardinal words for `n`, lower case (`1250000` → "một triệu hai trăm năm mươi
/// nghìn"). Every `u64` has words; `0` is "không".
pub fn cardinal(n: u64) -> String {
    if n == 0 {
        return "không".to_string();
    }
    let mut words = Vec::new();
    push_number(n, false, &mut words);
    words.join(" ")
}

/// Vietnamese ordinal words for `n`: "thứ nhất", "thứ hai", "thứ ba", "thứ tư", then "thứ" and
/// the cardinal ("thứ mười một").
pub fn ordinal(n: u64) -> String {
    match n {
        1 => "thứ nhất".to_string(),
        4 => "thứ tư".to_string(),
        _ => format!("thứ {}", cardinal(n)),
    }
}

/// The words of `n` (> 0). `full` when larger units come before it: its leading group is then
/// read in full ("không trăm linh năm").
fn push_number(n: u64, full: bool, out: &mut Vec<&'static str>) {
    if n >= BILLION {
        // Billions read as a number of their own followed by "tỷ": 10^12 is "một nghìn tỷ".
        push_number(n / BILLION, full, out);
        out.push("tỷ");
        let rest = n % BILLION;
        if rest > 0 {
            push_below_billion(rest, true, out);
        }
    } else {
        push_below_billion(n, full, out);
    }
}

/// The words of `n` (0 < n < 10^9): groups of three digits with "triệu" and "nghìn".
fn push_below_billion(n: u64, full: bool, out: &mut Vec<&'static str>) {
    let groups = [(n / 1_000_000 % 1000, Some("triệu")), (n / 1000 % 1000, Some("nghìn")), (n % 1000, None)];
    let mut started = full;
    for (g, unit) in groups {
        if g == 0 {
            continue;
        }
        push_group(g, started, out);
        if let Some(u) = unit {
            out.push(u);
        }
        started = true;
    }
}

/// The words of one group of three digits (0 < g < 1000). `full` reads the hundreds and a zero
/// tens digit even when they are zero (a group after a larger one).
fn push_group(g: u64, full: bool, out: &mut Vec<&'static str>) {
    let (h, t, u) = (g / 100 % 10, g / 10 % 10, g % 10);
    let hundreds = full || h > 0;
    if hundreds {
        out.push(digit(h));
        out.push("trăm");
    }
    match t {
        0 => {
            if u > 0 {
                if hundreds {
                    out.push("linh");
                }
                out.push(digit(u));
            }
        }
        1 => {
            out.push("mười");
            match u {
                0 => {}
                5 => out.push("lăm"),
                _ => out.push(digit(u)),
            }
        }
        _ => {
            out.push(digit(t));
            out.push("mươi");
            match u {
                0 => {}
                1 => out.push("mốt"),
                4 => out.push("tư"),
                5 => out.push("lăm"),
                _ => out.push(digit(u)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The BA table (NexusOffices C-14b) and the orthography rules in the module note.
    #[test]
    fn cardinal_table() {
        let table: &[(u64, &str)] = &[
            (0, "không"),
            (1, "một"),
            (4, "bốn"),
            (5, "năm"),
            (10, "mười"),
            (11, "mười một"),
            (14, "mười bốn"),
            (15, "mười lăm"),
            (20, "hai mươi"),
            (21, "hai mươi mốt"),
            (24, "hai mươi tư"),
            (25, "hai mươi lăm"),
            (100, "một trăm"),
            (101, "một trăm linh một"),
            (104, "một trăm linh bốn"),
            (105, "một trăm linh năm"),
            (110, "một trăm mười"),
            (111, "một trăm mười một"),
            (115, "một trăm mười lăm"),
            (121, "một trăm hai mươi mốt"),
            (1_000, "một nghìn"),
            (1_005, "một nghìn không trăm linh năm"),
            (1_050, "một nghìn không trăm năm mươi"),
            (10_000, "mười nghìn"),
            (21_000, "hai mươi mốt nghìn"),
            (100_000, "một trăm nghìn"),
            (1_000_000, "một triệu"),
            (1_000_005, "một triệu không trăm linh năm"),
            (1_005_000, "một triệu không trăm linh năm nghìn"),
            (1_250_000, "một triệu hai trăm năm mươi nghìn"),
            (21_000_000, "hai mươi mốt triệu"),
            (1_000_000_000, "một tỷ"),
            (2_000_000_005, "hai tỷ không trăm linh năm"),
            (3_750_000_000, "ba tỷ bảy trăm năm mươi triệu"),
            (1_000_000_000_000, "một nghìn tỷ"),
            (1_001_000_000_000, "một nghìn không trăm linh một tỷ"),
            (
                999_999_999_999_999,
                "chín trăm chín mươi chín nghìn chín trăm chín mươi chín tỷ chín trăm chín mươi chín triệu chín trăm chín mươi chín nghìn chín trăm chín mươi chín",
            ),
            (1_000_000_000_000_000_000, "một tỷ tỷ"),
            (
                u64::MAX,
                "mười tám tỷ bốn trăm bốn mươi sáu triệu bảy trăm bốn mươi tư nghìn không trăm bảy mươi ba tỷ bảy trăm linh chín triệu năm trăm năm mươi mốt nghìn sáu trăm mười lăm",
            ),
        ];
        for (n, words) in table {
            assert_eq!(cardinal(*n), *words, "{n}");
        }
    }

    #[test]
    fn ordinal_words() {
        assert_eq!(ordinal(1), "thứ nhất");
        assert_eq!(ordinal(2), "thứ hai");
        assert_eq!(ordinal(4), "thứ tư");
        assert_eq!(ordinal(5), "thứ năm");
        assert_eq!(ordinal(14), "thứ mười bốn");
        assert_eq!(ordinal(21), "thứ hai mươi mốt");
        assert_eq!(ordinal(0), "thứ không");
    }

    /// Every value has words, with no doubled or stray spaces.
    #[test]
    fn words_are_well_formed() {
        for n in (0..2_000u64).chain([u64::MAX, u64::MAX - 1, BILLION - 1, BILLION + 1, 1 << 40]) {
            let w = cardinal(n);
            assert!(!w.is_empty() && !w.contains("  ") && w.trim() == w, "{n}: {w:?}");
        }
    }
}
