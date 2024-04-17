<script lang="ts">
    import Post from "$lib/post.svelte";
    import posts_array from "../assets/posts.json";
    import { colorFromHue } from "$lib/hue.svelte";

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
        <div class="timeline">
            {#each posts as post, i}
                <div class="post">
                    <Post {post} />
                </div>
                {#if i < posts.length - 1}
                    <div
                        class="gradient"
                        style="
                        background: linear-gradient({colorFromHue(
                            posts[i].hue,
                        )}, {colorFromHue(posts[i + 1].hue)});
                    "
                    ></div>
                {/if}
            {/each}
        </div>
    </div>

    <div class="slider">
        <input type="range" min="0" max="360" bind:value={starting_hue} />

        {starting_hue}
    </div>

    <div class="title">
        <h1>patina</h1>
    </div>
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

    .timeline {
        width: 45%;
        display: flex;
        flex-direction: column;
    }

    .gradient {
        height: 40px;
        margin: none;
        padding: none;
    }

    .slider {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        position: fixed;
        right: 13%;
        top: 50%;
    }

    .title {
        position: fixed;
        right: 2%;
        top: 0.7%;
    }
</style>
