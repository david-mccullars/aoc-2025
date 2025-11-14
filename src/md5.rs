use rayon::prelude::*;

const CHUNK_SIZE: usize = 100_000;

pub fn md5_search<T, I, S>(prefix: &str, test: T, item: I) -> impl Iterator<Item = S>
where
    T: Fn(&[u8; 16]) -> bool + Send + Sync + Clone,
    I: Fn(&[u8; 16]) -> S + Send + Sync + Clone,
    S: Send + Sync + Ord,
{
    let mut context = md5::Context::new();
    context.consume(prefix.trim());

    let mut chunk_start = 1;

    std::iter::from_fn(move || {
        loop {
            let chunk_end = chunk_start + CHUNK_SIZE;
            let mut found: Vec<_> = (chunk_start..chunk_end)
                .into_par_iter()
                .filter_map(|n| {
                    let mut c2 = context.clone();
                    c2.consume(n.to_string());
                    let digest = c2.finalize();
                    test(&digest.0).then(|| (n, item(&digest.0)))
                })
                .collect();

            chunk_start = chunk_end;

            if !found.is_empty() {
                found.sort();
                return Some(found.into_iter().map(|(_, s)| s));
            }
        }
    })
    .flatten()
}

pub fn nested_md5_list(
    prefix: &str,
    nesting: usize,
    rng: std::ops::Range<usize>,
) -> impl ParallelIterator<Item = (usize, [u8; 16])> {
    let prefix = prefix.trim();

    rng.into_par_iter().map(move |n| {
        let mut context = md5::Context::new();
        context.consume(prefix);
        context.consume(n.to_string());
        let mut digest = context.finalize();

        for _ in 0..nesting {
            context = md5::Context::new();
            for n in digest.0 {
                context.consume(&[hexchar_u8(n / 16), hexchar_u8(n % 16)]);
            }
            digest = context.finalize();
        }

        (n, digest.0)
    })
}

#[inline]
fn hexchar_u8(n: u8) -> u8 {
    if n < 10 {
        48 + n
    } else if n < 16 {
        87 + n
    } else {
        panic!("Invalid hex value");
    }
}
