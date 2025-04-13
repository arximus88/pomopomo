<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Pomodoro from './Pomodoro.svelte';
  import Timer from './Timer.svelte';
  import Controls from './Controls.svelte';
  import ProgressIndicator from './ProgressIndicator.svelte';

  // Імпортуємо стори та функції керування
  import {
    remainingTime,
    timerStatus,
    currentMode,
    completedPomodoros,
    pomodorosPerCycle,
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
  <div class="content">
    <Pomodoro state={$currentMode} />
    
    <div class="timer-section">
      <Timer {minutes} {seconds} />
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
      onSettings={handleSettings} 
    />
    <ProgressIndicator completed={$completedPomodoros} total={$pomodorosPerCycle} />
  </div>
</div>

<style>
  .widget {
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    padding: 16px;
    width: 100%; /* Зробимо ширину 100% контейнера */
    box-sizing: border-box; /* Включаємо padding в ширину */
    user-select: none;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    cursor: grab; /* Показуємо, що можна тягнути */
  }

  .widget:active {
     cursor: grabbing;
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: 16px;
    align-items: center;
  }

  .timer-section {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 12px;
    align-items: center; /* Центруємо таймер */
  }

  .tabs {
    display: flex;
    justify-content: space-between;
    background: #f0f0f0;
    border-radius: 8px;
    padding: 4px;
    width: 100%;
  }

  .tabs button {
    flex: 1;
    border: none;
    background: none;
    padding: 8px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 600;
    color: #666;
    transition: all 0.2s ease;
  }

  .tabs button:hover {
    background: rgba(255, 255, 255, 0.5);
  }

  .tabs button.active {
    background: white;
    color: #333;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
</style>
