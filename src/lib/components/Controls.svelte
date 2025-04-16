<!-- Компонент для керування таймером -->
<script lang="ts">
	import type { TimerMode } from '$lib/stores/timer'; 
	
	export let isPlaying = false; // Додано нову властивість для кнопки паузи
	export let onPlayPause: () => void = () => {}; // Додано нову подію для кнопки паузи
	export let onReset: () => void = () => {}; // Додано нову подію для кнопки скидання
	export let onSettings: () => void = () => {}; // Додано нову подію для кнопки налаштувань
	export let mode: TimerMode = 'work'; // Додано нову властивість для режиму таймеру
</script>

<div class="controls">
	<div class="main-controls">
		<!-- Play/Pause Button -->
		<button
			class="control-button play-pause"
			class:work={mode === 'work'}
			class:break={mode === 'break'}
			class:relax={mode === 'relax'}
			on:click={onPlayPause}
			aria-label={isPlaying ? 'Pause' : 'Play'}
		>
			{#if isPlaying}
				<img src="/pause-circle.svg" alt="Pause" />
			{:else}
				<img src="/play-circle.svg" alt="Play" />
			{/if}
		</button>

		<!-- Reset Button -->
		<button class="control-button reset" on:click={onReset} aria-label="Reset">
			<img src="/refresh-ccw-01.svg" alt="Reset" />
		</button>
    <!-- Settings Button -->
    <button class="settings-button" on:click={onSettings} aria-label="Settings">
      <img src="/3-dots.svg" alt="Settings" />
    </button>
	</div>

</div>

<style>
	.controls {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		background-color: #f6dfbf;
		border-radius: 12px;
		position: relative;
	}

	.main-controls {
		display: flex;
		gap: 16px;
		margin: 0 auto;
	}

	.control-button {
		border: none;
		cursor: pointer;
		padding: 12px;
		border: 2px solid;
		border-radius: 12px;
		transition: all 0.2s ease;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		outline: none;
	}

	/* Кольори для різних режимів */
	.play-pause.work {
		background-color: #ba4325;
		border-color: #a32a0c;
	}

	.play-pause.break {
		background-color: #7F8A20;
		border-color: #5C6417;
	}

	.play-pause.relax {
		background-color: #20868A;
		border-color: #19696C;
	}

	.reset {
		background-color: #726a5e;
		border-color: #371d085f;
	}

	.control-button:hover {
		filter: brightness(1.1);
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.15);
	}

	.control-button:active {
		filter: brightness(0.9);
		box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	.control-button:focus {
		outline: none;
	}

	.control-button img {
		width: 24px;
		height: 24px;
		filter: invert(1) brightness(1);
	}

	.settings-button {
		background: none;
		border: none;
		padding: 4px;
		cursor: pointer;
		position: absolute;
		right: 4px;
		top: 0px;
		transform: translateY(-50%);
		opacity: 0.5;
		outline: none;
	}

	.settings-button:hover {
		opacity: 1;
	}

	.settings-button:focus {
		outline: none;
	}

	.settings-button img {
		width: 20px;
		height: 20px;
		filter: brightness(0.6);
	}
</style>
