






























use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TitleBrand {
    Clean,
    Rebuilt,
    Salvage,
    Flood,
    LemonBuyback,
}

impl TitleBrand {
    
    pub const ALL: [TitleBrand; 5] = [
        TitleBrand::Clean,
        TitleBrand::Rebuilt,
        TitleBrand::Salvage,
        TitleBrand::Flood,
        TitleBrand::LemonBuyback,
    ];

    
    pub fn enum_name(self) -> &'static str {
        match self {
            TitleBrand::Clean => "ETitleBrand::Clean",
            TitleBrand::Rebuilt => "ETitleBrand::Rebuilt",
            TitleBrand::Salvage => "ETitleBrand::Salvage",
            TitleBrand::Flood => "ETitleBrand::Flood",
            TitleBrand::LemonBuyback => "ETitleBrand::LemonBuyback",
        }
    }

    
    pub fn status_name(self) -> &'static str {
        match self {
            TitleBrand::Clean => "Clean",
            TitleBrand::Rebuilt => "Rebuilt",
            TitleBrand::Salvage => "Salvage",
            TitleBrand::Flood => "Flood",
            TitleBrand::LemonBuyback => "Lemon buyback",
        }
    }

    
    pub fn label(self) -> &'static str {
        self.status_name()
    }

    pub fn from_enum_name(value: &str) -> Option<Self> {
        let leaf = value.rsplit("::").next().unwrap_or(value).trim();
        Self::ALL.into_iter().find(|t| {
            t.enum_name().eq_ignore_ascii_case(value)
                || t.status_name().eq_ignore_ascii_case(leaf)
                || format!("{:?}", t).eq_ignore_ascii_case(leaf)
        })
    }

    pub fn from_status(value: &str) -> Option<Self> {
        let leaf = value.trim().trim_matches('"');
        Self::ALL
            .into_iter()
            .find(|t| t.status_name().eq_ignore_ascii_case(leaf))
    }

    
    
    
    
    
    
    
    
    pub fn value_factor(self) -> f64 {
        match self {
            TitleBrand::Clean => 1.0,
            TitleBrand::Rebuilt => 0.70,
            TitleBrand::Salvage => 0.55,
            TitleBrand::Flood => 0.45,
            TitleBrand::LemonBuyback => 0.80,
        }
    }

    
    pub fn is_branded(self) -> bool {
        !matches!(self, TitleBrand::Clean)
    }
}

impl fmt::Display for TitleBrand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VehicleModel {
    pub category: &'static str,
    pub make: &'static str,
    pub model: &'static str,
    pub key: &'static str,
    
    
    pub clean_value: i64,
    
    
    pub year_min: i32,
    pub year_max: i32,
    pub miles_min: i64,
    pub miles_max: i64,
}

impl VehicleModel {
    pub fn label(&self) -> String {
        format!("{} {}", self.make, self.model)
    }
}

