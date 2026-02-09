// Загрузка WASM-модуля
import init, { init_kb, analyze_event, get_plain_summary } from './pkg/noemafeed_mobile.js';

// Регистрация Service Worker (относительный путь!)
if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('./service-worker.js')
      .catch(err => console.warn('SW registration failed:', err));
  });
}

// Демо-новости (в реальности — из RSS или локального хранилища)
const DEMO_EVENTS = [
  {
    id: "event-1",
    title: "ЕС вводит мораторий на обучение ИИ на данных без согласия",
    content: "Европейская комиссия предложила запретить использование персональных данных для обучения ИИ без явного согласия...",
    source_url: "https://example.com/eu-ai-ban",
    published_at: "2026-02-05"
  },
  {
    id: "event-2",
    title: "Крупнейший банк внедрил алгоритмическое управление кредитами",
    content: "Алгоритм теперь решает не только о выдаче кредита, но и о социальной ценности заемщика...",
    source_url: "https://example.com/bank-ai",
    published_at: "2026-02-08"
  }
];

let currentEventIndex = 0;
let currentMode = 'analytical'; // 'constructive', 'analytical', 'critical'
let birthYear = 1993; // можно запросить у пользователя один раз

async function initApp() {
  try {
    await init();
    await init_kb(); // инициализация баз знаний
    renderCurrentNews();
  } catch (e) {
    console.error('Ошибка инициализации:', e);
    document.getElementById('news-analysis').innerText = 'Не удалось загрузить ядро анализа.';
  }
}

function getCurrentEvent() {
  return DEMO_EVENTS[currentEventIndex];
}

function renderCurrentNews() {
  const event = getCurrentEvent();
  document.getElementById('news-title').innerText = event.title;

  try {
    // Передаём событие как JSON-строку (как ожидает analyze_event)
    const inputJson = JSON.stringify(event);
    const resultJsValue = analyze_event(inputJson, currentMode, birthYear);
    const result = JSON.parse(resultJsValue.toString());

    document.getElementById('news-analysis').innerHTML = `
      <p><strong>Уровень ФУС:</strong> ${result.fus_level}</p>
      <p><strong>Нарушённый инвариант:</strong> ${result.affected_invariant}</p>
      <p><strong>Тип возмущения:</strong> ${result.perturbation_type}</p>
      <p><strong>Режим:</strong> ${result.mode}</p>
      <p><strong>Для темперамента:</strong> ${result.temperament_hint}</p>
      <hr>
      <pre>${result.summary}</pre>
    `;
  } catch (e) {
    console.error('Ошибка анализа:', e);
    document.getElementById('news-analysis').innerText = 'Ошибка при анализе события.';
  }
}

// === Навигация ===
function loadPrev() {
  if (currentEventIndex > 0) {
    currentEventIndex--;
    renderCurrentNews();
  }
}
function loadNext() {
  if (currentEventIndex < DEMO_EVENTS.length - 1) {
    currentEventIndex++;
    renderCurrentNews();
  }
}

// === Режимы ===
document.getElementById('mode-constructive')?.addEventListener('click', () => {
  currentMode = 'constructive';
  renderCurrentNews();
});
document.getElementById('mode-analytical')?.addEventListener('click', () => {
  currentMode = 'analytical';
  renderCurrentNews();
});
document.getElementById('mode-critical')?.addEventListener('click', () => {
  currentMode = 'critical';
  renderCurrentNews();
});

// === Копирование ===
document.getElementById('btn-copy')?.addEventListener('click', async () => {
  try {
    const event = getCurrentEvent();
    const inputJson = JSON.stringify(event);
    const resultJsValue = analyze_event(inputJson, currentMode, birthYear);
    const result = JSON.parse(resultJsValue.toString());
    
    const textToCopy = `[NoemaFeed • ${result.mode}]\n${result.summary}`;
    await navigator.clipboard.writeText(textToCopy);
    alert('Анализ скопирован');
  } catch (e) {
    console.error('Ошибка копирования:', e);
    alert('Не удалось скопировать анализ');
  }
});

// === Поделиться (пока через копирование дайджеста) ===
document.getElementById('btn-share')?.addEventListener('click', async () => {
  try {
    const event = getCurrentEvent();
    const inputJson = JSON.stringify(event);
    const resultJsValue = analyze_event(inputJson, currentMode, birthYear);
    const result = JSON.parse(resultJsValue.toString());
    
    const url = `https://max456im.github.io/noemafeed-mobile/?digest=${encodeURIComponent(result.digest)}`;
    if (navigator.share) {
      await navigator.share({ title: 'NoemaFeed Анализ', url });
    } else {
      await navigator.clipboard.writeText(url);
      alert('Ссылка на анализ скопирована');
    }
  } catch (e) {
    console.error('Ошибка генерации ссылки:', e);
    alert('Не удалось создать ссылку');
  }
});

// === Фреймы (заглушки) ===
document.querySelector('.cause-frame-btn')?.addEventListener('click', openCauseFrame);
document.querySelector('.effect-frame-btn')?.addEventListener('click', openEffectFrame);

function openCauseFrame() {
  alert('Фрейм причин: анализ структурных и исторических предпосылок.');
}
function openEffectFrame() {
  alert('Фрейм следствий: проекция системных последствий.');
}

// === Инициализация ===
window.addEventListener('DOMContentLoaded', initApp);