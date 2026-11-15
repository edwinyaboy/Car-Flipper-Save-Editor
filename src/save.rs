

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub type Res<T> = Result<T, String>;





struct Reader<'a> {
    d: &'a [u8],
    p: usize,
}

impl<'a> Reader<'a> {
    fn new(d: &'a [u8]) -> Self {
        Self { d, p: 0 }
    }

    fn need(&self, n: usize) -> Res<()> {
        if self.p.saturating_add(n) > self.d.len() {
            Err(format!(
                "unexpected end of file at 0x{:X} (needed {} bytes)",
                self.p, n
            ))
        } else {
            Ok(())
        }
    }

    fn i32(&mut self) -> Res<i32> {
        self.need(4)?;
        let v = i32::from_le_bytes(self.d[self.p..self.p + 4].try_into().unwrap());
        self.p += 4;
        Ok(v)
    }

    fn u32(&mut self) -> Res<u32> {
        self.need(4)?;
        let v = u32::from_le_bytes(self.d[self.p..self.p + 4].try_into().unwrap());
        self.p += 4;
        Ok(v)
    }

    fn i64(&mut self) -> Res<i64> {
        self.need(8)?;
        let v = i64::from_le_bytes(self.d[self.p..self.p + 8].try_into().unwrap());
        self.p += 8;
        Ok(v)
    }

    fn u16(&mut self) -> Res<u16> {
        self.need(2)?;
        let v = u16::from_le_bytes(self.d[self.p..self.p + 2].try_into().unwrap());
        self.p += 2;
        Ok(v)
    }

    fn u8(&mut self) -> Res<u8> {
        self.need(1)?;
        let v = self.d[self.p];
        self.p += 1;
        Ok(v)
    }

    fn f32(&mut self) -> Res<f32> {
        self.need(4)?;
        let v = f32::from_le_bytes(self.d[self.p..self.p + 4].try_into().unwrap());
        self.p += 4;
        Ok(v)
    }

    fn f64(&mut self) -> Res<f64> {
        self.need(8)?;
        let v = f64::from_le_bytes(self.d[self.p..self.p + 8].try_into().unwrap());
        self.p += 8;
        Ok(v)
    }

    fn skip(&mut self, n: usize) -> Res<()> {
        self.need(n)?;
        self.p += n;
        Ok(())
    }

    
    fn fstr(&mut self) -> Res<String> {
        let n = self.i32()?;
        if n == 0 {
            return Ok(String::new());
        }
        if n < 0 {
            let count = (-n) as usize;
            self.need(count * 2)?;
            let mut units = Vec::with_capacity(count);
            for i in 0..count {
                let off = self.p + i * 2;
                units.push(u16::from_le_bytes([self.d[off], self.d[off + 1]]));
            }
            self.p += count * 2;
            Ok(String::from_utf16_lossy(&units)
                .trim_end_matches('\0')
                .to_string())
        } else {
            let count = n as usize;
            self.need(count)?;
            let bytes = &self.d[self.p..self.p + count];
            self.p += count;
            Ok(bytes
                .iter()
                .map(|&b| b as char)
                .collect::<String>()
                .trim_end_matches('\0')
                .to_string())
        }
    }

    
    fn type_nodes(&mut self) -> Res<Vec<(String, i32)>> {
        let mut out = Vec::new();
        let mut remaining: i32 = 1;
        while remaining > 0 {
            let name = self.fstr()?;
            let inner = self.i32()?;
            out.push((name, inner));
            remaining += inner - 1;
            if out.len() > 64 {
                return Err("property type name tree is too deep".into());
            }
        }
        Ok(out)
    }
}





#[derive(Clone)]
#[allow(dead_code)]
pub struct Node {
    pub path: String,
    pub typ: String,
    pub size: i32,
    pub flags: u8,
    pub val_off: usize,
    pub flag_off: usize,
    pub size_off: usize,
    pub ancestors: Vec<usize>,
    pub display: String,
    pub text: Option<String>,
    pub children: Vec<Node>,
}

