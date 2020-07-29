use {
    html5ever::{driver::ParseOpts, parse_document, tendril::TendrilSink},
    markup5ever_arcdom::{ArcDom, Handle, NodeData},
    std::io,
};

pub fn parse(body: impl AsRef<str>) -> io::Result<String> {
    let mut body = body.as_ref().as_bytes();

    let dom = parse_document(ArcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut body)?;

    let mut buf = String::with_capacity(body.len() / 6);

    convert(&mut buf, &dom.document);

    Ok(buf)
}

fn convert(buf: &mut String, document: &Handle) {
    let node = &document.data;

    match &node {
        NodeData::Comment { .. }
        | NodeData::Doctype { .. }
        | NodeData::ProcessingInstruction { .. } => {}
        NodeData::Document => {
            for node in document.children.borrow().iter() {
                convert(buf, node)
            }
        }
        NodeData::Text { contents } => {
            let mut prev = buf.is_empty() || buf.ends_with(' ') || buf.ends_with('\n');

            for c in contents.borrow().chars() {
                match c {
                    ' ' | '\n' => {
                        if !prev {
                            prev = true;

                            buf.push(' ');
                        }
                    }
                    _ => {
                        prev = false;

                        buf.push(c);
                    }
                }
            }
        }
        NodeData::Element { name, attrs, .. } => {
            let tag: &str = &name.local.to_ascii_lowercase().to_lowercase();
            let attrs = attrs.borrow();

            match tag {
                "head" | "style" | "script" => {}
                _ => {
                    match tag {
                        "a" => buf.push_str("["),
                        "b" | "strong" => buf.push_str("**"),
                        "i" | "em" => buf.push_str("*"),
                        "p" | "div" => {
                            double_newline(buf);
                        }
                        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                            buf.push_str(match tag {
                                "h1" => "# ",
                                "h2" => "## ",
                                "h3" => "### ",
                                "h4" => "#### ",
                                "h5" => "##### ",
                                "h6" => "###### ",
                                _ => unreachable!(),
                            });
                        }
                        "hr" => {
                            newline(buf);
                            buf.push_str("---");
                            newline(buf);
                        }
                        "br" => double_newline(buf),
                        "img" => {
                            let mut src = "";
                            let mut alt = "no alt text";

                            for attr in attrs.iter() {
                                let name: &str =
                                    &attr.name.local.to_ascii_lowercase().to_lowercase();

                                match name {
                                    "alt" => {
                                        alt = &attr.value;
                                    }
                                    "src" => {
                                        src = &attr.value;
                                    }
                                    _ => {}
                                }
                            }

                            buf.push_str("![");
                            buf.push_str(alt);
                            buf.push_str("](");
                            buf.push_str(src);
                            buf.push_str(")")
                        }
                        _ => {}
                    }

                    for node in document.children.borrow().iter() {
                        convert(buf, node)
                    }

                    match tag {
                        "a" => {
                            let mut url = "";

                            for attr in attrs.iter() {
                                let name: &str =
                                    &attr.name.local.to_ascii_lowercase().to_lowercase();

                                if let "href" = name {
                                    url = &attr.value;
                                }
                            }

                            buf.push_str("](");
                            buf.push_str(url);
                            buf.push_str(")")
                        }
                        "b" | "strong" => buf.push_str("**"),
                        "i" | "em" => buf.push_str("*"),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn trim_ending_whitespace(buf: &mut String) {
    while buf.ends_with(' ') || buf.ends_with('\t') {
        let end = buf.len() - 1;

        buf.remove(end);
    }
}

fn double_newline(buf: &mut String) {
    trim_ending_whitespace(buf);

    if !buf.ends_with("\n\n") {
        if buf.ends_with('\n') {
            buf.push('\n')
        } else if !buf.is_empty() {
            buf.push_str("\n\n")
        }
    }
}

fn newline(buf: &mut String) {
    trim_ending_whitespace(buf);

    if buf.ends_with('\n') {
    } else if !buf.is_empty() {
        buf.push('\n')
    }
}
