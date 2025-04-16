<!-- Компонент для відображення налаштувань -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { workDuration, breakDuration, relaxDuration } from '$lib/stores/timer';
	
	// Подія для закриття меню
	export let onClose = () => {};
	
	// Обробник для натискання Escape
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}
	
	// Функції для оновлення тривалості
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
	
	// Фокусуємо меню при його відкритті
	let menuElement: HTMLDivElement;
	
	onMount(() => {
		if (menuElement) {
			menuElement.focus();
		}
	});
</script>

<!-- svelte-ignore a11y_interactive_supports_focus -->
<div 
	class="settings-backdrop" 
	on:click={onClose}
	on:keydown={handleKeydown}
	role="dialog"
	aria-modal="true"
	aria-labelledby="settings-title"
>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div 
		class="settings-menu" 
		on:click|stopPropagation
		bind:this={menuElement}
		role="document"
		tabindex="-1"
	>
		<div class="settings-header">
			<h2 id="settings-title">Налаштування</h2>
			<button 
				class="close-button" 
				on:click={onClose}
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
					<p>Версія: 0.1.0</p>
					<p>Автор: Борис Харченко</p>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
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
		background-color: rgba(255, 255, 255, 0.32);
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
		padding: 8px;
		padding-left: 12px;
		border: 1px solid #deb98e;
		border-radius: 6px;
		background-color: #fff;
		font-family: 'UbuntuMono', Courier, monospace;
		font-size: 16px;
		text-align: left;
	}
	
	/* Стилі для фокусу */
	:focus {
		outline: 2px solid #ba4325;
		outline-offset: 2px;
	}
	
	.close-button:focus {
		outline: 2px solid #ba4325;
		outline-offset: 2px;
	}
</style> 