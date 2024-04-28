<script lang="ts">
    import { onMount } from "svelte";
    import Slider from "$lib/slider.svelte";
    import Title from "$lib/title.svelte";

    let starting_hue = 0;
    let current_time = Math.floor(new Date().getTime() / 1000);
</script>

<body>
    <div class="page">
        <slot />
    </div>

    <Slider bind:starting_hue />
    <Title />
</body>

<style>
    body {
        margin: 0;
        padding: 0;
        font-family: Helvetica, sans-serif;
        color: white;
        background-color: #222;
    }

    .page {
        width: 100vw;
        margin: 0;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
</style>
