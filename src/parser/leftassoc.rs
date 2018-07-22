macro_rules! left_associative {
    ($first:expr, $subsequent:expr, $combine:expr) => {
        ($first).then(|first| {
            use combine::many;

            // FIXME: Can we avoid creating the intermediate vec qos?
            many($subsequent).map(move |subs: Vec<_>| {
                // FIXME: Can we move-capture app so we don't need a clone?
                subs.into_iter().fold(first.clone(), $combine)
            })
        })
    };
}
