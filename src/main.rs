//! 静的サイト生成器。content/<教科>/*.md を読み、dist/ に HTML を書き出す。
//! 実行時サーバーが存在しない（静的ファイルのみ）ため、攻撃面が極めて小さく、
//! GitHub Pages / Cloudflare Pages などで無料公開できる。

use pulldown_cmark::{html, Event, Options, Parser};
use serde::Deserialize;
use std::{fs, path::Path};

const SITE_TITLE: &str = "国公立大 予想問題";

#[derive(Deserialize)]
struct Meta {
    title: String,
    /// "easy"（易化） or "hard"（難化）
    level: String,
    /// 想定年度・志望大学など自由記述（任意）
    #[serde(default)]
    note: String,
    /// 並び順（小さいほど先）
    #[serde(default)]
    order: u32,
}

struct Problem {
    slug: String,
    meta: Meta,
    body_html: String,
    /// 解答・解説（任意。本文中の `@@解答@@` 以降）
    answer_html: String,
}

/// HTML 特殊文字をエスケープ（XSS 対策）
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn level_label(level: &str) -> &'static str {
    match level {
        "easy" => "易化した場合",
        _ => "難化した場合",
    }
}

/// Markdown 本文を HTML に変換。生HTMLは無害化（テキスト扱い）し、数式は KaTeX 用に保持。
fn render_md(src: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_MATH);
    let events = Parser::new_ext(src, opts).map(|ev| match ev {
        // 生HTMLを許可しない
        Event::Html(t) | Event::InlineHtml(t) => Event::Text(t),
        Event::InlineMath(m) => Event::Html(format!("<span class=\"math\">\\({}\\)</span>", esc(&m)).into()),
        Event::DisplayMath(m) => Event::Html(format!("<div class=\"math\">\\[{}\\]</div>", esc(&m)).into()),
        e => e,
    });
    let mut out = String::new();
    html::push_html(&mut out, events);
    out
}

fn parse_file(path: &Path) -> Result<Problem, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let raw = raw.replace("\r\n", "\n");
    let rest = raw
        .strip_prefix("+++\n")
        .ok_or_else(|| format!("{}: 先頭に +++ のフロントマターが必要です", path.display()))?;
    let (front, body) = rest
        .split_once("\n+++")
        .ok_or_else(|| format!("{}: フロントマターが閉じられていません", path.display()))?;
    let meta: Meta = toml::from_str(front).map_err(|e| format!("{}: {e}", path.display()))?;
    if meta.level != "easy" && meta.level != "hard" {
        return Err(format!("{}: level は easy か hard", path.display()));
    }
    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'))
        .ok_or_else(|| format!("{}: ファイル名は英数字/-/_ のみ", path.display()))?
        .to_string();
    let (problem, answer) = body.split_once("@@解答@@").unwrap_or((body, ""));
    Ok(Problem { slug, body_html: render_md(problem), answer_html: render_md(answer), meta })
}

/// ページごとの SEO 情報
struct Seo {
    /// meta description（検索結果の説明文）
    desc: String,
    /// サイトルートからの相対パス（例: "math/easy-arts.html"、トップは ""）
    path: String,
    /// パンくず（名前, パス）。トップ以外で指定
    crumbs: Vec<(String, String)>,
    /// 学習教材ページなら true（構造化データを付ける）
    resource: bool,
}

/// 公開URL（環境変数 SITE_URL。末尾スラッシュ無し）。未設定ならローカル用で canonical 等を出さない。
fn site_url() -> Option<String> {
    std::env::var("SITE_URL")
        .ok()
        .map(|u| u.trim_end_matches('/').to_string())
        .filter(|u| u.starts_with("https://"))
}

/// JSON 文字列用のエスケープ（</script> 対策で < も無害化）
fn json_esc(s: &str) -> String {
    let mut o = String::new();
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '<' => o.push_str("\\u003c"),
            c if (c as u32) < 0x20 => o.push(' '),
            c => o.push(c),
        }
    }
    o
}