fn is_native_struct(name: &str) -> bool {
    matches!(
        name,
        "DateTime"
            | "Timespan"
            | "Guid"
            | "Vector"
            | "Vector2D"
            | "Vector4"
            | "Rotator"
            | "Quat"
            | "Transform"
            | "Matrix"
            | "Plane"
            | "Box"
            | "Box2D"
            | "Sphere"
            | "LinearColor"
            | "Color"
            | "IntPoint"
            | "IntVector"
            | "FrameNumber"
            | "SoftObjectPath"
            | "SoftClassPath"
            | "PrimaryAssetType"
            | "PrimaryAssetId"
            | "GameplayTagContainer"
            | "RandomStream"
            | "RichCurveKey"
    )
}

fn fmt_f32(v: f32) -> String {
    format!("{}", v)
}

fn fmt_f64(v: f64) -> String {
    format!("{}", v)
}

fn parse_props(r: &mut Reader, prefix: &str, depth: usize, ancestors: &[usize]) -> Res<Vec<Node>> {
    if depth > 64 {
        return Err("property nesting is too deep".into());
    }
    let mut out = Vec::new();
    loop {
        let tag_start = r.p;
        let name = r.fstr()?;
        if name == "None" {
            return Ok(out);
        }
        if name.is_empty()
            || name.len() > 512
            || !name.chars().all(|c| (32..127).contains(&(c as u32)))
        {
            return Err(format!("bad property name at 0x{:X}", tag_start));
        }
        let tnodes = r.type_nodes()?;
        let size_off = r.p;
        let size = r.i32()?;
        if !(0..=512 * 1024 * 1024).contains(&size) {
            return Err(format!(
                "invalid size {} for property '{}' at 0x{:X}",
                size, name, tag_start
            ));
        }
        let flags = r.u8()?;
        let flag_off = r.p - 1;
        let _array_index = if flags & 0x01 != 0 { r.i32()? } else { 0 };
        if flags & 0x02 != 0 {
            r.skip(16)?; 
        }
        if flags & 0x04 != 0 {
            let ext = r.u8()?;
            if ext & 0x02 != 0 {
                r.skip(2)?; 
            }
        }
        let val_off = r.p;
        let typ = tnodes[0].0.clone();
        let path = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}.{}", prefix, name)
        };
        let end = val_off
            .checked_add(size as usize)
            .ok_or_else(|| "property size overflow".to_string())?;
        if end > r.d.len() {
            return Err(format!(
                "property '{}' (size {}) runs past end of file",
                path, size
            ));
        }

        let mut children = Vec::new();
        let mut text = None;
        let display: String;
        let mut child_ancestors = ancestors.to_vec();
        child_ancestors.push(size_off);

        match typ.as_str() {
            "IntProperty" => display = r.i32()?.to_string(),
            "UInt32Property" => display = r.u32()?.to_string(),
            "Int64Property" => display = r.i64()?.to_string(),
            "FloatProperty" => display = fmt_f32(r.f32()?),
            "DoubleProperty" => display = fmt_f64(r.f64()?),
            "StrProperty" | "NameProperty" => {
                let s = r.fstr()?;
                display = format!("{:?}", s);
                text = Some(s);
            }
            "BoolProperty" => {
                display = if flags & 0x10 != 0 {
                    "true".into()
                } else {
                    "false".into()
                };
            }
            "ByteProperty" => display = r.u8()?.to_string(),
            "EnumProperty" => {
                let s = r.fstr()?;
                display = s.clone();
                text = Some(s);
            }
            "StructProperty" => {
                let sub = tnodes.get(1).map(|x| x.0.clone()).unwrap_or_default();
                if is_native_struct(&sub) {
                    display = format!("{} (raw {} bytes)", sub, size);
                } else {
                    let saved = r.p;
                    match parse_props(r, &path, depth + 1, &child_ancestors) {
                        Ok(kids) => {
                            children = kids;
                            display = format!("struct {}", sub);
                        }
                        Err(_) => {
                            r.p = saved;
                            display = format!("{} (raw {} bytes)", sub, size);
                        }
                    }
                }
            }
            "ArrayProperty" | "SetProperty" => {
                let inner = tnodes.get(1).map(|x| x.0.clone()).unwrap_or_default();
                let count = r.i32()?;
                if !(0..=10_000_000).contains(&count) {
                    return Err(format!("invalid array count {} for '{}'", count, path));
                }
                if inner == "StructProperty" {
                    for i in 0..count {
                        let ep = format!("{}[{}]", path, i);
                        let kids = parse_props(r, &ep, depth + 1, &child_ancestors)?;
                        children.push(Node {
                            path: ep,
                            typ: "StructProperty".into(),
                            size: 0,
                            flags: 0,
                            val_off: 0,
                            flag_off: 0,
                            size_off: 0,
                            ancestors: child_ancestors.clone(),
                            display: "struct element".into(),
                            text: None,
                            children: kids,
                        });
                    }
                    display = format!("array<struct>[{}]", count);
                } else {
                    for i in 0..count {
                        let ep = format!("{}[{}]", path, i);
                        let vo = r.p;
                        let mut t = None;
                        let d = match inner.as_str() {
                            "IntProperty" => r.i32()?.to_string(),
                            "UInt32Property" => r.u32()?.to_string(),
                            "Int64Property" => r.i64()?.to_string(),
                            "FloatProperty" => fmt_f32(r.f32()?),
                            "DoubleProperty" => fmt_f64(r.f64()?),
                            "StrProperty" | "NameProperty" => {
                                let s = r.fstr()?;
                                let d = format!("{:?}", s);
                                t = Some(s);
                                d
                            }
                            "ByteProperty" => r.u8()?.to_string(),
                            "BoolProperty" => {
                                if r.u8()? != 0 {
                                    "true".into()
                                } else {
                                    "false".into()
                                }
                            }
                            _ => {
                                
                                r.p = val_off + size as usize;
                                break;
                            }
                        };
                        children.push(Node {
                            path: ep,
                            typ: inner.clone(),
                            size: 0,
                            flags: 0,
                            val_off: vo,
                            flag_off: usize::MAX, 
                            size_off: 0,
                            ancestors: child_ancestors.clone(),
                            display: d,
                            text: t,
                            children: Vec::new(),
                        });
                    }
                    display = format!("array<{}>[{}]", inner, count);
                }
            }
            "MapProperty" => {
                let kt = tnodes.get(1).map(|x| x.0.clone()).unwrap_or_default();
                let vt = tnodes.get(2).map(|x| x.0.clone()).unwrap_or_default();
                display = format!("map<{},{}> ({} bytes)", kt, vt, size);
            }
            other => display = format!("{} ({} bytes)", other, size),
        }

        if r.p > end {
            return Err(format!("property '{}' read past its declared size", path));
        }
        r.p = end; 

        out.push(Node {
            path,
            typ,
            size,
            flags,
            val_off,
            flag_off,
            size_off,
            ancestors: ancestors.to_vec(),
            display,
            text,
            children,
        });
    }
}

