<script lang="ts">
  export let completed = 0;
  export let total = 4;
</script>

<div class="progress-indicator">
  {#each { length: total } as _, i}
    <div class="dot {i < completed ? 'completed' : ''}"></div>
  {/each}
</div>

<style>
  .progress-indicator {
    display: flex;
    justify-content: center;
    gap: 8px;
    padding: 8px 0;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #e0e0e0; /* Світло-сірий для неактивних */
    transition: background-color 0.3s ease;
  }

  .dot.completed {
    background-color: #ff6b6b; /* Червоний для завершених */
  }
</style>
