<script lang="ts">
    export let following = false;
</script>

<button class="button enabled"> follow </button>

<style>
    .button {
        width: 5rem;
        height: 2.5rem;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        border-radius: 1rem;
        padding: 1rem;
        margin-bottom: 1rem;
        text-decoration: none;
        background-color: transparent;
        font-family: "Spectral", serif;
        color: white;
    }

    .button:hover {
        background-color: rgb(255, 255, 255, 0.1);
    }

    .enabled {
        border: 1px solid white;
    }
</style>