pub fn parse_gvas(data: &[u8]) -> Res<(String, Vec<Node>, usize)> {
    if data.len() < 32 || &data[0..4] != b"GVAS" {
        return Err("not a GVAS save file".into());
    }
    let mut r = Reader::new(data);
    r.p = 4;
    let save_game_version = r.i32()?;
    let _ue4 = r.i32()?;
    let _ue5 = r.i32()?;
    if save_game_version >= 3 {
        let _major = r.u16()?;
        let _minor = r.u16()?;
        let _patch = r.u16()?;
        let _changelist = r.u32()?;
        let _branch = r.fstr()?;
    }
    let _custom_format = r.i32()?;
    let custom_count = r.i32()?;
    if !(0..=10_000).contains(&custom_count) {
        return Err("invalid custom version count".into());
    }
    r.skip(custom_count as usize * 20)?;
    let class = r.fstr()?;

    
    
    
    if r.p + 5 <= data.len() && data[r.p] == 0 {
        let peek_len = i32::from_le_bytes(data[r.p + 1..r.p + 5].try_into().unwrap());
        if (1..=200).contains(&peek_len) {
            r.p += 1;
        }
    }

    let nodes = parse_props(&mut r, "", 0, &[])?;
    Ok((class, nodes, r.p))
}

pub fn collect<'a>(nodes: &'a [Node], out: &mut Vec<&'a Node>) {
    for n in nodes {
        out.push(n);
        collect(&n.children, out);
    }
}

