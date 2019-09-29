#[path = "db-sqlite.rs"]
mod db;
#[cfg(debug_assertions)]
use actix_files::NamedFile;
#[cfg(debug_assertions)]
use actix_web::Result;
use actix_web::{http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::NaiveDateTime;
use db::partition;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
#[cfg(debug_assertions)]
use std::path::PathBuf;
type ConMap = HashMap<String, db::Connection>;

struct AppState {
    m: RefCell<ConMap>,
    valid_names: String,
}
impl AppState {
    fn new() -> Self {
        let mut args = env::args();
        let valid_names = load_valid_names(&args.nth(1).unwrap());
        AppState {
            m: RefCell::new(HashMap::new()),
            valid_names,
        }
    }
}
fn load_file(fname: &str) -> std::io::Result<String> {
    let mut content = String::new();
    let mut file = File::open(fname)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}
fn load_valid_names(fname: &str) -> String {
    match load_file(fname) {
        Ok(s) => s,
        Err(e) => {
            println!("Can't load valid file names: {:?}", e);
            String::new()
        }
    }
}
#[cfg(debug_assertions)]
fn st_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let rpath = PathBuf::from("leo-ver-serv/web/").join(path);
    let file = NamedFile::open(rpath)?;
    Ok(file.use_last_modified(true).use_etag(true))
}
#[cfg(not(debug_assertions))]
static APPJS: &str = include_str!("../web/js/app.js");

#[cfg(not(debug_assertions))]
static VENDORSJS: &str = include_str!("../web/js/vendors.js");

#[cfg(not(debug_assertions))]
static APPCSS: &str = include_str!("../web/css/app.css");

#[cfg(not(debug_assertions))]
static INDEXHTML: &str = include_str!("../web/index.html");

