<script lang="ts">
    import { colorFromHue } from "$lib/hue.svelte";
    import Post from "$lib/post.svelte";

    export let posts: any;
</script>

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

<style>
    .timeline {
        width: 45%;
        display: flex;
        flex-direction: column;
    }

    .gradient {
        height: 0px;
        margin: none;
        padding: none;
    }
</style>