pub fn lookup<'a>(nodes: &'a [Node], path: &str) -> Res<&'a Node> {
    let mut flat = Vec::new();
    collect(nodes, &mut flat);

    if let Some(n) = flat.iter().find(|n| n.path == path) {
        return Ok(n);
    }
    if let Some(n) = flat.iter().find(|n| n.path.eq_ignore_ascii_case(path)) {
        return Ok(n);
    }

    
    let tail = path.rsplit('.').next().unwrap_or(path).to_ascii_lowercase();
    let mut hints: Vec<String> = flat
        .iter()
        .filter(|n| n.path.to_ascii_lowercase().contains(&tail))
        .take(12)
        .map(|n| n.path.clone())
        .collect();
    hints.sort();
    hints.dedup();
    let mut msg = format!("property '{}' not found", path);
    if !hints.is_empty() {
        msg.push_str("\nDid you mean:\n");
        for h in hints {
            msg.push_str("  ");
            msg.push_str(&h);
            msg.push('\n');
        }
    }
    Err(msg)
}





pub fn parse_int(s: &str) -> Res<i64> {
    s.trim()
        .replace('_', "")
        .parse::<i64>()
        .map_err(|_| format!("'{}' is not a valid integer", s))
}

pub fn parse_f64(s: &str) -> Res<f64> {
    s.trim()
        .parse::<f64>()
        .map_err(|_| format!("'{}' is not a valid number", s))
}

fn write_i32(d: &mut [u8], off: usize, v: i32) -> Res<()> {
    if off + 4 > d.len() {
        return Err("write out of bounds".into());
    }
    d[off..off + 4].copy_from_slice(&v.to_le_bytes());
    Ok(())
}

fn write_u32(d: &mut [u8], off: usize, v: u32) -> Res<()> {
    if off + 4 > d.len() {
        return Err("write out of bounds".into());
    }
    d[off..off + 4].copy_from_slice(&v.to_le_bytes());
    Ok(())
}

fn write_i64(d: &mut [u8], off: usize, v: i64) -> Res<()> {
    if off + 8 > d.len() {
        return Err("write out of bounds".into());
    }
    d[off..off + 8].copy_from_slice(&v.to_le_bytes());
    Ok(())
}

fn write_u8(d: &mut [u8], off: usize, v: u8) -> Res<()> {
    if off >= d.len() {
        return Err("write out of bounds".into());
    }
    d[off] = v;
    Ok(())
}

fn patch_string_resize(d: &mut Vec<u8>, node: &Node, new: &str) -> Res<()> {
    if !new.is_ascii() {
        return Err("only ASCII text is supported".into());
    }
    let off = node.val_off;
    if off + 4 > d.len() {
        return Err("write out of bounds".into());
    }
    let len = i32::from_le_bytes(d[off..off + 4].try_into().unwrap());
    if len < 0 {
        return Err("UTF-16 strings are not supported".into());
    }
    
    let old_total = if len == 0 { 4 } else { 4 + len as usize };
    if off + old_total > d.len() {
        return Err("string runs past end of file".into());
    }

    let new_total = 4 + new.len() + 1;
    let delta = new_total as i64 - old_total as i64;
    let mut bytes = Vec::with_capacity(new_total);
    bytes.extend_from_slice(&((new.len() + 1) as i32).to_le_bytes());
    bytes.extend_from_slice(new.as_bytes());
    bytes.push(0);
    splice_and_fix_sizes(d, node, off, old_total, bytes, delta)
}






