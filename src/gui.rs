#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod save;
mod table_layout;
mod vehicle_gen;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use eframe::egui::{
    self, Align, Align2, Color32, FontId, Frame, Layout, Margin, Rect, Response, RichText,
    Rounding, Sense, Stroke, Ui, Vec2,
};
use save::{Editor, Node, Res};
use table_layout::{
    ADVERT_COLUMNS, COLUMN_SPACING, CONTROL_HEIGHT, HEADER_HEIGHT, ROW_HEIGHT, ROW_SPACING,
};
use vehicle_gen::{PriceBasis, TitleBrand, VehicleModel};





const BG: Color32 = Color32::from_rgb(10, 10, 14);
const PANEL: Color32 = Color32::from_rgb(15, 15, 21);
const CARD: Color32 = Color32::from_rgb(22, 22, 31);
const CARD_ALT: Color32 = Color32::from_rgb(29, 29, 41);
const BORDER: Color32 = Color32::from_rgb(42, 42, 58);
const TEXT: Color32 = Color32::from_rgb(238, 238, 246);
const MUTED: Color32 = Color32::from_rgb(140, 140, 160);
const ACCENT: Color32 = Color32::from_rgb(163, 100, 255);
const ACCENT_DIM: Color32 = Color32::from_rgb(47, 29, 84);
const CHIP: Color32 = Color32::from_rgb(34, 34, 48);
const SUCCESS: Color32 = Color32::from_rgb(52, 211, 153);
const SUCCESS_DIM: Color32 = Color32::from_rgb(19, 58, 44);
const WARN: Color32 = Color32::from_rgb(251, 191, 36);
const WARN_DIM: Color32 = Color32::from_rgb(58, 44, 18);
const DANGER: Color32 = Color32::from_rgb(248, 113, 113);
const DANGER_DIM: Color32 = Color32::from_rgb(58, 29, 34);







use vehicle_gen::VEHICLES;

fn vehicle_index(make: &str, model: &str) -> Option<usize> {
    vehicle_gen::index_of(make, model)
}

fn vehicle_label(v: &VehicleModel) -> String {
    v.label()
}





#[derive(Clone, Copy, PartialEq, Eq)]
enum Icon {
    Grid,
    Trend,
    Car,
    List,
    Sliders,
    Wallet,
    Bolt,
    Level,
    Check,
    Refresh,
    Folder,
    Search,
    Save,
    Info,
    Close,
}

fn draw_icon(p: &egui::Painter, rect: Rect, icon: Icon, color: Color32) {
    let side = rect.width().min(rect.height());
    let r = Rect::from_center_size(rect.center(), Vec2::splat(side));
    let u = side / 24.0;
    let pt = |x: f32, y: f32| egui::pos2(r.min.x + x * u, r.min.y + y * u);
    let stroke = Stroke::new((1.9 * u).max(1.2), color);

    match icon {
        Icon::Grid => {
            for (x, y) in [(4.0, 4.0), (13.0, 4.0), (4.0, 13.0), (13.0, 13.0)] {
                p.rect_filled(
                    Rect::from_min_size(pt(x, y), Vec2::splat(7.0 * u)),
                    Rounding::same(1.7 * u),
                    color,
                );
            }
        }
        Icon::Trend => {
            p.add(egui::Shape::line(
                vec![pt(3.5, 18.0), pt(9.0, 12.0), pt(13.0, 15.0), pt(20.0, 6.5)],
                stroke,
            ));
            p.add(egui::Shape::convex_polygon(
                vec![pt(20.5, 6.0), pt(14.5, 6.0), pt(20.0, 11.5)],
                color,
                Stroke::NONE,
            ));
        }
        Icon::Car => {
            p.add(egui::Shape::closed_line(
                vec![
                    pt(3.0, 17.0),
                    pt(4.0, 12.0),
                    pt(7.5, 9.0),
                    pt(16.5, 9.0),
                    pt(20.0, 12.0),
                    pt(21.0, 17.0),
                ],
                stroke,
            ));
            p.add(egui::Shape::line(
                vec![pt(7.5, 9.0), pt(9.5, 5.5), pt(14.5, 5.5), pt(16.5, 9.0)],
                stroke,
            ));
            p.circle_stroke(pt(7.0, 17.8), 2.2 * u, stroke);
            p.circle_stroke(pt(17.0, 17.8), 2.2 * u, stroke);
        }
        Icon::List => {
            for (i, y) in [6.0, 12.0, 18.0].iter().enumerate() {
                let _ = i;
                p.circle_filled(pt(5.0, *y), 1.7 * u, color);
                p.add(egui::Shape::line(vec![pt(9.5, *y), pt(20.0, *y)], stroke));
            }
        }
        Icon::Sliders => {
            for (y, x) in [(6.0, 9.0), (12.0, 16.0), (18.0, 7.0)] {
                p.add(egui::Shape::line(vec![pt(3.0, y), pt(21.0, y)], stroke));
                p.circle_filled(pt(x, y), 2.7 * u, color);
            }
        }
        Icon::Wallet => {
            p.rect_stroke(
                Rect::from_min_size(pt(2.5, 6.5), Vec2::new(19.0 * u, 12.0 * u)),
                Rounding::same(2.5 * u),
                stroke,
            );
            p.rect_filled(
                Rect::from_min_size(pt(14.0, 10.5), Vec2::new(7.5 * u, 4.5 * u)),
                Rounding::same(1.6 * u),
                color,
            );
            p.add(egui::Shape::line(vec![pt(5.0, 9.0), pt(12.0, 9.0)], stroke));
        }
        Icon::Bolt => {
            p.add(egui::Shape::convex_polygon(
                vec![
                    pt(13.5, 2.0),
                    pt(5.0, 14.0),
                    pt(11.0, 14.0),
                    pt(9.0, 22.0),
                    pt(19.0, 9.5),
                    pt(12.5, 9.5),
                ],
                color,
                Stroke::NONE,
            ));
        }
        Icon::Level => {
            p.add(egui::Shape::line(
                vec![pt(4.0, 13.0), pt(12.0, 5.5), pt(20.0, 13.0)],
                stroke,
            ));
            p.add(egui::Shape::line(
                vec![pt(4.0, 19.5), pt(12.0, 12.0), pt(20.0, 19.5)],
                stroke,
            ));
        }
        Icon::Check => {
            p.add(egui::Shape::line(
                vec![pt(4.0, 12.5), pt(9.5, 18.0), pt(20.0, 6.0)],
                Stroke::new((2.6 * u).max(1.6), color),
            ));
        }
        Icon::Refresh => {
            let mut pts = Vec::new();
            for i in 0..=16 {
                let a = -0.7 + i as f32 * (4.9 / 16.0);
                pts.push(pt(12.0 + 7.2 * a.cos(), 12.0 + 7.2 * a.sin()));
            }
            p.add(egui::Shape::line(pts, stroke));
            p.add(egui::Shape::convex_polygon(
                vec![pt(19.8, 4.2), pt(14.0, 5.2), pt(18.9, 10.0)],
                color,
                Stroke::NONE,
            ));
        }
        Icon::Folder => {
            p.add(egui::Shape::closed_line(
                vec![
                    pt(3.0, 7.0),
                    pt(9.0, 7.0),
                    pt(11.5, 9.5),
                    pt(21.0, 9.5),
                    pt(21.0, 19.0),
                    pt(3.0, 19.0),
                ],
                stroke,
            ));
        }
        Icon::Search => {
            p.circle_stroke(pt(10.5, 10.5), 6.0 * u, stroke);
            p.add(egui::Shape::line(
                vec![pt(15.2, 15.2), pt(21.0, 21.0)],
                Stroke::new((2.2 * u).max(1.4), color),
            ));
        }
        Icon::Save => {
            p.rect_stroke(
                Rect::from_min_size(pt(3.0, 3.5), Vec2::new(18.0 * u, 17.0 * u)),
                Rounding::same(2.2 * u),
                stroke,
            );
            p.rect_filled(
                Rect::from_min_size(pt(7.5, 4.0), Vec2::new(9.0 * u, 6.0 * u)),
                Rounding::same(1.0 * u),
                color,
            );
            p.rect_stroke(
                Rect::from_min_size(pt(7.0, 14.0), Vec2::new(10.0 * u, 6.0 * u)),
                Rounding::same(1.0 * u),
                stroke,
            );
        }
        Icon::Info => {
            p.circle_stroke(pt(12.0, 12.0), 8.6 * u, stroke);
            p.circle_filled(pt(12.0, 7.8), 1.5 * u, color);
            p.add(egui::Shape::line(
                vec![pt(12.0, 11.0), pt(12.0, 17.0)],
                Stroke::new((2.2 * u).max(1.4), color),
            ));
        }
        Icon::Close => {
            p.add(egui::Shape::line(
                vec![pt(6.0, 6.0), pt(18.0, 18.0)],
                Stroke::new((2.2 * u).max(1.4), color),
            ));
            p.add(egui::Shape::line(
                vec![pt(18.0, 6.0), pt(6.0, 18.0)],
                Stroke::new((2.2 * u).max(1.4), color),
            ));
        }
    }
}

fn icon(ui: &mut Ui, kind: Icon, size: f32, color: Color32) -> Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    draw_icon(ui.painter(), rect, kind, color);
    resp
}





#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Dashboard,
    Progression,
    Listings,
    Advanced,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StatusKind {
    Info,
    Success,
    Error,
}

struct Status {
    kind: StatusKind,
    text: String,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            kind: StatusKind::Info,
            text: "Load a save to begin".into(),
        }
    }
}

#[derive(Clone)]
struct SaveSlot {
    path: PathBuf,
    name: String,
}

#[derive(Clone)]
struct SlotRow {
    index: usize,
    hour: f32,
    board: u8,
    cheap: bool,
    done: bool,
    seed: i32,
    advert: Option<usize>,
    o_hour: f32,
    o_board: u8,
    o_cheap: bool,
    o_done: bool,
    o_seed: i32,
}

impl SlotRow {
    fn changed(&self) -> bool {
        self.hour != self.o_hour
            || self.board != self.o_board
            || self.cheap != self.o_cheap
            || self.done != self.o_done
            || self.seed != self.o_seed
    }
}

#[derive(Clone)]
struct AdvertRow {
    index: usize,
    seed: i32,
    board: u8,
    year: i64,
    make: String,
    model: String,
    key: String,
    price: i64,
    
    
    
    price_known: bool,
    mileage: i64,
    seller: String,
    photo: Option<PathBuf>,
    title: TitleBrand,
    
    
    price_basis: PriceBasis,
    
    
    deal_floor_ratio: f64,
    deal_asking: i64,
    deal_price: i64,
    deal_floor: i64,
    deal_mechanic_drop: i64,
    
    
    odometer_rolled_back: bool,
    o_year: i64,
    o_make: String,
    o_model: String,
    o_key: String,
    o_price: i64,
    o_mileage: i64,
    o_seed: i32,
    o_title: TitleBrand,
}

impl AdvertRow {
    fn changed(&self) -> bool {
        self.year != self.o_year
            || self.make != self.o_make
            || self.model != self.o_model
            || self.key != self.o_key
            || self.price != self.o_price
            || self.mileage != self.o_mileage
            || self.title != self.o_title
    }

