// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Maksim Zapevalov (max456im)
//
// Онтологический движок NoemaFeed Mobile
// Координирует: загрузку KB → диагностику ФУС → реконструкцию

use oxigraph::store::Store;
use oxigraph::io::{RdfFormat, RdfParser};
use std::io::Cursor;

/// Уровни ФУС (упрощённо)
#[derive(Debug, Clone)]
pub enum FusLevel {
    Ontological,
    Structural,
    Archetypal,
    Identity,
    Existential,
}

impl FusLevel {
    pub fn uri(&self) -> &'static str {
        match self {
            FusLevel::Ontological => "fus:OntologicalLevel",
            FusLevel::Structural => "fus:StructuralLevel",
            FusLevel::Archetypal => "fus:ArchetypalLevel",
            FusLevel::Identity => "fus:IdentityLevel",
            FusLevel::Existential => "fus:ExistentialLevel",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Ontological" => FusLevel::Ontological,
            "Structural" => FusLevel::Structural,
            "Archetypal" => FusLevel::Archetypal,
            "Identity" => FusLevel::Identity,
            _ => FusLevel::Existential,
        }
    }
}

/// Результат диагностики события
#[derive(Debug, Clone)]
pub struct Diagnosis {
    pub affected_level: FusLevel,
    pub violated_invariant: String, // URI инварианта, например: "fus:ObserverPosition"
}

/// Загружает все локальные базы знаний в единое хранилище
pub fn load_knowledge_base() -> Result<Store, Box<dyn std::error::Error>> {
    let store = Store::new()?;

    // === ФУС-ядро ===
    load_ttl_from_str(&store, include_str!("../kb/fus/core.ttl"))?;
    load_ttl_from_str(&store, include_str!("../kb/fus/structural.ttl"))?;
    load_ttl_from_str(&store, include_str!("../kb/fus/archetypal.ttl"))?;
    load_ttl_from_str(&store, include_str!("../kb/fus/identity.ttl"))?;
    load_ttl_from_str(&store, include_str!("../kb/fus/existential.ttl"))?;

    // === Профили субъектов ===
    load_ttl_from_str(&store, include_str!("../kb/subjects/subject-registry.ttl"))?;
    load_ttl_from_str(&store, include_str!("../kb/subjects/astro-profiles.ttl"))?;

    // === Динамические библиотеки ===
    load_ttl_from_str(&store, include_str!("../kb/dynamics/perturbations.ttl"))?;
    load_ttl_from_str(&store, include_str!("../kb/dynamics/ontogenesis.ttl"))?;
    load_ttl_from_str(&store, include_str!("../kb/dynamics/context-index.ttl"))?;

    Ok(store)
}

fn load_ttl_from_str(store: &Store, ttl: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parser = RdfParser::from_format(RdfFormat::Turtle);
    store.load_from_read(parser, Cursor::new(ttl.as_bytes()))?;
    Ok(())
}

/// Представление события для анализа
#[derive(Debug, Clone)]
pub struct NewsEvent {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub source: Option<String>,
}

/// Режим анализа
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisMode {
    Constructive,   // 🟢
    Analytical,     // 🔵
    Critical,       // 🔴
}

impl AnalysisMode {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "constructive" => Self::Constructive,
            "analytical" => Self::Analytical,
            _ => Self::Critical,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AnalysisMode::Constructive => "Конструктивный режим",
            AnalysisMode::Analytical => "Аналитический режим",
            AnalysisMode::Critical => "Критический режим",
        }
    }
}

/// Полный результат анализа
#[derive(Debug, Clone)]
pub struct OntoScene {
    pub event_id: String,
    pub mode: AnalysisMode,
    pub affected_level: FusLevel,
    pub violated_invariant: String,
    pub perturbation_type: String,
    pub temperament_hint: String,
    pub reconstruction: String,
    pub summary: String,
}

/// Диагностика события по заголовку (заглушка вместо SPARQL)
pub fn diagnose_event(_store: &Store, title: &str) -> Result<Diagnosis, Box<dyn std::error::Error>> {
    // Простейшая эвристика: если есть "наблюдатель", "позиция" → нарушение ObserverPosition
    let title_lower = title.to_lowercase();
    if title_lower.contains("наблюд") || title_lower.contains("observer") {
        Ok(Diagnosis {
            affected_level: FusLevel::Ontological,
            violated_invariant: "fus:ObserverPosition".to_string(),
        })
    } else if title_lower.contains("структура") || title_lower.contains("system") {
        Ok(Diagnosis {
            affected_level: FusLevel::Structural,
            violated_invariant: "fus:SystemIntegrity".to_string(),
        })
    } else {
        Ok(Diagnosis {
            affected_level: FusLevel::Existential,
            violated_invariant: "fus:FreedomInDeterminism".to_string(),
        })
    }
}

/// Генерация реконструкции (заглушка)
pub fn reconstruct(diagnosis: &Diagnosis, temperament: &str) -> String {
    format!(
        "Реконструкция для {} при нарушении {}: адаптировать нарратив под {} темперамент.",
        diagnosis.affected_level.uri(),
        diagnosis.violated_invariant,
        temperament
    )
}

/// Определяет темперамент по году рождения (китайский гороскоп → элемент → темперамент)
fn determine_temperament(year: u32) -> String {
    match year % 12 {
        5 | 6 => "choleric",     // Змея, Лошадь → Огонь
        3 | 4 => "sanguine",    // Кролик, Дракон → Дерево
        7 | 8 => "phlegmatic",  // Коза, Обезьяна → Земля
        _ => "melancholic",     // Остальные
    }.to_string()
}

/// Основная функция: построение онтосцены
pub fn build_onto_scene(
    store: &Store,
    event: &NewsEvent,
    mode: AnalysisMode,
    birth_year: u32,
) -> Result<OntoScene, Box<dyn std::error::Error>> {
    // 1. Диагностика ФУС
    let diagnosis = diagnose_event(store, &event.title)?;

    // 2. Определение темперамента
    let temperament = determine_temperament(birth_year);

    // 3. Сопоставление с типом возмущения (упрощённо)
    let pert_type = if event.title.to_lowercase().contains("ban") && event.title.to_lowercase().contains("ai") {
        "pert:RegulatoryPrecautionism".to_string()
    } else if event.title.to_lowercase().contains("manipulat") {
        "pert:AlgorithmicManipulation".to_string()
    } else {
        "pert:GenericDisturbance".to_string()
    };

    // 4. Реконструкция
    let reconstruction = reconstruct(&diagnosis, &temperament);

    // 5. Формирование вывода
    let level_label = diagnosis.affected_level.uri().split('#').last().unwrap_or("Unknown");
    let summary = format!(
        "[NoemaFeed • {}]\nНарушение: {} ({})\nТип возмущения: {}\nДля темперамента: {}",
        mode.label(),
        diagnosis.violated_invariant,
        level_label,
        pert_type,
        temperament
    );

    Ok(OntoScene {
        event_id: event.id.clone(),
        mode,
        affected_level: diagnosis.affected_level,
        violated_invariant: diagnosis.violated_invariant,
        perturbation_type: pert_type,
        temperament_hint: temperament,
        reconstruction,
        summary,
    })
}