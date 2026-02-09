// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Maksim Zapevalov (max456im)
//
// NoemaFeed Mobile — Ontological News Companion
// Core engine: FUS diagnostics, perturbation analysis, reconstruction

use wasm_bindgen::prelude::*;
use oxigraph::store::Store;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

// === Типы данных ===

#[derive(Serialize, Deserialize)]
pub struct EventInput {
    pub title: String,
    pub content: Option<String>,
    pub source_url: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AnalysisResult {
    pub fus_level: String,
    pub affected_invariant: String,
    pub perturbation_type: String,
    pub mode: String,
    pub temperament_hint: String,
    pub summary: String,
    pub digest: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShareDigest {
    event_id: String,
    affected_invariant: String,
    fus_level: String,
    mode: String,
    temperament_hint: String,
}

// === Глобальное состояние ===
static mut STORE: Option<Store> = None;

// === Инициализация баз знаний ===
#[wasm_bindgen]
pub fn init_kb() -> Result<(), JsValue> {
    let store = Store::new().map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Загрузка TTL-файлов из kb/
    let files = [
        include_str!("../kb/fus/core.ttl"),
        include_str!("../kb/fus/structural.ttl"),
        include_str!("../kb/fus/archetypal.ttl"),
        include_str!("../kb/fus/identity.ttl"),
        include_str!("../kb/fus/existential.ttl"),
        include_str!("../kb/dynamics/perturbations.ttl"),
        include_str!("../kb/dynamics/context-index.ttl"),
        include_str!("../kb/subjects/astro-profiles.ttl"),
    ];

    for ttl in files {
        store
            .load_from_reader(oxigraph::io::RdfFormat::Turtle, ttl.as_bytes())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
    }

    unsafe {
        STORE = Some(store);
    }

    Ok(())
}

// === Основная функция анализа ===
#[wasm_bindgen]
pub fn analyze_event(input_json: &str, mode: &str, birth_year: u32) -> Result<JsValue, JsValue> {
    let event: EventInput = serde_json::from_str(input_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let store = unsafe { STORE.as_ref().ok_or(JsValue::from_str("KB not initialized"))? };

    // 1. Темперамент по году рождения
    let temperament = determine_temperament(birth_year);

    // 2. Диагностика ФУС (заглушка)
    let (fus_level, invariant) = diagnose_fus_level(&event)?;

    // 3. Тип возмущения
    let pert_type = match_perturbation_type(&event);

    // 4. Режим без лишних пробелов
    let clean_mode = mode.trim();
    let mode_label = match clean_mode {
        "constructive" => "Конструктивный режим",
        "analytical" => "Аналитический режим",
        "critical" => "Критический режим",
        _ => "Режим анализа",
    };

    // 5. Сводка
    let inv_label = if invariant == "fus:ObserverPosition" {
        "Позиция наблюдателя"
    } else {
        "Онтологический инвариант"
    };

    let summary = format!(
        "[NoemaFeed • {}]\nНарушение: {} ({})\nТип возмущения: {}\nДля темперамента: {}",
        mode_label, inv_label, fus_level, pert_type, temperament
    );

    // 6. Онтологический дайджест
    let digest_obj = ShareDigest {
        event_id: format!("event_{}", &event.title[..event.title.len().min(20)]),
        affected_invariant: invariant.clone(),
        fus_level: fus_level.clone(),
        mode: clean_mode.to_string(),
        temperament_hint: temperament.clone(),
    };

    let digest_json = serde_json::to_string(&digest_obj)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let digest_b64 = general_purpose::STANDARD.encode(digest_json);

    // 7. Формирование результата
    let result = AnalysisResult {
        fus_level,
        affected_invariant: invariant,
        perturbation_type: pert_type,
        mode: clean_mode.to_string(),
        temperament_hint: temperament,
        summary,
        digest: digest_b64,
    };

    let json_output = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(JsValue::from_str(&json_output))
}

// === Вспомогательные функции ===

fn determine_temperament(year: u32) -> String {
    match year % 12 {
        5 | 6 => "choleric",   // Змея, Лошадь → Огонь
        3 | 4 => "sanguine",  // Кролик, Дракон → Дерево
        7 | 8 => "phlegmatic", // Коза, Обезьяна → Земля
        _ => "melancholic",
    }.to_string()
}

fn diagnose_fus_level(_event: &EventInput) -> Result<(String, String), JsValue> {
    Ok(("OntologicalLevel".to_string(), "fus:ObserverPosition".to_string()))
}

fn match_perturbation_type(event: &EventInput) -> String {
    let title = event.title.to_lowercase();
    if title.contains("ban") && title.contains("ai") {
        "pert:RegulatoryPrecautionism".to_string()
    } else if title.contains("manipulat") {
        "pert:AlgorithmicManipulation".to_string()
    } else {
        "pert:GenericDisturbance".to_string()
    }
}

// === Экспорт: восстановление сводки из дайджеста ===
#[wasm_bindgen]
pub fn get_plain_summary(digest_b64: &str) -> Result<String, JsValue> {
    let json_bytes = general_purpose::STANDARD
        .decode(digest_b64)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let digest: ShareDigest = serde_json::from_slice(&json_bytes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let inv_label = if digest.affected_invariant == "fus:ObserverPosition" {
        "Позиция наблюдателя"
    } else {
        "Онтологический инвариант"
    };

    let mode_label = match digest.mode.as_str() {
        "constructive" => "Конструктивный режим",
        "analytical" => "Аналитический режим",
        "critical" => "Критический режим",
        _ => "Режим анализа",
    };

    Ok(format!(
        "[NoemaFeed • {}]\nУровень: {}\nИнвариант: {}\nТемперамент: {}",
        mode_label, digest.fus_level, inv_label, digest.temperament_hint
    ))
}