pub const VEHICLES: [VehicleModel; 16] = [
    VehicleModel {
        category: "Sedans",
        make: "Elaris",
        model: "Sedan",
        key: "elaris",
        clean_value: 4400,
        year_min: 2009,
        year_max: 2023,
        miles_min: 45_000,
        miles_max: 150_000,
    },
    VehicleModel {
        category: "Sedans",
        make: "Vantor",
        model: "Solace",
        key: "vantor",
        clean_value: 3700,
        year_min: 2009,
        year_max: 2023,
        miles_min: 45_000,
        miles_max: 150_000,
    },
    VehicleModel {
        category: "Sedans",
        make: "Rexter",
        model: "Sedan",
        key: "rexter",
        clean_value: 4000,
        year_min: 2009,
        year_max: 2023,
        miles_min: 45_000,
        miles_max: 150_000,
    },
    VehicleModel {
        category: "Sedans",
        make: "Volkra",
        model: "Sedan",
        key: "volkra",
        clean_value: 3900,
        year_min: 2009,
        year_max: 2023,
        miles_min: 45_000,
        miles_max: 150_000,
    },
    VehicleModel {
        category: "SUVs and trucks",
        make: "Prowler",
        model: "SUV",
        key: "prowler",
        clean_value: 5200,
        year_min: 2009,
        year_max: 2023,
        miles_min: 50_000,
        miles_max: 160_000,
    },
    VehicleModel {
        category: "SUVs and trucks",
        make: "Enforcer",
        model: "SUV",
        key: "enforcer",
        clean_value: 5600,
        year_min: 2009,
        year_max: 2023,
        miles_min: 50_000,
        miles_max: 160_000,
    },
    VehicleModel {
        category: "SUVs and trucks",
        make: "Dominator",
        model: "Pickup",
        key: "dominator",
        clean_value: 6100,
        year_min: 2009,
        year_max: 2023,
        miles_min: 50_000,
        miles_max: 170_000,
    },
    VehicleModel {
        category: "Supercars",
        make: "Apex",
        model: "Supercar",
        key: "apex",
        clean_value: 220_000,
        year_min: 2010,
        year_max: 2023,
        miles_min: 30_000,
        miles_max: 120_000,
    },
    VehicleModel {
        category: "Supercars",
        make: "Cavero",
        model: "Supercar",
        key: "cavero",
        clean_value: 120_000,
        year_min: 2010,
        year_max: 2023,
        miles_min: 30_000,
        miles_max: 120_000,
    },
    VehicleModel {
        category: "Supercars",
        make: "PhantomGT",
        model: "Supercar",
        key: "phantomgt",
        clean_value: 233_000,
        year_min: 2010,
        year_max: 2023,
        miles_min: 30_000,
        miles_max: 120_000,
    },
    VehicleModel {
        category: "Supercars",
        make: "Raptor",
        model: "Supercar",
        key: "raptor",
        clean_value: 112_500,
        year_min: 2010,
        year_max: 2023,
        miles_min: 30_000,
        miles_max: 120_000,
    },
    VehicleModel {
        category: "Coupes and muscle",
        make: "Eclipse",
        model: "Coupe",
        key: "eclipse",
        clean_value: 45_000,
        year_min: 2009,
        year_max: 2023,
        miles_min: 35_000,
        miles_max: 140_000,
    },
    VehicleModel {
        category: "Coupes and muscle",
        make: "Panther",
        model: "Muscle Car",
        key: "panther",
        clean_value: 35_000,
        year_min: 2009,
        year_max: 2023,
        miles_min: 35_000,
        miles_max: 140_000,
    },
    VehicleModel {
        category: "Coupes and muscle",
        make: "Stormline",
        model: "Coupe",
        key: "stormline",
        clean_value: 90_000,
        year_min: 2010,
        year_max: 2023,
        miles_min: 35_000,
        miles_max: 140_000,
    },
    VehicleModel {
        category: "Coupes and muscle",
        make: "Typhoon",
        model: "Coupe",
        key: "typhoon",
        clean_value: 70_000,
        year_min: 2009,
        year_max: 2023,
        miles_min: 35_000,
        miles_max: 140_000,
    },
    VehicleModel {
        category: "Coupes and muscle",
        make: "Zephyr",
        model: "Coupe",
        key: "zephyr",
        clean_value: 55_000,
        year_min: 2010,
        year_max: 2023,
        miles_min: 35_000,
        miles_max: 140_000,
    },
];

pub fn find(make: &str, model: &str) -> Option<&'static VehicleModel> {
    VEHICLES
        .iter()
        .find(|v| v.make.eq_ignore_ascii_case(make) && v.model.eq_ignore_ascii_case(model))
}

pub fn find_key(key: &str) -> Option<&'static VehicleModel> {
    VEHICLES.iter().find(|v| v.key.eq_ignore_ascii_case(key))
}

pub fn index_of(make: &str, model: &str) -> Option<usize> {
    VEHICLES
        .iter()
        .position(|v| v.make.eq_ignore_ascii_case(make) && v.model.eq_ignore_ascii_case(model))
}












#[derive(Clone, Debug)]
pub struct RandomStream {
    seed: u32,
}

impl RandomStream {
    pub fn new(seed: i32) -> Self {
        Self { seed: seed as u32 }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.seed = self
            .seed
            .wrapping_mul(196_314_165)
            .wrapping_add(907_633_515);
        self.seed
    }

    
    pub fn fraction(&mut self) -> f64 {
        self.next_u32() as f64 / (u32::MAX as f64 + 1.0)
    }

    pub fn range_i64(&mut self, min: i64, max: i64) -> i64 {
        if max <= min {
            return min;
        }
        min + (self.fraction() * (max - min + 1) as f64) as i64
    }

    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        self.range_i64(min as i64, max as i64) as i32
    }

    pub fn chance(&mut self, probability: f64) -> bool {
        self.fraction() < probability
    }

    pub fn pick<'a, T>(&mut self, values: &'a [T]) -> &'a T {
        &values[self.range_i64(0, values.len() as i64 - 1) as usize]
    }
}


