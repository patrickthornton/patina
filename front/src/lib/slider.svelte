<script lang="ts">
    import { colorFromHue } from "$lib/hue.svelte";

    export let hue: number = 0;
</script>

<div class="slider" style="background-color: {colorFromHue(hue)};">
    <input type="range" min="0" max="360" bind:value={hue} />

    current hue: {hue}
</div>

<style>
    .slider {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        color: white;
        border-radius: 0.5rem;
        padding: 1rem;
    }
</style>
