<script lang="ts">
    import { goto } from "$app/navigation";

    export let author: string = "";

    function followees() {
        goto(`/user/${author}/followees`);
    }
</script>

<button class="button enabled" on:click={followees}> followees </button>

<style>
    .button {
        width: 6rem;
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
        font-weight: normal;
    }

    .button:hover {
        background-color: rgb(255, 255, 255, 0.1);
    }

    .enabled {
        border: 1px solid white;
        color: white;
    }

    .disabled {
        border: 1px solid #aaa;
        color: #aaa;
    }
</style>