    fn model(&self) -> Option<&'static VehicleModel> {
        vehicle_gen::find(&self.make, &self.model)
    }

    
    
    
    fn recalculate_price(&mut self) {
        if !self.price_known {
            return;
        }
        let Some(model) = self.model() else {
            return;
        };
        self.price = self.price_basis.price_for(model, self.mileage, self.title);
    }

    
    
    fn select_vehicle(&mut self, model: &'static VehicleModel) {
        self.make = model.make.to_string();
        self.model = model.model.to_string();
        self.key = model.key.to_string();
        
        
        self.photo = None;
        
        
        self.price_basis = self.price_basis.retarget(model, self.mileage, self.title);
        self.price_known = true;
        self.price = self.price_basis.price_for(model, self.mileage, self.title);
    }
}

#[derive(Clone, Copy)]
enum PreviewTarget {
    Slot(usize),
    Advert(usize),
}

#[derive(Clone, Copy)]
struct Preview {
    target: PreviewTarget,
    vehicle: usize,
}

#[derive(Clone, Copy)]
enum Confirm {
    Reseed,
    RespawnAll,
    RerollAllSeeds,
}

struct App {
    saves: Vec<SaveSlot>,
    selected: usize,
    editor: Option<Editor>,
    tab: Tab,

    cash: i64,
    level: i64,
    xp: i64,
    career_seed: i64,
    o_cash: i64,
    o_level: i64,
    o_xp: i64,
    o_career_seed: i64,

    slots: Vec<SlotRow>,
    adverts: Vec<AdvertRow>,

    cars_sold: i64,
    day: i64,
    weekday: String,
    play_seconds: f64,

    status: Status,
    textures: HashMap<String, Option<egui::TextureHandle>>,
    preview: Option<Preview>,
    expanded_slot: Option<usize>,
    expanded_advert: Option<usize>,
    show_help: bool,
    confirm: Option<Confirm>,

    search: String,
    raw_path: String,
    raw_value: String,
}

impl Default for App {
    fn default() -> Self {
        let initial_tab = match std::env::var("CF_TAB")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "listings" => Tab::Listings,
            "progression" => Tab::Progression,
            "advanced" => Tab::Advanced,
            _ => Tab::Dashboard,
        };
        let mut app = Self {
            saves: Vec::new(),
            selected: 0,
            editor: None,
            tab: initial_tab,
            cash: 0,
            level: 1,
            xp: 0,
            career_seed: 0,
            o_cash: 0,
            o_level: 1,
            o_xp: 0,
            o_career_seed: 0,
            slots: Vec::new(),
            adverts: Vec::new(),
            cars_sold: 0,
            day: 0,
            weekday: String::new(),
            play_seconds: 0.0,
            status: Status::default(),
            textures: HashMap::new(),
            preview: None,
            expanded_slot: None,
            expanded_advert: None,
            show_help: false,
            confirm: None,
            search: String::new(),
            raw_path: String::new(),
            raw_value: String::new(),
        };
        app.reload_save_list();
        app
    }
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn reload_save_list(&mut self) {
        let mut saves = Vec::new();
        if let Ok(paths) = save::list_save_files() {
            for p in paths {
                let name = p
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("save.sav")
                    .to_string();
                if name.starts_with("CF_Index") || name.starts_with("CF_Profile") {
                    continue;
                }
                saves.push(SaveSlot { path: p, name });
            }
        }
        
        saves.sort_by(|a, b| {
            let ta = std::fs::metadata(&a.path).and_then(|m| m.modified()).ok();
            let tb = std::fs::metadata(&b.path).and_then(|m| m.modified()).ok();
            tb.cmp(&ta)
        });
        self.saves = saves;
        if self.selected >= self.saves.len() {
            self.selected = 0;
        }
        if !self.saves.is_empty() && self.editor.is_none() {
            self.load_current();
        }
    }

    fn load_current(&mut self) {
        let Some(slot) = self.saves.get(self.selected).cloned() else {
            self.editor = None;
            self.set_error("No save found. Start Car Flipper once, then hit Refresh.");
            return;
        };
        match Editor::load(&slot.path) {
            Ok(ed) => {
                self.editor = Some(ed);
                self.refresh_from_editor();
                self.expanded_slot = None;
                self.expanded_advert = None;
                self.set_ok(format!("Loaded {}", slot.name));
            }
            Err(e) => {
                self.editor = None;
                self.set_error(format!("Could not read save: {}", e));
            }
        }
    }

    fn refresh_from_editor(&mut self) {
        let Some(ed) = self.editor.as_ref() else {
            return;
        };
        let cash = get_i(ed, "Phone.Cash").unwrap_or(0);
        let level = get_i(ed, "Phone.Level").unwrap_or(1);
        let xp = get_i(ed, "Phone.Xp").unwrap_or(0);
        let career_seed = get_i(ed, "Phone.BoardFeed.CareerSeed").unwrap_or(0);
        let cars_sold = get_i(ed, "Phone.Lifetime.CarsSold").unwrap_or(0);
        let day = get_i(ed, "Phone.Day").unwrap_or(0);
        let weekday = get_s(ed, "Phone.WeekDay").unwrap_or_default();
        let play_seconds = get_f64(ed, "PlaySeconds").unwrap_or(0.0);
        let adverts = build_adverts(ed);
        let slots = build_slots(ed, &adverts);

        self.cash = cash;
        self.level = level;
        self.xp = xp;
        self.career_seed = career_seed;
        self.cars_sold = cars_sold;
        self.day = day;
        self.weekday = weekday;
        self.play_seconds = play_seconds;
        self.adverts = adverts;
        self.slots = slots;
        self.sync_originals();
    }

    fn sync_originals(&mut self) {
        self.o_cash = self.cash;
        self.o_level = self.level;
        self.o_xp = self.xp;
        self.o_career_seed = self.career_seed;
        for s in &mut self.slots {
            s.o_hour = s.hour;
            s.o_board = s.board;
            s.o_cheap = s.cheap;
            s.o_done = s.done;
            s.o_seed = s.seed;
        }
        for a in &mut self.adverts {
            a.o_year = a.year;
            a.o_make = a.make.clone();
            a.o_model = a.model.clone();
            a.o_key = a.key.clone();
            a.o_price = a.price;
            a.o_mileage = a.mileage;
            a.o_seed = a.seed;
            a.o_title = a.title;
        }
    }

    fn is_dirty(&self) -> bool {
        self.cash != self.o_cash
            || self.level != self.o_level
            || self.xp != self.o_xp
            || self.career_seed != self.o_career_seed
            || self.slots.iter().any(SlotRow::changed)
            || self.adverts.iter().any(AdvertRow::changed)
            || self.editor.as_ref().map(|e| e.is_dirty()).unwrap_or(false)
    }

    fn set_ok(&mut self, text: impl Into<String>) {
        self.status = Status {
            kind: StatusKind::Success,
            text: text.into(),
        };
    }

    fn set_error(&mut self, text: impl Into<String>) {
        self.status = Status {
            kind: StatusKind::Error,
            text: text.into(),
        };
    }

    fn set_info(&mut self, text: impl Into<String>) {
        self.status = Status {
            kind: StatusKind::Info,
            text: text.into(),
        };
    }

    fn apply_changes(&mut self) {
        let Some(mut ed) = self.editor.clone() else {
            self.set_error("No save loaded");
            return;
        };
        if let Err(e) = apply_to(&mut ed, self) {
            self.set_error(e);
            return;
        }
        match ed.save() {
            Ok(backup) => {
                let mut note = String::new();
                if let Ok(Some(msg)) = save::sync_index_for(&ed.path, &ed.data, &ed.nodes) {
                    note = format!("  ({})", msg);
                }
                self.editor = Some(ed);
                self.sync_originals();
                self.set_ok(format!(
                    "Saved successfully. Backup: {}{}",
                    backup.display(),
                    note
                ));
            }
            Err(e) => self.set_error(e),
        }
    }

    fn apply_raw(&mut self) {
        let path = self.raw_path.trim().to_string();
        if path.is_empty() {
            self.set_error("Enter a property path first.");
            return;
        }
        let Some(mut ed) = self.editor.clone() else {
            self.set_error("No save loaded");
            return;
        };
        let value = self.raw_value.clone();
        match ed.set(&path, &value).and_then(|_| ed.save()) {
            Ok(backup) => {
                let _ = save::sync_index_for(&ed.path, &ed.data, &ed.nodes);
                self.editor = Some(ed);
                self.refresh_from_editor();
                self.set_ok(format!("Set {}. Backup: {}", path, backup.display()));
            }
            Err(e) => self.set_error(e),
        }
    }

    fn open_save_folder(&mut self) {
        let _ = std::process::Command::new("explorer")
            .arg(save::save_dir())
            .spawn();
        self.set_info("Opened the save folder.");
    }

    fn reseed(&mut self) {
        self.career_seed = vehicle_gen::random_seed() as i64;
        self.set_info("New career seed queued. Press APPLY CHANGES to keep it.");
    }

    fn respawn_all(&mut self) {
        for s in &mut self.slots {
            s.done = false;
        }
        self.set_info("All spawn slots marked unused. Press APPLY CHANGES.");
    }

    fn reroll_all_seeds(&mut self) {
        for s in &mut self.slots {
            s.seed = vehicle_gen::random_seed();
        }
        self.set_info("Spawn seeds rerolled. Press APPLY CHANGES.");
    }

    fn car_texture(
        &mut self,
        ctx: &egui::Context,
        make: &str,
        model: &str,
        key: &str,
    ) -> Option<egui::TextureHandle> {
        let file_names = [
            format!("{}.png", key),
            format!("{}.jpg", key),
            format!("{}.jpeg", key),
        ];
        let mut dirs: Vec<PathBuf> = Vec::new();
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                dirs.push(parent.join("cars"));
                dirs.push(parent.join("assets").join("cars"));
            }
        }
        if let Ok(cwd) = std::env::current_dir() {
            dirs.push(cwd.join("cars"));
            dirs.push(cwd.join("assets").join("cars"));
        }
        for dir in dirs {
            for name in &file_names {
                let path = dir.join(name);
                if path.exists() {
                    let cache_key = format!("car:{}", path.display());
                    if let Some(tex) = load_texture(ctx, &mut self.textures, &cache_key, &path) {
                        return Some(tex);
                    }
                }
            }
        }

        
        let representative = self
            .adverts
            .iter()
            .find(|a| a.make.eq_ignore_ascii_case(make) && a.model.eq_ignore_ascii_case(model))
            .and_then(|a| a.photo.clone());
        if let Some(path) = representative {
            let cache_key = format!("photo:{}", path.display());
            return load_texture(ctx, &mut self.textures, &cache_key, &path);
        }
        None
    }

    fn advert_texture(&mut self, ctx: &egui::Context, index: usize) -> Option<egui::TextureHandle> {
        let row = self.adverts.get(index)?;
        let photo = row.photo.clone();
        let make = row.make.clone();
        let model = row.model.clone();
        let key = row.key.clone();

        if let Some(path) = photo {
            let cache_key = format!("photo:{}", path.display());
            if let Some(tex) = load_texture(ctx, &mut self.textures, &cache_key, &path) {
                return Some(tex);
            }
        }
        self.car_texture(ctx, &make, &model, &key)
    }

    fn confirm_preview(&mut self) {
        let Some(preview) = self.preview.take() else {
            return;
        };
        let vehicle = &VEHICLES[preview.vehicle];
        let make = vehicle.make.to_string();
        let model = vehicle.model.to_string();
        let label = vehicle_label(vehicle);

        match preview.target {
            PreviewTarget::Slot(slot_index) => {
                let advert_index = self.slots.get(slot_index).and_then(|s| s.advert);
                if let Some(ai) = advert_index {
                    if let Some(ad) = self.adverts.get_mut(ai) {
                        if let Some(model_def) = vehicle_gen::find(&make, &model) {
                            ad.select_vehicle(model_def);
                        }
                    }
                    self.set_ok(format!(
                        "Slot #{:02} queued: {}. Price recalculated; other advert fields preserved.",
                        slot_index + 1,
                        label
                    ));
                }
            }
            PreviewTarget::Advert(advert_index) => {
                if let Some(ad) = self.adverts.get_mut(advert_index) {
                    if let Some(model_def) = vehicle_gen::find(&make, &model) {
                        ad.select_vehicle(model_def);
                    }
                }
                self.set_ok(format!(
                    "Listing #{} queued: {}. Price recalculated; miles, seller, board, seed and title preserved.",
                    advert_index + 1,
                    label
                ));
            }
        }
    }

    fn preview_window(&mut self, ctx: &egui::Context) {
        let Some(preview) = self.preview else { return };
        let vehicle = &VEHICLES[preview.vehicle];
        let (title, subtitle) = match preview.target {
            PreviewTarget::Slot(i) => (
                format!("Spawn slot #{:02}", i + 1),
                "Confirm to queue this car for the slot.",
            ),
            PreviewTarget::Advert(i) => (
                format!("Listing #{}", i + 1),
                "Confirm to queue this car for the listing.",
            ),
        };
        let texture = self.car_texture(ctx, vehicle.make, vehicle.model, vehicle.key);
        let pos = egui::pos2(ctx.screen_rect().right() - 376.0, 220.0);
        egui::Area::new(egui::Id::new("vehicle_preview"))
            .order(egui::Order::Foreground)
            .fixed_pos(pos)
            .show(ctx, |ui| {
                Frame::none()
                    .fill(CARD)
                    .stroke(Stroke::new(1.0_f32, BORDER))
                    .rounding(Rounding::same(14.0))
                    .inner_margin(Margin::same(16.0))
                    .show(ui, |ui| {
                        ui.set_width(300.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("VEHICLE PREVIEW")
                                    .size(10.0)
                                    .strong()
                                    .color(ACCENT),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if chip(ui, "Close").clicked() {
                                    self.preview = None;
                                }
                            });
                        });
                        ui.add_space(4.0);
                        ui.label(RichText::new(&title).size(16.0).strong().color(TEXT));
                        ui.label(RichText::new(subtitle).size(11.0).color(MUTED));
                        ui.add_space(8.0);
                        paint_thumb(
                            ui,
                            Vec2::new(300.0, 170.0),
                            texture.as_ref(),
                            Some(&vehicle_label(vehicle)),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("{}  /  {}", vehicle.category, vehicle.key))
                                .size(10.5)
                                .color(MUTED),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("The car preview will be generated once loaded.")
                                .size(10.0)
                                .color(MUTED)
                                .italics(),
                        );
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if primary_button(ui, "Confirm", true).clicked() {
                                self.confirm_preview();
                            }
                            if secondary_button(ui, "Cancel").clicked() {
                                self.preview = None;
                            }
                        });
                    });
            });
    }
}