fn patch_string_array(d: &mut Vec<u8>, node: &Node, items: &[String]) -> Res<()> {
    let off = node.val_off;
    if off + 4 > d.len() {
        return Err("array count out of bounds".into());
    }
    let count = i32::from_le_bytes(d[off..off + 4].try_into().unwrap());
    if !(0..=1_000_000).contains(&count) {
        return Err(format!("invalid array count {}", count));
    }
    let mut pos = off + 4;
    for _ in 0..count {
        if pos + 4 > d.len() {
            return Err("array element out of bounds".into());
        }
        let len = i32::from_le_bytes(d[pos..pos + 4].try_into().unwrap());
        if len < 0 {
            return Err("UTF-16 array elements are not supported".into());
        }
        pos += 4 + len as usize;
        if pos > d.len() {
            return Err("array element runs past end of file".into());
        }
    }
    let old_total = pos - off;

    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(items.len() as i32).to_le_bytes());
    for item in items {
        if !item.is_ascii() {
            return Err("only ASCII names are supported".into());
        }
        bytes.extend_from_slice(&((item.len() + 1) as i32).to_le_bytes());
        bytes.extend_from_slice(item.as_bytes());
        bytes.push(0);
    }
    let delta = bytes.len() as i64 - old_total as i64;
    splice_and_fix_sizes(d, node, off, old_total, bytes, delta)
}

fn splice_and_fix_sizes(
    d: &mut Vec<u8>,
    node: &Node,
    off: usize,
    old_total: usize,
    bytes: Vec<u8>,
    delta: i64,
) -> Res<()> {
    d.splice(off..off + old_total, bytes);

    
    
    
    let mut fields = Vec::new();
    if node.size_off != 0 {
        fields.push(node.size_off);
    }
    fields.extend(node.ancestors.iter().copied());
    for so in fields {
        if so + 4 > d.len() {
            return Err("size field out of bounds".into());
        }
        let old = i32::from_le_bytes(d[so..so + 4].try_into().unwrap());
        let nv = old
            .checked_add(delta as i32)
            .ok_or_else(|| "size overflow".to_string())?;
        d[so..so + 4].copy_from_slice(&nv.to_le_bytes());
    }
    Ok(())
}

fn patch_node(d: &mut Vec<u8>, node: &Node, raw: &str) -> Res<()> {
    match node.typ.as_str() {
        "IntProperty" => {
            let v = parse_int(raw)?;
            if v < i32::MIN as i64 || v > i32::MAX as i64 {
                return Err(format!("{} is out of range for a 32-bit int", v));
            }
            write_i32(d, node.val_off, v as i32)
        }
        "UInt32Property" => {
            let v = parse_int(raw)?;
            if v < 0 || v > u32::MAX as i64 {
                return Err(format!("{} is out of range for an unsigned 32-bit int", v));
            }
            write_u32(d, node.val_off, v as u32)
        }
        "Int64Property" => write_i64(d, node.val_off, parse_int(raw)?),
        "FloatProperty" => {
            let v = parse_f64(raw)? as f32;
            if node.val_off + 4 > d.len() {
                return Err("write out of bounds".into());
            }
            d[node.val_off..node.val_off + 4].copy_from_slice(&v.to_le_bytes());
            Ok(())
        }
        "DoubleProperty" => {
            let v = parse_f64(raw)?;
            if node.val_off + 8 > d.len() {
                return Err("write out of bounds".into());
            }
            d[node.val_off..node.val_off + 8].copy_from_slice(&v.to_le_bytes());
            Ok(())
        }
        "BoolProperty" => {
            let v = match raw.trim().to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" | "on" => true,
                "0" | "false" | "no" | "off" => false,
                _ => return Err(format!("'{}' is not a boolean (true/false)", raw)),
            };
            if node.flag_off == usize::MAX {
                
                return write_u8(d, node.val_off, if v { 1 } else { 0 });
            }
            if node.flag_off >= d.len() {
                return Err("write out of bounds".into());
            }
            if v {
                d[node.flag_off] |= 0x10;
            } else {
                d[node.flag_off] &= !0x10;
            }
            Ok(())
        }
        "ByteProperty" => {
            let v = parse_int(raw)?;
            if !(0..=255).contains(&v) {
                return Err(format!("{} is out of range for a byte", v));
            }
            write_u8(d, node.val_off, v as u8)
        }
        "StrProperty" | "NameProperty" | "EnumProperty" => patch_string_resize(d, node, raw),
        other => Err(format!(
            "editing {} properties is not supported (path: {})",
            other, node.path
        )),
    }
}





