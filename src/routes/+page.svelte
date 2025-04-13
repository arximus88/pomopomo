<script lang="ts">
  import Widget from '$lib/components/Widget.svelte';
</script>

<div class="container">
  <Widget />
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
    font-family: system-ui, -apple-system, sans-serif;
    overflow: hidden; /* Забороняємо скрол */
  }

  .container {
    width: 100vw;
    height: 100vh;
    display: flex;
    /* Не центруємо, віджет сам займає всю ширину */
    /* justify-content: center; */
    /* align-items: center; */
    background: transparent;
    padding: 0; /* Забираємо зайві відступи, якщо є */
  }
</style>
