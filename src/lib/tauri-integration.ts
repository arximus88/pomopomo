// Інтеграція з Tauri для обробки подій з системного трея
import { startTimer, pauseTimer, timerStatus } from '$lib/stores/timer';
import { get } from 'svelte/store';

// Імена подій, які ми будемо слухати
const OPEN_SETTINGS_EVENT = 'pomopomo://open-settings';
const TOGGLE_PLAY_PAUSE_EVENT = 'pomopomo://toggle-play-pause';

// Ініціалізуємо слухачів подій 
export function initTauriIntegration(openSettingsCallback: () => void) {
  try {
    console.log('[Tauri Integration] Initializing event listeners');
    
    // Обробник для відкриття налаштувань
    const handleOpenSettings = () => {
      console.log('[Tauri Integration] Received open-settings event');
      openSettingsCallback();
    };
    
    // Обробник для переключення відтворення/паузи
    const handleTogglePlayPause = () => {
      console.log('[Tauri Integration] Received toggle-play-pause event');
      const status = get(timerStatus);
      if (status === 'running') {
        pauseTimer();
      } else {
        startTimer();
      }
    };
    
    // Додаємо слухачів подій
    window.addEventListener(OPEN_SETTINGS_EVENT, handleOpenSettings);
    window.addEventListener(TOGGLE_PLAY_PAUSE_EVENT, handleTogglePlayPause);
    
    console.log('[Tauri Integration] Event listeners initialized successfully');
    
    // Функція для видалення слухачів при потребі
    return () => {
      window.removeEventListener(OPEN_SETTINGS_EVENT, handleOpenSettings);
      window.removeEventListener(TOGGLE_PLAY_PAUSE_EVENT, handleTogglePlayPause);
      console.log('[Tauri Integration] Event listeners removed');
    };
  } catch (error) {
    console.error('[Tauri Integration] Failed to initialize event listeners:', error);
    return () => {}; // Повертаємо пусту функцію як fallback
  }
} 