pub fn random_seed() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x1234_5678_9ABC_DEF0);
    let mut x = nanos ^ 0x9E37_79B9_7F4A_7C15;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    let v = (x & 0x7FFF_FFFF) as i32;
    if v == 0 {
        1
    } else {
        v
    }
}










#[derive(Clone, Debug)]
pub struct PriceBasis {
    
    pub model_clean: i64,
    
    
    
    
    pub market: f64,
    
    pub saved_price: i64,
    pub saved_mileage: i64,
    pub saved_title: TitleBrand,
}

impl PriceBasis {
    pub fn from_saved(
        model: &VehicleModel,
        saved_price: i64,
        mileage: i64,
        title: TitleBrand,
    ) -> Self {
        let clean = clean_value_at(model, mileage);
        let denom = (clean * title.value_factor()).max(1.0);
        
        
        let market = if saved_price > 0 {
            
            
            
            (saved_price as f64 / denom).max(1e-9)
        } else {
            1.0
        };
        Self {
            model_clean: model.clean_value,
            market,
            saved_price,
            saved_mileage: mileage,
            saved_title: title,
        }
    }

    
    pub fn retarget(&self, model: &VehicleModel, mileage: i64, title: TitleBrand) -> Self {
        let mut next = self.clone();
        next.model_clean = model.clean_value;
        next.saved_mileage = mileage;
        next.saved_title = title;
        next.saved_price = next.price_for(model, mileage, title);
        next
    }

    pub fn price_for(&self, model: &VehicleModel, mileage: i64, title: TitleBrand) -> i64 {
        let clean = clean_value_at(model, mileage);
        let raw = clean * self.market * title.value_factor();
        round_price(raw.round() as i64)
    }
}














pub fn mileage_factor(miles: i64) -> f64 {
    let k = miles.max(0) as f64 / 1_000.0;
    let v = if k < 10.0 {
        (10.0 - k) * 0.007 + 1.15
    } else {
        (1.3 - k * 0.005).clamp(0.45, 1.15)
    };
    v / 1.22
}







pub fn clean_value_at(model: &VehicleModel, mileage: i64) -> f64 {
    let anchor = model.clean_value as f64;
    (anchor * mileage_factor(mileage)).max(anchor * 0.25)
}



pub fn round_price(value: i64) -> i64 {
    let v = value.max(0);
    ((v + 12) / 25) * 25
}





#[derive(Clone, Debug)]
pub struct GeneratedVehicle {
    pub year: i32,
    pub make: String,
    pub model: String,
    pub key: String,
    pub mileage: i64,
    pub asking_price: i64,
    pub title: TitleBrand,
    pub vin: String,
    pub price_basis: PriceBasis,
}










pub fn generate_vehicle(model: &VehicleModel, seed: i32) -> GeneratedVehicle {
    let mut rng = RandomStream::new(seed);
    let year = rng.range_i32(model.year_min, model.year_max);
    let raw_miles = rng.range_i64(model.miles_min, model.miles_max);
    let mileage = (raw_miles / 500) * 500;
    let title = roll_title(&mut rng);
    let vin = random_vin(&mut rng);
    
    
    let ask_multiplier = 0.85 + rng.fraction() * 0.25;
    let mut basis = PriceBasis::from_saved(model, 0, mileage, title);
    basis.market = ask_multiplier;
    let asking_price = basis.price_for(model, mileage, title);
    GeneratedVehicle {
        year,
        make: model.make.to_string(),
        model: model.model.to_string(),
        key: model.key.to_string(),
        mileage,
        asking_price,
        title,
        vin,
        price_basis: basis,
    }
}

fn roll_title(rng: &mut RandomStream) -> TitleBrand {
    
    
    
    
    if rng.chance(0.08) {
        let brands = [
            TitleBrand::Rebuilt,
            TitleBrand::Salvage,
            TitleBrand::Flood,
            TitleBrand::LemonBuyback,
        ];
        *rng.pick(&brands)
    } else {
        TitleBrand::Clean
    }
}








