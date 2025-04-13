<script lang="ts">
	import { onMount } from 'svelte';
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
						<button class:active={$currentMode === 'work'} on:click={() => handleTabClick('work')}
							>РОБОТА</button
						>
						<button class:active={$currentMode === 'break'} on:click={() => handleTabClick('break')}
							>ПАУЗА</button
						>
						<button class:active={$currentMode === 'relax'} on:click={() => handleTabClick('relax')}
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
					/>
				</div>
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
		flex-direction: column;
		background-color: transparent;
		border-radius: 16px;
		box-sizing: border-box;
		user-select: none;
		overflow: visible;
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
