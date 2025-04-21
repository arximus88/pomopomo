import { writable, derived, get } from 'svelte/store';

export type TimerMode = 'work' | 'break' | 'relax';
export type TimerStatus = 'idle' | 'running' | 'paused';

// Налаштування тривалості (в секундах)
export const workDuration = writable(25 * 60); // 25 хвилин
export const breakDuration = writable(5 * 60); // 5 хвилин
export const relaxDuration = writable(15 * 60); // 15 хвилин (довга пауза)

// Поточний режим таймера
export const currentMode = writable<TimerMode>('work');

// Поточний статус таймера
export const timerStatus = writable<TimerStatus>('idle');

// Стор для отримання тривалості поточного режиму
export const currentDuration = derived(
  [currentMode, workDuration, breakDuration, relaxDuration],
  ([$currentMode, $workDuration, $breakDuration, $relaxDuration]) => {
    switch ($currentMode) {
      case 'work': return $workDuration; // Робочий режим
      case 'break': return $breakDuration; // Пауза
      case 'relax': return $relaxDuration; // Довга пауза
      default: return $workDuration; // Якщо не відповідає жодному режиму, повертаємо робочий режим
    }
  }
);

// Час, що залишився (в секундах)
// Ініціалізуємо початковим значенням workDuration
export const remainingTime = writable(get(workDuration));

// Кількість завершених помідорів у поточному циклі
export const completedPomodoros = writable(0);

// Загальна кількість помідорів у циклі до довгої паузи
export const pomodorosPerCycle = writable(4);

// --- Логіка Таймера ---

let timerInterval: number | null = null;

// Функція для оновлення часу
function tick() {
  remainingTime.update(time => {
    if (time > 0) {
      return time - 1;
    } else {
      // Час вийшов, зупиняємо таймер та переходимо до наступного режиму
      stopInterval();
      timerStatus.set('idle');
      switchToNextMode();
      return 0; // Повертаємо 0, хоча це значення вже не буде використовуватись до наступного запуску
    }
  });
}

// Зупинка таймера
function stopInterval() {
  if (timerInterval !== null) {
    clearInterval(timerInterval);
    timerInterval = null;
  }
}

// Перехід до наступного режиму
function switchToNextMode() {
  const current = get(currentMode);
  const completed = get(completedPomodoros);
  const cycleLength = get(pomodorosPerCycle);

  if (current === 'work') {
    completedPomodoros.update(n => n + 1);
    const nextCompleted = completed + 1;
    if (nextCompleted % cycleLength === 0) {
      setMode('relax');
    } else {
      setMode('break');
    }
  } else { // Якщо поточний режим - break або relax
    setMode('work');
  }
  // Автоматично запускаємо наступний таймер? Поки що ні.
  // startTimer(); // Можна додати, якщо потрібен автозапуск
}

// --- Функції для керування таймером ---

// Запуск таймера
export function startTimer() {
  if (get(timerStatus) === 'running') return; // Вже запущено

  timerStatus.set('running');
  if (timerInterval === null) { // Запускаємо інтервал тільки якщо він ще не запущений
    timerInterval = setInterval(tick, 1000);
  }
}

// Пауза таймера
export function pauseTimer() {
  if (get(timerStatus) !== 'running') return; // Не запущено або вже на паузі

  timerStatus.set('paused');
  stopInterval(); // Зупиняємо інтервал
}

// Скидання таймера
export function resetTimer() {
  stopInterval(); // Зупиняємо інтервал
  timerStatus.set('idle');
  // Скидаємо час до тривалості поточного режиму
  remainingTime.set(get(currentDuration));
}

// Ця функція тепер використовується в tick
// export function skipTimer() {
//   stopInterval();
//   timerStatus.set('idle');
//   switchToNextMode();
// }

export function setMode(mode: TimerMode) {
  stopInterval(); // Зупиняємо таймер при ручній зміні режиму
  timerStatus.set('idle');
  currentMode.set(mode);
  remainingTime.set(get(currentDuration));
}

// Ініціалізація: встановлюємо початковий час при завантаженні
remainingTime.set(get(workDuration));

// --- Допоміжні функції ---

// Функція для отримання тривалості поточного режиму
export function getCurrentDuration(): number {
  const mode = get(currentMode);
  switch (mode) {
    case 'work': return get(workDuration);
    case 'break': return get(breakDuration);
    case 'relax': return get(relaxDuration);
    default: return get(workDuration);
  }
} 