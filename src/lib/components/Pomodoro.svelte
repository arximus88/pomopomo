<script lang="ts">
  // Визначаємо можливі стани помідора
  export let state: 'work' | 'break' | 'relax' = 'work';
</script>

<div class="pomodoro {state}">
  <!-- Тут буде SVG або анімація помідора -->
  <div class="placeholder-tomato">🍅</div>
</div>

<style>
  .pomodoro {
    width: 120px;
    height: 120px;
    display: flex;
    align-items: center;
    justify-content: center;
    /* Додамо трохи перспективи для анімацій */
    perspective: 400px;
  }

  .placeholder-tomato {
    font-size: 80px;
    transition: transform 0.5s ease-out;
  }

  /* Анімації для різних станів */
  .work .placeholder-tomato {
    animation: work-pulse 1.5s infinite alternate;
  }

  .break .placeholder-tomato {
    animation: break-spin 2s linear infinite;
  }

  .relax .placeholder-tomato {
    animation: relax-float 4s ease-in-out infinite;
  }

  @keyframes work-pulse {
    from { transform: scale(1); }
    to { transform: scale(1.05); }
  }

  @keyframes break-spin {
    from { transform: rotateY(0deg); }
    to { transform: rotateY(360deg); }
  }

  @keyframes relax-float {
    0% { transform: translateY(0px) rotate(-2deg); }
    50% { transform: translateY(-10px) rotate(2deg); }
    100% { transform: translateY(0px) rotate(-2deg); }
  }
</style>
