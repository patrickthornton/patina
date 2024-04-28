<script lang="ts">
    import posts_array from "../assets/posts.json";
    import Timeline from "$lib/timeline.svelte";
    import Slider from "$lib/slider.svelte";
    import Title from "$lib/title.svelte";

    let posts = posts_array;

    let starting_hue = 0;

    $: {
        posts.sort(
            (a, b) =>
                ((a.hue + starting_hue) % 360) - ((b.hue + starting_hue) % 360),
        );
        posts = posts;
    }
</script>

<body>
    <div class="page">
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
</style>
