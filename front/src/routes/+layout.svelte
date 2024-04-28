<script lang="ts">
    import { onMount } from "svelte";
    import Title from "$lib/title.svelte";
    import { now } from "$lib/hue.svelte";
</script>

<body>
    <div class="page">
        <slot />
    </div>

    <Title />
</body>

<style>
    @import url("https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Spectral:ital,wght@0,200;0,300;0,400;0,500;0,600;0,700;0,800;1,200;1,300;1,400;1,500;1,600;1,700;1,800&display=swap");

    body {
        margin: 0;
        padding: 0;
        font-family: "Spectral", serif;
        color: white;
        background-color: #222;
    }

    .page {
        width: 100vw;
        height: 100vh;
        margin: 0;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
    }
</style>