impl App {
    fn expanded_photo_window(&mut self, ctx: &egui::Context) {
        let (tex, title) = if let Some(i) = self.expanded_slot {
            let ai = self.slots.get(i).and_then(|s| s.advert);
            let tex = ai.and_then(|ai| self.advert_texture(ctx, ai));
            (tex, format!("Spawn slot #{}", i + 1))
        } else if let Some(i) = self.expanded_advert {
            let tex = self.advert_texture(ctx, i);
            (tex, format!("Listing #{}", i + 1))
        } else {
            return;
        };

        let mut open = true;
        egui::Window::new("Photo preview")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_width(420.0)
            .show(ctx, |ui| {
                ui.label(RichText::new(&title).size(14.0).strong().color(TEXT));
                ui.add_space(4.0);
                paint_thumb(ui, Vec2::new(400.0, 225.0), tex.as_ref(), None);
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Click the photo again to close.")
                        .size(10.0)
                        .color(MUTED),
                );
                ui.add_space(8.0);
                if secondary_button(ui, "Close").clicked() {
                    self.expanded_slot = None;
                    self.expanded_advert = None;
                }
            });
        if !open {
            self.expanded_slot = None;
            self.expanded_advert = None;
        }
    }
}

fn load_texture(
    ctx: &egui::Context,
    cache: &mut HashMap<String, Option<egui::TextureHandle>>,
    key: &str,
    path: &Path,
) -> Option<egui::TextureHandle> {
    if !cache.contains_key(key) {
        let texture = std::fs::read(path)
            .ok()
            .and_then(|bytes| image::load_from_memory(&bytes).ok())
            .map(|img| {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                ctx.load_texture(key.to_string(), color, egui::TextureOptions::LINEAR)
            });
        cache.insert(key.to_string(), texture);
    }
    cache.get(key).and_then(|t| t.clone())
}

fn paint_thumb(
    ui: &mut Ui,
    size: Vec2,
    texture: Option<&egui::TextureHandle>,
    label: Option<&str>,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let painter = ui.painter();
    painter.rect_filled(rect, Rounding::same(9.0), CARD_ALT);
    let inner = rect.shrink(1.0);

    if let Some(tex) = texture {
        let tex_size = tex.size_vec2();
        let scale = (inner.width() / tex_size.x).max(inner.height() / tex_size.y);
        let displayed = tex_size * scale;
        let uv = Rect::from_center_size(
            egui::pos2(0.5, 0.5),
            Vec2::new(inner.width() / displayed.x, inner.height() / displayed.y),
        );
        painter.image(tex.id(), inner, uv, Color32::WHITE);
    } else {
        painter.rect_filled(inner, Rounding::same(8.0), Color32::from_rgb(30, 24, 44));
        let icon_size = inner.height() * 0.45;
        draw_icon(
            painter,
            Rect::from_center_size(
                egui::pos2(inner.center().x, inner.min.y + icon_size - 1.0),
                Vec2::splat(icon_size),
            ),
            Icon::Car,
            ACCENT,
        );
        painter.text(
            egui::pos2(inner.center().x, inner.max.y - 9.0),
            Align2::CENTER_CENTER,
            "preview art",
            FontId::proportional(9.5),
            MUTED,
        );
    }

    if let Some(label) = label {
        let height = 20.0;
        let strip = Rect::from_min_max(egui::pos2(inner.min.x, inner.max.y - height), inner.max);
        painter.rect_filled(
            strip,
            Rounding {
                nw: 0.0,
                ne: 0.0,
                sw: 8.0,
                se: 8.0,
            },
            Color32::from_black_alpha(170),
        );
        painter.text(
            egui::pos2(strip.min.x + 8.0, strip.center().y),
            Align2::LEFT_CENTER,
            label,
            FontId::proportional(11.0),
            TEXT,
        );
    }
    response
}





fn child<'a>(n: &'a Node, name: &str) -> Option<&'a Node> {
    n.children
        .iter()
        .find(|c| c.path.rsplit('.').next() == Some(name))
}

fn child_i(n: &Node, name: &str) -> i64 {
    child(n, name)
        .and_then(|c| c.display.trim().parse::<i64>().ok())
        .unwrap_or(0)
}

fn child_f(n: &Node, name: &str) -> f32 {
    child(n, name)
        .and_then(|c| c.display.trim().parse::<f32>().ok())
        .unwrap_or(0.0)
}

fn child_b(n: &Node, name: &str) -> bool {
    child(n, name).map(|c| c.display == "true").unwrap_or(false)
}

fn child_s(n: &Node, name: &str) -> String {
    child(n, name)
        .and_then(|c| c.text.clone())
        .unwrap_or_default()
}

fn get_i(ed: &Editor, path: &str) -> Option<i64> {
    ed.get(path)
        .ok()
        .and_then(|n| n.display.trim().parse::<i64>().ok())
}

fn get_f64(ed: &Editor, path: &str) -> Option<f64> {
    ed.get(path)
        .ok()
        .and_then(|n| n.display.trim().parse::<f64>().ok())
}

fn get_s(ed: &Editor, path: &str) -> Option<String> {
    ed.get(path).ok().and_then(|n| n.text.clone())
}

fn build_adverts(ed: &Editor) -> Vec<AdvertRow> {
    let mut out = Vec::new();
    let Ok(list) = ed.get("Phone.BoardFeed.Adverts") else {
        return out;
    };
    for (i, el) in list.children.iter().enumerate() {
        let seed = child_i(el, "Seed") as i32;
        let board = child_i(el, "Board") as u8;
        let listing = child(el, "Listing");
        let (year, make, model, key, price, mileage, seller, title) = match listing {
            Some(l) => {
                let status = child_s(l, "TitleStatus");
                let brand = child(l, "Truth")
                    .and_then(|t| child(t, "Paperwork"))
                    .map(|p| child_s(p, "TitleBrand"))
                    .unwrap_or_default();
                let title = TitleBrand::from_enum_name(&brand)
                    .or_else(|| TitleBrand::from_status(&status))
                    .unwrap_or(TitleBrand::Clean);
                (
                    child_i(l, "Year"),
                    child_s(l, "Make"),
                    child_s(l, "Model"),
                    child_s(l, "ModelKey"),
                    child_i(l, "AskingPrice"),
                    child_i(l, "Mileage"),
                    child_s(l, "SellerName"),
                    title,
                )
            }
            None => (
                0,
                String::new(),
                String::new(),
                String::new(),
                0,
                0,
                String::new(),
                TitleBrand::Clean,
            ),
        };
        let photo_rel = child_s(el, "PhotoFile");
        let photo = if photo_rel.is_empty() {
            None
        } else {
            let base = save::save_dir()
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default();
            let p = base.join(photo_rel);
            if p.exists() {
                Some(p)
            } else {
                None
            }
        };
        let deal_asking = child_i(el, "DealAsking");
        let deal_price = child_i(el, "DealPrice");
        let deal_floor = child_i(el, "DealFloor");
        let deal_mechanic_drop = child_i(el, "DealMechanicDrop");
        let price_basis = match vehicle_gen::find(&make, &model) {
            Some(model_def) => PriceBasis::from_saved(model_def, price, mileage, title),
            None => PriceBasis {
                model_clean: price.max(1_000),
                market: 1.0,
                saved_price: price,
                saved_mileage: mileage,
                saved_title: title,
            },
        };
        let deal_floor_ratio = if price > 0 && deal_floor > 0 {
            (deal_floor as f64 / price as f64).clamp(0.5, 1.0)
        } else {
            0.85
        };
        let odometer_rolled_back = child(el, "Listing")
            .and_then(|l| child(l, "Truth"))
            .and_then(|t| child(t, "Masking"))
            .map(|m| child_b(m, "bOdometerRolledBack"))
            .unwrap_or(false);
        out.push(AdvertRow {
            index: i,
            seed,
            board,
            year,
            photo,
            title,
            price_basis,
            deal_floor_ratio,
            deal_asking,
            deal_price,
            deal_floor,
            deal_mechanic_drop,
            odometer_rolled_back,
            o_year: year,
            o_make: make.clone(),
            o_model: model.clone(),
            o_key: key.clone(),
            o_price: price,
            o_mileage: mileage,
            o_seed: seed,
            o_title: title,
            make,
            model,
            key,
            price,
            price_known: price > 0,
            mileage,
            seller,
        });
    }
    out
}

fn build_slots(ed: &Editor, adverts: &[AdvertRow]) -> Vec<SlotRow> {
    let mut out = Vec::new();
    let Ok(list) = ed.get("Phone.BoardFeed.Slots") else {
        return out;
    };
    for (i, el) in list.children.iter().enumerate() {
        let hour = child_f(el, "Hour");
        let board = child_i(el, "Board") as u8;
        let cheap = child_b(el, "bCheap");
        let done = child_b(el, "bDone");
        let seed = child_i(el, "Seed") as i32;
        let advert = adverts.iter().position(|a| a.seed == seed);
        out.push(SlotRow {
            index: i,
            hour,
            board,
            cheap,
            done,
            seed,
            advert,
            o_hour: hour,
            o_board: board,
            o_cheap: cheap,
            o_done: done,
            o_seed: seed,
        });
    }
    out
}

