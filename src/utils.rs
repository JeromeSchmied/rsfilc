use ekreta::{OptIrval, Res};
use log::{debug, info};

/// use `cache_t` as `interval.0` (from) if some
pub fn fix_from(cache_t: Option<ekreta::LDateTime>, mut irval: OptIrval) -> OptIrval {
    debug!("got interval: {irval:?}");
    if let Some(ct) = cache_t.map(|ct| ct.date_naive()) {
        if irval
            .0
            .is_none_or(|from| from < ct && irval.1.is_none_or(|to| to > ct))
        {
            info!("from cached, replacing {:?} to {ct:?}", irval.0);
            irval.0 = Some(ct);
        }
    }
    irval
}
/// convert type name of `T` to a kind name, used for cache
pub fn type_to_kind_name<T>() -> Res<String> {
    let type_name = std::any::type_name::<T>();
    let kind = type_name.split("::").last().ok_or("invalid type_name")?;
    let kind = kind.trim_matches(['<', '>']).to_ascii_lowercase();
    Ok(kind)
}

#[macro_export]
/// generate get fn named `fn_name` for type `ep`, specify whether
/// content once `cached_can_change` or not then sort with `sorting`
macro_rules! gen_get_for {
    ($fn_name:ident, $ep:ty, $cached_can_change:expr, $sorting:expr) => {
        /// get all items between `from` and `to`
        /// # Errors
        /// net
        pub fn $fn_name(&self, mut interval: OptIrval) -> Res<Vec<$ep>> {
            let orig_irval = interval;
            self.load_n_fetch::<$ep>(&mut interval, !$cached_can_change)
                .map(|mut items| {
                    $sorting(&mut items);
                    if orig_irval.0.is_none() {
                        self.store_cache(&items)?;
                    }
                    Ok(items)
                })?
        }
    };
}

/// Fill under `this` with many `with` [`char`]s, inlaying `hint` if any.
///
/// this:   "123456789" <- len: 9
/// with:   '#'
/// hint:   "bab" <- len: 3
///
/// so:     "123456789" <- len: 9
/// result: "12 bab 89" <- len: 9
pub fn fill(this: &str, with: char, hint: Option<&str>) {
    let longest = this.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let inlay_hint = if let Some(il_hint) = hint {
        [" ", il_hint, " "].concat()
    } else {
        String::new()
    };

    let left = (longest - inlay_hint.chars().count()) / 2;
    println!(
        "{}{}{}",
        with.to_string().repeat(left),
        inlay_hint,
        with.to_string()
            .repeat(longest - left - inlay_hint.chars().count())
    );
}

/// print `items` using `to_str`
pub fn print_them_basic<T>(items: impl Iterator<Item = T>, to_str: impl Fn(T) -> String) {
    for item in items {
        let as_str = to_str(item);
        println!("\n\n{as_str}");
        fill(&as_str, '-', None);
    }
}

/// print `num` `items` using `to_str`, reversed if `rev` otherwise not
pub fn print_to_or_rev<T>(items: &[T], num: usize, rev: bool, to_str: impl Fn(&T) -> String) {
    if rev {
        print_them_basic(items.iter().rev().take(num), to_str);
    } else {
        print_them_basic(items.iter().take(num), to_str);
    }
}
// type Disp<T, D: Display> = fn(&T) -> Vec<D>;

type Disp<T> = fn(&T) -> Vec<String>;
/// print `num` `items` using `to_str`, reversed if `rev` otherwise not
pub fn print_table<T, S1: ToString, I: Iterator<Item = S1>>(
    items: &[T],
    headers: I,
    rev: bool,
    num: usize,
    to_str: Disp<T>,
) {
    let mut table = ascii_table::AsciiTable::default();
    for (i, head) in headers.into_iter().enumerate() {
        table.column(i).set_header(head.to_string());
    }
    let data: Vec<_> = if rev {
        items.into_iter().rev().take(num).map(to_str).collect()
    } else {
        items.into_iter().take(num).map(to_str).collect()
    };
    table.print(data);
}
