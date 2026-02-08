// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Maksim Zapevalov (max456im)
//
// NoemaFeed Mobile — Ontological News Companion
// Core engine: FUS diagnostics, perturbation analysis, reconstruction

use wasm_bindgen::prelude::*;
use oxigraph::store::Store;
use serde::{Deserialize, Serialize};

// === Типы данных ===

#[derive(Serialize, Deserialize)]
pub struct EventInput {
    pub title: String,
    pub content: Option<String>, // может быть пустым при первом открытии
    pub source_url: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AnalysisResult {
    pub fus_level: String,           // "OntologicalLevel", "PsychicLevel", ...
    pub affected_invariant: String,  // "fus:ObserverPosition"
    pub perturbation_type: String,   // "pert:RegulatoryPrecautionism"
    pub mode: String,                // "constructive", "analytical", "critical"
    pub temperament_hint: String,    // "choleric", "sanguine", ...
    pub summary: String,             // человекочитаемый вывод
    pub digest: String,              // онтологический дайджест (Base64 JSON)
}

#[derive(Serialize, Deserialize)]
pub struct ShareDigest {
    event_id: String,
    affected_invariant: String,
    fus_level: String,
    mode: String,
    temperament_hint: String,
}

// === Глобальное состояние (в реальном проекте — лучше через RefCell или async-каналы) ===

static mut STORE: Option<Store> = None;

// === Инициализация ===

#[wasm_bindgen]
pub fn init_kb() -> Result<(), JsValue> {
    // Загрузка всех TTL-файлов из kb/ в oxigraph Store
    let store = Store::new().map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Здесь — имитация загрузки из статических ресурсов.
    // В реальной сборке: include_str!("../kb/fus/core.ttl") и т.д.
    // Для упрощения — предположим, что все данные уже встроены как строки.

    // Пример: парсинг core.ttl
    let core_ttl = include_str!("../kb/fus/core.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, core_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let structural_ttl = include_str!("../kb/fus/structural.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, structural_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let archetypal_ttl = include_str!("../kb/fus/archetypal.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, archetypal_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let identity_ttl = include_str!("../kb/fus/identity.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, identity_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let existential_ttl = include_str!("../kb/fus/existential.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, existential_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Динамика
    let perturbations_ttl = include_str!("../kb/dynamics/perturbations.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, perturbations_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let context_index_ttl = include_str!("../kb/dynamics/context-index.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, context_index_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Профили
    let astro_ttl = include_str!("../kb/subjects/astro-profiles.ttl");
    store
        .load_from_read(oxigraph::io::RdfFormat::Turtle, astro_ttl.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    unsafe {
        STORE = Some(store);
    }

    Ok(())
}

// === Основная функция анализа ===

#[wasm_bindgen]
pub fn analyze_event(input: &JsValue, mode: &str, birth_year: u32) -> Result<JsValue, JsValue> {
    let event: EventInput = input.into_serde().map_err(|e| JsValue::from_str(&e.to_string()))?;

    let store = unsafe { STORE.as_ref().ok_or(JsValue::from_str("KB not initialized"))? };

    // 1. Определение темперамента по году рождения (китайский гороскоп)
    let temperament = determine_temperament(birth_year);

    // 2. Диагностика ФУС (упрощённая логика)
    let (fus_level, invariant) = diagnose_fus_level(&event, store)?;

    // 3. Сопоставление с типом возмущения
    let pert_type = match_furcation_type(&event, store)?;

    // 4. Генерация человекочитаемого вывода
    let summary = generate_summary(&fus_level, &invariant, &pert_type, mode, &temperament);

    // 5. Онтологический дайджест для обмена
    let digest_obj = ShareDigest {
        event_id: format!("event_{}", event.title.get(..20).unwrap_or(&event.title)),
        affected_invariant: invariant.clone(),
        fus_level: fus_level.clone(),
        mode: mode.to_string(),
        temperament_hint: temperament.clone(),
    };
    let digest_json = serde_json::to_string(&digest_obj).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let digest_b64 = base64::encode(digest_json);

    let result = AnalysisResult {
        fus_level,
        affected_invariant: invariant,
        perturbation_type: pert_type,
        mode: mode.to_string(),
        temperament_hint: temperament,
        summary,
        digest: digest_b64,
    };

    Ok(JsValue::from_serde(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
}

// === Вспомогательные функции ===

fn determine_temperament(year: u32) -> String {
    // Упрощённо: холерики — годы Огня (змея, лошадь и др.)
    // В реальности — полный расчёт по 5 элементам и 12 животным
    match year % 12 {
        5 | 6 => "choleric".to_string(),   // Змея, Лошадь → Огонь
        3 | 4 => "sanguine".to_string(),   // Кролик, Дракон → Дерево
        7 | 8 => "phlegmatic".to_string(), // Коза, Обезьяна → Земля
        _ => "melancholic".to_string(),    // Остальные
    }
}

fn diagnose_fus_level(_event: &EventInput, _store: &Store) -> Result<(String, String), JsValue> {
    // В реальной версии — SPARQL-подобный запрос к store
    // Здесь — заглушка для демонстрации
    Ok((
        "OntologicalLevel".to_string(),
        "fus:ObserverPosition".to_string(),
    ))
}

fn match_furcation_type(_event: &EventInput, _store: &Store) -> Result<String, JsValue> {
    // Пример: если в заголовке есть "ban" и "AI" → RegulatoryPrecautionism
    Ok("pert:RegulatoryPrecautionism".to_string())
}

fn generate_summary(
    level: &str,
    invariant: &str,
    pert: &str,
    mode: &str,
    temper: &str,
) -> String {
    let inv_label = match invariant {
        "fus:ObserverPosition" => "Позиция наблюдателя",
        _ => "Онтологический инвариант",
    };

    let mode_label = match mode {
        "constructive" => "Конструктивный режим",
        "analytical" => "Аналитический режим",
        "critical" => "Критический режим",
        _ => "Режим анализа",
    };

    format!(
        "[NoemaFeed • {}]\nНарушение: {} ({})\nТип возмущения: {}\nДля темперамента: {}",
        mode_label, inv_label, level, pert, temper
    )
}

// === Экспорт функций для копирования и обмена ===

#[wasm_bindgen]
pub fn get_plain_summary(digest_b64: &str) -> Result<String, JsValue> {
    let json = base64::decode(digest_b64).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let digest: ShareDigest = serde_json::from_slice(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let inv_label = match digest.affected_invariant.as_str() {
        "fus:ObserverPosition" => "Позиция наблюдателя",
        _ => "Онтологический инвариант",
    };

    Ok(format!(
        "[NoemaFeed • {}]\nУровень: {}\nИнвариант: {}\nТемперамент: {}",
        digest.mode, digest.fus_level, inv_label, digest.temperament_hint
    ))
}