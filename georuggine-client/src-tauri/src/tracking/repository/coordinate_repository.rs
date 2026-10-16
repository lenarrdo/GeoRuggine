use crate::tracking::domain::position::CoordinatePoint;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub struct CoordinateRepository;

impl CoordinateRepository {
    pub fn find_coordinates_file() -> Option<PathBuf> {
        let candidate_paths = [
            "coordinates.json",
            "positions.json",
            "../coordinates.json",
            "../positions.json",
            "coordinates.csv",
            "../coordinates.csv",
        ];

        for path in &candidate_paths {
            let p = Path::new(path);
            if p.exists() {
                println!("[Tracking] Trovato file coordinate: {}", p.display());
                return Some(p.to_path_buf());
            }
        }
        None
    }

    pub fn load_coordinates() -> Vec<(i64, i64)> {
        if let Some(path) = Self::find_coordinates_file() {
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(points) = serde_json::from_reader::<_, Vec<CoordinatePoint>>(reader) {
                        if !points.is_empty() {
                            println!("[Tracking] Caricate {} coordinate da JSON", points.len());
                            return points.into_iter().map(|p| (p.x, p.y)).collect();
                        }
                    }
                } else {
                    let mut points = Vec::new();
                    for line in reader.lines().flatten() {
                        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                        if parts.len() >= 2 {
                            if let (Ok(x), Ok(y)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                                points.push((x, y));
                            }
                        }
                    }
                    if !points.is_empty() {
                        println!("[Tracking] Caricate {} coordinate da CSV", points.len());
                        return points;
                    }
                }
            }
        }

        println!("[Tracking] File non trovato o non leggibile, impiego percorso predefinito");
        vec![
            (45010, 7680),
            (45015, 7685),
            (45020, 7690),
            (45025, 7695),
            (45030, 7700),
        ]
    }
}
