<!-- Компонент для відображення пана помідора -->
<script lang="ts">
  // Визначаємо можливі стани помідора
  export let state: 'work' | 'break' | 'relax' = 'work';
  // TODO: Додати логіку для різних SVG помідорів залежно від стану
</script>

<div class="pomodoro {state}">
  {#if state === 'work'}
    <img src="/vector-pomodoro-work.svg" alt="Pomodoro Work" class="pomodoro-icon" />
  {:else if state === 'break'}
    <img src="/vector-pomodoro-pause.svg" alt="Pomodoro Break" class="pomodoro-icon" />
  {:else if state === 'relax'}
    <img src="/vector-pomodoro-relax-more.svg" alt="Pomodoro Relax" class="pomodoro-icon" />
  {/if}
</div>

<style>
  .pomodoro {
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    z-index: 1;
  }

  .pomodoro-icon {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  /* Приберемо старі анімації для placeholder */
  /* .work .placeholder-tomato {...} */
  /* .break .placeholder-tomato {...} */
  /* .relax .placeholder-tomato {...} */
</style>
