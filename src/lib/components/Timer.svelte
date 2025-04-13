<script lang="ts">
  export let minutes = 24;
  export let seconds = 59;

  // Функція для форматування часу з ведучими нулями
  const formatTime = (time: number) => time.toString().padStart(2, '0');
</script>

<div class="timer-container">
  <div class="timer-display">
    {formatTime(minutes)}:{formatTime(seconds)}
  </div>
  <div class="timer-label">Вйо працювати!</div>
  <div class="timer-post"></div>
</div>

<style>
  .timer-container {
    position: relative;
    display: inline-block;
    margin-top: 10px;
  }

  .timer-display {
    background-color: #3a3a3a;
    color: #ff6b6b;
    font-family: 'Courier New', Courier, monospace;
    font-size: 36px;
    font-weight: bold;
    padding: 5px 15px;
    border-radius: 8px;
    border: 2px solid #2c2c2c;
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.2), 0 2px 2px rgba(255, 255, 255, 0.2);
    z-index: 2;
    position: relative;
  }

  .timer-label {
    position: absolute;
    bottom: -18px;
    left: 50%;
    transform: translateX(-50%);
    font-size: 10px;
    color: #4a4a4a;
    white-space: nowrap;
    background-color: #e4c9af;
    padding: 1px 4px;
    border-radius: 4px;
    z-index: 3;
  }

  .timer-post {
    position: absolute;
    bottom: -25px;
    left: 50%;
    transform: translateX(-50%);
    width: 8px;
    height: 25px;
    background-color: #8b5e3c;
    border-radius: 2px;
    z-index: 1;
  }
</style>
