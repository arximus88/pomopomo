<!-- Головний компонент для відображення додатку -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Pomodoro from './Pomodoro.svelte';
	import Timer from './Timer.svelte';
	import Controls from './Controls.svelte';
	import PomodoroProgress from './PomodoroProgress.svelte';
	import SettingsMenu from './SettingsMenu.svelte';
	import { initTauriIntegration } from '$lib/tauri-integration';
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
		workDuration,
		breakDuration,
		relaxDuration,
		type TimerMode
	} from '$lib/stores/timer';

	// Стан для відображення меню налаштувань
	let showSettings = false;

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

	// Обробник для Settings
	function handleSettings() {
		console.log('Відкриваємо налаштування');
		showSettings = true;
	}

	// Обробник для закриття Settings
	function closeSettings() {
		console.log('Закриваємо налаштування');
		showSettings = false;
	}

	// Обробник для клавіатурних подій у меню налаштувань - ВИДАЛЕНО
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			closeSettings();
		}
	}

	// Обробник для кліку по вкладці
	function handleTabClick(mode: TimerMode) {
		setMode(mode);
	}

	// Функції для оновлення налаштувань тривалості - ВИДАЛЕНО

	// Для зберігання функції відписки від подій
	let unlistenTauriEvents: (() => void) | undefined;

	// Ініціалізуємо інтеграцію з Tauri під час монтування компонента
	onMount(() => {
		console.log('Widget mounted, initializing Tauri integration');
		try {
			unlistenTauriEvents = initTauriIntegration(handleSettings);
			console.log('Tauri integration initialized successfully');
		} catch (error) {
			console.error('Failed to initialize Tauri integration:', error);
		}

		// Функція для очищення при розмонтуванні
		return () => {
			if (unlistenTauriEvents) {
				unlistenTauriEvents();
			}
		};
	});
</script>

<div class="widget" data-tauri-drag-region role="button" aria-label="Pomodoro Widget" tabindex="0">
	<div class="top-section" data-tauri-drag-region>
		<div class="pomodoro-area" data-tauri-drag-region>
			<Pomodoro state={$currentMode} />
		</div>
		<div class="progress-timer-area" data-tauri-drag-region>
			<PomodoroProgress />
			<Timer 
				{minutes} 
				{seconds} 
				isPlaying={$timerStatus === 'running'} 
				mode={$currentMode} 
			/>
		</div>
	</div>

	<div class="bottom-container">
		<div class="bottom-section">
			<div class="border-clipper">
				<div class="tabs-wrapper" data-tauri-drag-region>
					<div class="tabs">
						<button 
							class:active={$currentMode === 'work'} 
							on:click={() => handleTabClick('work')}
							aria-pressed={$currentMode === 'work'}
							>РОБОТА</button
						>
						<button 
							class:active={$currentMode === 'break'} 
							on:click={() => handleTabClick('break')}
							aria-pressed={$currentMode === 'break'}
							>ПАУЗА</button
						>
						<button 
							class:active={$currentMode === 'relax'} 
							on:click={() => handleTabClick('relax')}
							aria-pressed={$currentMode === 'relax'}
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
						onSettings={handleSettings}
						mode={$currentMode}
					/>
				</div>
			</div>
		</div>
	</div>

	{#if showSettings}
		<SettingsMenu onClose={closeSettings} />
	{/if}
</div>

<style>
	.widget {
		width: 240px;
		height: 240px;
		position: relative;
		display: flex;
		flex-direction: column;
		background-color: transparent;
		border-radius: 16px;
		box-sizing: border-box;
		user-select: none;
		overflow: visible;
		outline: none; /* Прибираємо рамку фокусу */
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
	.border-clipper {
		position: relative;
		overflow: hidden;
		left: 0;
		bottom: 0;
		width: 100%;
		filter: drop-shadow(0px 0px 1px #d9903d);
		border-radius: 4px;
	}
	.bottom-container {
		position: relative;
		overflow: hidden;
		left: 0;
		bottom: 0;
		width: 100%;
		border-radius: 12px;
		background: #321f10;
		padding: 4px;
		box-sizing: border-box;
		z-index: 0;
		flex-shrink: 0;
	}

	.bottom-section {
		background: #f8dcb5;
		border-radius: 8px;
		padding: 4px;
		overflow: hidden;
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
		font-family: 'UbuntuMono', Courier, monospace;
		font-size: 14px;
		font-weight: 500;
		color: #45311f;
		transition: all 0.2s ease;
		text-align: center;
		text-transform: uppercase;
		outline: none; /* Прибираємо рамку фокусу */
	}

	.tabs button:hover {
		background: rgba(255, 255, 255, 0.5);
	}

	.tabs button.active {
		color: #fce9c9;
	}

	/* Різні кольори для активних вкладок */
	.tabs button.active:nth-child(1) {
		background: #ba4325;
		border: 1px solid #a32a0c;
	}

	.tabs button.active:nth-child(2) {
		background: #7F8A20;
		border: 1px solid #5C6417;
	}

	.tabs button.active:nth-child(3) {
		background: #20868A;
		border: 1px solid #19696C;
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

	/* Стилі для меню налаштувань - ВИДАЛЕНО */

	:global(*) {
		outline: none !important; /* Глобально прибираємо рамку для всіх елементів */
	}

	:global(*:focus) {
		outline: none !important;
	}
</style>
