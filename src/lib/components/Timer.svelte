<!-- Компонент для відображення таймера та підбадьорливих фраз -->
<script lang="ts">
	import { getRandomPhrase, type TimerMode } from '$lib/data/phrases';
	
	export let minutes = 24;
	export let seconds = 59;
	export let isPlaying = false;
	export let mode: TimerMode;

	let currentPhrase = '';

	$: {
		if (isPlaying) {
			currentPhrase = getRandomPhrase(mode);
		} else {
			currentPhrase = '';
		}
	}

	// Функція для форматування часу з ведучими нулями
	const formatTime = (time: number) => time.toString().padStart(2, '0');
</script>

<div class="timer-wrapper">
	<div class="timer-container">
		<div class="timer-display" class:work={mode === 'work'} class:break={mode === 'break'} class:relax={mode === 'relax'}>
			{formatTime(minutes)}:{formatTime(seconds)}
		</div>
		<div class="timer-label" class:visible={currentPhrase}>
			{currentPhrase}
		</div>
	</div>
	<div class="timer-post"></div>
</div>

<style>
	@font-face {
		font-family: 'UbuntuMono';
		src: url('/fonts/UbuntuMono-Bold.ttf') format('truetype');
		font-weight: bold;
		font-style: normal;
		font-display: swap;
	}

	.timer-wrapper {
		display: flex;
		flex-direction: column;
		align-items: center;
		position: relative;
	}

	.timer-container {
		position: relative;
		display: inline-flex;
		flex-direction: column;
		align-items: center;
		padding: 2px;
		border-radius: 10px;
		background: #321f10;
		z-index: 2;
		margin-bottom: -2px;
	}

	.timer-display {
		background-color: #fce9c9;
		font-family: 'UbuntuMono', Courier, monospace;
		font-size: 24px;
		font-weight: bold;
		padding: 4px 10px;
		border-radius: 8px;
		border: none;
		box-shadow: none;
		line-height: 1;
	}

	/* Кольори для різних режимів */
	.timer-display.work {
		color: #ba4325;
	}

	.timer-display.break {
		color: #7F8A20;
	}

	.timer-display.relax {
		color: #20868A;
	}

	.timer-label {
		font-family: 'UbuntuMono', Courier, monospace;
		font-size: 10px;
		font-weight: 500;
		color: #fff;
		white-space: nowrap;
		background-color: transparent;
		padding: 2px 0 0 0;
		position: static;
		transform: none;
		z-index: auto;
		opacity: 0;
		height: 12px;
		transition: opacity 0.3s ease;
	}

	.timer-label.visible {
		opacity: 1;
		height: 12px;
	}

	.timer-post {
		width: 8px;
		height: 40px;
		background-color: #321f10;
		border-radius: 0;
		z-index: 1;
		position: relative;
	}
</style>