fn apply_to(ed: &mut Editor, app: &App) -> Res<()> {
    if app.cash != app.o_cash {
        ed.set("Phone.Cash", &app.cash.to_string())?;
    }
    if app.level != app.o_level {
        ed.set("Phone.Level", &app.level.to_string())?;
    }
    if app.xp != app.o_xp {
        ed.set("Phone.Xp", &app.xp.to_string())?;
    }
    if app.career_seed != app.o_career_seed {
        ed.set("Phone.BoardFeed.CareerSeed", &app.career_seed.to_string())?;
    }

    for s in &app.slots {
        if !s.changed() {
            continue;
        }
        let base = format!("Phone.BoardFeed.Slots[{}]", s.index);
        if s.hour != s.o_hour {
            ed.set(&format!("{}.Hour", base), &format!("{}", s.hour))?;
        }
        if s.board != s.o_board {
            ed.set(&format!("{}.Board", base), &s.board.to_string())?;
        }
        if s.cheap != s.o_cheap {
            ed.set(
                &format!("{}.bCheap", base),
                if s.cheap { "true" } else { "false" },
            )?;
        }
        if s.done != s.o_done {
            ed.set(
                &format!("{}.bDone", base),
                if s.done { "true" } else { "false" },
            )?;
        }
        if s.seed != s.o_seed {
            ed.set(&format!("{}.Seed", base), &s.seed.to_string())?;
        }
    }

    for a in &app.adverts {
        if !a.changed() {
            continue;
        }
        let ad = format!("Phone.BoardFeed.Adverts[{}]", a.index);
        let base = format!("{}.Listing", ad);
        let truth = format!("{}.Truth", base);
        let mut patches: Vec<(String, String)> = Vec::new();

        if a.make != a.o_make {
            patches.push((format!("{}.Make", base), a.make.clone()));
        }
        if a.model != a.o_model {
            patches.push((format!("{}.Model", base), a.model.clone()));
        }
        if a.key != a.o_key {
            patches.push((format!("{}.ModelKey", base), a.key.clone()));
        }
        if a.year != a.o_year {
            patches.push((format!("{}.Year", base), a.year.to_string()));
        }
        let identity_changed = a.make != a.o_make || a.model != a.o_model || a.key != a.o_key;
        if identity_changed && a.seed == a.o_seed {
            
            
            
            
            patches.push((format!("{}.PhotoFile", ad), String::new()));
        }

        if a.price != a.o_price {
            patches.push((format!("{}.AskingPrice", base), a.price.to_string()));
            patches.push((format!("{}.DealAsking", ad), a.price.to_string()));

            
            
            let deal_price = if a.deal_price == a.o_price || a.o_price == 0 {
                a.price
            } else {
                a.deal_price
            };
            patches.push((format!("{}.DealPrice", ad), deal_price.to_string()));
            let floor =
                vehicle_gen::round_price((a.price as f64 * a.deal_floor_ratio).round() as i64);
            patches.push((format!("{}.DealFloor", ad), floor.to_string()));
        }

        if a.mileage != a.o_mileage {
            patches.push((format!("{}.Mileage", base), a.mileage.to_string()));
            if !a.odometer_rolled_back {
                patches.push((
                    format!("{}.Paperwork.TrueMileage", truth),
                    a.mileage.to_string(),
                ));
            }
        }

        if a.title != a.o_title {
            patches.push((
                format!("{}.TitleStatus", base),
                a.title.status_name().to_string(),
            ));
            patches.push((
                format!("{}.Paperwork.TitleBrand", truth),
                a.title.enum_name().to_string(),
            ));
            patches.push((
                format!("{}.Mechanical.bTitleProblem", truth),
                if a.title.is_branded() {
                    "true"
                } else {
                    "false"
                }
                .to_string(),
            ));
        }

        if a.seed != a.o_seed {
            patches.push((format!("{}.Seed", ad), a.seed.to_string()));
            patches.push((format!("{}.Seed", base), a.seed.to_string()));
            
            
            let stem = ed.path.file_stem().and_then(|x| x.to_str()).unwrap_or("");
            let save_id = stem.strip_prefix("CF_").unwrap_or(stem);
            patches.push((
                format!("{}.PhotoFile", ad),
                format!("SaveGames/AdvertPhotos/{}/{}.jpg", save_id, a.seed),
            ));
        }

        
        
        if let Ok(blurb_node) = ed.get(&format!("{}.Blurb", base)) {
            if let Some(text) = blurb_node.text.clone() {
                if let Some(rewritten) = vehicle_gen::rewrite_blurb(
                    &text,
                    &a.o_make,
                    &a.o_model,
                    &a.make,
                    &a.model,
                    a.o_price,
                    a.price,
                    a.o_mileage,
                    a.mileage,
                ) {
                    patches.push((format!("{}.Blurb", base), rewritten));
                }
            }
        }

        if !patches.is_empty() {
            let refs: Vec<(&str, &str)> = patches
                .iter()
                .map(|(p, v)| (p.as_str(), v.as_str()))
                .collect();
            ed.set_many(&refs)?;
        }
    }
    Ok(())
}





fn fmt_num(n: i64) -> String {
    let neg = n < 0;
    let digits = n.abs().to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    let s: String = out.chars().rev().collect();
    if neg {
        format!("-{}", s)
    } else {
        s
    }
}

fn fmt_playtime(secs: f64) -> String {
    let total = secs.max(0.0) as i64;
    let h = total / 3600;
    let m = (total / 60) % 60;
    if h > 0 {
        format!("{}h {}m", h, m)
    } else {
        format!("{}m", m)
    }
}

fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::none()
        .fill(CARD)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .rounding(Rounding::same(14.0))
        .inner_margin(Margin::same(18.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add(ui)
        })
        .inner
}

fn fill_widths(base: &[f32], spacing: f32, available: f32) -> Vec<f32> {
    let n = base.len() as f32;
    let spacing_total = spacing * (n - 1.0);
    let base_total: f32 = base.iter().sum();
    let delta = (available - spacing_total - base_total) / n;
    base.iter().map(|w| (w + delta).max(24.0)).collect()
}

fn cell<R>(ui: &mut Ui, width: f32, height: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.allocate_ui_with_layout(
        Vec2::new(width, height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_max_width(width);
            add(ui)
        },
    )
    .inner
}

fn card_width<R>(ui: &mut Ui, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::none()
        .fill(CARD)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .rounding(Rounding::same(14.0))
        .inner_margin(Margin::same(18.0))
        .show(ui, |ui| {
            ui.set_width(width.max(120.0));
            add(ui)
        })
        .inner
}

fn card_title(ui: &mut Ui, kind: Icon, title: &str, subtitle: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
        ui.painter()
            .rect_filled(rect, Rounding::same(10.0), ACCENT_DIM);
        draw_icon(ui.painter(), rect.shrink(9.0), kind, ACCENT);
        ui.vertical(|ui| {
            ui.label(RichText::new(title).size(16.0).strong().color(TEXT));
            if !subtitle.is_empty() {
                ui.label(RichText::new(subtitle).size(12.0).color(MUTED));
            }
        });
    });
    ui.add_space(4.0);
}

fn page_header(ui: &mut Ui, title: &str, subtitle: &str) {
    ui.label(RichText::new(title).size(24.0).strong().color(TEXT));
    ui.label(RichText::new(subtitle).size(13.0).color(MUTED));
    ui.add_space(4.0);
}

fn chip(ui: &mut Ui, text: &str) -> Response {
    ui.add(
        egui::Button::new(RichText::new(text).size(12.0).color(TEXT))
            .fill(CHIP)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(14.0))
            .min_size(Vec2::new(0.0, 28.0)),
    )
}

fn chip_sized(ui: &mut Ui, text: &str, width: f32, height: f32) -> Response {
    ui.add(
        egui::Button::new(RichText::new(text).size(12.0).color(TEXT))
            .fill(CHIP)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(8.0))
            .min_size(Vec2::new(width, height)),
    )
}

fn toggle_square(ui: &mut Ui, value: &mut bool, size: f32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    let (bg, stroke_color) = if *value {
        (ACCENT_DIM, ACCENT)
    } else {
        (CHIP, BORDER)
    };
    ui.painter().rect_filled(rect, Rounding::same(8.0), bg);
    ui.painter().rect_stroke(
        rect,
        Rounding::same(8.0),
        Stroke::new(1.0_f32, stroke_color),
    );
    if *value {
        draw_icon(ui.painter(), rect.shrink(size * 0.26), Icon::Check, ACCENT);
    }
    if response.clicked() {
        *value = !*value;
    }
    response
}

fn primary_button(ui: &mut Ui, text: &str, enabled: bool) -> Response {
    let fill = if enabled { ACCENT } else { CHIP };
    ui.add_enabled(
        enabled,
        egui::Button::new(
            RichText::new(text)
                .size(14.0)
                .strong()
                .color(Color32::WHITE),
        )
        .fill(fill)
        .rounding(Rounding::same(9.0))
        .min_size(Vec2::new(0.0, 40.0)),
    )
}

fn secondary_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(
        egui::Button::new(RichText::new(text).size(13.0).color(TEXT))
            .fill(CHIP)
            .stroke(Stroke::new(1.0_f32, BORDER))
            .rounding(Rounding::same(9.0))
            .min_size(Vec2::new(0.0, 34.0)),
    )
}

fn icon_button(ui: &mut Ui, kind: Icon, tip: &str) -> Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(34.0), Sense::click());
    let bg = if resp.hovered() {
        CARD_ALT
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, Rounding::same(9.0), bg);
    draw_icon(
        ui.painter(),
        rect.shrink(8.0),
        kind,
        if resp.hovered() { TEXT } else { MUTED },
    );
    resp.on_hover_text(tip)
}

fn nav_item(ui: &mut Ui, selected: bool, kind: Icon, label: &str) -> Response {
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 42.0), Sense::click());
    let bg = if selected {
        ACCENT_DIM
    } else if resp.hovered() {
        CARD
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, Rounding::same(9.0), bg);
    if selected {
        ui.painter().rect_filled(
            Rect::from_min_size(rect.min + Vec2::new(0.0, 10.0), Vec2::new(3.0, 22.0)),
            Rounding::same(1.5),
            ACCENT,
        );
    }
    draw_icon(
        ui.painter(),
        Rect::from_center_size(
            egui::pos2(rect.min.x + 26.0, rect.center().y),
            Vec2::splat(18.0),
        ),
        kind,
        if selected { ACCENT } else { MUTED },
    );
    ui.painter().text(
        egui::pos2(rect.min.x + 46.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(14.0),
        if selected { TEXT } else { MUTED },
    );
    resp
}

fn pill(ui: &mut Ui, text: &str, color: Color32, bg: Color32) {
    Frame::none()
        .fill(bg)
        .rounding(Rounding::same(20.0))
        .inner_margin(Margin::symmetric(10.0, 5.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(Vec2::splat(7.0), Sense::hover());
                ui.painter().circle_filled(r.center(), 3.5, color);
                ui.label(RichText::new(text).size(10.5).strong().color(color));
            });
        });
}