pub fn save_dir() -> PathBuf {
    match env::var_os("LOCALAPPDATA") {
        Some(p) => PathBuf::from(p)
            .join("CarFlipping")
            .join("Saved")
            .join("SaveGames"),
        None => PathBuf::from("."),
    }
}

pub fn list_save_files() -> Res<Vec<PathBuf>> {
    let dir = save_dir();
    if !dir.exists() {
        return Err(format!("save folder not found: {}", dir.display()));
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| format!("{}: {}", dir.display(), e))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let p = entry.path();
        if p.extension()
            .map(|e| e.eq_ignore_ascii_case("sav"))
            .unwrap_or(false)
        {
            out.push(p);
        }
    }
    out.sort();
    Ok(out)
}

pub fn backup_file(path: &Path) -> Res<PathBuf> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("save.sav");

    
    let mut candidate = path.with_file_name(format!("{}.{}.bak", file_name, ts));
    let mut n = 2;
    while candidate.exists() {
        candidate = path.with_file_name(format!("{}.{}.{}.bak", file_name, ts, n));
        n += 1;
    }
    fs::copy(path, &candidate).map_err(|e| format!("failed to create backup: {}", e))?;
    Ok(candidate)
}

#[derive(Clone)]
pub struct Editor {
    pub path: PathBuf,
    pub data: Vec<u8>,
    pub class: String,
    pub nodes: Vec<Node>,
    pub dirty: bool,
    pub end: usize,
    pub backup: Option<PathBuf>,
}

#[allow(dead_code)]
impl Editor {
    pub fn load(path: &Path) -> Res<Self> {
        let data = fs::read(path).map_err(|e| format!("{}: {}", path.display(), e))?;
        let (class, nodes, end) = parse_gvas(&data)?;
        if end > data.len() {
            return Err("save parser overran the file".into());
        }
        Ok(Self {
            path: path.to_path_buf(),
            data,
            class,
            nodes,
            dirty: false,
            end,
            backup: None,
        })
    }

    pub fn get(&self, path: &str) -> Res<&Node> {
        lookup(&self.nodes, path)
    }

    pub fn set(&mut self, path: &str, raw: &str) -> Res<String> {
        let results = self.set_many(&[(path, raw)])?;
        Ok(results.into_iter().next().unwrap_or_default())
    }

    
    
    
    
    
    
