<script lang="ts">
  export let isPlaying = false;
  export let onPlayPause: () => void = () => {};
  export let onReset: () => void = () => {};
  export let onSettings: () => void = () => {};
</script>

<div class="controls">
  <!-- Play/Pause Button -->
  <button on:click={onPlayPause} aria-label={isPlaying ? 'Pause' : 'Play'}>
    {#if isPlaying}
      <!-- Pause Icon -->
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="6" y="4" width="4" height="16"/>
        <rect x="14" y="4" width="4" height="16"/>
      </svg>
    {:else}
      <!-- Play Icon -->
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polygon points="5 3 19 12 5 21 5 3"/>
      </svg>
    {/if}
  </button>

  <!-- Reset Button -->
  <button on:click={onReset} aria-label="Reset">
    <!-- Reset Icon -->
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M3 2v6h6"/>
      <path d="M21 12A9 9 0 0 0 6 5.3L3 8"/>
      <path d="M21 22v-6h-6"/>
      <path d="M3 12a9 9 0 0 0 15 6.7l3-2.7"/>
    </svg>
  </button>

  <!-- Settings Button -->
  <button on:click={onSettings} aria-label="Settings">
    <!-- Settings Icon (Vertical Ellipsis) -->
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="1"/>
      <circle cx="12" cy="5" r="1"/>
      <circle cx="12" cy="19" r="1"/>
    </svg>
  </button>
</div>

<style>
  .controls {
    display: flex;
    justify-content: center;
    gap: 16px;
    width: 100%;
    padding: 8px 0;
  }

  button {
    background: none;
    border: none;
    cursor: pointer;
    padding: 8px;
    border-radius: 50%;
    color: #666;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
  }

  button:hover {
    background: rgba(0, 0, 0, 0.05);
    color: #333;
  }

  svg {
    width: 20px;
    height: 20px;
  }
</style>
