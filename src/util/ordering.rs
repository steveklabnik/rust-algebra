// Proxy is isomorphic to Ordering
pub type Proxy = Result<(),Result<(),()>>;

#[allow(dead_code)]
#[inline]
pub fn reify(p:Proxy) -> Ordering {
    match p {
        Ok(_)       => { Less    },
        Err(Ok (_)) => { Equal   },
        Err(Err(_)) => { Greater },
    }
}