#[allow(clippy::too_many_arguments)]
fn stat_editor(
    ui: &mut Ui,
    kind: Icon,
    title: &str,
    subtitle: &str,
    value: &mut i64,
    slider_range: std::ops::RangeInclusive<i64>,
    clamp_range: std::ops::RangeInclusive<i64>,
    speed: f64,
    presets: &[(&str, i64)],
    money: bool,
) {
    Frame::none()
        .fill(CARD)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .rounding(Rounding::same(13.0))
        .inner_margin(Margin::same(18.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, Rounding::same(8.0), ACCENT_DIM);
                draw_icon(ui.painter(), rect.shrink(7.5), kind, ACCENT);
                ui.vertical(|ui| {
                    ui.label(RichText::new(title).size(14.5).strong().color(TEXT));
                    ui.label(RichText::new(subtitle).size(11.0).color(MUTED));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let mut dv = egui::DragValue::new(value)
                        .speed(speed)
                        .clamp_range(clamp_range.clone());
                    if money {
                        dv = dv.prefix("$");
                    }
                    ui.add(dv);
                });
            });
            ui.add_space(6.0);
            ui.spacing_mut().slider_width = (ui.available_width() - 2.0).max(140.0);
            ui.add(egui::Slider::new(value, slider_range).show_value(false));
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Presets").size(11.0).color(MUTED));
                for (label, v) in presets {
                    if chip(ui, label).clicked() {
                        *value = *v;
                    }
                }
            });
        });
}

fn stat_card(ui: &mut Ui, kind: Icon, label: &str, value: &str, tint: Color32) {
    Frame::none()
        .fill(CARD)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .rounding(Rounding::same(13.0))
        .inner_margin(Margin::same(16.0))
        .show(ui, |ui| {
            ui.set_min_height(88.0);
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, Rounding::same(8.0), ACCENT_DIM);
                draw_icon(ui.painter(), rect.shrink(7.0), kind, tint);
                ui.add_space(2.0);
                ui.label(RichText::new(label).size(11.5).color(MUTED));
            });
            ui.add_space(8.0);
            ui.label(RichText::new(value).size(22.0).strong().color(TEXT));
        });
}

fn vehicle_picker(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    make: &str,
    model: &str,
    width: f32,
    height: f32,
) -> Option<usize> {
    let current = vehicle_index(make, model);
    let mut sel = current;
    let label = current
        .map(|i| vehicle_label(&VEHICLES[i]))
        .unwrap_or_else(|| format!("{} {}", make, model).trim().to_string());
    
    
    
    let display = elide_to_width(ui, &label, (width - 30.0).max(24.0), 12.5);
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.y = height;
        ui.spacing_mut().button_padding = Vec2::new(4.0, 0.0);
        let response = egui::ComboBox::from_id_source(id)
            .width(width)
            .height(340.0)
            .selected_text(RichText::new(display).size(12.5))
            .show_ui(ui, |ui| {
                let mut last = "";
                for (i, v) in VEHICLES.iter().enumerate() {
                    if v.category != last {
                        if !last.is_empty() {
                            ui.separator();
                        }
                        ui.label(RichText::new(v.category).size(10.0).strong().color(MUTED));
                        last = v.category;
                    }
                    ui.selectable_value(&mut sel, Some(i), vehicle_label(v));
                }
            })
            .response;
        response.on_hover_text(&label);
    });
    if sel != current {
        return sel;
    }
    None
}