    pub fn set_many(&mut self, patches: &[(&str, &str)]) -> Res<Vec<String>> {
        if patches.is_empty() {
            return Ok(Vec::new());
        }

        let mut work: Vec<(Node, String)> = Vec::with_capacity(patches.len());
        for (path, raw) in patches {
            let node = lookup(&self.nodes, path)?.clone();
            work.push((node, (*raw).to_string()));
        }
        work.sort_by_key(|(node, _)| std::cmp::Reverse(node.val_off));

        let mut data = self.data.clone();
        for (node, raw) in &work {
            patch_node(&mut data, node, raw)?;
        }

        
        let (_, new_nodes, new_end) = parse_gvas(&data)?;
        let mut before = Vec::new();
        let mut after = Vec::new();
        collect(&self.nodes, &mut before);
        collect(&new_nodes, &mut after);
        if before.len() != after.len() {
            return Err("edit changed the save structure; refusing to write".into());
        }

        let mut results = Vec::with_capacity(work.len());
        for (node, _) in &work {
            results.push(lookup(&new_nodes, &node.path)?.display.clone());
        }

        self.data = data;
        self.nodes = new_nodes;
        self.end = new_end;
        self.dirty = true;
        Ok(results)
    }

    
    
    
    
    
    
    
    pub fn set_name_array(&mut self, path: &str, items: &[String]) -> Res<String> {
        let node = lookup(&self.nodes, path)?.clone();
        if node.typ != "ArrayProperty" {
            return Err(format!("{} is not an array", path));
        }
        let inner = node
            .children
            .first()
            .map(|c| c.typ.as_str())
            .unwrap_or_default();
        if inner != "NameProperty" && inner != "StrProperty" {
            return Err(format!("{} holds {} values, not names", path, inner));
        }

        let mut data = self.data.clone();
        patch_string_array(&mut data, &node, items)?;

        let (_, new_nodes, new_end) = parse_gvas(&data)?;
        let reparsed = lookup(&new_nodes, path)?;
        if reparsed.children.len() != items.len() {
            return Err("array resize did not round-trip; refusing to write".into());
        }
        let display = reparsed.display.clone();

        self.data = data;
        self.nodes = new_nodes;
        self.end = new_end;
        self.dirty = true;
        Ok(display)
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn save(&mut self) -> Res<PathBuf> {
        if self.backup.is_none() {
            self.backup = Some(backup_file(&self.path)?);
        }
        let tmp = self.path.with_extension("sav.tmp");
        fs::write(&tmp, &self.data).map_err(|e| format!("write failed: {}", e))?;
        fs::rename(&tmp, &self.path).map_err(|e| format!("replace failed: {}", e))?;
        self.dirty = false;
        Ok(self.backup.clone().unwrap())
    }

    pub fn print_info(&self) {
        println!("file    : {}", self.path.display());
        println!("class   : {}", self.class);
        println!("size    : {} bytes", self.data.len());
        println!();
        let fields = [
            ("ID", "ID"),
            ("Name", "Name"),
            ("OwnerId", "OwnerId"),
            ("OwnerName", "OwnerName"),
            ("LookSeed", "LookSeed"),
            ("PlaySeconds", "PlaySeconds"),
            ("Phone.Cash", "Cash"),
            ("Phone.Level", "Level"),
            ("Phone.Xp", "Xp"),
            ("Phone.Day", "Day"),
            ("Phone.WeekDay", "WeekDay"),
            ("Phone.SpentToday", "SpentToday"),
            ("Phone.BestDrift", "BestDrift"),
            ("Phone.Lifetime.CarsSold", "CarsSold"),
            ("Slots[0].ID", "SlotID"),
            ("Slots[0].Name", "SlotName"),
            ("Slots[0].Cash", "SlotCash"),
            ("Slots[0].Level", "SlotLevel"),
        ];
        for (path, label) in fields {
            if let Ok(n) = self.get(path) {
                let shown = match (&n.typ[..], &n.text) {
                    ("StrProperty", Some(t)) | ("NameProperty", Some(t)) => t.clone(),
                    _ => n.display.clone(),
                };
                println!("{:<10}: {}", label, shown);
            }
        }
        
        if let Ok(n) = self.get("Phone.SoldCars") {
            println!("{:<10}: {}", "SoldCars", n.children.len());
        }
        if self.dirty {
            println!("\n(unsaved changes)");
        }
    }
}

pub fn print_tree(nodes: &[Node], ind: usize) {
    for n in nodes {
        let pad = "  ".repeat(ind);
        if n.typ == "StructProperty" && !n.children.is_empty() {
            println!("{}{} : {} @0x{:X}", pad, n.path, n.display, n.val_off);
            print_tree(&n.children, ind + 1);
        } else if n.children.is_empty() {
            println!(
                "{}{} : {} = {} @0x{:X}",
                pad, n.path, n.typ, n.display, n.val_off
            );
        } else {
            println!(
                "{}{} : {} = {} @0x{:X}",
                pad, n.path, n.typ, n.display, n.val_off
            );
            print_tree(&n.children, ind + 1);
        }
    }
}





pub fn board_label(v: &str) -> &'static str {
    match v {
        "0" => "Curbside",
        "1" => "Dougslist",
        _ => "?",
    }
}

