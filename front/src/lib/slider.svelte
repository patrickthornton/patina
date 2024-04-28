<script lang="ts">
    import { colorFromHue } from "$lib/hue.svelte";

    export let starting_hue: number;
</script>

<div
    class="slider"
    style="background-color: {colorFromHue(360 - starting_hue)};"
>
    <input type="range" min="0" max="360" bind:value={starting_hue} />

    current hue: {starting_hue}
</div>

<style>
    .slider {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        position: fixed;
        right: 13%;
        top: 50%;
        color: white;
        border-radius: 0.5rem;
        padding: 1rem;
    }
</style>