#[cfg(not(debug_assertions))]
fn st_file(req: HttpRequest) -> impl Responder {
    match req.match_info().query("filename") {
        "js/app.js" => HttpResponse::Ok()
            .content_type("application/javascript")
            .body(APPJS),
        "js/vendors.js" => HttpResponse::Ok()
            .content_type("application/javascript")
            .body(VENDORSJS),
        "css/app.css" => HttpResponse::Ok().content_type("text/css").body(APPCSS),
        "index.html" => HttpResponse::Ok().content_type("text/html").body(INDEXHTML),
        _ => HttpResponse::NotFound().finish(),
    }
}
fn ensure_connection(hmap: &mut ConMap, fname: &str) {
    if !hmap.contains_key(fname) {
        let conn = make_connection(fname);
        hmap.insert(String::from(fname), conn);
    }
}
fn dbname(a: &str) -> String {
    let mut res = String::from(a);
    res.push_str(".history");
    res
}
fn make_connection(fname: &str) -> db::Connection {
    db::connect(&dbname(fname)).unwrap()
}
fn post_snapshot(data: web::Data<AppState>, text: String) -> &'static str {
    let (fname, _, rest) = partition(&text, "\n");
    let (t, sep, snapdata) = partition(rest, "\n");
    let ok = sep.len() > 0;
    let ok = ok && NaiveDateTime::parse_from_str(t, "%Y-%m-%dT%H:%M:%S%.f").is_ok();
    let ok = ok && {
        let mut hmap = data.m.borrow_mut();
        if data.valid_names.lines().all(|x| x != fname) {
            return "Unknwon file";
        }
        ensure_connection(&mut hmap, fname);
        let ok = match db::add_snapshot(hmap.get_mut(fname).unwrap(), t, snapdata) {
            Ok(a) => a,
            Err(e) => {
                println!("post_snapshot:{:?}", e);
                false
            }
        };
        ok
    };
    println!("post snapshot {} bytes at {}", snapdata.len(), t);
    if ok {
        "Ok"
    } else {
        "Err"
    }
}
fn post_node_at(data: web::Data<AppState>, text: String) -> HttpResponse {
    let (fname, _, rest) = partition(&text, "\n");
    let (gnx, _, tstamp) = partition(rest, " ");

    let mut hmap = data.m.borrow_mut();
    if data.valid_names.lines().all(|x| x != fname) {
        return HttpResponse::NotFound()
            .header("x-err", format!("Unknown file:[{}]", fname))
            .finish();
    }
    ensure_connection(&mut hmap, fname);

    match db::get_node_at(hmap.get_mut(fname).unwrap(), gnx, tstamp) {
        Ok(s) => HttpResponse::Ok().content_type("text/plain").body(s),
        Err(e) => {
            println!("Failed node {}\n{} at {}\n {:?}", fname, gnx, tstamp, e);
            HttpResponse::NotFound()
                .header("x-err", format!("Error: {:?}", e))
                .finish()
        }
    }
}
fn post_node_rev(data: web::Data<AppState>, text: String) -> HttpResponse {
    let (fname, _, rest) = partition(&text, "\n");
    let (gnx, _, num) = partition(rest, " ");
    let num: usize = match num.trim().parse() {
        Ok(i) => i,
        _ => return HttpResponse::BadRequest().body(format!("Bad request: {:?}", num)),
    };
    let mut hmap = data.m.borrow_mut();
    if data.valid_names.lines().all(|x| x != fname) {
        return HttpResponse::NotFound()
            .header("x-err", format!("Unknown file:[{}]", fname))
            .finish();
    }
    ensure_connection(&mut hmap, fname);
    match db::get_node_revision(hmap.get_mut(fname).unwrap(), gnx, num) {
        Ok(s) => HttpResponse::Ok().content_type("text/plain").body(s),
        Err(e) => {
            println!("Failed node {}\n{} rev {}\n {:?}", fname, gnx, num, e);
            HttpResponse::NotFound()
                .header("x-err", format!("Error: {:?}", e))
                .finish()
        }
    }
}
fn post_snapshot_rev(data: web::Data<AppState>, text: String) -> HttpResponse {
    let (fname, _, rest) = partition(&text, "\n");
    let num: usize = match rest.trim().parse() {
        Ok(i) => i,
        _ => return HttpResponse::BadRequest().body("Bad request"),
    };
    let mut hmap = data.m.borrow_mut();
    if data.valid_names.lines().all(|x| x != fname) {
        return HttpResponse::NotFound().body("Unknown file");
    }
    ensure_connection(&mut hmap, fname);
    match db::get_all_revision(hmap.get_mut(fname).unwrap(), num) {
        Ok(s) => HttpResponse::Ok().content_type("text/plain").body(s),
        Err(e) => {
            println!("Failed snapshot {} rev {}\n {:?}", fname, num, e);
            HttpResponse::NotFound().body(format!("\n\nError: {:?}", e))
        }
    }
}
#[derive(Serialize)]
struct RevCount {
    tmin: String,
    tmax: String,
    n: u32,
}
fn post_node_rev_count(data: web::Data<AppState>, text: String) -> HttpResponse {
    let (fname, _, gnx) = partition(&text, "\n");
    let mut hmap = data.m.borrow_mut();
    if data.valid_names.lines().all(|x| x != fname) {
        return HttpResponse::NotFound().body("Unknown file");
    }
    ensure_connection(&mut hmap, fname);
    match db::get_node_rev_count(hmap.get_mut(fname).unwrap(), gnx) {
        Ok((tmin, tmax, n)) => HttpResponse::Ok().json(RevCount { tmin, tmax, n }),
        Err(e) => {
            println!("Failed node rev count {}\n{}\n {:?}", fname, gnx, e);
            HttpResponse::NotFound().body(format!("\n\nError: {:?}", e))
        }
    }
}
fn post_snapshot_at(data: web::Data<AppState>, text: String) -> HttpResponse {
    let (fname, _, tstamp) = partition(&text, "\n");
    let mut hmap = data.m.borrow_mut();
    if data.valid_names.lines().all(|x| x != fname) {
        return HttpResponse::NotFound().body("Unknown file");
    }
    ensure_connection(&mut hmap, fname);
    match db::get_all_at(hmap.get_mut(fname).unwrap(), tstamp) {
        Ok(s) => HttpResponse::Ok().content_type("text/plain").body(s),
        Err(e) => {
            println!("Failed snapshot {} at {}\n {:?}", fname, tstamp, e);
            HttpResponse::NotFound().body(format!("\n\nError: {:?}", e))
        }
    }
}
fn index() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::MOVED_PERMANENTLY)
        .set_header("Location", "/public/index.html")
        .finish()
}
static FICON: &[u8] = include_bytes!("../favicon.ico");
fn favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/vnd.microsoft.icon")
        .body(FICON)
}
fn get_leo_files(data: web::Data<AppState>) -> String {
    data.valid_names.clone()
}
fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Usage:  leo-ver-serv <valid-filenames> <port>");
    } else {
        let port = args.nth(2).unwrap_or(String::from("8088"));
        HttpServer::new(|| {
            App::new()
                .data(AppState::new())
                .data(web::PayloadConfig::new(1 << 25))
                .route("/", web::get().to(index))
                .route("/public/{filename:.*}", web::get().to(st_file))
                .route("/favicon.ico", web::get().to(favicon))
                .route("/add-snapshot", web::post().to(post_snapshot))
                .route("/snapshot-at", web::post().to(post_snapshot_at))
                .route("/node-at", web::post().to(post_node_at))
                .route("/node-rev", web::post().to(post_node_rev))
                .route("/node-rev-count", web::post().to(post_node_rev_count))
                .route("/snapshot-rev", web::post().to(post_snapshot_rev))
                .route("/leo-files", web::get().to(get_leo_files))
        })
        .bind(format!("127.0.0.1:{}", port))
        .unwrap()
        .run()
        .unwrap();
    }
}
