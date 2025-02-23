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
/// generate get fn named `fn_name` for type `ep` and sort with `sorting`
macro_rules! gen_get_for {
    ($fn_name:ident, $ep:ty, $sorting:expr) => {
        /// get all items between `from` and `to`
        /// # Errors
        /// net
        pub fn $fn_name(&self, mut interval: OptIrval) -> Res<Vec<$ep>> {
            let orig_irval = interval;
            self.load_n_fetch::<$ep>(&mut interval).map(|mut items| {
                $sorting(&mut items);
                if orig_irval.0.is_none() {
                    self.store_cache(&items)?;
                }
                Ok(items)
            })?
        }
    };
}
