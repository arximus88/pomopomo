<script lang="ts">
  export let minutes = 24;
  export let seconds = 59;

  // Функція для форматування часу з ведучими нулями
  const formatTime = (time: number) => time.toString().padStart(2, '0');
</script>

<div class="timer">
  {formatTime(minutes)}:{formatTime(seconds)}
</div>

<style>
  .timer {
    font-size: 48px;
    font-weight: bold;
    color: #333;
    text-align: center;
    font-variant-numeric: tabular-nums;
  }
</style>
