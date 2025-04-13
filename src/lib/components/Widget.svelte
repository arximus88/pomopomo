<script lang="ts">
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import Pomodoro from './Pomodoro.svelte';
	import Timer from './Timer.svelte';
	import Controls from './Controls.svelte';
	import PomodoroProgress from './PomodoroProgress.svelte';
	// import ProgressIndicator from './ProgressIndicator.svelte'; // Прибираємо індикатор

	// Імпортуємо стори та функції керування
	import {
		remainingTime,
		timerStatus,
		currentMode,
		completedPomodoros,
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
		// Перевіряємо, чи клік був не на кнопці чи інтерактивному елементі
		const targetElement = e.target as HTMLElement;
		if (targetElement.closest('button, a, input, [role="button"]:not(.widget)')) {
			return; // Не починаємо перетягування, якщо клік на кнопці
		}
		isDragging = true;
		startX = e.clientX;
		startY = e.clientY;
		getCurrentWindow()?.startDragging();
	}

	// Додаємо обробники для скидання isDragging, якщо потрібно
	function handleMouseUp() {
		if (isDragging) {
			isDragging = false;
		}
	}

	function handleMouseLeave() {
		if (isDragging) {
			// Можливо, зупинити перетягування тут, якщо користувач вийшов за межі вікна
			// getCurrentWindow()?.stopDragging(); // Цього методу немає, але логіка така
			isDragging = false;
		}
	}

	// onMount(() => {
	//   // Немає потреби додавати слухачі на document для базового перетягування
	//   return () => {};
	// });
</script>

<div
	class="widget"
	on:mousedown={handleMouseDown}
	on:mouseup={handleMouseUp}
	on:mouseleave={handleMouseLeave}
	role="button"
	aria-label="Pomodoro Widget"
	tabindex="0"
>
	<div class="top-section">
		<div class="pomodoro-area">
			<Pomodoro state={$currentMode} />
		</div>
		<div class="progress-timer-area">
			<PomodoroProgress />
			<Timer {minutes} {seconds} />
		</div>
	</div>

	<div class="bottom-container">
		<div class="bottom-section">
			<div class="tabs-wrapper">
				<div class="tabs">
					<button class:active={$currentMode === 'work'} on:click|stopPropagation={() => handleTabClick('work')}
						>РОБОТА</button
					>
					<button class:active={$currentMode === 'break'} on:click|stopPropagation={() => handleTabClick('break')}
						>ПАУЗА</button
					>
					<button class:active={$currentMode === 'relax'} on:click|stopPropagation={() => handleTabClick('relax')}
						>РЕЛАКС</button
					>
				</div>
			</div>
			<div class="controls-divider"></div>
			<div class="controls-wrapper">
				<Controls
					isPlaying={$timerStatus === 'running'}
					onPlayPause={handlePlayPause}
					onReset={handleReset}
				/>
			</div>
		</div>
	</div>
	<!-- <ProgressIndicator completed={$completedPomodoros} total={$pomodorosPerCycle} /> -->
	<!-- Прибираємо індикатор -->
</div>

<style>
	.widget {
		width: 240px;
		height: 240px;
		position: relative;
		display: flex;
    gap: 0;
		flex-direction: column;
		background-color: transparent;
		border-radius: 16px;
		box-sizing: border-box;
		user-select: none;
		cursor: grab;
		overflow: visible;
	}

	.widget:active {
		cursor: grabbing;
	}

	.top-section {
		display: flex;
		justify-content: space-between;
		align-items: flex-end;
		position: relative;
		z-index: 1;
	}

	.pomodoro-area {
		display: flex;
		align-items: flex-end;
		height: 100%;
	}

	.progress-timer-area {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		position: relative;
	}

	:global(.progress-timer-area .progress-container) {
		position: absolute;
		bottom: 55px;
		right: 0;
		padding-bottom: 0;
		z-index: 3;
	}

	:global(.progress-timer-area .timer-wrapper) {
		position: relative;
		z-index: 2;
	}

	.bottom-container {
		width: 100%;
		border-radius: 12px;
		background: #321f10;
		padding: 4px;
		box-sizing: border-box;
		z-index: 0;
	}

	.bottom-section {
		background: #f8dcb5;
		border-radius: 8px;
		overflow: hidden;
		filter: drop-shadow(0px 0px 1px #d9903d);
		display: flex;
		flex-direction: column;
	}

	.tabs-wrapper {
		padding: 8px;
		background: #efcfa5;
	}

	.tabs {
		display: flex;
		background: #fce9c9;
		border: 1px solid #deb98e;
		border-radius: 16px;
		padding: 0;
		height: 24px;
		align-items: center;
	}

	.tabs button {
		flex: 1;
		border: none;
		background: none;
		height: 100%;
		padding: 4px 10px;
		border-radius: 16px;
		cursor: pointer;
		font-size: 10px;
		font-weight: 500;
		color: #45311f;
		transition: all 0.2s ease;
		text-align: center;
		text-transform: uppercase;
	}

	.tabs button:hover {
		background: rgba(255, 255, 255, 0.5);
	}

	.tabs button.active {
		background: #ba4325;
		color: #fce9c9;
		border: 1px solid #a32a0c;
		box-shadow: none;
	}
	
	.controls-divider {
		height: 1px;
		background: #deb98e;
		flex-shrink: 0;
	}

	.controls-wrapper {
		padding: 12px 0;
		background: #f6dfbf;
		display: flex;
		justify-content: center;
		align-items: center;
		position: relative;
	}
</style>
