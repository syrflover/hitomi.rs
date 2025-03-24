use reqwest::{
    Method,
    header::{self, HeaderName, HeaderValue},
};

use crate::network::http::{BASE_DOMAIN, request_with_headers};

#[derive(Debug, Clone, Copy)]
pub enum Language {
    All,
    Korean,
    Japanese,
    English,
}

impl Language {
    fn to_nozomi_url(self) -> String {
        match self {
            Language::All => format!("https://ltn.{}/index-all.nozomi", BASE_DOMAIN),
            Language::Korean => format!("https://ltn.{}/index-korean.nozomi", BASE_DOMAIN),
            Language::Japanese => format!("https://ltn.{}/index-japanese.nozomi", BASE_DOMAIN),
            Language::English => format!("https://ltn.{}/index-english.nozomi", BASE_DOMAIN),
        }
    }
}

fn range(page: usize, per_page: usize) -> (usize, usize) {
    let start_bytes = (page - 1) * per_page * 4;
    let end_bytes = start_bytes + per_page * 4 - 1;

    (start_bytes, end_bytes)
}

pub async fn parse(
    lang: impl Into<Option<Language>>,
    page: usize,
    per_page: usize,
) -> crate::Result<Vec<u32>> {
    let lang = (lang.into() as Option<_>).unwrap_or(Language::All);
    let url = lang.to_nozomi_url();

    let (start_bytes, end_bytes) = range(page, per_page);

    tracing::debug!("start_bytes = {}", start_bytes);
    tracing::debug!("end_bytes = {}", end_bytes);

    let range: (HeaderName, HeaderValue) = (
        header::RANGE,
        format!("bytes={}-{}", start_bytes, end_bytes)
            .try_into()
            .unwrap(),
    );

    let resp = request_with_headers(Method::GET, [range].into_iter(), &url).await?;

    let bytes = resp.bytes().await?;

    let mut res = vec![];

    for i in (0..bytes.len()).step_by(4) {
        let mut temp = 0;

        for j in 0..3 {
            // https://github.com/Project-Madome/Madome-Synchronizer/issues/1
            if let Some(a) = bytes.get(i + (3 - j)) {
                let a: u32 = (*a).into();
                temp += a << (j << 3);
            } else {
                break;
            }
        }

        // tracing::debug!("id = {}", temp);

        res.push(temp);
    }

    res.sort_by(|a, b| b.cmp(a));

    tracing::debug!("ids = {res:?}");

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::tests::tracing;

    use super::*;

    #[tokio::test]
    async fn parse_nozomi() {
        tracing();

        let _ids = parse(Language::Korean, 1, 25).await.unwrap();
    }
}
