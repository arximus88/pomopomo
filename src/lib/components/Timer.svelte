<script lang="ts">
	export let minutes = 24;
	export let seconds = 59;

	// Функція для форматування часу з ведучими нулями
	const formatTime = (time: number) => time.toString().padStart(2, '0');
</script>

<div class="timer-wrapper">
	<div class="timer-container">
		<div class="timer-display">
			{formatTime(minutes)}:{formatTime(seconds)}
		</div>
		<div class="timer-label">Вйо працювати!</div>
	</div>
	<div class="timer-post"></div>
</div>

<style>
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
		color: #ba4325;
		font-family: 'Courier New', Courier, monospace;
		font-size: 24px;
		font-weight: bold;
		padding: 4px 10px;
		border-radius: 8px;
		border: none;
		box-shadow: none;
		line-height: 1;
	}

	.timer-label {
		font-size: 10px;
		font-weight: 700;
		color: #230f09;
		white-space: nowrap;
		background-color: transparent;
		padding: 2px 0 0 0;
		position: static;
		transform: none;
		z-index: auto;
	}

	.timer-post {
		width: 10px;
		height: 55px;
		background-color: #321f10;
		border-radius: 0;
		z-index: 1;
		position: relative;
	}
</style>
