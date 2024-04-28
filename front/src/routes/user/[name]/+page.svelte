<script lang="ts">
    import Timeline from "$lib/timeline.svelte";
    import Slider from "$lib/slider.svelte";
    import Title from "$lib/title.svelte";
    import { colorFromHue } from "$lib/hue.svelte";
    export let data: any;

    let posts = data.posts;
    let starting_hue = 0;

    $: {
        posts.sort(
            (a: any, b: any) =>
                ((a.hue + starting_hue) % 360) - ((b.hue + starting_hue) % 360),
        );
        posts = posts;
    }
</script>

<body>
    <div class="page">
        <div
            class="author-header"
            style="background-color: {colorFromHue(360 - starting_hue)};"
        >
            <h2>{data.author}</h2>
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