#[allow(clippy::too_many_arguments)]
pub fn rewrite_blurb(
    blurb: &str,
    old_make: &str,
    old_model: &str,
    new_make: &str,
    new_model: &str,
    old_price: i64,
    new_price: i64,
    old_miles: i64,
    new_miles: i64,
) -> Option<String> {
    if blurb.is_empty() || !blurb.is_ascii() {
        return None;
    }
    let mut out = blurb.to_string();

    if !old_make.eq_ignore_ascii_case(new_make) {
        out = replace_ci(&out, old_make, new_make);
    }
    if !old_model.eq_ignore_ascii_case(new_model) {
        out = replace_ci(&out, old_model, new_model);
    }

    if old_price != new_price && old_price > 0 {
        let old = format_number(old_price);
        let new = format_number(new_price);
        out = replace_ci(&out, &format!("${}", old), &format!("${}", new));
        out = replace_ci(&out, &old, &new);
    }

    if old_miles != new_miles && old_miles > 0 {
        let old = format_number(old_miles);
        let new = format_number(new_miles);
        out = replace_ci(&out, &old, &new);
        if old_miles % 1_000 == 0 && new_miles % 1_000 == 0 {
            out = replace_ci(
                &out,
                &format!("{}k", old_miles / 1_000),
                &format!("{}k", new_miles / 1_000),
            );
        }
    }

    if out == blurb || !out.is_ascii() {
        None
    } else {
        Some(out)
    }
}

fn replace_ci(haystack: &str, needle: &str, replacement: &str) -> String {
    if needle.is_empty() {
        return haystack.to_string();
    }
    let lower_hay = haystack.to_ascii_lowercase();
    let lower_needle = needle.to_ascii_lowercase();
    let mut out = String::with_capacity(haystack.len());
    let mut pos = 0usize;
    while let Some(rel) = lower_hay[pos..].find(&lower_needle) {
        let at = pos + rel;
        out.push_str(&haystack[pos..at]);
        out.push_str(replacement);
        pos = at + needle.len();
    }
    out.push_str(&haystack[pos..]);
    out
}

