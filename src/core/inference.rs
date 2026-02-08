// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Maksim Zapevalov

use oxigraph::store::Store;
use oxigraph::sparql::QueryResults;
use serde::{Deserialize, Serialize};

use crate::core::fus::{FusLevel, Invariant, KNOWN_INVARIANTS};

#[derive(Serialize, Deserialize)]
pub struct Diagnosis {
    pub affected_level: FusLevel,
    pub violated_invariant: Invariant,
    pub severity: f32, // 0.0–1.0
}

/// Диагностика: какие инварианты нарушены событием?
pub fn diagnose_event(store: &Store, event_id: &str) -> Result<Diagnosis, String> {
    // В реальной версии: SPARQL-запрос к store
    // Здесь — упрощённая логика для демонстрации

    // Эвристика: если в событии есть "ban", "AI", "school" → нарушение ObserverPosition
    let mut max_severity = 0.0;
    let mut best_invariant = None;
    let mut best_level = FusLevel::Existential;

    for (id, label, level, desc) in KNOWN_INVARIANTS.iter() {
        let severity = estimate_severity(event_id, id);
        if severity > max_severity {
            max_severity = severity;
            best_invariant = Some(Invariant::new(id, label, *level, desc));
            best_level = *level;
        }
    }

    Ok(Diagnosis {
        affected_level: best_level,
        violated_invariant: best_invariant.unwrap_or_else(|| Invariant::new(
            "fus:Unknown", "Неизвестный инвариант", FusLevel::Ontological, ""
        )),
        severity: max_severity,
    })
}

fn estimate_severity(event_id: &str, invariant_id: &str) -> f32 {
    // Простейший матчинг по ключевым словам
    let lower = event_id.to_lowercase();
    match invariant_id {
        "fus:ObserverPosition" => {
            if lower.contains("ban") && (lower.contains("ai") || lower.contains("algorithm")) {
                0.92
            } else {
                0.0
            }
        }
        "fus:AgencyResponsibility" => {
            if lower.contains("manipulate") || lower.contains("control") {
                0.85
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

/// Реконструкция: предложить путь восстановления
pub fn reconstruct(diagnosis: &Diagnosis, temperament: &str) -> String {
    match diagnosis.violated_invariant.id.as_str() {
        "fus:ObserverPosition" => {
            if temperament == "choleric" {
                "Восстановить право на независимую оценку: открыть исходный код, обеспечить прозрачность."
            } else {
                "Обеспечить прозрачность алгоритма и право на объяснение решения."
            }
        }
        _ => "Требуется этическая реконструкция в рамках ФУС-рамок.",
    }
}