pub fn print_boards(ed: &Editor) -> Res<()> {
    let slots = ed.get("Phone.BoardFeed.Slots")?;
    println!("idx   hour          board       done   cheap  seed");
    println!("----  ------------  ----------  -----  -----  ----------");
    for (i, el) in slots.children.iter().enumerate() {
        let field = |suffix: &str| -> String {
            el.children
                .iter()
                .find(|c| c.path.ends_with(&format!(".{}", suffix)))
                .map(|c| c.display.clone())
                .unwrap_or_default()
        };
        let board = field("Board");
        println!(
            "[{:<2}]  {:>12}  {:<10}  {:<5}  {:<5}  {}",
            i,
            field("Hour"),
            format!("{} ({})", board_label(&board), board),
            field("bDone"),
            field("bCheap"),
            field("Seed")
        );
    }
    if let Ok(seed) = ed.get("Phone.BoardFeed.CareerSeed") {
        println!("\nCareerSeed: {}", seed.display);
    }
    Ok(())
}

pub fn respawn_all(ed: &mut Editor) -> Res<usize> {
    let slots = ed.get("Phone.BoardFeed.Slots")?.children.len();
    for i in 0..slots {
        let p = format!("Phone.BoardFeed.Slots[{}].bDone", i);
        ed.set(&p, "false")?;
    }
    Ok(slots)
}





pub fn sync_index_for(
    main_path: &Path,
    main_data: &[u8],
    main_nodes: &[Node],
) -> Res<Option<String>> {
    let index_path = match main_path.parent() {
        Some(dir) => dir.join("CF_Index.sav"),
        None => return Ok(None),
    };
    if !index_path.exists() {
        return Ok(None);
    }

    let save_id = match lookup(main_nodes, "ID") {
        Ok(n) => n.text.clone().unwrap_or_default(),
        Err(_) => return Ok(None),
    };
    let cash = lookup(main_nodes, "Phone.Cash")
        .ok()
        .map(|n| n.display.clone());
    let level = lookup(main_nodes, "Phone.Level")
        .ok()
        .map(|n| n.display.clone());
    if cash.is_none() && level.is_none() {
        return Ok(None);
    }

    let mut idx_data =
        fs::read(&index_path).map_err(|e| format!("{}: {}", index_path.display(), e))?;
    let (_, idx_nodes, _) = parse_gvas(&idx_data)?;

    let mut slot_prefix: Option<String> = None;
    if let Ok(slots) = lookup(&idx_nodes, "Slots") {
        for el in &slots.children {
            let id = el
                .children
                .iter()
                .find(|c| c.path.ends_with(".ID"))
                .and_then(|c| c.text.clone());
            if id.as_deref() == Some(save_id.as_str()) {
                slot_prefix = Some(el.path.clone());
                break;
            }
        }
    }

    let prefix = match slot_prefix {
        Some(p) => p,
        None => return Ok(None),
    };

    let mut changed = false;
    if let (Some(cash), Ok(field)) = (cash, lookup(&idx_nodes, &format!("{}.Cash", prefix))) {
        if field.display != cash {
            write_i32(&mut idx_data, field.val_off, parse_int(&cash)? as i32)?;
            changed = true;
        }
    }
    if let (Some(level), Ok(field)) = (level, lookup(&idx_nodes, &format!("{}.Level", prefix))) {
        if field.display != level {
            write_i32(&mut idx_data, field.val_off, parse_int(&level)? as i32)?;
            changed = true;
        }
    }

    if changed {
        
        parse_gvas(&idx_data)?;
        let bak = backup_file(&index_path)?;
        fs::write(&index_path, &idx_data)
            .map_err(|e| format!("failed to write {}: {}", index_path.display(), e))?;
        let _ = main_data; 
        return Ok(Some(format!(
            "updated {} (backup: {})",
            index_path.display(),
            bak.display()
        )));
    }
    Ok(None)
}
