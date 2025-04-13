<script lang="ts">
  export let isPlaying = false;
  export let onPlayPause: () => void = () => {};
  export let onReset: () => void = () => {};
  // Прибираємо кнопку налаштувань, її немає в дизайні
  // export let onSettings: () => void = () => {}; 
</script>

<div class="controls">
  <!-- Play/Pause Button -->
  <button class="control-button play-pause" on:click={onPlayPause} aria-label={isPlaying ? 'Pause' : 'Play'}>
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

  <!-- Прибираємо кнопку налаштувань -->
  <!-- <button on:click={onSettings} aria-label="Settings">...</button> -->
</div>

<style>
  .controls {
    display: flex;
    justify-content: center;
    gap: 20px; /* Збільшимо відступ */
    width: 100%;
    padding: 10px 0;
    background-color: #f3eade; /* Фон згідно дизайну */
    border-bottom-left-radius: 12px; /* Закруглення нижніх кутів */
    border-bottom-right-radius: 12px;
    margin-top: -8px; /* Невеликий нахлест на вкладки */
  }

  .control-button {
    background-color: #e4c9af; /* Колір кнопок */
    border: none;
    cursor: pointer;
    padding: 12px;
    border-radius: 10px;
    color: #6f4e37; /* Колір іконок */
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 60px; /* Фіксована ширина */
    height: 45px; /* Фіксована висота */
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .control-button:hover {
    background-color: #d8b89f; /* Світліший при наведенні */
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.15);
  }

  .control-button:active {
    background-color: #cba78f; /* Темніший при натисканні */
    box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .control-button img {
    width: 24px; /* Розмір іконок */
    height: 24px;
  }
  
  .hidden {
      display: none;
  }
</style>
