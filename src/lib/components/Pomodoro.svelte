<script lang="ts">
  // Визначаємо можливі стани помідора
  export let state: 'work' | 'break' | 'relax' = 'work';
  // TODO: Додати логіку для різних SVG помідорів залежно від стану
</script>

<div class="pomodoro {state}">
  <img src="/vector-pomodoro.svg" alt="Pomodoro" class="pomodoro-icon" />
</div>

<style>
  .pomodoro {
    width: 80px; /* Зменшимо трохи розмір */
    height: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    /* margin-bottom: -20px; */ /* Прибираємо */
    position: relative;
    z-index: 1;
  }

  .pomodoro-icon {
    width: 100%;
    height: auto;
    /* TODO: Додати стилі анімацій згідно дизайну */
  }

  /* Приберемо старі анімації для placeholder */
  /* .work .placeholder-tomato {...} */
  /* .break .placeholder-tomato {...} */
  /* .relax .placeholder-tomato {...} */
</style>