fn elide_to_width(ui: &Ui, text: &str, max_width: f32, size: f32) -> String {
    let font = FontId::proportional(size);
    let width_of = |s: &str| {
        ui.fonts(|f| {
            f.layout_no_wrap(s.to_owned(), font.clone(), Color32::WHITE)
                .size()
                .x
        })
    };
    if width_of(text) <= max_width {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let ellipsis = "…";
    let ellipsis_width = width_of(ellipsis);
    let mut lo = 0usize;
    let mut hi = chars.len();
    while lo < hi {
        let mid = lo + (hi - lo).div_ceil(2);
        let candidate: String = chars[..mid].iter().collect();
        if width_of(&candidate) + ellipsis_width <= max_width {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let mut out: String = chars[..lo].iter().collect();
    out.push_str(ellipsis);
    out
}

fn board_combo(ui: &mut Ui, id: impl std::hash::Hash, board: &mut u8, height: f32) -> bool {
    let before = *board;
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.y = height;
        ui.spacing_mut().button_padding.y = 0.0;
        egui::ComboBox::from_id_source(id)
            .width(104.0)
            .selected_text(RichText::new(save::board_label(&board.to_string())).size(12.5))
            .show_ui(ui, |ui| {
                ui.selectable_value(board, 0, "Curbside");
                ui.selectable_value(board, 1, "Dougslist");
            });
    });
    *board != before
}





impl App {
    fn top_bar(&mut self, ctx: &egui::Context) {
        enum Act {
            Load(usize),
            Reload,
            Folder,
            Help,
        }
        let mut act: Option<Act> = None;

        egui::TopBottomPanel::top("top")
            .exact_height(84.0)
            .frame(
                Frame::none()
                    .fill(PANEL)
                    .stroke(Stroke::new(1.0_f32, BORDER))
                    .inner_margin(Margin {
                        left: 20.0,
                        right: 20.0,
                        top: 6.0,
                        bottom: 6.0,
                    }),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    
                    let (logo, _) = ui.allocate_exact_size(Vec2::splat(40.0), Sense::hover());
                    ui.painter()
                        .rect_filled(logo, Rounding::same(11.0), ACCENT_DIM);
                    draw_icon(ui.painter(), logo.shrink(9.0), Icon::Car, ACCENT);

                    let text_x = logo.right() + 10.0;
                    ui.painter().text(
                        egui::pos2(text_x, logo.top()),
                        Align2::LEFT_TOP,
                        "CAR FLIPPER",
                        FontId::proportional(16.0),
                        TEXT,
                    );
                    ui.painter().text(
                        egui::pos2(text_x, logo.top() + 19.0),
                        Align2::LEFT_TOP,
                        "SAVE EDITOR",
                        FontId::proportional(10.0),
                        ACCENT,
                    );
                    ui.add_space(122.0);
                    ui.add_space(20.0);

                    
                    ui.scope(|ui| {
                        ui.spacing_mut().interact_size.y = 34.0;
                        ui.spacing_mut().button_padding.y = 8.0;
                        let names: Vec<String> =
                            self.saves.iter().map(|s| s.name.clone()).collect();
                        let mut sel = self.selected;
                        let current = names
                            .get(sel)
                            .cloned()
                            .unwrap_or_else(|| "No save found".into());
                        egui::ComboBox::from_id_source("save_selector")
                            .width(238.0)
                            .selected_text(RichText::new(current).size(12.5))
                            .show_ui(ui, |ui| {
                                for (i, name) in names.iter().enumerate() {
                                    if ui.selectable_value(&mut sel, i, name).clicked() {
                                        act = Some(Act::Load(i));
                                    }
                                }
                            });
                        if icon_button(ui, Icon::Refresh, "Reload the selected save").clicked() {
                            act = Some(Act::Reload);
                        }
                        if icon_button(ui, Icon::Folder, "Open the save folder").clicked() {
                            act = Some(Act::Folder);
                        }
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if icon_button(ui, Icon::Info, "How to use").clicked() {
                            act = Some(Act::Help);
                        }
                        if self.is_dirty() {
                            pill(ui, "UNSAVED", WARN, WARN_DIM);
                        }
                        if self.editor.is_some() {
                            pill(ui, "SAVE LOADED", SUCCESS, SUCCESS_DIM);
                        } else {
                            pill(ui, "NO SAVE", MUTED, CARD);
                        }
                    });
                });
            });

        match act {
            Some(Act::Load(i)) => {
                self.selected = i;
                self.load_current();
            }
            Some(Act::Reload) => {
                self.reload_save_list();
                self.load_current();
            }
            Some(Act::Folder) => self.open_save_folder(),
            Some(Act::Help) => self.show_help = true,
            None => {}
        }
    }

    fn footer(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("footer")
            .exact_height(64.0)
            .frame(
                Frame::none()
                    .fill(PANEL)
                    .stroke(Stroke::new(1.0_f32, BORDER))
                    .inner_margin(Margin::symmetric(20.0, 0.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    let (color, bg) = match self.status.kind {
                        StatusKind::Info => (MUTED, CARD),
                        StatusKind::Success => (SUCCESS, SUCCESS_DIM),
                        StatusKind::Error => (DANGER, DANGER_DIM),
                    };
                    ui.label(RichText::new(&self.status.text).size(11.5).color(color));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let dirty = self.is_dirty();
                        if primary_button(ui, "APPLY CHANGES", dirty).clicked() {
                            self.apply_changes();
                        }
                        if ui
                            .add_enabled(
                                !dirty,
                                egui::Button::new(RichText::new("Revert").size(12.5).color(MUTED))
                                    .fill(CHIP)
                                    .stroke(Stroke::new(1.0_f32, BORDER)),
                            )
                            .clicked()
                        {
                            self.load_current();
                        }
                        let _ = bg;
                    });
                });
            });
    }

    fn sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("nav")
            .exact_width(216.0)
            .frame(
                Frame::none()
                    .fill(PANEL)
                    .inner_margin(Margin::symmetric(14.0, 18.0)),
            )
            .show(ctx, |ui| {
                ui.label(RichText::new("WORKSHOP").size(10.0).strong().color(MUTED));
                ui.add_space(8.0);

                if nav_item(ui, self.tab == Tab::Dashboard, Icon::Grid, "Dashboard").clicked() {
                    self.tab = Tab::Dashboard;
                }
                if nav_item(ui, self.tab == Tab::Progression, Icon::Trend, "Progression").clicked()
                {
                    self.tab = Tab::Progression;
                }
                if nav_item(ui, self.tab == Tab::Listings, Icon::List, "Listings").clicked() {
                    self.tab = Tab::Listings;
                }
                if nav_item(ui, self.tab == Tab::Advanced, Icon::Sliders, "Advanced").clicked() {
                    self.tab = Tab::Advanced;
                }

                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("Creates a .bak before every write")
                            .size(10.0)
                            .color(MUTED),
                    );
                    ui.label(
                        RichText::new("Close the game before editing")
                            .size(10.0)
                            .color(MUTED),
                    );
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("v1.0  /  UE 5.8 GVAS")
                            .size(10.0)
                            .color(MUTED),
                    );
                });
            });
    }

    fn central(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(BG).inner_margin(Margin::same(22.0)))
            .show(ctx, |ui| {
                if self.tab == Tab::Listings {
                    self.page_listings(ui);
                } else {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| match self.tab {
                            Tab::Dashboard => self.page_dashboard(ui),
                            Tab::Progression => self.page_progression(ui),
                            Tab::Advanced => self.page_advanced(ui),
                            _ => {}
                        });
                }
            });
    }

    fn page_dashboard(&mut self, ui: &mut Ui) {
        page_header(
            ui,
            "Dashboard",
            "Your save at a glance. Change values, then press APPLY CHANGES.",
        );

        if self.editor.is_none() {
            card(ui, |ui| {
                ui.label(
                    RichText::new("No save loaded")
                        .size(18.0)
                        .strong()
                        .color(TEXT),
                );
                ui.label(
                    RichText::new("Start Car Flipper once so it creates a save, then hit Refresh in the top bar.")
                        .size(12.5)
                        .color(MUTED),
                );
                ui.add_space(10.0);
                if secondary_button(ui, "Open save folder").clicked() {
                    self.open_save_folder();
                }
            });
            return;
        }

        ui.columns(4, |cols| {
            stat_card(
                &mut cols[0],
                Icon::Wallet,
                "CASH",
                &format!("${}", fmt_num(self.cash)),
                ACCENT,
            );
            stat_card(
                &mut cols[1],
                Icon::Level,
                "LEVEL",
                &self.level.to_string(),
                ACCENT,
            );
            stat_card(&mut cols[2], Icon::Bolt, "XP", &fmt_num(self.xp), ACCENT);
            stat_card(
                &mut cols[3],
                Icon::Car,
                "CARS SOLD",
                &fmt_num(self.cars_sold),
                ACCENT,
            );
        });

        ui.add_space(4.0);

        card(ui, |ui| {
            card_title(
                ui,
                Icon::Bolt,
                "Quick actions",
                "One click fills the value. APPLY CHANGES writes it to the save.",
            );
            ui.horizontal(|ui| {
                ui.label(RichText::new("Cash").size(12.5).color(MUTED));
                for v in [10_000i64, 100_000, 1_000_000, 5_000_000] {
                    if chip(ui, &format!("${}", fmt_num(v))).clicked() {
                        self.cash = v;
                    }
                }
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Level").size(12.5).color(MUTED));
                for v in [10i64, 25, 50, 100] {
                    if chip(ui, &v.to_string()).clicked() {
                        self.level = v;
                    }
                }
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("XP").size(12.5).color(MUTED));
                for (label, v) in [
                    ("Reset", 0i64),
                    ("1K", 1_000),
                    ("10K", 10_000),
                    ("100K", 100_000),
                ] {
                    if chip(ui, label).clicked() {
                        self.xp = v;
                    }
                }
            });
        });

        ui.add_space(4.0);

        card(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
                ui.painter().rect_filled(rect, Rounding::same(6.0), ACCENT_DIM);
                draw_icon(ui.painter(), rect.shrink(4.5), Icon::Info, ACCENT);
                ui.label(RichText::new("Before you edit").size(13.0).strong().color(TEXT));
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!(
                        "Day {}  /  {}  /  {}   -   close the game first, a .bak is created on every write",
                        self.day,
                        self.weekday,
                        fmt_playtime(self.play_seconds)
                    ))
                    .size(11.5)
                    .color(MUTED),
                );
            });
        });
    }

    fn page_progression(&mut self, ui: &mut Ui) {
        page_header(
            ui,
            "Progression",
            "Cash, level and XP. Use the slider for a quick change or type an exact value.",
        );

        stat_editor(
            ui,
            Icon::Wallet,
            "Cash",
            "Money in your pocket.",
            &mut self.cash,
            0i64..=10_000_000i64,
            0i64..=999_999_999i64,
            100.0,
            &[
                ("$10K", 10_000),
                ("$100K", 100_000),
                ("$1M", 1_000_000),
                ("$5M", 5_000_000),
            ],
            true,
        );

        ui.add_space(4.0);

        stat_editor(
            ui,
            Icon::Level,
            "Level",
            "Your flip rank. The bar may look odd until the next XP gain.",
            &mut self.level,
            1i64..=100i64,
            1i64..=100i64,
            1.0,
            &[("10", 10), ("25", 25), ("50", 50), ("100", 100)],
            false,
        );

        ui.add_space(4.0);

        stat_editor(
            ui,
            Icon::Bolt,
            "XP",
            "XP inside the current level. Set to 0 after changing the level.",
            &mut self.xp,
            0i64..=100_000i64,
            0i64..=999_999_999i64,
            50.0,
            &[("0", 0), ("1K", 1_000), ("10K", 10_000), ("100K", 100_000)],
            false,
        );
    }

    fn page_listings(&mut self, ui: &mut Ui) {
        page_header(
            ui,
            "Listings",
            "All cars currently for sale. Pick a vehicle, set miles and title. Price is calculated.",
        );
        let curbside = self.slots.iter().filter(|s| s.board == 0).count();
        let dougs = self.slots.iter().filter(|s| s.board == 1).count();
        let unused = self.slots.iter().filter(|s| !s.done).count();

        card(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, Rounding::same(8.0), ACCENT_DIM);
                draw_icon(ui.painter(), rect.shrink(7.0), Icon::Sliders, ACCENT);
                ui.label(
                    RichText::new("Board schedule")
                        .size(14.0)
                        .strong()
                        .color(TEXT),
                );
                ui.add_space(10.0);
                ui.label(RichText::new("Career seed").size(11.5).color(MUTED));
                ui.add(
                    egui::DragValue::new(&mut self.career_seed)
                        .speed(1.0)
                        .clamp_range(0i64..=2_147_483_647i64),
                );
                if chip(ui, "Reroll").clicked() {
                    self.confirm = Some(Confirm::Reseed);
                }
                if chip(ui, "Respawn all").clicked() {
                    self.confirm = Some(Confirm::RespawnAll);
                }
                if chip(ui, "Reroll all cars").clicked() {
                    self.confirm = Some(Confirm::RerollAllSeeds);
                }
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                pill(ui, &format!("CURBSIDE {}", curbside), TEXT, CARD_ALT);
                pill(ui, &format!("DOUGSLIST {}", dougs), TEXT, CARD_ALT);
                pill(ui, &format!("UNUSED {}", unused), SUCCESS, SUCCESS_DIM);
                pill(
                    ui,
                    &format!("LISTINGS {}", self.adverts.len()),
                    ACCENT,
                    ACCENT_DIM,
                );
            });
        });
        ui.add_space(12.0);
        card(ui, |ui| {
            card_title(
                ui,
                Icon::List,
                "All listings",
                "Select a vehicle, edit miles and title. Price is calculated from the saved listing.",
            );
            let scroll_h = (ui.available_height() - 4.0).max(240.0);
            self.adverts_table(ui, scroll_h);
        });
    }

    fn page_advanced(&mut self, ui: &mut Ui) {
        page_header(
            ui,
            "Advanced",
            "Raw property access for power users. Everything here is optional.",
        );

        card(ui, |ui| {
            card_title(
                ui,
                Icon::Save,
                "Save information",
                "The exact file the tool is editing.",
            );
            if let Some(slot) = self.saves.get(self.selected) {
                ui.label(
                    RichText::new(slot.path.display().to_string())
                        .size(11.5)
                        .monospace()
                        .color(TEXT),
                );
            }
            if let Some(ed) = &self.editor {
                ui.label(
                    RichText::new(format!("{}  /  {} bytes", ed.class(), ed.data_len()))
                        .size(11.0)
                        .color(MUTED),
                );
            }
        });

        ui.add_space(4.0);

        card(ui, |ui| {
            card_title(
                ui,
                Icon::Search,
                "Find a property",
                "Search by path, e.g. Cash, Xp, Adverts, Asleep.",
            );
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Search properties...")
                        .desired_width(320.0),
                );
                if !self.search.is_empty() && chip(ui, "Clear").clicked() {
                    self.search.clear();
                }
            });
            ui.add_space(4.0);

            let mut results: Vec<(String, String, String, String)> = Vec::new();
            if let Some(ed) = &self.editor {
                if !self.search.trim().is_empty() {
                    let q = self.search.trim().to_ascii_lowercase();
                    let mut flat = Vec::new();
                    save::collect(ed.nodes(), &mut flat);
                    for n in flat {
                        if n.path.to_ascii_lowercase().contains(&q) {
                            results.push((
                                n.path.clone(),
                                n.typ.clone(),
                                n.display.clone(),
                                n.text.clone().unwrap_or_default(),
                            ));
                            if results.len() >= 200 {
                                break;
                            }
                        }
                    }
                }
            }

            if results.is_empty() {
                ui.label(RichText::new("No matches yet.").size(12.0).color(MUTED));
            } else {
                egui::ScrollArea::vertical()
                    .id_source("search_scroll")
                    .max_height(150.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Grid::new("search_grid")
                            .num_columns(3)
                            .spacing([16.0, 5.0])
                            .show(ui, |ui| {
                                for (path, typ, val, text) in results {
                                    ui.label(
                                        RichText::new(path.clone())
                                            .monospace()
                                            .size(11.0)
                                            .color(TEXT),
                                    );
                                    ui.label(RichText::new(typ).size(10.0).color(MUTED));
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(val).size(11.0).color(MUTED));
                                        if chip(ui, "Edit").clicked() {
                                            self.raw_path = path;
                                            self.raw_value = text;
                                        }
                                    });
                                    ui.end_row();
                                }
                            });
                    });
            }
        });

        ui.add_space(4.0);

        card(ui, |ui| {
            card_title(
                ui,
                Icon::Save,
                "Set a property",
                "Applies immediately and writes the save.",
            );
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(50.0, 24.0),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.label(RichText::new("Path").size(12.0).color(MUTED));
                    },
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.raw_path)
                        .hint_text("Phone.Cash")
                        .desired_width(320.0),
                );
            });
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(50.0, 24.0),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.label(RichText::new("Value").size(12.0).color(MUTED));
                    },
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.raw_value)
                        .hint_text("100000")
                        .desired_width(320.0),
                );
            });
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                
                
                ui.allocate_ui_with_layout(
                    Vec2::new(50.0, 24.0),
                    Layout::right_to_left(Align::Center),
                    |_ui| {},
                );
                if secondary_button(ui, "Set & Save").clicked() {
                    self.apply_raw();
                }
            });
        });
    }
}

impl App {}