/// 静的ファイル(style.css, render.js)の内容から作るキャッシュ回避用の版数（FNV-1a）
fn asset_ver() -> &'static str {
    static V: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    V.get_or_init(|| {
        let mut h: u64 = 0xcbf29ce484222325;
        for f in ["static/style.css", "static/render.js"] {
            for b in fs::read(f).unwrap_or_default() {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        format!("{:08x}", h >> 32)
    })
}

fn page(title: &str, depth_prefix: &str, main: &str, seo: &Seo) -> String {
    let mut head_seo = format!(
        "<meta name=\"description\" content=\"{d}\">\n<meta name=\"robots\" content=\"index,follow\">\n<meta property=\"og:type\" content=\"{ty}\">\n<meta property=\"og:locale\" content=\"ja_JP\">\n<meta property=\"og:site_name\" content=\"{site}\">\n<meta property=\"og:title\" content=\"{t}\">\n<meta property=\"og:description\" content=\"{d}\">\n<meta name=\"twitter:card\" content=\"summary\">\n<meta name=\"twitter:title\" content=\"{t}\">\n<meta name=\"twitter:description\" content=\"{d}\">\n",
        d = esc(&seo.desc),
        t = esc(title),
        site = esc(SITE_TITLE),
        ty = if seo.resource { "article" } else { "website" },
    );
    if let Some(base) = site_url() {
        let url = format!("{}/{}", base, seo.path);
        head_seo.push_str(&format!(
            "<link rel=\"canonical\" href=\"{u}\">\n<meta property=\"og:url\" content=\"{u}\">\n",
            u = esc(&url)
        ));
        // 構造化データ（JSON-LD）
        let mut graph = Vec::new();
        if seo.path.is_empty() {
            graph.push(format!(
                "{{\"@type\":\"WebSite\",\"name\":\"{}\",\"url\":\"{}\",\"inLanguage\":\"ja\"}}",
                json_esc(SITE_TITLE),
                json_esc(&format!("{}/", base))
            ));
        }
        if !seo.crumbs.is_empty() {
            let items: Vec<String> = seo
                .crumbs
                .iter()
                .enumerate()
                .map(|(i, (n, pth))| {
                    format!(
                        "{{\"@type\":\"ListItem\",\"position\":{},\"name\":\"{}\",\"item\":\"{}\"}}",
                        i + 1,
                        json_esc(n),
                        json_esc(&format!("{}/{}", base, pth))
                    )
                })
                .collect();
            graph.push(format!(
                "{{\"@type\":\"BreadcrumbList\",\"itemListElement\":[{}]}}",
                items.join(",")
            ));
        }
        if seo.resource {
            graph.push(format!(
                "{{\"@type\":\"LearningResource\",\"name\":\"{}\",\"description\":\"{}\",\"url\":\"{}\",\"inLanguage\":\"ja\",\"isAccessibleForFree\":true,\"learningResourceType\":\"練習問題\",\"educationalLevel\":\"大学入試\",\"about\":\"数学\"}}",
                json_esc(title),
                json_esc(&seo.desc),
                json_esc(&url)
            ));
        }
        if !graph.is_empty() {
            head_seo.push_str(&format!(
                "<script type=\"application/ld+json\">{{\"@context\":\"https://schema.org\",\"@graph\":[{}]}}</script>\n",
                graph.join(",")
            ));
        }
    }
    format!(
        r##"<!doctype html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'self' https://cdn.jsdelivr.net; script-src 'self' https://cdn.jsdelivr.net; font-src https://cdn.jsdelivr.net; img-src 'self' data:; base-uri 'none'; form-action 'none'">
<meta name="referrer" content="no-referrer">
<meta name="theme-color" content="#1f4e8c">
<meta name="google-site-verification" content="io-t3XTjl84jCqitU8efk720dZ0Vqz74OxqG7ggCUs4">
<title>{title}</title>
{head_seo}<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'%3E%3Crect width='32' height='32' rx='6' fill='%231f4e8c'/%3E%3Ctext x='16' y='23' font-size='20' text-anchor='middle' fill='white' font-family='serif'%3E%E2%88%AB%3C/text%3E%3C/svg%3E">
<link rel="preconnect" href="https://cdn.jsdelivr.net" crossorigin>
<link rel="stylesheet" href="{p}style.css?v={ver}">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/contrib/auto-render.min.js"></script>
<script defer src="{p}render.js?v={ver}"></script>
</head>
<body>
<header><a href="{p}index.html">{site}</a></header>
<main>
{main}
</main>
<footer>無断転載禁止。内容は作成者による予想であり、実際の入試問題とは関係ありません。</footer>
</body>
</html>
"##,
        title = esc(title),
        head_seo = head_seo,
        ver = asset_ver(),
        p = depth_prefix,
        site = esc(SITE_TITLE),
        main = main
    )
}

#[derive(Deserialize)]
struct SubjectMeta {
    name: String,
    #[serde(default)]
    desc: String,
    #[serde(default)]
    order: u32,
}

struct Subject {
    slug: String,
    meta: SubjectMeta,
    problems: Vec<Problem>,
}

fn is_safe_slug(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn load_subjects(root: &Path) -> Result<Vec<Subject>, String> {
    let mut subjects = Vec::new();
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let dir = entry.map_err(|e| e.to_string())?.path();
        if !dir.is_dir() {
            continue;
        }
        let slug = dir.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        if !is_safe_slug(&slug) {
            return Err(format!("{}: 教科フォルダ名は英数字/-/_ のみ", dir.display()));
        }
        let meta_path = dir.join("_subject.toml");
        let raw = fs::read_to_string(&meta_path).map_err(|e| format!("{}: {e}", meta_path.display()))?;
        let meta: SubjectMeta = toml::from_str(&raw).map_err(|e| format!("{}: {e}", meta_path.display()))?;
        let mut problems = Vec::new();
        for f in fs::read_dir(&dir).map_err(|e| e.to_string())? {
            let path = f.map_err(|e| e.to_string())?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                problems.push(parse_file(&path)?);
            }
        }
        problems.sort_by(|a, b| (a.meta.order, &a.slug).cmp(&(b.meta.order, &b.slug)));
        subjects.push(Subject { slug, meta, problems });
    }
    subjects.sort_by(|a, b| (a.meta.order, &a.slug).cmp(&(b.meta.order, &b.slug)));
    Ok(subjects)
}

