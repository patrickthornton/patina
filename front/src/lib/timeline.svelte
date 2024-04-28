<script lang="ts">
    import { colorFromHue, now, hueFromTime } from "$lib/hue.svelte";
    import Post from "$lib/post.svelte";

    export let posts: any;

    // here starts the color nonsense
    var current_time = now();
    setInterval(() => {
        current_time = now();
    }, 200);

    $: starting_hue = hueFromTime(current_time);
    $: {
        posts.sort(
            (a, b) =>
                ((a.hue + starting_hue) % 360) - ((b.hue + starting_hue) % 360),
        );
        posts = posts;
    }
</script>

<div class="timeline">
    {#each posts as post, i}
        <div class="post">
            <Post {post} />
        </div>
    {/each}
</div>

<style>
    .timeline {
        width: 100%;
        display: flex;
        flex-direction: column;
    }
</style>