impl App {
    #[allow(clippy::needless_range_loop)]
    fn adverts_table(&mut self, ui: &mut Ui, scroll_height: f32) {
        if self.adverts.is_empty() {
            ui.label(
                RichText::new("No listings right now.")
                    .size(12.0)
                    .color(MUTED),
            );
            return;
        }

        let ctx = ui.ctx().clone();
        let count = self.adverts.len();
        let mut photos: Vec<Option<egui::TextureHandle>> = Vec::with_capacity(count);
        for i in 0..count {
            photos.push(self.advert_texture(&ctx, i));
        }

        let widths = table_layout::widths((ui.available_width() - 2.0).max(0.0), COLUMN_SPACING);
        let content_width =
            widths.iter().sum::<f32>() + COLUMN_SPACING * (ADVERT_COLUMNS.len() - 1) as f32;

        egui::ScrollArea::vertical()
            .id_source("adv_scroll")
            .max_height(scroll_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::ScrollArea::horizontal()
                    .id_source("adv_scroll_h")
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        ui.set_min_width(content_width.max(ui.available_width()));
                        egui::Grid::new("adv_grid")
                            .num_columns(ADVERT_COLUMNS.len())
                            .spacing([COLUMN_SPACING, ROW_SPACING])
                            .show(ui, |ui| {
                                for (spec, width) in ADVERT_COLUMNS.iter().zip(&widths) {
                                    table_header(ui, *width, spec);
                                }
                                ui.end_row();

                                for i in 0..count {
                                    let mut picked = None;
                                    let mut price_rect: Option<Rect> = None;
                                    {
                                        let row = &mut self.adverts[i];
                                        
                                        
                                        
                                        row.recalculate_price();

                                        
                                        cell(ui, widths[0], ROW_HEIGHT, |ui| {
                                            let size = Vec2::new(64.0, 44.0);
                                            ui.add_space(((widths[0] - size.x) / 2.0).max(0.0));
                                            let resp =
                                                paint_thumb(ui, size, photos[i].as_ref(), None);
                                            if resp.clicked() {
                                                if self.expanded_advert == Some(i) {
                                                    self.expanded_advert = None;
                                                } else {
                                                    self.expanded_advert = Some(i);
                                                    self.expanded_slot = None;
                                                }
                                            }
                                        });

                                        
                                        cell(ui, widths[1], ROW_HEIGHT, |ui| {
                                            picked = vehicle_picker(
                                                ui,
                                                ("adv_vehicle", i),
                                                &row.make,
                                                &row.model,
                                                (widths[1] - 12.0).max(40.0),
                                                CONTROL_HEIGHT,
                                            );
                                        });

                                        
                                        
                                        
                                        
                                        cell(ui, widths[2], ROW_HEIGHT, |ui| {
                                            price_rect = Some(price_cell_rect(ui, widths[2]));
                                        });

                                        
                                        cell(ui, widths[3], ROW_HEIGHT, |ui| {
                                            ui.spacing_mut().interact_size.y = CONTROL_HEIGHT;
                                            let resp = ui.add_sized(
                                                [widths[3] - 12.0, CONTROL_HEIGHT],
                                                egui::DragValue::new(&mut row.mileage)
                                                    .speed(500.0)
                                                    .clamp_range(0i64..=2_000_000i64),
                                            );
                                            if resp.changed() {
                                                row.price_known = true;
                                                row.recalculate_price();
                                            }
                                        });

                                        
                                        cell(ui, widths[4], ROW_HEIGHT, |ui| {
                                            text_cell(
                                                ui,
                                                widths[4],
                                                &row.seller,
                                                FontId::proportional(11.5),
                                                MUTED,
                                                table_layout::Align::Left,
                                            );
                                        });

                                        
                                        cell(ui, widths[5], ROW_HEIGHT, |ui| {
                                            text_cell(
                                                ui,
                                                widths[5],
                                                save::board_label(&row.board.to_string()),
                                                FontId::proportional(11.5),
                                                MUTED,
                                                table_layout::Align::Center,
                                            );
                                        });

                                        
                                        cell(ui, widths[6], ROW_HEIGHT, |ui| {
                                            text_cell(
                                                ui,
                                                widths[6],
                                                &row.seed.to_string(),
                                                FontId::monospace(10.5),
                                                MUTED,
                                                table_layout::Align::Center,
                                            );
                                        });

                                        
                                        cell(ui, widths[7], ROW_HEIGHT, |ui| {
                                            let before = row.title;
                                            title_combo(
                                                ui,
                                                ("adv_title", i),
                                                &mut row.title,
                                                (widths[7] - 12.0).max(40.0),
                                            );
                                            if row.title != before {
                                                row.price_known = true;
                                                row.recalculate_price();
                                            }
                                        });
                                    }
                                    if let Some(rect) = price_rect {
                                        paint_price_cell(ui, rect, &self.adverts[i]);
                                    }
                                    ui.end_row();

                                    if let Some(v) = picked {
                                        self.preview = Some(Preview {
                                            target: PreviewTarget::Advert(i),
                                            vehicle: v,
                                        });
                                    }
                                }
                            });
                    });
            });
    }
}

fn table_header(ui: &mut Ui, width: f32, spec: &table_layout::ColumnSpec) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, HEADER_HEIGHT), Sense::hover());
    let (pos, align) = match spec.align {
        table_layout::Align::Left => (
            egui::pos2(rect.left() + 6.0, rect.center().y),
            Align2::LEFT_CENTER,
        ),
        table_layout::Align::Center => (rect.center(), Align2::CENTER_CENTER),
    };
    ui.painter()
        .text(pos, align, spec.header, FontId::proportional(10.0), MUTED);
}

fn price_cell_rect(ui: &mut Ui, width: f32) -> Rect {
    let size = Vec2::new((width - 12.0).max(8.0), CONTROL_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter()
        .rect_filled(rect, Rounding::same(6.0), CARD_ALT);
    rect
}

fn paint_price_cell(ui: &Ui, rect: Rect, row: &AdvertRow) {
    let (text, font, color) = if row.price_known {
        (
            format!("${}", fmt_num(row.price)),
            FontId::proportional(12.5),
            TEXT,
        )
    } else {
        ("—".to_string(), FontId::proportional(12.0), MUTED)
    };
    ui.painter().with_clip_rect(rect.shrink(4.0)).text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        font,
        color,
    );
}

fn text_cell(
    ui: &mut Ui,
    width: f32,
    text: &str,
    font: FontId,
    color: Color32,
    align: table_layout::Align,
) {
    let size = Vec2::new((width - 12.0).max(8.0), CONTROL_HEIGHT);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter()
        .rect_filled(rect, Rounding::same(6.0), CARD_ALT);
    let clip = rect.shrink(4.0);
    let painter = ui.painter().with_clip_rect(clip);
    let (pos, a) = match align {
        table_layout::Align::Left => (
            egui::pos2(rect.left() + 8.0, rect.center().y),
            Align2::LEFT_CENTER,
        ),
        table_layout::Align::Center => (rect.center(), Align2::CENTER_CENTER),
    };
    painter.text(pos, a, text, font, color);
    resp.on_hover_text(text);
}

fn title_combo(ui: &mut Ui, id: impl std::hash::Hash, title: &mut TitleBrand, width: f32) {
    let mut sel = *title;
    let display = elide_to_width(ui, sel.label(), (width - 30.0).max(24.0), 11.5);
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.y = CONTROL_HEIGHT;
        ui.spacing_mut().button_padding = Vec2::new(4.0, 0.0);
        let response = egui::ComboBox::from_id_source(id)
            .width(width)
            .selected_text(RichText::new(display).size(11.5))
            .show_ui(ui, |ui| {
                for t in TitleBrand::ALL {
                    ui.selectable_value(&mut sel, t, t.label());
                }
            })
            .response;
        response.on_hover_text(sel.label());
    });
    *title = sel;
}





impl App {
    fn confirm_window(&mut self, ctx: &egui::Context) {
        let Some(kind) = self.confirm else { return };
        let (title, body) = match kind {
            Confirm::Reseed => (
                "Reroll the schedule?",
                "This changes the career seed. The next listings the game generates will be different.",
            ),
            Confirm::RespawnAll => (
                "Respawn every slot?",
                "All 24 spawn slots will be marked unused and can fire again. This can flood the boards.",
            ),
            Confirm::RerollAllSeeds => (
                "Reroll every car?",
                "Each spawn slot gets a new seed, so different cars will be generated.",
            ),
        };
        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label(RichText::new(body).size(12.5).color(MUTED));
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if primary_button(ui, "Yes, do it", true).clicked() {
                        self.confirm = None;
                        match kind {
                            Confirm::Reseed => self.reseed(),
                            Confirm::RespawnAll => self.respawn_all(),
                            Confirm::RerollAllSeeds => self.reroll_all_seeds(),
                        }
                    }
                    if secondary_button(ui, "Cancel").clicked() {
                        self.confirm = None;
                    }
                });
            });
    }

    fn help_window(&mut self, ctx: &egui::Context) {
        if !self.show_help {
            return;
        }
        let mut open = true;
        egui::Window::new("How to use")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_width(460.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("1. Close Car Flipper").size(13.0).strong().color(TEXT));
                ui.label(
                    RichText::new("The game rewrites its save when it exits, so edit while it is closed.")
                        .size(12.0)
                        .color(MUTED),
                );
                ui.add_space(8.0);
                ui.label(RichText::new("2. Pick a save").size(13.0).strong().color(TEXT));
                ui.label(
                    RichText::new("Use the dropdown in the top bar, then hit Refresh after playing.")
                        .size(12.0)
                        .color(MUTED),
                );
                ui.add_space(8.0);
                ui.label(RichText::new("3. Change values").size(13.0).strong().color(TEXT));
                ui.label(
                    RichText::new("Sliders and number fields update the preview. Nothing is written yet.")
                        .size(12.0)
                        .color(MUTED),
                );
                ui.add_space(8.0);
                ui.label(RichText::new("4. Press APPLY CHANGES").size(13.0).strong().color(TEXT));
                ui.label(
                    RichText::new("The save is written and a timestamped .bak is created next to it.")
                        .size(12.0)
                        .color(MUTED),
                );
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(RichText::new("Listings").size(13.0).strong().color(TEXT));
                ui.label(
                    RichText::new("On Listings, pick a vehicle and confirm the preview. Miles and title are editable; seller, board and seed are preserved. Price follows the saved listing's basis and the selected title.")
                        .size(12.0)
                        .color(MUTED),
                );
            });
        if !open {
            self.show_help = false;
        }
    }
}





impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_bar(ctx);
        self.footer(ctx);
        self.sidebar(ctx);
        self.central(ctx);
        self.preview_window(ctx);
        self.expanded_photo_window(ctx);
        self.confirm_window(ctx);
        self.help_window(ctx);
    }
}

fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let mut v = egui::Visuals::dark();
    v.panel_fill = BG;
    v.window_fill = CARD;
    v.extreme_bg_color = Color32::from_rgb(8, 8, 12);
    v.faint_bg_color = PANEL;
    v.window_rounding = Rounding::same(14.0);
    v.window_stroke = Stroke::new(1.0_f32, BORDER);
    v.menu_rounding = Rounding::same(10.0);
    v.selection.bg_fill = ACCENT_DIM;
    v.selection.stroke = Stroke::new(1.0_f32, ACCENT);
    v.hyperlink_color = ACCENT;

    v.widgets.noninteractive.bg_fill = CARD;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, BORDER);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT);
    v.widgets.noninteractive.rounding = Rounding::same(8.0);

    v.widgets.inactive.bg_fill = CHIP;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, BORDER);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
    v.widgets.inactive.rounding = Rounding::same(8.0);

    v.widgets.hovered.bg_fill = CARD_ALT;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, ACCENT);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, TEXT);
    v.widgets.hovered.rounding = Rounding::same(8.0);

    v.widgets.active.bg_fill = ACCENT_DIM;
    v.widgets.active.bg_stroke = Stroke::new(1.0_f32, ACCENT);
    v.widgets.active.fg_stroke = Stroke::new(1.0_f32, TEXT);
    v.widgets.active.rounding = Rounding::same(8.0);

    v.widgets.open.bg_fill = CARD_ALT;
    v.widgets.open.bg_stroke = Stroke::new(1.0_f32, BORDER);
    v.widgets.open.fg_stroke = Stroke::new(1.0_f32, TEXT);
    v.widgets.open.rounding = Rounding::same(8.0);

    style.visuals = v;
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(14.0, 8.0);
    style.spacing.slider_width = 180.0;
    style
        .text_styles
        .insert(egui::TextStyle::Heading, FontId::proportional(26.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, FontId::proportional(14.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, FontId::proportional(13.5));
    style
        .text_styles
        .insert(egui::TextStyle::Small, FontId::proportional(11.0));
    style
        .text_styles
        .insert(egui::TextStyle::Monospace, FontId::monospace(12.5));
    ctx.set_style(style);
}

fn make_icon() -> egui::IconData {
    let w = 64usize;
    let h = 64usize;
    let mut rgba = vec![0u8; w * h * 4];
    let r = 14.0f32;
    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;
            let cx = fx.clamp(r, w as f32 - r);
            let cy = fy.clamp(r, h as f32 - r);
            let inside = (fx - cx).powi(2) + (fy - cy).powi(2) <= r * r;

            let mut px = [0u8, 0, 0, 0];
            if inside {
                px = [150, 88, 245, 255];
                let body = (14.0..=50.0).contains(&fx) && (32.0..=43.0).contains(&fy);
                let cabin = (22.0..=42.0).contains(&fx) && (24.0..=32.0).contains(&fy);
                let wheel = |wx: f32, wy: f32| {
                    let d = ((fx - wx).powi(2) + (fy - wy).powi(2)).sqrt();
                    d <= 5.0
                };
                let hub = |wx: f32, wy: f32| {
                    let d = ((fx - wx).powi(2) + (fy - wy).powi(2)).sqrt();
                    d <= 2.2
                };
                if body || cabin || wheel(22.0, 45.0) || wheel(42.0, 45.0) {
                    px = [255, 255, 255, 255];
                }
                if hub(22.0, 45.0) || hub(42.0, 45.0) {
                    px = [150, 88, 245, 255];
                }
            }
            let i = (y * w + x) * 4;
            rgba[i..i + 4].copy_from_slice(&px);
        }
    }
    egui::IconData {
        rgba,
        width: w as u32,
        height: h as u32,
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Car Flipper Save Editor")
            .with_icon(std::sync::Arc::new(make_icon()))
            .with_inner_size([1220.0, 840.0])
            .with_min_inner_size([1040.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Car Flipper Save Editor",
        options,
        Box::new(|cc| {
            setup_style(&cc.egui_ctx);
            Box::new(App::new(cc))
        }),
    )
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::fs;
    use std::time::SystemTime;

    fn newest_fixture() -> Option<PathBuf> {
        let dir = save::save_dir();
        let mut best: Option<(SystemTime, PathBuf)> = None;
        for entry in fs::read_dir(dir).ok()? {
            let path = entry.ok()?.path();
            let name = path.file_name()?.to_str()?.to_ascii_lowercase();
            if !name.starts_with("cf_")
                || name.starts_with("cf_index")
                || name.starts_with("cf_profile")
                || path.extension()?.to_str()? != "sav"
            {
                continue;
            }
            let modified = fs::metadata(&path)
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH);
            if best.as_ref().map(|(t, _)| modified > *t).unwrap_or(true) {
                best = Some((modified, path));
            }
        }
        best.map(|(_, p)| p)
    }

    fn defect_names(ed: &Editor, index: usize) -> Vec<String> {
        let path = format!("Phone.BoardFeed.Adverts[{}].Listing.Truth.Defects", index);
        ed.get(&path)
            .map(|n| n.children.iter().filter_map(|c| c.text.clone()).collect())
            .unwrap_or_default()
    }

    fn listing_path(index: usize) -> String {
        format!("Phone.BoardFeed.Adverts[{}].Listing", index)
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn advert_edits_round_trip_and_preserve_unrelated_fields() {
        let Some(src) = newest_fixture() else {
            eprintln!("skipping: no live save fixture available");
            return;
        };
        let dir = std::env::temp_dir().join(format!("cf_editor_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let tmp = dir.join("CF_test.sav");
        fs::copy(&src, &tmp).unwrap();

        let mut ed = Editor::load(&tmp).unwrap();
        let mut app = App::default();
        app.editor = Some(ed.clone());
        app.refresh_from_editor();

        assert!(
            !app.is_dirty(),
            "loading/opening the editor must not mark the save dirty"
        );
        let idx = app
            .adverts
            .iter()
            .position(|a| {
                a.price > 0
                    && a.mileage > 0
                    && a.title == TitleBrand::Clean
                    && !a.seller.is_empty()
                    && a.model().is_some()
            })
            .expect("fixture should contain at least one complete clean listing");
        let old = app.adverts[idx].clone();
        let old_defects = defect_names(&ed, idx);
        let base = listing_path(idx);
        let old_vin = get_s(&ed, &format!("{}.Vin", base));
        let untouched: Vec<(usize, i64, i64, i32)> = app
            .adverts
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != idx)
            .map(|(i, a)| (i, a.price, a.mileage, a.seed))
            .collect();

        
        app.adverts[idx].title = TitleBrand::Rebuilt;
        app.adverts[idx].recalculate_price();
        let rebuilt_price = app.adverts[idx].price;
        assert_ne!(rebuilt_price, old.price);
        apply_to(&mut ed, &app).unwrap();

        assert_eq!(
            get_s(&ed, &format!("{}.TitleStatus", base)).as_deref(),
            Some("Rebuilt")
        );
        assert_eq!(
            get_s(&ed, &format!("{}.Truth.Paperwork.TitleBrand", base)).as_deref(),
            Some("ETitleBrand::Rebuilt")
        );
        assert_eq!(
            get_i(&ed, &format!("{}.AskingPrice", base)),
            Some(rebuilt_price)
        );
        assert_eq!(get_i(&ed, &format!("{}.Mileage", base)), Some(old.mileage));
        assert_eq!(
            get_s(&ed, &format!("{}.SellerName", base)),
            Some(old.seller.clone())
        );
        assert_eq!(
            get_i(&ed, &format!("Phone.BoardFeed.Adverts[{}].Seed", idx)),
            Some(old.seed as i64)
        );
        assert_eq!(defect_names(&ed, idx), old_defects);

        
        app.sync_originals();
        app.adverts[idx].title = TitleBrand::Clean;
        app.adverts[idx].recalculate_price();
        assert_eq!(app.adverts[idx].price, old.price);
        apply_to(&mut ed, &app).unwrap();
        assert_eq!(
            get_i(&ed, &format!("{}.AskingPrice", base)),
            Some(old.price)
        );

        
        app.sync_originals();
        let mileage_before = app.adverts[idx].mileage;
        let title_before = app.adverts[idx].title;
        app.adverts[idx].mileage = mileage_before + 5_000;
        app.adverts[idx].recalculate_price();
        let mileage_price = app.adverts[idx].price;
        apply_to(&mut ed, &app).unwrap();
        assert_eq!(
            get_i(&ed, &format!("{}.Mileage", base)),
            Some(mileage_before + 5_000)
        );
        assert_eq!(
            get_i(&ed, &format!("{}.Truth.Paperwork.TrueMileage", base)),
            Some(mileage_before + 5_000)
        );
        assert_eq!(
            get_i(&ed, &format!("{}.AskingPrice", base)),
            Some(mileage_price)
        );
        assert_eq!(
            get_s(&ed, &format!("{}.TitleStatus", base)).as_deref(),
            Some(title_before.status_name())
        );
        assert_eq!(
            get_s(&ed, &format!("{}.SellerName", base)),
            Some(old.seller.clone())
        );
        assert_eq!(defect_names(&ed, idx), old_defects);

        
        app.sync_originals();
        let phantom = vehicle_gen::find("PhantomGT", "Supercar").unwrap();
        app.adverts[idx].select_vehicle(phantom);
        let phantom_price = app.adverts[idx].price;
        apply_to(&mut ed, &app).unwrap();
        assert_eq!(
            get_s(&ed, &format!("{}.Make", base)).as_deref(),
            Some("PhantomGT")
        );
        assert_eq!(
            get_s(&ed, &format!("{}.Model", base)).as_deref(),
            Some("Supercar")
        );
        assert_eq!(
            get_s(&ed, &format!("{}.ModelKey", base)).as_deref(),
            Some("phantomgt")
        );
        assert_eq!(
            get_i(&ed, &format!("{}.AskingPrice", base)),
            Some(phantom_price)
        );
        assert_eq!(
            get_i(&ed, &format!("{}.Mileage", base)),
            Some(mileage_before + 5_000)
        );
        assert_eq!(
            get_s(&ed, &format!("{}.SellerName", base)),
            Some(old.seller.clone())
        );
        assert_eq!(
            get_i(&ed, &format!("Phone.BoardFeed.Adverts[{}].Seed", idx)),
            Some(old.seed as i64)
        );
        assert_eq!(
            get_s(&ed, &format!("{}.TitleStatus", base)).as_deref(),
            Some(title_before.status_name())
        );
        assert_eq!(
            get_i(&ed, &format!("{}.Board", base)),
            Some(old.board as i64)
        );
        assert_eq!(get_s(&ed, &format!("{}.Vin", base)), old_vin);
        assert_eq!(
            get_s(&ed, &format!("Phone.BoardFeed.Adverts[{}].PhotoFile", idx))
                .map(|s| s.is_empty()),
            Some(true)
        );
        assert_eq!(defect_names(&ed, idx), old_defects);
        for (other, price, mileage, seed) in &untouched {
            let other_base = listing_path(*other);
            assert_eq!(
                get_i(&ed, &format!("{}.AskingPrice", other_base)),
                Some(*price),
                "unrelated listing {} price changed",
                other
            );
            assert_eq!(
                get_i(&ed, &format!("{}.Mileage", other_base)),
                Some(*mileage),
                "unrelated listing {} mileage changed",
                other
            );
            assert_eq!(
                get_i(&ed, &format!("Phone.BoardFeed.Adverts[{}].Seed", other)),
                Some(*seed as i64),
                "unrelated listing {} seed changed",
                other
            );
        }

        
        ed.save().unwrap();
        let reloaded = Editor::load(&tmp).unwrap();
        assert_eq!(
            get_s(&reloaded, &format!("{}.Make", base)).as_deref(),
            Some("PhantomGT")
        );
        assert_eq!(
            get_i(&reloaded, &format!("{}.Mileage", base)),
            Some(mileage_before + 5_000)
        );
        assert_eq!(
            get_i(&reloaded, &format!("{}.AskingPrice", base)),
            Some(phantom_price)
        );
        assert_eq!(
            get_s(&reloaded, &format!("{}.SellerName", base)),
            Some(old.seller.clone())
        );
        assert_eq!(
            get_i(&reloaded, &format!("Phone.BoardFeed.Adverts[{}].Seed", idx)),
            Some(old.seed as i64)
        );
        assert_eq!(
            get_s(
                &reloaded,
                &format!("Phone.BoardFeed.Adverts[{}].PhotoFile", idx)
            )
            .map(|s| s.is_empty()),
            Some(true)
        );
        assert_eq!(defect_names(&reloaded, idx), old_defects);

        let _ = fs::remove_dir_all(&dir);
    }

    
    
    #[test]
    fn defect_array_can_grow_and_shrink() {
        let Some(src) = newest_fixture() else {
            eprintln!("skipping: no live save fixture available");
            return;
        };
        let dir = std::env::temp_dir().join(format!("cf_array_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let tmp = dir.join("CF_array.sav");
        fs::copy(&src, &tmp).unwrap();

        let mut ed = Editor::load(&tmp).unwrap();
        let arr = "Phone.BoardFeed.Adverts[0].Listing.Truth.Defects";
        let original: Vec<String> = ed
            .get(arr)
            .unwrap()
            .children
            .iter()
            .filter_map(|c| c.text.clone())
            .collect();
        assert!(!original.is_empty(), "fixture has no defects to resize");

        
        let mut grown = original.clone();
        grown.push(original[0].clone());
        ed.set_name_array(arr, &grown).unwrap();
        assert_eq!(ed.get(arr).unwrap().children.len(), grown.len());
        assert!(ed.get("Phone.BoardFeed.Adverts[0].Listing.Mileage").is_ok());
        assert!(ed
            .get("Phone.BoardFeed.Adverts[0].Listing.Truth.Presentation")
            .is_ok());
        assert!(ed
            .get("Phone.BoardFeed.Adverts[0].Listing.Truth.Paperwork.TrueMileage")
            .is_ok());

        
        ed.set_name_array(arr, &original[..1]).unwrap();
        assert_eq!(ed.get(arr).unwrap().children.len(), 1);

        
        ed.save().unwrap();
        let reloaded = Editor::load(&tmp).unwrap();
        let names: Vec<String> = reloaded
            .get(arr)
            .unwrap()
            .children
            .iter()
            .filter_map(|c| c.text.clone())
            .collect();
        assert_eq!(names, vec![original[0].clone()]);
        assert!(reloaded
            .get("Phone.BoardFeed.Adverts[0].Listing.Truth.Paperwork.TrueMileage")
            .is_ok());
        let _ = fs::remove_dir_all(&dir);
    }
}