fn main() -> Result<(), String> {
    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(dist).map_err(|e| e.to_string())?;
    let subjects = load_subjects(Path::new("content"))?;
    let mut total = 0;
    // サイトマップ用のパス一覧
    let mut urls: Vec<String> = vec![String::new()];

    for s in &subjects {
        let sdir = dist.join(&s.slug);
        fs::create_dir_all(&sdir).map_err(|e| e.to_string())?;
        let sname = esc(&s.meta.name);
        let subject_crumbs = vec![
            ("トップ".to_string(), String::new()),
            (s.meta.name.clone(), format!("{}/", s.slug)),
        ];

        // 各問題ページ（解答は別ページ）
        for p in &s.problems {
            total += 1;
            let has_ans = !p.answer_html.trim().is_empty();
            let qcount = p.body_html.matches("<h2>").count();
            let note = if p.meta.note.is_empty() {
                String::new()
            } else {
                format!("<p class=\"note\">{}</p>\n", esc(&p.meta.note))
            };
            let tag = format!("<span class=\"tag {}\">{}</span>", p.meta.level, level_label(&p.meta.level));
            let ans_link = if has_ans {
                format!("\n<p class=\"answer-link\"><a href=\"{}-answer.html\">解答・解説を見る →</a></p>", p.slug)
            } else {
                String::new()
            };
            let main = format!(
                "<p class=\"crumb\"><a href=\"../index.html\">トップ</a> / <a href=\"index.html\">{sname}</a> / {tag}</p>\n<h1>{t}</h1>\n{note}<article>{b}</article>{ans_link}",
                t = esc(&p.meta.title),
                b = p.body_html,
            );
            let path = format!("{}/{}.html", s.slug, p.slug);
            let mut crumbs = subject_crumbs.clone();
            crumbs.push((p.meta.title.clone(), path.clone()));
            let seo = Seo {
                desc: format!(
                    "{}の予想問題（全{}問）を無料で公開。{}",
                    p.meta.note,
                    qcount,
                    if has_ans { "解答・解説付き。" } else { "" }
                ),
                path: path.clone(),
                crumbs,
                resource: true,
            };
            let html = page(&format!("{} {} 予想問題 | {}", p.meta.title, s.meta.name, SITE_TITLE), "../", &main, &seo);
            fs::write(dist.join(&path), html).map_err(|e| e.to_string())?;
            urls.push(path);

            if has_ans {
                let main = format!(
                    "<p class=\"crumb\"><a href=\"../index.html\">トップ</a> / <a href=\"index.html\">{sname}</a> / {tag}</p>\n<h1>{t} 解答・解説</h1>\n{note}<article>{b}</article>\n<p class=\"answer-link\"><a href=\"{slug}.html\">← 問題に戻る</a></p>",
                    t = esc(&p.meta.title),
                    b = p.answer_html,
                    slug = p.slug,
                );
                let apath = format!("{}/{}-answer.html", s.slug, p.slug);
                let mut crumbs = subject_crumbs.clone();
                crumbs.push((format!("{} 解答・解説", p.meta.title), apath.clone()));
                let seo = Seo {
                    desc: format!("{}の解答・解説（全{}問）を無料で公開。", p.meta.note, qcount),
                    path: apath.clone(),
                    crumbs,
                    resource: true,
                };
                let html = page(&format!("{} 解答・解説 {} | {}", p.meta.title, s.meta.name, SITE_TITLE), "../", &main, &seo);
                fs::write(dist.join(&apath), html).map_err(|e| e.to_string())?;
                urls.push(apath);
            }
        }

        // 教科ページ（易化 / 難化）
        let mut main = format!("<p class=\"crumb\"><a href=\"../index.html\">トップ</a> / {sname}</p>\n<h1>{sname} 予想問題</h1>\n");
        for (lv, desc) in [("easy", "全体的に易しくなった場合を想定"), ("hard", "全体的に難しくなった場合を想定")] {
            main.push_str(&format!(
                "<section><h2><span class=\"tag {lv}\">{}</span></h2>\n<p class=\"desc\">{desc}</p>\n<ul class=\"sets\">\n",
                level_label(lv)
            ));
            // セット（例: 易化年①）ごとに1行にまとめる。同じセットの文科・理科は slug の末尾（-arts / -sci）以外が共通。
            let mut rows: Vec<(String, String, Vec<(String, String)>)> = Vec::new(); // (key, セット名, [(科, slug)])
            for p in s.problems.iter().filter(|p| p.meta.level == lv) {
                let key = p.slug.rsplit_once('-').map(|x| x.0).unwrap_or(&p.slug).to_string();
                let title = p.meta.title.as_str();
                let split = title.find(" 文科").or_else(|| title.find(" 理科"));
                let (set_name, course) = match split {
                    Some(i) => (&title[..i], &title[i + 1..]),
                    None => (title, title),
                };
                match rows.last_mut() {
                    Some(last) if last.0 == key => last.2.push((course.to_string(), p.slug.clone())),
                    _ => rows.push((key, set_name.to_string(), vec![(course.to_string(), p.slug.clone())])),
                }
            }
            for (_, set_name, courses) in &rows {
                main.push_str(&format!("<li class=\"setrow\"><span class=\"setname\">{}</span><span class=\"setlinks\">", esc(set_name)));
                for (course, slug) in courses {
                    main.push_str(&format!("<a href=\"{}.html\">{}</a>", slug, esc(course)));
                }
                main.push_str("</span></li>\n");
            }
            if rows.is_empty() {
                main.push_str("<li class=\"empty\">準備中</li>\n");
            }
            main.push_str("</ul></section>\n");
        }
        let spath = format!("{}/", s.slug);
        let seo = Seo {
            desc: format!("{}。易化した場合・難化した場合の予想問題を、文科・理科ごとに無料で公開（解答・解説付き）。", s.meta.desc),
            path: spath.clone(),
            crumbs: subject_crumbs.clone(),
            resource: false,
        };
        fs::write(
            sdir.join("index.html"),
            page(&format!("{} 予想問題一覧 | {}", s.meta.name, SITE_TITLE), "../", &main, &seo),
        )
        .map_err(|e| e.to_string())?;
        urls.push(spath);
    }

    // トップ：教科選択
    let mut main = String::from(
        "<h1>国公立大 予想問題</h1>\n<p>国公立大学の入試予想問題を無料で公開しています。教科を選んでください。</p>\n<ul class=\"subjects\">\n",
    );
    for s in &subjects {
        main.push_str(&format!(
            "<li><a href=\"{slug}/index.html\"><strong>{n}</strong><span>{d}</span></a></li>\n",
            slug = s.slug,
            n = esc(&s.meta.name),
            d = esc(&s.meta.desc)
        ));
    }
    main.push_str("</ul>\n");
    let subj_names: Vec<&str> = subjects.iter().map(|s| s.meta.name.as_str()).collect();
    let seo = Seo {
        desc: format!(
            "国公立大学の入試予想問題を無料公開。{}の予想問題（易化版・難化版、文科・理科、解答・解説付き）。",
            subj_names.join("・")
        ),
        path: String::new(),
        crumbs: Vec::new(),
        resource: false,
    };
    fs::write(
        dist.join("index.html"),
        page(&format!("{} 無料公開【東大数学ほか】", SITE_TITLE), "", &main, &seo),
    )
    .map_err(|e| e.to_string())?;

    // 404 ページ（検索エンジンに載せない）
    let main404 = "<h1>ページが見つかりません</h1>\n<p><a href=\"index.html\">トップへ戻る</a></p>".to_string();
    let seo404 = Seo {
        desc: "ページが見つかりません。".into(),
        path: "404.html".into(),
        crumbs: Vec::new(),
        resource: false,
    };
    let html404 = page(&format!("404 | {}", SITE_TITLE), "", &main404, &seo404)
        .replace("content=\"index,follow\"", "content=\"noindex\"");
    fs::write(dist.join("404.html"), html404).map_err(|e| e.to_string())?;

    // sitemap.xml / robots.txt（SITE_URL がある時のみサイトマップ）
    let mut robots = String::from("User-agent: *\nAllow: /\n");
    if let Some(base) = site_url() {
        let mut xml = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
        );
        for u in &urls {
            xml.push_str(&format!("  <url><loc>{}</loc></url>\n", esc(&format!("{}/{}", base, u))));
        }
        xml.push_str("</urlset>\n");
        fs::write(dist.join("sitemap.xml"), xml).map_err(|e| e.to_string())?;
        robots.push_str(&format!("Sitemap: {}/sitemap.xml\n", base));
    }
    fs::write(dist.join("robots.txt"), robots).map_err(|e| e.to_string())?;

    // 静的ファイルをコピー
    for entry in fs::read_dir("static").map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_file() {
            fs::copy(&path, dist.join(path.file_name().unwrap())).map_err(|e| e.to_string())?;
        }
    }
    println!("{} 教科 / {} 問を生成しました -> dist/", subjects.len(), total);
    Ok(())
}
