// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Maksim Zapevalov

/// Онтологические уровни устойчивого существования
#[derive(Debug, Clone, PartialEq)]
pub enum FusLevel {
    Ontological,    // Различение, Отношение, Позиция наблюдателя
    Physical,       // Энергия, Симметрия, Иерархия
    Biological,     // Целостность, Адаптация, Наследственность
    Psychic,        // Агентность, Нарратив, Этический каркас
    Existential,    // Конечность, Диалог с небытием, Свобода-в-детерминизме
}

impl FusLevel {
    pub fn uri(&self) -> &'static str {
        match self {
            FusLevel::Ontological => "http://onto16.org/fus#OntologicalLevel",
            FusLevel::Physical => "http://onto16.org/fus#PhysicalLevel",
            FusLevel::Biological => "http://onto16.org/fus#BiologicalLevel",
            FusLevel::Psychic => "http://onto16.org/fus#PsychicLevel",
            FusLevel::Existential => "http://onto16.org/fus#ExistentialLevel",
        }
    }

    /// Глубина уровня: чем меньше число — тем глубже
    pub fn depth(&self) -> u8 {
        match self {
            FusLevel::Ontological => 0,
            FusLevel::Physical => 1,
            FusLevel::Biological => 2,
            FusLevel::Psychic => 3,
            FusLevel::Existential => 4,
        }
    }
}

/// Базовые инварианты ФУС
#[derive(Debug, Clone)]
pub struct Invariant {
    pub id: String,
    pub label: String,
    pub level: FusLevel,
    pub description: String,
}

impl Invariant {
    pub fn new(id: &str, label: &str, level: FusLevel, desc: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            level,
            description: desc.to_string(),
        }
    }
}

/// Предопределённые инварианты (загружаются из kb/fus/*.ttl)
pub const KNOWN_INVARIANTS: &[(&str, &str, FusLevel, &str)] = &[
    ("fus:Distinction", "Различение", FusLevel::Ontological, "Способность различать 'бытие' и 'небытие'"),
    ("fus:ObserverPosition", "Позиция наблюдателя", FusLevel::Ontological, "Независимость субъекта от объекта наблюдения"),
    ("fus:EnergyHierarchy", "Энергетическая иерархия", FusLevel::Physical, "Энергия распределяется иерархически, а не хаотично"),
    ("fus:Integrity", "Целостность", FusLevel::Biological, "Живая система сохраняет границу 'внутри/снаружи'"),
    ("fus:AgencyResponsibility", "Агентность с ответственностью", FusLevel::Psychic, "Действие невозможно без этической ответственности"),
    ("fus:FreedomInDeterminism", "Свобода-в-детерминизме", FusLevel::Existential, "Свобода возможна только внутри детерминированных рамок"),
];