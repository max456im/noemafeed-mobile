// Загрузка WASM
import init, { analyze_event, generate_plain_summary, generate_digest } from '../pkg/noemafeed_mobile.js';

let currentEventId = 'demo-event-2026';
let currentMode = 'analytical'; // по умолчанию

async function initApp() {
  await init();

  // Имитация загрузки новости (в реальности — из RSS)
  renderNews(currentEventId);
}

function renderNews(eventId) {
  const analysis = analyze_event(eventId, currentMode);
  document.getElementById('news-title').innerText = analysis.title;
  document.getElementById('news-analysis').innerHTML = analysis.html; // или .text
}

// Режимы
document.getElementById('mode-constructive').onclick = () => { currentMode = 'constructive'; renderNews(currentEventId); };
document.getElementById('mode-analytical').onclick = () => { currentMode = 'analytical'; renderNews(currentEventId); };
document.getElementById('mode-critical').onclick = () => { currentMode = 'critical'; renderNews(currentEventId); };

// Копирование
document.getElementById('btn-copy').onclick = async () => {
  const text = generate_plain_summary(currentEventId, currentMode);
  await navigator.clipboard.writeText(text);
  alert('Анализ скопирован');
};

// Поделиться (онтологический дайджест)
document.getElementById('btn-share').onclick = async () => {
  const digest = generate_digest(currentEventId, currentMode);
  const url = `https://max456im.github.io/noemafeed-mobile?digest=${btoa(JSON.stringify(digest))}`;
  if (navigator.share) {
    try {
      await navigator.share({ title: 'NoemaFeed Анализ', url });
    } catch (e) {
      // fallback: копировать URL
      await navigator.clipboard.writeText(url);
      alert('Ссылка скопирована');
    }
  } else {
    await navigator.clipboard.writeText(url);
    alert('Ссылка на анализ скопирована');
  }
};

// Загрузка полного текста (из RSS или вручную)
async function loadFullText() {
  // В реальной версии: запрос к RSS или открытие поля ввода
  const full = "Полный текст новости будет загружен при первом нажатии...";
  document.getElementById('full-text').innerText = full;
  document.getElementById('full-text').classList.remove('hidden');
}

// Навигация (заглушки)
function loadPrev() { /* переключение события */ }
function loadNext() { /* переключение события */ }

// Фреймы (заглушки)
function openCauseFrame() { alert('Фрейм причин'); }
function openEffectFrame() { alert('Фрейм следствий'); }

// Инициализация
window.addEventListener('DOMContentLoaded', initApp);