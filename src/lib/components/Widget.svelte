<!-- Головний компонент для відображення додатку -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Pomodoro from './Pomodoro.svelte';
	import Timer from './Timer.svelte';
	import Controls from './Controls.svelte';
	import PomodoroProgress from './PomodoroProgress.svelte';
	import SettingsMenu from './SettingsMenu.svelte';
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

	// Обробник для клавіатурних подій у меню налаштувань
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			closeSettings();
		}
	}

	// Обробник для кліку по вкладці
	function handleTabClick(mode: TimerMode) {
		setMode(mode);
	}

	// Функції для оновлення налаштувань тривалості
	function updateWorkDuration(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input) {
			const value = parseInt(input.value);
			if (!isNaN(value) && value > 0) {
				workDuration.set(value * 60);
			}
		}
	}
	
	function updateBreakDuration(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input) {
			const value = parseInt(input.value);
			if (!isNaN(value) && value > 0) {
				breakDuration.set(value * 60);
			}
		}
	}
	
	function updateRelaxDuration(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input) {
			const value = parseInt(input.value);
			if (!isNaN(value) && value > 0) {
				relaxDuration.set(value * 60);
			}
		}
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
		<div 
			class="settings-backdrop" 
			on:click={closeSettings}
			on:keydown={handleKeydown}
			role="dialog"
			aria-modal="true"
			aria-labelledby="settings-title"
		>
			<div 
				class="settings-menu" 
				on:click|stopPropagation
				role="document"
				tabindex="-1"
			>
				<div class="settings-header">
					<h2 id="settings-title">Налаштування</h2>
					<button 
						class="close-button" 
						on:click={closeSettings}
						type="button"
						aria-label="Закрити налаштування"
					>×</button>
				</div>
				
				<div class="settings-content">
					<div class="settings-section">
						<h3 id="duration-title">Тривалість (хвилини)</h3>
						<div class="settings-inputs" aria-labelledby="duration-title">
							<div class="input-group">
								<label for="work-duration">Робота</label>
								<input 
									type="number" 
									id="work-duration" 
									min="1" 
									max="60" 
									value={Math.floor($workDuration / 60)}
									on:change={updateWorkDuration}
									aria-label="Тривалість роботи у хвилинах"
								/>
							</div>
							
							<div class="input-group">
								<label for="break-duration">Пауза</label>
								<input 
									type="number" 
									id="break-duration" 
									min="1" 
									max="30" 
									value={Math.floor($breakDuration / 60)}
									on:change={updateBreakDuration}
									aria-label="Тривалість паузи у хвилинах"
								/>
							</div>
							
							<div class="input-group">
								<label for="relax-duration">Релакс</label>
								<input 
									type="number" 
									id="relax-duration" 
									min="1" 
									max="60" 
									value={Math.floor($relaxDuration / 60)}
									on:change={updateRelaxDuration}
									aria-label="Тривалість релаксації у хвилинах"
								/>
							</div>
						</div>
					</div>
					
					<div class="settings-section">
						<h3 id="about-title">Про додаток</h3>
						<div aria-labelledby="about-title">
							<p>Pomopomo - це простий таймер для техніки Pomodoro.</p>
							<p>Версія: 1.0.0</p>
							<p>Автор: Борис Брага</p>
						</div>
					</div>
				</div>
			</div>
		</div>
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

	/* Стилі для меню налаштувань */
	.settings-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 1000;
	}
	
	.settings-menu {
		background-color: #fce9c9;
		border-radius: 12px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
		width: 300px;
		max-width: 90%;
		max-height: 90vh;
		overflow-y: auto;
		border: 2px solid #deb98e;
	}
	
	.settings-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 16px;
		border-bottom: 1px solid #deb98e;
		background-color: #efcfa5;
		border-radius: 10px 10px 0 0;
	}
	
	.settings-header h2 {
		margin: 0;
		font-size: 18px;
		color: #321f10;
		font-family: 'UbuntuMono', Courier, monospace;
	}
	
	.close-button {
		background: none;
		border: none;
		font-size: 24px;
		color: #321f10;
		cursor: pointer;
		padding: 0;
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		transition: background-color 0.2s;
	}
	
	.close-button:hover {
		background-color: rgba(0, 0, 0, 0.1);
	}
	
	.settings-content {
		padding: 16px;
	}
	
	.settings-section {
		margin-bottom: 20px;
	}
	
	.settings-section h3 {
		font-size: 16px;
		margin: 0 0 12px 0;
		color: #321f10;
		font-family: 'UbuntuMono', Courier, monospace;
	}
	
	.settings-section p {
		margin: 8px 0;
		color: #454545;
		font-size: 14px;
		font-family: 'UbuntuMono', Courier, monospace;
	}
	
	.settings-inputs {
		display: grid;
		grid-template-columns: 1fr;
		gap: 12px;
	}
	
	.input-group {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	
	.input-group label {
		font-family: 'UbuntuMono', Courier, monospace;
		font-size: 14px;
		color: #321f10;
	}
	
	.input-group input {
		width: 60px;
		padding: 6px;
		border: 1px solid #deb98e;
		border-radius: 6px;
		background-color: #fff;
		font-family: 'UbuntuMono', Courier, monospace;
		text-align: center;
	}

	:global(*) {
		outline: none !important; /* Глобально прибираємо рамку для всіх елементів */
	}

	:global(*:focus) {
		outline: none !important;
	}
</style>
