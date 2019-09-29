//use rusqlite::types::ToSql;
use chrono::NaiveDateTime;
use fossil_delta::{delta, deltainv};
pub use rusqlite::{params, Connection, Result, Statement, Transaction, NO_PARAMS};
use std::collections::HashMap;
/* pub fn rpartition<'a>(input:&'a str, sep:&str) -> (&'a str, &'a str, &'a str) {
  match input.rfind(sep) {
    Some(i) => {
      let j = i + sep.len();
      (&input[..i], &input[i..j], &input[j..])
    },
    None => {
      (&input, "", "")
    }
  }
}
*/
pub fn partition<'a>(input: &'a str, sep: &str) -> (&'a str, &'a str, &'a str) {
    match input.find(sep) {
        Some(i) => {
            let j = i + sep.len();
            (&input[..i], &input[i..j], &input[j..])
        }
        None => (&input, "", ""),
    }
}
fn decode_snapshot(txt: &str) -> HashMap<String, String> {
    let (outline, _, rest) = partition(txt, "\n\n");
    let mut nodes = HashMap::new();
    nodes.insert(String::from("__outline__"), String::from(outline));
    let mut i = 0;
    let nrest = rest.len();
    while i < nrest {
        let (n, _, x) = partition(&rest[i..], " ");
        i += n.len() + 1;
        let n: usize = n.parse().unwrap();
        let (gnx, _, hb) = partition(&x[..n], "\n");
        i += n;
        nodes.insert(String::from(gnx), String::from(hb));
    }
    nodes
}
fn process_snapshot(tx: &Transaction, tstamp: &str, txt: &str) -> Result<()> {
    let mut nodes = decode_snapshot(txt);
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut stmt = tx.prepare("select gnx, hb from present")?;
    let mut rows = stmt.query(NO_PARAMS)?;
    while let Ok(Some(row)) = rows.next() {
        let gnx: String = row.get(0).unwrap();
        let hb: String = row.get(1).unwrap();
        match nodes.remove(&gnx) {
            Some(s) => {
                if hb != s {
                    a.push((tstamp, gnx.clone(), delta(&hb, &s)));
                    b.push((gnx, s, false));
                }
            }
            None => {
                // this node is deleted
                if hb.len() > 0 {
                    a.push((tstamp, gnx.clone(), delta(&hb, "")));
                }
                b.push((gnx, String::new(), true));
            }
        }
    }
    for (gnx, hb) in nodes.drain() {
        // these nodes are new
        a.push((tstamp, gnx.clone(), delta("", &hb)));
        b.push((gnx, hb, false));
    }
    let mut stmt = tx.prepare("insert or replace into present(gnx, hb, x) values (?, ?, ?)")?;
    for (x, y, z) in b {
        stmt.execute(params![x, y, z])?;
    }
    let mut stmt = tx.prepare("insert into changes(t, gnx, d) values (?, ?, ?)")?;
    for (x, y, z) in a {
        stmt.execute(params![x, y, z])?;
    }
    Ok(())
}
pub fn add_snapshot(conn: &mut Connection, tstamp: &str, data: &str) -> Result<bool> {
    match NaiveDateTime::parse_from_str(tstamp, "%Y-%m-%dT%H:%M:%S%.f") {
        Ok(_) => {
            let tx = conn.transaction()?;
            // it used to be useful to keep all snapshots in case history needs to be rebuilt
            // but in production it would consume too much space to keep all raw snapshots in db
            // {
            //   let mut stmt = tx.prepare("replace into snapshots(t, data) values(?, ?)")?;
            //   stmt.execute(params![tstamp, data])?;
            // }
            process_snapshot(&tx, tstamp, data)?;
            tx.commit()?;
            Ok(true)
        }
        _ => Ok(false),
    }
}
pub fn get_all_at(conn: &Connection, tstamp: &str) -> Result<String> {
    let mut nodes = HashMap::new();
    {
        let mut stmt = conn.prepare("select gnx, hb from present where not x")?;
        let mut rows = stmt.query(NO_PARAMS)?;
        while let Some(row) = rows.next()? {
            let gnx: String = row.get(0)?;
            let hb: String = row.get(1)?;
            nodes.insert(gnx, hb);
        }
    }
    {
        let mut stmt = conn.prepare("select gnx, d from changes where t >= ? order by t desc")?;
        let mut rows = stmt.query(params![tstamp])?;
        while let Some(row) = rows.next()? {
            let gnx: String = row.get(0)?;
            let d: Vec<u8> = row.get(1)?;
            if let Some(b) = nodes.get_mut(&gnx) {
                *b = deltainv(b, &d);
            }
        }
    }
    let mut o = nodes.remove("__outline__").unwrap();
    o.push('\n');
    o.push('\n');
    for (gnx, hb) in nodes.iter() {
        let n = gnx.len() + hb.len() + 1;
        o.push_str(&format!("{} {}\n{}", n, &gnx, &hb));
    }
    Ok(o)
}
pub fn get_all_revision(conn: &Connection, rev: usize) -> Result<String> {
    let mut nodes = HashMap::new();
    {
        let mut stmt = conn.prepare("select gnx, hb from present where not x")?;
        let mut rows = stmt.query(NO_PARAMS)?;
        while let Some(row) = rows.next()? {
            let gnx: String = row.get(0)?;
            let hb: String = row.get(1)?;
            nodes.insert(gnx, hb);
        }
    }
    let mut tstamp = String::new();
    if rev > 0 {
        let mut stmt = conn.prepare("select gnx, d, t from changes order by t desc")?;
        let mut rows = stmt.query(NO_PARAMS)?;
        let mut i = 0;
        while let Some(row) = rows.next()? {
            let gnx: String = row.get(0)?;
            let d: Vec<u8> = row.get(1)?;
            if let Some(b) = nodes.get_mut(&gnx) {
                *b = deltainv(b, &d);
            }
            i += 1;
            if i == rev {
                tstamp = row.get(2)?;
                break;
            }
        }
    }
    let mut o = nodes.remove("__outline__").unwrap();
    o.insert(0, '\n');
    o.insert_str(0, &tstamp);
    o.push('\n');
    o.push('\n');
    for (gnx, hb) in nodes.iter() {
        let n = gnx.len() + hb.len() + 1;
        o.push_str(&format!("{} {}\n{}", n, &gnx, &hb));
    }
    Ok(o)
}
pub fn get_node_at(conn: &Connection, gnx: &str, tstamp: &str) -> Result<String> {
    let mut stmt = conn.prepare("select hb from present where gnx = ?")?;
    let mut rows = stmt.query(params![gnx])?;
    let mut hb = if let Some(row) = rows.next()? {
        row.get(0)?
    } else {
        String::new()
    };
    let mut stmt = conn.prepare("select t, d from changes where gnx=? order by t desc")?;
    let mut rows = stmt.query(params![gnx])?;
    while let Some(row) = rows.next()? {
        let t: String = row.get(0)?;
        if t.as_str() < tstamp {
            return Ok(hb);
        }
        let d: Vec<u8> = row.get(1)?;
        hb = deltainv(&hb, &d);
    }
    Ok(hb)
}
pub fn get_node_revision(conn: &Connection, gnx: &str, num: usize) -> Result<String> {
    let mut stmt =
        conn.prepare("select hb, datetime('now', 'localtime') from present where gnx = ?")?;
    let mut rows = stmt.query(params![gnx])?;
    let mut tstamp = String::new();
    let mut hb = match rows.next()? {
        Some(row) => {
            let t: String = row.get(1)?;
            tstamp.push_str(&t[..10]);
            tstamp.push('T');
            tstamp.push_str(&t[11..]);
            tstamp.push_str(".000000");
            row.get(0)?
        }
        _ => String::new(),
    };
    if num == 0 {
        tstamp.push('\n');
        tstamp.push_str(&hb);
        return Ok(tstamp);
    }
    let mut stmt = conn.prepare("select d,t from changes where gnx=? order by t desc")?;
    let mut rows = stmt.query(params![gnx])?;

    let mut i = num;
    while let Some(row) = rows.next()? {
        let d: Vec<u8> = row.get(0)?;
        hb = deltainv(&hb, &d);
        i -= 1;
        if i == 0 {
            tstamp = row.get(1)?;
            break;
        }
    }
    tstamp.push('\n');
    tstamp.push_str(&hb);
    Ok(tstamp)
}
pub fn get_node_rev_count(conn: &Connection, gnx: &str) -> Result<(String, String, u32)> {
    let mut stmt = conn.prepare(
        "select Coalesce(min(t), strftime('now')), 
              Coalesce(max(t), strftime('now')),
              count(t) from changes where gnx=?",
    )?;
    let mut rows = stmt.query(params![gnx])?;
    if let Some(row) = rows.next()? {
        let n: u32 = row.get(2)?;
        let t1: String = row.get(0)?;
        let t2: String = row.get(1)?;
        Ok((t1, t2, n))
    } else {
        Ok((String::new(), String::new(), 0))
    }
}
pub fn connect(pth: &str) -> Result<Connection> {
    let conn = Connection::open(pth)?;
    create_tables(&conn)?;
    Ok(conn)
}
fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists snapshots(t primary key, data)",
        params![],
    )?;
    conn.execute(
        "create table if not exists present(gnx primary key, hb, x)",
        params![],
    )?;
    conn.execute(
        "create table if not exists changes(t, gnx, d, primary key (t, gnx))",
        params![],
    )?;
    conn.execute(
        "create index if not exists gnx_changes on changes(gnx)",
        params![],
    )?;
    Ok(())
}
