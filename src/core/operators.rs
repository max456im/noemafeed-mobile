// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Maksim Zapevalov

use crate::core::fus::FusLevel;

/// Вертикальный оператор согласования: передаёт устойчивость между уровнями
/// Пример: Физическая энергия ⊛ Биологический метаболизм
pub fn vertical_agreement(from: &FusLevel, to: &FusLevel) -> bool {
    // Устойчивость может передаваться только "вверх" по иерархии
    from.depth() < to.depth()
}

/// Горизонтальный оператор согласования: баланс внутри уровня
/// Пример: Агентность × Открытость → гибкое "Я"
pub fn horizontal_balance(level: &FusLevel, factor_a: f32, factor_b: f32) -> bool {
    // Простая эвристика: оба фактора должны быть в допустимом диапазоне
    match level {
        FusLevel::Psychic => {
            // Агентность (0.0–1.0) и Открытость (0.0–1.0) — оба > 0.3
            factor_a > 0.3 && factor_b > 0.3
        }
        _ => true, // Для других уровней — упрощённо
    }
}

/// Диагональная рефлексия: связь инварианта с нормативным слоем
pub fn manifests_in_structure(invariant: &str) -> Option<&'static str> {
    match invariant {
        "fus:ObserverPosition" => Some("gdpr:RightToExplanation"),
        "fus:FreedomInDeterminism" => Some("un:HumanRightsDeclaration_Art19"),
        _ => None,
    }
}