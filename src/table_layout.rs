





#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
}

#[derive(Clone, Copy, Debug)]
pub struct ColumnSpec {
    pub header: &'static str,
    
    pub base: f32,
    
    pub min: f32,
    
    pub max: f32,
    pub align: Align,
}

pub const ADVERT_COLUMNS: [ColumnSpec; 8] = [
    ColumnSpec {
        header: "PHOTO",
        base: 78.0,
        min: 68.0,
        max: 78.0,
        align: Align::Center,
    },
    ColumnSpec {
        header: "VEHICLE",
        base: 180.0,
        min: 128.0,
        max: 250.0,
        align: Align::Left,
    },
    ColumnSpec {
        header: "PRICE",
        base: 94.0,
        min: 76.0,
        max: 126.0,
        align: Align::Center,
    },
    ColumnSpec {
        header: "MILES",
        base: 96.0,
        min: 76.0,
        max: 126.0,
        align: Align::Center,
    },
    ColumnSpec {
        header: "SELLER",
        base: 150.0,
        min: 66.0,
        max: 230.0,
        align: Align::Left,
    },
    ColumnSpec {
        header: "BOARD",
        base: 104.0,
        min: 70.0,
        max: 130.0,
        align: Align::Center,
    },
    ColumnSpec {
        header: "SEED",
        base: 112.0,
        min: 74.0,
        max: 150.0,
        align: Align::Center,
    },
    ColumnSpec {
        header: "TITLE",
        base: 132.0,
        min: 128.0,
        max: 170.0,
        align: Align::Center,
    },
];

pub const HEADER_HEIGHT: f32 = 24.0;
pub const ROW_HEIGHT: f32 = 58.0;
pub const COLUMN_SPACING: f32 = 2.0;
pub const ROW_SPACING: f32 = 1.0;
pub const CONTROL_HEIGHT: f32 = 32.0;







pub fn widths(available: f32, spacing: f32) -> Vec<f32> {
    let n = ADVERT_COLUMNS.len();
    let spacing_total = spacing * (n.saturating_sub(1)) as f32;
    let usable = (available - spacing_total).max(0.0);

    let base_total: f32 = ADVERT_COLUMNS.iter().map(|c| c.base).sum();
    let min_total: f32 = ADVERT_COLUMNS.iter().map(|c| c.min).sum();
    let max_total: f32 = ADVERT_COLUMNS.iter().map(|c| c.max).sum();

    if usable >= base_total {
        
        
        let extra = (usable - base_total).min(max_total - base_total);
        let headroom: f32 = ADVERT_COLUMNS.iter().map(|c| c.max - c.base).sum();
        if headroom <= 0.0 {
            return ADVERT_COLUMNS.iter().map(|c| c.base).collect();
        }
        ADVERT_COLUMNS
            .iter()
            .map(|c| c.base + extra * (c.max - c.base) / headroom)
            .collect()
    } else if usable >= min_total {
        
        let t = if base_total > min_total {
            (usable - min_total) / (base_total - min_total)
        } else {
            1.0
        };
        ADVERT_COLUMNS
            .iter()
            .map(|c| c.min + (c.base - c.min) * t)
            .collect()
    } else {
        ADVERT_COLUMNS.iter().map(|c| c.min).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_are_the_required_eight_in_order() {
        let got: Vec<&str> = ADVERT_COLUMNS.iter().map(|c| c.header).collect();
        assert_eq!(
            got,
            ["PHOTO", "VEHICLE", "PRICE", "MILES", "SELLER", "BOARD", "SEED", "TITLE"]
        );
    }

    #[test]
    fn widths_are_stable_and_bounded() {
        for available in [0.0, 300.0, 600.0, 800.0, 1000.0, 1400.0, 2000.0] {
            let ws = widths(available, COLUMN_SPACING);
            assert_eq!(ws.len(), ADVERT_COLUMNS.len());
            for (w, c) in ws.iter().zip(ADVERT_COLUMNS.iter()) {
                assert!(*w >= c.min - 0.01, "width {} below min {}", w, c.min);
                assert!(*w <= c.max + 0.01, "width {} above max {}", w, c.max);
                assert!(w.is_finite());
            }
        }
    }

    #[test]
    fn wide_panels_use_the_available_width() {
        let available = 1200.0;
        let ws = widths(available, COLUMN_SPACING);
        let total: f32 =
            ws.iter().sum::<f32>() + COLUMN_SPACING * (ADVERT_COLUMNS.len() - 1) as f32;
        let max_total: f32 = ADVERT_COLUMNS.iter().map(|c| c.max).sum::<f32>()
            + COLUMN_SPACING * (ADVERT_COLUMNS.len() - 1) as f32;
        assert!((total - available).abs() < 0.5 || (total - max_total).abs() < 0.5);
    }

    #[test]
    fn narrow_panel_fits_without_horizontal_scroll() {
        for available in [700.0, 710.0, 720.0] {
            let ws = widths(available, COLUMN_SPACING);
            let total = ws.iter().sum::<f32>() + COLUMN_SPACING * (ADVERT_COLUMNS.len() - 1) as f32;
            assert!(
                total <= available + 0.01,
                "content {} exceeded available {}",
                total,
                available
            );
        }
    }

    #[test]
    fn same_widths_drive_header_and_rows() {
        
        
        let ws = widths(1000.0, COLUMN_SPACING);
        assert_eq!(ws.len(), ADVERT_COLUMNS.len());
        assert!(ws.iter().all(|w| *w > 0.0));
    }
}
