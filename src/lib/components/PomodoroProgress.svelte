<script lang="ts">
  import { completedPomodoros } from '$lib/stores/timer';

  // Для відладки покажемо 3 помідори
  let debugCount = 3; 

  // Іконка помідора (можна використати SVG або emoji)
  const tomatoIcon = '🍅';
</script>

<div class="progress-container">
  {#each Array($completedPomodoros) as _, i}
    <span class="pomodoro-icon">{tomatoIcon}</span>
  {/each}
</div>

<style>
  .progress-container {
    display: flex;
    gap: 4px; /* Відстань між помідорами */
    padding-bottom: 5px; /* Щоб вирівняти по низу з таймером */
    align-items: flex-end;
    height: 100%; /* Займаємо всю висоту .progress-area */
  }

  .pomodoro-icon {
    font-size: 16px; /* Розмір іконки-помідора */
    line-height: 1; /* Щоб вирівняти */
    /* Додаткові стилі, якщо потрібно */
  }
</style> 