fn format_number(n: i64) -> String {
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

fn random_vin(rng: &mut RandomStream) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPRSTUVWXYZ0123456789";
    let mut vin = String::with_capacity(17);
    for _ in 0..17 {
        vin.push(ALPHABET[rng.range_i64(0, ALPHABET.len() as i64 - 1) as usize] as char);
    }
    vin
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_values_are_the_shipped_five() {
        let names: Vec<&str> = TitleBrand::ALL.iter().map(|t| t.status_name()).collect();
        assert_eq!(
            names,
            ["Clean", "Rebuilt", "Salvage", "Flood", "Lemon buyback"]
        );
        for t in TitleBrand::ALL {
            assert_eq!(TitleBrand::from_enum_name(t.enum_name()), Some(t));
            assert_eq!(TitleBrand::from_status(t.status_name()), Some(t));
        }
    }

    #[test]
    fn title_ladder_is_the_verified_one() {
        assert_eq!(TitleBrand::Clean.value_factor(), 1.0);
        assert_eq!(TitleBrand::Rebuilt.value_factor(), 0.70);
        assert_eq!(TitleBrand::Salvage.value_factor(), 0.55);
        assert_eq!(TitleBrand::Flood.value_factor(), 0.45);
        assert_eq!(TitleBrand::LemonBuyback.value_factor(), 0.80);
    }

    #[test]
    fn mileage_curve_matches_the_shipped_routine() {
        
        assert!((mileage_factor(0) - 1.0).abs() < 1e-12);
        
        assert!((mileage_factor(10_000) - 1.15 / 1.22).abs() < 1e-12);
        assert!((mileage_factor(170_000) - 0.45 / 1.22).abs() < 1e-12);
        assert!((mileage_factor(500_000) - 0.45 / 1.22).abs() < 1e-12);
        
        assert!(mileage_factor(10_000) > mileage_factor(60_000));
        assert!(mileage_factor(60_000) > mileage_factor(120_000));
    }

    #[test]
    fn model_catalogue_has_unique_keys_and_panther_body() {
        let mut keys: Vec<&str> = VEHICLES.iter().map(|v| v.key).collect();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), VEHICLES.len());
        assert_eq!(find("Panther", "Muscle Car").unwrap().key, "panther");
    }

    #[test]
    fn random_stream_is_deterministic() {
        let a: Vec<u32> = {
            let mut r = RandomStream::new(12345);
            (0..8).map(|_| r.next_u32()).collect()
        };
        let b: Vec<u32> = {
            let mut r = RandomStream::new(12345);
            (0..8).map(|_| r.next_u32()).collect()
        };
        assert_eq!(a, b);
    }

    #[test]
    fn price_basis_reproduces_the_saved_price_and_does_not_compound() {
        let model = find("Elaris", "Sedan").unwrap();
        let basis = PriceBasis::from_saved(model, 4900, 73_000, TitleBrand::Clean);
        assert_eq!(basis.price_for(model, 73_000, TitleBrand::Clean), 4900);

        
        let rebuilt = basis.price_for(model, 73_000, TitleBrand::Rebuilt);
        assert!(rebuilt < 4900);
        assert_eq!(basis.price_for(model, 73_000, TitleBrand::Clean), 4900);

        
        for _ in 0..10 {
            let _ = basis.price_for(model, 73_000, TitleBrand::Rebuilt);
        }
        assert_eq!(basis.price_for(model, 73_000, TitleBrand::Clean), 4900);
    }

    #[test]
    fn branded_title_basis_is_reversible() {
        let model = find("Elaris", "Sedan").unwrap();
        let basis = PriceBasis::from_saved(model, 3050, 70_000, TitleBrand::Rebuilt);
        assert_eq!(basis.price_for(model, 70_000, TitleBrand::Rebuilt), 3050);
        let clean = basis.price_for(model, 70_000, TitleBrand::Clean);
        assert!(clean > 3050);
        assert_eq!(basis.price_for(model, 70_000, TitleBrand::Rebuilt), 3050);
    }

    #[test]
    fn retarget_keeps_mileage_and_title_and_changes_anchor() {
        let old = find("Elaris", "Sedan").unwrap();
        let new = find("PhantomGT", "Supercar").unwrap();
        let basis = PriceBasis::from_saved(old, 4900, 73_000, TitleBrand::Clean);
        let retargeted = basis.retarget(new, 73_000, TitleBrand::Clean);
        let old_price = retargeted.price_for(old, 73_000, TitleBrand::Clean);
        let new_price = retargeted.price_for(new, 73_000, TitleBrand::Clean);
        assert!(new_price > old_price);
        assert_eq!(retargeted.saved_mileage, 73_000);
        assert_eq!(retargeted.saved_title, TitleBrand::Clean);
    }

    #[test]
    fn mileage_only_changes_price_when_the_basis_is_recomputed_from_original() {
        let model = find("Elaris", "Sedan").unwrap();
        let basis = PriceBasis::from_saved(model, 4900, 73_000, TitleBrand::Clean);
        let more = basis.price_for(model, 90_000, TitleBrand::Clean);
        let less = basis.price_for(model, 50_000, TitleBrand::Clean);
        assert!(more < 4900);
        assert!(less > 4900);
        assert_eq!(basis.price_for(model, 73_000, TitleBrand::Clean), 4900);
    }

    #[test]
    fn selected_model_is_reflected_in_generated_identity() {
        let model = find("PhantomGT", "Supercar").unwrap();
        let g = generate_vehicle(model, 540_112_158);
        assert_eq!(g.make, "PhantomGT");
        assert_eq!(g.model, "Supercar");
        assert_eq!(g.key, "phantomgt");
        assert_eq!(g.vin.len(), 17);
    }

    #[test]
    fn blurb_rewrite_keeps_the_seller_voice_and_updates_related_tokens() {
        let old = "Selling my 2016 Elaris Sedan, about 73k miles on it. Asking $4,900. There's some rust. Priced for that.";
        let out = rewrite_blurb(
            old,
            "Elaris",
            "Sedan",
            "PhantomGT",
            "Supercar",
            4_900,
            141_100,
            73_000,
            73_000,
        )
        .unwrap();
        assert!(out.contains("PhantomGT Supercar"));
        assert!(out.contains("$141,100"));
        assert!(out.contains("rust"));
        assert!(!out.contains("Elaris"));
        assert!(!out.contains("$4,900"));
    }

    #[test]
    fn blurb_rewrite_returns_none_when_nothing_changes() {
        let blurb = "Clean car, no surprises.";
        assert_eq!(
            rewrite_blurb(
                blurb, "Elaris", "Sedan", "Elaris", "Sedan", 4_900, 4_900, 73_000, 73_000
            ),
            None
        );
    }

    #[test]
    fn same_seed_same_generated_vehicle() {
        let model = find("Apex", "Supercar").unwrap();
        let a = generate_vehicle(model, 653_876_839);
        let b = generate_vehicle(model, 653_876_839);
        assert_eq!(a.year, b.year);
        assert_eq!(a.mileage, b.mileage);
        assert_eq!(a.title, b.title);
        assert_eq!(a.asking_price, b.asking_price);
        assert_eq!(a.vin, b.vin);
    }
}
