use {crate::readable::Readable, std::fmt};

pub struct Pagination {
    infix: Option<&'static str>,
    postfix: Option<&'static str>,
    pagers: Vec<Pager>,
    url: String,
}

impl Pagination {
    pub fn new(
        url: impl Into<String>,
        infix: Option<&'static str>,
        postfix: Option<&'static str>,
        pages: u32,
        page: u32,
    ) -> Self {
        Self {
            infix,
            postfix,
            pagers: Self::paginate(pages, page),
            url: url.into(),
        }
    }

    fn paginate(pages: u32, page: u32) -> Vec<Pager> {
        let mut buff = Vec::with_capacity(11);

        buff.push(Pager::Priv(page == 1, if page == 1 { 1 } else { page - 1 }));

        for i in 1..=pages {
            if i == 1 {
                buff.push(Pager::Num(i == page, i));

                continue;
            }

            if i == pages {
                buff.push(Pager::Num(i == page, i));

                continue;
            }

            if (page.checked_sub(1).unwrap_or_else(|| page)
                ..=page.checked_add(1).unwrap_or_else(|| page))
                .contains(&i)
            {
                buff.push(Pager::Num(i == page, i));
            } else if let Some(l) = buff.last_mut() {
                if *l == Pager::Ellipse {
                    continue;
                } else {
                    buff.push(Pager::Ellipse);
                }
            }
        }

        buff.push(Pager::Next(
            page == pages,
            if page == pages { pages } else { page + 1 },
        ));

        buff
    }
}

impl fmt::Display for Pagination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, r#"<div class="pagination">"#)?;

        let infix = self.infix.as_ref().unwrap_or_else(|| &"?page=");

        for pager in &self.pagers {
            match pager {
                Pager::Priv(d, n) => {
                    write!(f, r#"<li class="pagination__item"#)?;
                    if *d {
                        write!(f, " pagination__item--disabled")?;
                    }
                    write!(f, r#"">"#)?;

                    write!(f, r#"<a href="{}{}{}"#, self.url, infix, n)?;
                    if let Some(postfix) = self.postfix.as_ref() {
                        write!(f, "{}", postfix)?;
                    }
                    write!(f, r#"">prev</a>"#)?;

                    writeln!(f, "</li>")?;
                }

                Pager::Num(d, n) => writeln!(
                    f,
                    r#"<li class="pagination__item{}"><a href="{}{}{}{}">{}</a></li>"#,
                    if *d {
                        " pagination__item--disabled"
                    } else {
                        ""
                    },
                    self.url,
                    infix,
                    n,
                    if let Some(postfix) = self.postfix.as_ref() {
                        *postfix
                    } else {
                        ""
                    },
                    n.readable(),
                )?,
                Pager::Ellipse => writeln!(f, r#"<li class="pagination__item"><p>...</p></li>"#)?,

                Pager::Next(d, n) => writeln!(
                    f,
                    r#"<li class="pagination__item{}"><a href="{}{}{}{}">next</a></li>"#,
                    if *d {
                        " pagination__item--disabled"
                    } else {
                        ""
                    },
                    self.url,
                    infix,
                    n,
                    if let Some(postfix) = self.postfix.as_ref() {
                        *postfix
                    } else {
                        ""
                    },
                )?,
            }
        }

        writeln!(f, "</div>")?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Pager {
    Priv(bool, u32),
    Num(bool, u32),
    Ellipse,
    Next(bool, u32),
}
