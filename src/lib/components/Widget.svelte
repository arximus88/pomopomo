<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Pomodoro from './Pomodoro.svelte';
  import Timer from './Timer.svelte';
  import Controls from './Controls.svelte';
  // import ProgressIndicator from './ProgressIndicator.svelte'; // Прибираємо індикатор

  // Імпортуємо стори та функції керування
  import {
    remainingTime,
    timerStatus,
    currentMode,
    // completedPomodoros, // Прибираємо
    // pomodorosPerCycle, // Прибираємо
    startTimer,
    pauseTimer,
    resetTimer,
    setMode,
    type TimerMode
  } from '$lib/stores/timer';

  // Обчислюємо хвилини та секунди з remainingTime
  let minutes = 0;
  let seconds = 0;
  remainingTime.subscribe((time: number) => {
    minutes = Math.floor(time / 60);
    seconds = time % 60;
  });

  // Обробник для Play/Pause
  function handlePlayPause() {
    if ($timerStatus === 'running') {
      pauseTimer();
    } else {
      startTimer();
    }
  }

  // Обробник для Reset
  function handleReset() {
    resetTimer();
  }

  // Обробник для Settings (поки заглушка)
  function handleSettings() {
    console.log('Settings clicked');
  }

  // Обробник для кліку по вкладці
  function handleTabClick(mode: TimerMode) {
    setMode(mode);
  }

  // Логіка перетягування (залишається без змін)
  let startX: number;
  let startY: number;
  let isDragging = false;

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    startX = e.clientX;
    startY = e.clientY;
    const win = getCurrentWindow();
    if (win) {
      win.startDragging();
    }
  }

  // mousemove та mouseup тепер не потрібні для базового перетягування
  // function handleMouseMove(e: MouseEvent) {
  //   if (!isDragging) return;
  //   // Логіка може бути додана тут, якщо потрібно щось робити під час перетягування
  // }
  
  // function handleMouseUp() {
  //   isDragging = false;
  // }

  // onMount(() => {
  //   // Немає потреби додавати слухачі на document для базового перетягування
  //   return () => {};
  // });
</script>

<div 
  class="widget" 
  on:mousedown={handleMouseDown}
  role="button" 
  aria-label="Draggable Pomodoro Widget"
  tabindex="0"
>
  <Pomodoro state={$currentMode} />
  
  <div class="timer-section">
    <Timer {minutes} {seconds} />
  </div>
  
  <div class="tabs-container">
    <div class="tabs">
      <button class:active={$currentMode === 'work'} on:click={() => handleTabClick('work')}>РОБОТА</button>
      <button class:active={$currentMode === 'break'} on:click={() => handleTabClick('break')}>ПАУЗА</button>
      <button class:active={$currentMode === 'relax'} on:click={() => handleTabClick('relax')}>РЕЛАКС</button>
    </div>
  </div>

  <Controls 
    isPlaying={$timerStatus === 'running'} 
    onPlayPause={handlePlayPause} 
    onReset={handleReset} 
    />
  <!-- <ProgressIndicator completed={$completedPomodoros} total={$pomodorosPerCycle} /> --> <!-- Прибираємо індикатор -->
</div>

<style>
  .widget {
    background-color: #fbf3e9; /* Світлий фон віджета */
    backdrop-filter: none; /* Прибираємо блюр */
    border-radius: 16px; /* Більше закруглення */
    padding: 0; /* Забираємо внутрішні відступи, компоненти самі їх мають */
    width: 260px; /* Ширина згідно дизайну */
    box-sizing: border-box; 
    user-select: none;
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
    cursor: grab;
    overflow: hidden; /* Щоб внутрішні елементи не вилазили */
    display: flex; /* Використовуємо flex для основного контейнера */
    flex-direction: column;
    align-items: center; /* Центруємо все */
  }

  .widget:active {
     cursor: grabbing;
  }

  /* Прибираємо старий .content, його роль виконує сам .widget */

  .timer-section {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0; /* Забираємо відступ між таймером та вкладками */
    align-items: center; 
    margin-top: -10px; /* Нахлест таймера на помідор */
    position: relative;
    z-index: 2;
  }
  
  .tabs-container {
      width: calc(100% - 20px); /* Ширина мінус відступи */
      padding: 10px;
      background-color: #f3eade; /* Фон як у контролів */
      border-top-left-radius: 12px;
      border-top-right-radius: 12px;
      margin-top: -8px; /* Нахлест на таймер */
      position: relative;
      z-index: 1;
  }

  .tabs {
    display: flex;
    background: #e4c9af; /* Фон вкладок */
    border-radius: 8px;
    padding: 4px;
    width: 100%;
    box-sizing: border-box;
    gap: 4px; /* Відступ між кнопками */
  }

  .tabs button {
    flex: 1;
    border: none;
    background: none; 
    padding: 6px 0; /* Вертикальний відступ */
    border-radius: 6px;
    cursor: pointer;
    font-size: 11px;
    font-weight: 600;
    color: #8b5e3c; /* Коричневий текст */
    transition: all 0.2s ease;
    text-align: center;
  }

  .tabs button:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .tabs button.active {
    background: #d1a377; /* Активний колір */
    color: white;
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1);
  }
  
  /* Прибираємо стилі для .progress-indicator, якщо вони були */
</style>
