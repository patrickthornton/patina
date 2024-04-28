<script lang="ts">
    import { onMount } from "svelte";
    import Timeline from "$lib/timeline.svelte";
    import Slider from "$lib/slider.svelte";
    import Title from "$lib/title.svelte";
    import Post from "$lib/interfaces.svelte";
    import { colorFromHue } from "$lib/hue.svelte";

    export let data;
    let author: number = data.name;

    let posts: Post[] = [];
    let starting_hue = 0;
    let current_time = Math.floor(new Date().getTime() / 1000);

    onMount(async () => {
        posts = await fetch(`http://127.0.0.1:8000/user/${author}/liked`).then(
            (r) => r.json(),
        );
    });
</script>

<body>
    <div class="page">
        <div
            class="author-header"
            style="background-color: {colorFromHue(360 - starting_hue)};"
        >
            <h2>{author}</h2>
        </div>

        <Timeline {posts} />
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

    .author-header {
        font-family: Spectral, serif;
        width: 45%;
        font-weight: bold;
        text-align: center;
    }
</style>
